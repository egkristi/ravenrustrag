//! Search pipeline and retrieval orchestrator for RavenRustRAG.
//!
//! Provides `DocumentIndex` (split, embed, store, search), hybrid BM25+vector search,
//! parent-child retrieval, multi-collection routing, and evaluation metrics.

use raven_core::{Chunk, Document, RavenError, Result, SearchResult};
use raven_embed::Embedder;
use raven_split::Splitter;
use raven_store::{MetadataFilter, VectorStore};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub mod bm25;
pub use bm25::{reciprocal_rank_fusion, Bm25Index};

pub mod eval;
pub use eval::{
    eval_summary, evaluate_batch, mrr, ndcg_at_k, precision_at_k, recall_at_k, EvalResult,
};

pub mod semantic_split;
pub use semantic_split::SemanticSplitter;

pub mod multi_query;
pub use multi_query::expand_query;

pub mod graph;
pub use graph::{
    extract_entities, graph_vector_fusion, Entity, GraphRetriever, KnowledgeGraph, Relation,
};

pub mod rerank;
#[cfg(feature = "onnx")]
pub use rerank::OnnxReranker;
pub use rerank::{rerank, KeywordReranker, Reranker};

/// Main document index — the heart of RavenRustRAG
pub struct DocumentIndex {
    store: Arc<dyn VectorStore>,
    embedder: Arc<dyn Embedder>,
    bm25: RwLock<Bm25Index>,
    embed_batch_size: usize,
    store_batch_size: usize,
}

impl DocumentIndex {
    pub fn new(store: Arc<dyn VectorStore>, embedder: Arc<dyn Embedder>) -> Self {
        Self {
            store,
            embedder,
            bm25: RwLock::new(Bm25Index::new()),
            embed_batch_size: 64,
            store_batch_size: 100,
        }
    }

    pub fn builder() -> DocumentIndexBuilder {
        DocumentIndexBuilder::default()
    }

    /// Add documents (chunks must already have embeddings)
    pub async fn add_chunks(&self, chunks: &[Chunk]) -> Result<()> {
        let before = self.bm25.read().await.count();
        self.bm25.write().await.add(chunks);

        // Persist BM25 terms for new chunks
        let term_data = self.bm25.read().await.get_term_data(before);
        for (chunk_id, terms, doc_length) in &term_data {
            self.store
                .save_bm25_terms(chunk_id, terms, *doc_length)
                .await
                .ok(); // Best-effort persistence
        }

        self.store.add(chunks).await
    }

    /// Add raw documents: split, embed, store
    #[tracing::instrument(skip_all, fields(num_documents = documents.len()))]
    pub async fn add_documents(
        &self,
        documents: Vec<Document>,
        splitter: &dyn Splitter,
    ) -> Result<()> {
        let chunks = {
            let _span = tracing::info_span!("split", num_documents = documents.len()).entered();
            splitter.split(documents)
        };

        if chunks.is_empty() {
            return Ok(());
        }

        let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
        info!("Split into {} chunks", texts.len());

        // Embed in batches
        info!(num_chunks = texts.len(), "Embedding chunks");

        // Embed in parallel batches (up to 4 concurrent batches)
        let all_embeddings = if texts.len() <= self.embed_batch_size {
            self.embedder.embed(&texts).await?
        } else {
            let batches: Vec<Vec<String>> = texts
                .chunks(self.embed_batch_size)
                .map(<[String]>::to_vec)
                .collect();

            let mut handles = Vec::with_capacity(batches.len());
            let semaphore = Arc::new(tokio::sync::Semaphore::new(4));

            for batch in batches {
                let embedder = Arc::clone(&self.embedder);
                let sem = Arc::clone(&semaphore);
                handles.push(tokio::spawn(async move {
                    let _permit = sem
                        .acquire()
                        .await
                        .map_err(|e| RavenError::Embed(format!("Semaphore closed: {e}")))?;
                    embedder.embed(&batch).await
                }));
            }

            let mut all = Vec::with_capacity(texts.len());
            for handle in handles {
                let batch_result = handle
                    .await
                    .map_err(|e| raven_core::RavenError::Embed(format!("Task join error: {e}")))?;
                all.extend(batch_result?);
            }
            all
        };

        // Attach embeddings to chunks
        let embedded_chunks: Vec<Chunk> = chunks
            .into_iter()
            .zip(all_embeddings)
            .map(|(mut chunk, embedding)| {
                chunk.embedding = Some(embedding);
                chunk
            })
            .collect();

        // Store in batches
        info!(num_chunks = embedded_chunks.len(), "Storing chunks");
        for batch in embedded_chunks.chunks(self.store_batch_size) {
            self.store.add(batch).await?;
        }

        // Record embedding metadata (model + dimensions) for versioning
        let model_name = self.embedder.model_name();
        let dimensions = self.embedder.dimension();
        if let Ok(existing) = self.store.get_embedding_metadata().await {
            match existing {
                None => {
                    self.store
                        .set_embedding_metadata(model_name, dimensions)
                        .await
                        .ok();
                }
                Some((ref stored_model, stored_dims)) => {
                    if stored_dims != dimensions {
                        return Err(RavenError::Config(format!(
                            "Embedding dimension mismatch: index was created with model '{stored_model}' ({stored_dims} dims), \
                             but current embedder '{model_name}' produces {dimensions} dims. \
                             Run `raven clear` and re-index to switch models."
                        )));
                    }
                    if stored_model != model_name {
                        warn!(
                            "Embedding model changed from '{stored_model}' to '{model_name}' (same dimensions: {dimensions}). \
                             Results may be degraded — consider re-indexing."
                        );
                        self.store
                            .set_embedding_metadata(model_name, dimensions)
                            .await
                            .ok();
                    }
                }
            }
        }

        // Add to BM25 index and persist
        let before = self.bm25.read().await.count();
        self.bm25.write().await.add(&embedded_chunks);

        let term_data = self.bm25.read().await.get_term_data(before);
        for (chunk_id, terms, doc_length) in &term_data {
            self.store
                .save_bm25_terms(chunk_id, terms, *doc_length)
                .await
                .ok();
        }

        Ok(())
    }

    /// Query the index
    #[tracing::instrument(skip(self), fields(top_k))]
    pub async fn query(&self, query_text: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embedder.embed(&[query_text.to_string()]).await?;
        let embedding = query_embedding
            .into_iter()
            .next()
            .ok_or_else(|| raven_core::RavenError::NotFound("Empty embedding".to_string()))?;

        self.store.search(&embedding, top_k).await
    }

    /// Query with metadata filtering
    #[tracing::instrument(skip(self), fields(top_k))]
    pub async fn query_filtered(
        &self,
        query_text: &str,
        top_k: usize,
        filter: &MetadataFilter,
    ) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embedder.embed(&[query_text.to_string()]).await?;
        let embedding = query_embedding
            .into_iter()
            .next()
            .ok_or_else(|| raven_core::RavenError::NotFound("Empty embedding".to_string()))?;

        self.store.search_filtered(&embedding, top_k, filter).await
    }

    /// Hybrid query: combine vector search and BM25 with Reciprocal Rank Fusion.
    /// `alpha` controls the blend: 1.0 = pure vector, 0.0 = pure BM25.
    #[tracing::instrument(skip(self), fields(top_k, alpha))]
    pub async fn query_hybrid(
        &self,
        query_text: &str,
        top_k: usize,
        alpha: f32,
    ) -> Result<Vec<SearchResult>> {
        // Fetch more than top_k from each source for better fusion
        let fetch_k = top_k * 3;

        let vector_results = self.query(query_text, fetch_k).await?;
        let bm25_results = self.bm25.read().await.search(query_text, fetch_k);

        Ok(reciprocal_rank_fusion(
            &vector_results,
            &bm25_results,
            alpha,
            top_k,
        ))
    }

    /// Format results as LLM prompt with citations
    pub async fn query_for_prompt(&self, query_text: &str, top_k: usize) -> Result<String> {
        let results = self.query(query_text, top_k).await?;
        Ok(format_prompt(query_text, &results, None))
    }

    /// Format results with a custom template
    pub async fn query_for_prompt_with_template(
        &self,
        query_text: &str,
        top_k: usize,
        template: &str,
    ) -> Result<String> {
        let results = self.query(query_text, top_k).await?;
        Ok(format_prompt(query_text, &results, Some(template)))
    }

    pub async fn count(&self) -> Result<usize> {
        self.store.count().await
    }

    /// Parent-child retrieval: search chunks, then group by parent document
    /// and return all sibling chunks for each matching parent.
    /// This gives full document context instead of isolated chunks.
    #[tracing::instrument(skip(self), fields(top_k))]
    pub async fn query_parent(&self, query_text: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        // Search more chunks to find diverse parents
        let results = self.query(query_text, top_k * 3).await?;

        // Collect unique parent doc_ids (preserving order by best score)
        let mut seen_parents = std::collections::HashSet::new();
        let mut parent_ids = Vec::new();
        for r in &results {
            let parent_id = r
                .chunk
                .metadata
                .get("source_id")
                .cloned()
                .unwrap_or_else(|| r.chunk.doc_id.clone());
            if seen_parents.insert(parent_id.clone()) {
                parent_ids.push(parent_id);
            }
            if parent_ids.len() >= top_k {
                break;
            }
        }

        // For each parent, fetch only its chunks from the store
        let mut parent_results = Vec::new();

        for parent_id in &parent_ids {
            // Find the best scoring chunk for this parent (for the score)
            let best_result = results.iter().find(|r| {
                let pid = r.chunk.metadata.get("source_id").unwrap_or(&r.chunk.doc_id);
                pid == parent_id
            });

            // Fetch only chunks belonging to this parent
            let sibling_chunks_raw = self.store.get_by_doc_id(parent_id).await?;
            let mut sibling_chunks: Vec<_> = sibling_chunks_raw.iter().collect();
            sibling_chunks.sort_by_key(|c| {
                c.metadata
                    .get("chunk_index")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0)
            });

            // Merge sibling chunks into a single result
            if let Some(best) = best_result {
                let merged_text = sibling_chunks
                    .iter()
                    .map(|c| c.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n");

                let mut merged_chunk = raven_core::Chunk::new(&best.chunk.doc_id, &merged_text);
                merged_chunk.metadata.clone_from(&best.chunk.metadata);
                merged_chunk
                    .metadata
                    .insert("retrieval_mode".to_string(), "parent".to_string());
                merged_chunk
                    .metadata
                    .insert("child_chunks".to_string(), sibling_chunks.len().to_string());

                parent_results.push(SearchResult {
                    chunk: merged_chunk,
                    score: best.score,
                    distance: best.distance,
                });
            }
        }

        Ok(parent_results)
    }

    pub async fn delete(&self, doc_id: &str) -> Result<()> {
        // Incrementally remove from in-memory BM25 index
        self.bm25.write().await.remove_by_doc_id(doc_id);
        self.store.delete_bm25_terms(doc_id).await.ok();
        self.store.delete(doc_id).await
    }

    pub async fn clear(&self) -> Result<()> {
        self.bm25.write().await.clear();
        self.store.clear_bm25().await.ok();
        self.store.clear().await
    }

    pub fn embedder(&self) -> &Arc<dyn Embedder> {
        &self.embedder
    }

    pub fn store(&self) -> &Arc<dyn VectorStore> {
        &self.store
    }

    /// Load persisted BM25 data from the store.
    /// Call this after creating a DocumentIndex to restore BM25 state.
    pub async fn load_bm25_from_store(&self) -> Result<usize> {
        let data = self.store.load_bm25_data().await?;
        let count = data.len();
        if count > 0 {
            self.bm25.write().await.load_from_stored(&data);
            info!(num_terms = count, "Loaded BM25 index from store");
        }
        Ok(count)
    }

    /// Stream query results one at a time via a channel.
    /// Returns a receiver that yields results as they are found.
    pub async fn query_stream(
        &self,
        query_text: &str,
        top_k: usize,
    ) -> Result<tokio::sync::mpsc::Receiver<SearchResult>> {
        let results = self.query(query_text, top_k).await?;
        let (tx, rx) = tokio::sync::mpsc::channel(top_k.max(1));

        tokio::spawn(async move {
            for result in results {
                if tx.send(result).await.is_err() {
                    break;
                }
            }
        });

        Ok(rx)
    }
}

// =============================================================================
// Context formatting
// =============================================================================

pub fn format_prompt(query: &str, results: &[SearchResult], template: Option<&str>) -> String {
    let sources: Vec<String> = results
        .iter()
        .map(|r| {
            r.chunk
                .metadata
                .get("source")
                .cloned()
                .unwrap_or_else(|| r.chunk.doc_id.clone())
        })
        .collect();

    if let Some(tmpl) = template {
        let context = results
            .iter()
            .enumerate()
            .map(|(i, r)| format!("[{}] {}", i + 1, r.chunk.text))
            .collect::<Vec<_>>()
            .join("\n\n");
        tmpl.replace("{context}", &context)
            .replace("{query}", query)
            .replace("{sources}", &sources.join(", "))
    } else {
        use std::fmt::Write;
        let mut prompt = format!("Query: {query}\n\nContext:\n");
        for (i, result) in results.iter().enumerate() {
            let source = result
                .chunk
                .metadata
                .get("source")
                .unwrap_or(&result.chunk.doc_id);
            let _ = write!(
                prompt,
                "\n[{}] Source: {}\n{}\n",
                i + 1,
                source,
                result.chunk.text
            );
        }
        prompt.push_str("\n---\nAnswer the query using the provided context.");
        prompt
    }
}

// =============================================================================
// Multi-collection routing
// =============================================================================

/// Routes queries across multiple `DocumentIndex` instances and fuses results.
pub struct MultiCollectionRouter {
    collections: Vec<(String, Arc<DocumentIndex>)>,
}

impl MultiCollectionRouter {
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
        }
    }

    /// Add a named collection (index) to the router.
    pub fn add(&mut self, name: impl Into<String>, index: Arc<DocumentIndex>) {
        self.collections.push((name.into(), index));
    }

    /// Query all collections and fuse results by score, returning top_k.
    pub async fn query(&self, query_text: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        let mut all_results = Vec::new();

        for (name, index) in &self.collections {
            match index.query(query_text, top_k).await {
                Ok(mut results) => {
                    // Tag results with their collection name
                    for r in &mut results {
                        r.chunk
                            .metadata
                            .insert("collection".to_string(), name.clone());
                    }
                    all_results.extend(results);
                }
                Err(e) => {
                    warn!("Query failed for collection '{name}': {e}");
                }
            }
        }

        // Sort by score descending and take top_k
        all_results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        all_results.truncate(top_k);
        Ok(all_results)
    }

    /// List available collections with chunk counts.
    pub async fn collections(&self) -> Vec<(String, usize)> {
        let mut result = Vec::new();
        for (name, index) in &self.collections {
            let count = index.count().await.unwrap_or(0);
            result.push((name.clone(), count));
        }
        result
    }
}

impl Default for MultiCollectionRouter {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Builder
// =============================================================================

#[derive(Default)]
pub struct DocumentIndexBuilder {
    store: Option<Arc<dyn VectorStore>>,
    embedder: Option<Arc<dyn Embedder>>,
    embed_batch_size: Option<usize>,
    store_batch_size: Option<usize>,
}

impl DocumentIndexBuilder {
    pub fn store(mut self, store: Arc<dyn VectorStore>) -> Self {
        self.store = Some(store);
        self
    }

    pub fn embedder(mut self, embedder: Arc<dyn Embedder>) -> Self {
        self.embedder = Some(embedder);
        self
    }

    pub fn embed_batch_size(mut self, size: usize) -> Self {
        self.embed_batch_size = Some(size);
        self
    }

    pub fn store_batch_size(mut self, size: usize) -> Self {
        self.store_batch_size = Some(size);
        self
    }

    pub fn build(self) -> Result<DocumentIndex> {
        let store = self
            .store
            .ok_or_else(|| raven_core::RavenError::Config("Store not configured".to_string()))?;
        let embedder = self
            .embedder
            .ok_or_else(|| raven_core::RavenError::Config("Embedder not configured".to_string()))?;

        let mut index = DocumentIndex::new(store, embedder);
        if let Some(size) = self.embed_batch_size {
            index.embed_batch_size = size;
        }
        if let Some(size) = self.store_batch_size {
            index.store_batch_size = size;
        }
        Ok(index)
    }
}

// =============================================================================
// File watcher
// =============================================================================

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};

/// Watch a directory and auto-index changed files with debounce.
pub async fn watch_directory(
    index: Arc<DocumentIndex>,
    store: Arc<dyn VectorStore>,
    splitter: Arc<dyn Splitter>,
    watch_path: &Path,
    extensions: &[&str],
    debounce_ms: u64,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (tx, mut rx) = mpsc::channel::<Event>(256);

    let mut watcher = RecommendedWatcher::new(
        move |result: std::result::Result<Event, notify::Error>| {
            if let Ok(event) = result {
                let _ = tx.blocking_send(event);
            }
        },
        notify::Config::default(),
    )?;

    watcher.watch(watch_path, RecursiveMode::Recursive)?;
    info!("Watching {:?} for changes...", watch_path);

    let ext_set: HashSet<String> = extensions
        .iter()
        .map(|e| e.trim_start_matches('.').to_string())
        .collect();

    let debounce = Duration::from_millis(debounce_ms);
    let mut pending: HashSet<std::path::PathBuf> = HashSet::new();
    let mut last_event = Instant::now();

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        for path in &event.paths {
                            if path.is_file() {
                                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                    if ext_set.contains(ext) {
                                        pending.insert(path.clone());
                                        last_event = Instant::now();
                                    }
                                }
                            }
                        }
                    }
                    EventKind::Remove(_) => {
                        for path in &event.paths {
                            let path_str = path.to_string_lossy().to_string();
                            info!("File removed: {}", path_str);
                            if let Err(e) = store.delete(&path_str).await {
                                warn!("Failed to delete chunks for {}: {}", path_str, e);
                            }
                            if let Err(e) = store.delete_fingerprint(&path_str).await {
                                warn!("Failed to delete fingerprint for {}: {}", path_str, e);
                            }
                            pending.remove(path);
                        }
                    }
                    _ => {}
                }
            }
            () = tokio::time::sleep(Duration::from_millis(100)) => {
                if !pending.is_empty() && last_event.elapsed() >= debounce {
                    let files: Vec<_> = pending.drain().collect();
                    for file_path in files {
                        let path_str = file_path.to_string_lossy().to_string();

                        let content = match std::fs::read_to_string(&file_path) {
                            Ok(c) => c,
                            Err(e) => {
                                warn!("Failed to read {}: {}", path_str, e);
                                continue;
                            }
                        };

                        let hash = raven_core::fingerprint(&content);

                        // Check fingerprint
                        if let Ok(Some(existing)) = store.get_fingerprint(&path_str).await {
                            if existing == hash {
                                continue;
                            }
                            store.delete(&path_str).await.ok();
                        }

                        match raven_load::Loader::from_file(&file_path) {
                            Ok(doc) => {
                                let doc = doc.with_metadata("source_path", &path_str);
                                if let Err(e) = index.add_documents(vec![doc], splitter.as_ref()).await {
                                    warn!("Failed to index {}: {}", path_str, e);
                                } else {
                                    store.set_fingerprint(&path_str, &hash).await.ok();
                                    info!("Re-indexed: {}", path_str);
                                }
                            }
                            Err(e) => warn!("Failed to load {}: {}", path_str, e),
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raven_embed::DummyEmbedder;
    use raven_split::TextSplitter;
    use raven_store::MemoryStore;

    #[tokio::test]
    async fn test_document_index() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(100, 10);
        let docs = vec![Document::new(
            "This is a test document about Rust programming.",
        )];

        index.add_documents(docs, &splitter).await.unwrap();
        assert_eq!(index.count().await.unwrap(), 1);

        let results = index.query("Rust", 5).await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_query_for_prompt() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let docs = vec![Document::new("RAG is retrieval-augmented generation.")
            .with_metadata("source", "rag.md")];

        index.add_documents(docs, &splitter).await.unwrap();

        let prompt = index.query_for_prompt("What is RAG?", 3).await.unwrap();
        assert!(prompt.contains("What is RAG?"));
        assert!(prompt.contains("retrieval-augmented generation"));
        assert!(prompt.contains("rag.md"));
    }

    #[tokio::test]
    async fn test_builder() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::default());

        let index = DocumentIndex::builder()
            .store(store)
            .embedder(embedder)
            .build()
            .unwrap();

        assert_eq!(index.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_hybrid_query() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let docs = vec![
            Document::new("Rust programming is fast and memory safe"),
            Document::new("Python is great for machine learning"),
            Document::new("JavaScript runs in the browser"),
        ];

        index.add_documents(docs, &splitter).await.unwrap();
        assert_eq!(index.count().await.unwrap(), 3);

        // Hybrid query should return results
        let results = index
            .query_hybrid("Rust programming", 3, 0.5)
            .await
            .unwrap();
        assert!(!results.is_empty());
    }

    #[tokio::test]
    async fn test_query_parent() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        // Use a small chunk size so the document gets split into multiple chunks
        let splitter = TextSplitter::new(30, 5);
        let docs = vec![Document::new(
            "Rust is fast. Rust is safe. Rust is concurrent. Rust is awesome.",
        )];

        index.add_documents(docs, &splitter).await.unwrap();
        let chunk_count = index.count().await.unwrap();
        assert!(
            chunk_count > 1,
            "Document should be split into multiple chunks"
        );

        // Parent query should return merged results
        let results = index.query_parent("Rust", 2).await.unwrap();
        assert!(!results.is_empty());
        // The merged text should be longer than individual chunks
        assert!(results[0].chunk.text.contains("Rust"));
        assert_eq!(
            results[0]
                .chunk
                .metadata
                .get("retrieval_mode")
                .map(String::as_str),
            Some("parent")
        );
    }

    #[tokio::test]
    async fn test_multi_collection_router() {
        let store1 = Arc::new(MemoryStore::new());
        let embedder1 = Arc::new(DummyEmbedder::new(3));
        let index1 = Arc::new(DocumentIndex::new(store1, embedder1));

        let store2 = Arc::new(MemoryStore::new());
        let embedder2 = Arc::new(DummyEmbedder::new(3));
        let index2 = Arc::new(DocumentIndex::new(store2, embedder2));

        let splitter = TextSplitter::new(200, 10);
        index1
            .add_documents(vec![Document::new("Rust is fast")], &splitter)
            .await
            .unwrap();
        index2
            .add_documents(vec![Document::new("Python is slow")], &splitter)
            .await
            .unwrap();

        let mut router = MultiCollectionRouter::new();
        router.add("rust-docs", index1);
        router.add("python-docs", index2);

        let results = router.query("programming", 5).await.unwrap();
        assert_eq!(results.len(), 2);
        // Each result should have a collection tag
        assert!(results
            .iter()
            .all(|r| r.chunk.metadata.contains_key("collection")));

        let collections = router.collections().await;
        assert_eq!(collections.len(), 2);
    }

    #[tokio::test]
    async fn test_query_stream() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let docs = vec![Document::new("Rust is fast"), Document::new("Rust is safe")];
        index.add_documents(docs, &splitter).await.unwrap();

        let mut rx = index.query_stream("Rust", 5).await.unwrap();
        let mut count = 0;
        while let Some(_result) = rx.recv().await {
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_query_no_results() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        // Empty index should return empty results
        let results = index.query("anything", 5).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_query_single_document() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let docs = vec![Document::new("The only document")];
        index.add_documents(docs, &splitter).await.unwrap();

        let results = index.query("document", 10).await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_then_query() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let doc = Document::new("Test document").with_id("test-doc-1");
        index.add_documents(vec![doc], &splitter).await.unwrap();
        assert_eq!(index.count().await.unwrap(), 1);

        index.delete("test-doc-1").await.unwrap();
        assert_eq!(index.count().await.unwrap(), 0);

        let results = index.query("test", 5).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_clear_resets_everything() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let docs = vec![Document::new("A"), Document::new("B"), Document::new("C")];
        index.add_documents(docs, &splitter).await.unwrap();
        assert_eq!(index.count().await.unwrap(), 3);

        index.clear().await.unwrap();
        assert_eq!(index.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_query_filtered() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));
        let index = DocumentIndex::new(store, embedder);

        let splitter = TextSplitter::new(200, 10);
        let docs = vec![
            Document::new("English greeting hello").with_metadata("lang", "en"),
            Document::new("French greeting bonjour").with_metadata("lang", "fr"),
            Document::new("English farewell goodbye").with_metadata("lang", "en"),
        ];
        index.add_documents(docs, &splitter).await.unwrap();

        let filter = raven_store::MetadataFilter::new().with("lang", "en");
        let results = index.query_filtered("greeting", 10, &filter).await.unwrap();
        assert_eq!(results.len(), 2);
        for r in &results {
            assert_eq!(r.chunk.metadata.get("lang").unwrap(), "en");
        }
    }

    #[tokio::test]
    async fn test_builder_with_batch_sizes() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(3));

        let index = DocumentIndex::builder()
            .store(store)
            .embedder(embedder)
            .embed_batch_size(32)
            .store_batch_size(50)
            .build()
            .unwrap();

        assert_eq!(index.embed_batch_size, 32);
        assert_eq!(index.store_batch_size, 50);
    }

    #[tokio::test]
    async fn test_builder_missing_store() {
        let embedder = Arc::new(DummyEmbedder::new(3));
        let result = DocumentIndex::builder().embedder(embedder).build();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_builder_missing_embedder() {
        let store = Arc::new(MemoryStore::new());
        let result = DocumentIndex::builder().store(store).build();
        assert!(result.is_err());
    }
}
