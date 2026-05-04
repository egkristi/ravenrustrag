use raven_core::{Chunk, Document, Result, SearchResult};
use raven_embed::Embedder;
use raven_split::Splitter;
use raven_store::VectorStore;
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

/// Main document index — the heart of RavenRustRAG
pub struct DocumentIndex {
    store: Arc<dyn VectorStore>,
    embedder: Arc<dyn Embedder>,
    bm25: RwLock<Bm25Index>,
}

impl DocumentIndex {
    pub fn new(store: Arc<dyn VectorStore>, embedder: Arc<dyn Embedder>) -> Self {
        Self {
            store,
            embedder,
            bm25: RwLock::new(Bm25Index::new()),
        }
    }

    pub fn builder() -> DocumentIndexBuilder {
        DocumentIndexBuilder::default()
    }

    /// Add documents (chunks must already have embeddings)
    pub async fn add_chunks(&self, chunks: &[Chunk]) -> Result<()> {
        self.bm25.write().await.add(chunks);
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
        let batch_size = 64;
        let mut all_embeddings = Vec::with_capacity(texts.len());

        info!(num_chunks = texts.len(), "Embedding chunks");
        for batch in texts.chunks(batch_size) {
            let embeddings = self.embedder.embed(batch).await?;
            all_embeddings.extend(embeddings);
        }

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
        for batch in embedded_chunks.chunks(100) {
            self.store.add(batch).await?;
        }

        // Also add to BM25 index
        self.bm25.write().await.add(&embedded_chunks);

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

    pub async fn delete(&self, doc_id: &str) -> Result<()> {
        self.store.delete(doc_id).await
    }

    pub async fn clear(&self) -> Result<()> {
        self.bm25.write().await.clear();
        self.store.clear().await
    }

    pub fn embedder(&self) -> &Arc<dyn Embedder> {
        &self.embedder
    }

    pub fn store(&self) -> &Arc<dyn VectorStore> {
        &self.store
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
        let mut prompt = format!("Query: {}\n\nContext:\n", query);
        for (i, result) in results.iter().enumerate() {
            let source = result
                .chunk
                .metadata
                .get("source")
                .unwrap_or(&result.chunk.doc_id);
            prompt.push_str(&format!(
                "\n[{}] Source: {}\n{}\n",
                i + 1,
                source,
                result.chunk.text
            ));
        }
        prompt.push_str("\n---\nAnswer the query using the provided context.");
        prompt
    }
}

// =============================================================================
// Builder
// =============================================================================

#[derive(Default)]
pub struct DocumentIndexBuilder {
    store: Option<Arc<dyn VectorStore>>,
    embedder: Option<Arc<dyn Embedder>>,
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

    pub fn build(self) -> Result<DocumentIndex> {
        let store = self
            .store
            .ok_or_else(|| raven_core::RavenError::Config("Store not configured".to_string()))?;
        let embedder = self
            .embedder
            .ok_or_else(|| raven_core::RavenError::Config("Embedder not configured".to_string()))?;

        Ok(DocumentIndex::new(store, embedder))
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
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
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
}
