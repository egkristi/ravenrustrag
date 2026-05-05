//! # RavenRustRAG
//!
//! A local-first, embeddable RAG (Retrieval-Augmented Generation) engine in Rust.
//!
//! This crate provides a unified API for the entire RavenRustRAG pipeline:
//! load documents, split into chunks, embed with local or cloud models,
//! store in SQLite, and query with vector/hybrid/graph search.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ravenrustrag::{DocumentIndex, TextSplitter, Loader, SqliteStore, create_cached_embedder};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load documents
//!     let docs = Loader::from_directory("./docs", Some(&["md", "txt"]))?;
//!
//!     // Set up pipeline
//!     let store = Arc::new(SqliteStore::new("./raven.db", 768).await?);
//!     let embedder = create_cached_embedder("ollama", "nomic-embed-text", None, None, 10_000);
//!     let index = DocumentIndex::new(store, embedder);
//!
//!     // Index
//!     let splitter = TextSplitter::new(512, 50);
//!     index.add_documents(docs, &splitter).await?;
//!
//!     // Query
//!     let results = index.query("What is RAG?", 5).await?;
//!     for r in &results {
//!         println!("[{:.4}] {}", r.score, r.chunk.text);
//!     }
//!
//!     Ok(())
//! }
//! ```

// Re-export core types
pub use raven_core::{
    cosine_similarity, fingerprint, Chunk, Config, Document, RavenError, Result, SearchResult,
};

// Re-export embedding
pub use raven_embed::{
    create_cached_embedder, create_embedder, CachedEmbedder, DummyEmbedder, Embedder,
    EmbeddingCache, OllamaBackend, OpenAIBackend,
};

// Re-export storage
pub use raven_store::{MemoryStore, MetadataFilter, SqliteStore, VectorStore};

// Re-export splitting
pub use raven_split::{SentenceSplitter, Splitter, TextSplitter, TokenSplitter};

// Re-export loading
pub use raven_load::{export_jsonl, import_jsonl, Loader};

// Re-export search pipeline
pub use raven_search::{
    eval_summary, evaluate_batch, expand_query, extract_entities, graph_vector_fusion, mrr,
    ndcg_at_k, precision_at_k, recall_at_k, reciprocal_rank_fusion, rerank, Bm25Index,
    DocumentIndex, Entity, EvalResult, GraphRetriever, KeywordReranker, KnowledgeGraph, Relation,
    Reranker, SemanticSplitter,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_full_pipeline() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(128));
        let index = DocumentIndex::new(store, embedder);

        let docs = vec![
            Document::new("Rust is a systems programming language."),
            Document::new("RAG combines retrieval with generation."),
            Document::new("Embeddings map text to vector space."),
        ];

        let splitter = TextSplitter::new(512, 50);
        index.add_documents(docs, &splitter).await.unwrap();

        let results = index.query("What is Rust?", 3).await.unwrap();
        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }

    #[tokio::test]
    async fn test_hybrid_search() {
        let store = Arc::new(MemoryStore::new());
        let embedder = Arc::new(DummyEmbedder::new(128));
        let index = DocumentIndex::new(store, embedder);

        let docs = vec![
            Document::new("The quick brown fox jumps over the lazy dog."),
            Document::new("A fast red fox leaps above a sleeping hound."),
            Document::new("Machine learning models process data efficiently."),
        ];

        let splitter = TextSplitter::new(512, 50);
        index.add_documents(docs, &splitter).await.unwrap();

        let results = index.query_hybrid("fox jumping", 3, 0.5).await.unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_loader() {
        let temp = tempfile::TempDir::new().unwrap();
        std::fs::write(temp.path().join("a.txt"), "Hello world").unwrap();
        std::fs::write(temp.path().join("b.md"), "# Title\nContent").unwrap();

        let docs = Loader::from_directory(temp.path(), Some(&["txt", "md"])).unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_splitter() {
        let splitter = TextSplitter::new(50, 10);
        let docs = vec![Document::new("Hello world. This is a test document with enough text to split into multiple chunks for testing purposes.")];
        let chunks = splitter.split(docs);
        assert!(chunks.len() >= 2);
    }

    #[test]
    fn test_metadata_filter() {
        let filter = MetadataFilter::new().with("source", "test.md");
        assert_eq!(
            filter.conditions.get("source"),
            Some(&"test.md".to_string())
        );
    }
}
