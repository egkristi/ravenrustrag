use async_trait::async_trait;
use raven_core::{Chunk, Document, Result, SearchResult};
use raven_embed::Embedder;
use raven_split::Splitter;
use raven_store::VectorStore;
use std::sync::Arc;

/// Main document index — the heart of RavenRustRAG
pub struct DocumentIndex {
    store: Arc<dyn VectorStore>,
    embedder: Arc<dyn Embedder>,
}

impl DocumentIndex {
    pub fn new(
        store: Arc<dyn VectorStore>,
        embedder: Arc<dyn Embedder>,
    ) -> Self {
        Self { store, embedder }
    }

    pub fn builder() -> DocumentIndexBuilder {
        DocumentIndexBuilder::default()
    }

    /// Add documents (chunks must already have embeddings)
    pub async fn add_chunks(&self, chunks: &[Chunk]) -> Result<()> {
        self.store.add(chunks).await
    }

    /// Add raw documents: split, embed, store
    pub async fn add_documents(
        &self,
        documents: Vec<Document>,
        splitter: &dyn Splitter,
    ) -> Result<()> {
        let chunks = splitter.split(documents);
        
        if chunks.is_empty() {
            return Ok(());
        }

        let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
        
        // Embed
        let embeddings = self.embedder.embed(&texts).await?;
        
        // Attach embeddings to chunks
        let mut embedded_chunks = Vec::with_capacity(chunks.len());
        for (mut chunk, embedding) in chunks.into_iter().zip(embeddings.into_iter()) {
            chunk.embedding = Some(embedding);
            embedded_chunks.push(chunk);
        }

        // Store
        self.store.add(&embedded_chunks).await
    }

    /// Query the index
    pub async fn query(&self,
        query_text: &str,
        top_k: usize,
    ) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embedder.embed(&[query_text.to_string()]).await?;
        let embedding = query_embedding.into_iter().next()
            .ok_or_else(|| raven_core::RavenError::NotFound("Empty embedding".to_string()))?;
        
        self.store.search(&embedding, top_k).await
    }

    /// Format results as LLM prompt
    pub async fn query_for_prompt(
        &self,
        query_text: &str,
        top_k: usize,
    ) -> Result<String> {
        let results = self.query(query_text, top_k).await?;
        
        let mut prompt = format!("Query: {}\n\nContext:\n", query_text);
        
        for (i, result) in results.iter().enumerate() {
            let source = result.chunk.metadata.get("source")
                .unwrap_or(&result.chunk.doc_id);
            
            prompt.push_str(&format!(
                "\n[{}] Source: {}\n{}\n",
                i + 1,
                source,
                result.chunk.text
            ));
        }
        
        prompt.push_str("\n---\nAnswer the query using the provided context.");
        
        Ok(prompt)
    }

    pub async fn count(&self) -> Result<usize> {
        self.store.count().await
    }

    pub async fn delete(&self, doc_id: &str) -> Result<()> {
        self.store.delete(doc_id).await
    }

    pub async fn clear(&self) -> Result<()> {
        self.store.clear().await
    }
}

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
        let store = self.store
            .ok_or_else(|| raven_core::RavenError::Config("Store not configured".to_string()))?;
        let embedder = self.embedder
            .ok_or_else(|| raven_core::RavenError::Config("Embedder not configured".to_string()))?;
        
        Ok(DocumentIndex::new(store, embedder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raven_embed::DummyEmbedder;
    use raven_store::MemoryStore;
    use raven_split::TextSplitter;

    struct TestEmbedder;

    #[async_trait::async_trait]
    impl Embedder for TestEmbedder {
        async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
            Ok(texts.iter().map(|_| vec![1.0, 0.0, 0.0]).collect())
        }

        fn dimension(&self) -> usize {
            3
        }

        fn model_name(&self) -> &str {
            "test"
        }
    }

    #[tokio::test]
    async fn test_document_index() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(TestEmbedder);
        let index = DocumentIndex::new(store, embedder);
        
        let splitter = TextSplitter::new(100, 10);
        let docs = vec![
            Document::new("This is a test document about Rust programming."),
        ];

        index.add_documents(docs, &splitter).await.unwrap();
        assert_eq!(index.count().await.unwrap(), 1);

        let results = index.query("Rust", 5).await.unwrap();
        assert_eq!(results.len(), 1);
    }
}