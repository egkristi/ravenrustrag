//! Stress tests and scaling validation for RavenRustRAG.
//!
//! These tests validate system stability under concurrent load and large document volumes.
//! Run with: cargo test --test stress_tests --release -- --nocapture

use raven_core::{Chunk, Document};
use raven_embed::DummyEmbedder;
use raven_search::{Bm25Index, DocumentIndex};
use raven_split::TextSplitter;
use raven_store::MemoryStore;
use std::sync::Arc;
use std::time::Instant;

/// Generate N documents with realistic content
fn generate_docs(n: usize) -> Vec<Document> {
    (0..n)
        .map(|i| {
            let text = format!(
                "Document {i} discusses Rust programming and memory safety. \
                 The borrow checker ensures references are valid at compile time. \
                 Fearless concurrency allows data-race-free multithreading. \
                 Zero-cost abstractions provide performance without overhead. \
                 Cargo manages dependencies and builds reproducibly. \
                 Pattern matching with match expressions is exhaustive. \
                 Traits provide polymorphism similar to interfaces. \
                 Lifetimes annotate reference scopes for the compiler. \
                 The type system prevents null pointer dereferences. \
                 Ownership semantics eliminate use-after-free bugs. {i}"
            );
            Document::new(text)
                .with_metadata("source", format!("doc_{i}.md"))
                .with_metadata("category", format!("cat_{}", i % 10))
        })
        .collect()
}

#[tokio::test]
async fn test_10k_document_indexing() {
    let embedder = Arc::new(DummyEmbedder::new(128));
    let store = Arc::new(MemoryStore::new());
    let index = DocumentIndex::new(store.clone(), embedder);
    let splitter = TextSplitter::new(200, 20);

    let docs = generate_docs(10_000);
    let start = Instant::now();
    index.add_documents(docs, &splitter).await.unwrap();
    let elapsed = start.elapsed();

    let count = index.count().await.unwrap();
    println!("10k docs -> {count} chunks in {elapsed:?}");
    println!(
        "  Throughput: {:.0} docs/sec",
        10_000.0 / elapsed.as_secs_f64()
    );
    assert!(count > 10_000); // multiple chunks per doc
    assert!(
        elapsed.as_secs() < 30,
        "10k indexing took too long: {elapsed:?}"
    );
}

#[tokio::test]
async fn test_10k_query_latency() {
    let embedder = Arc::new(DummyEmbedder::new(128));
    let store = Arc::new(MemoryStore::new());
    let index = DocumentIndex::new(store, embedder);
    let splitter = TextSplitter::new(200, 20);

    let docs = generate_docs(10_000);
    index.add_documents(docs, &splitter).await.unwrap();

    // Warm up
    let _ = index.query("Rust programming", 5).await.unwrap();

    // Measure query latency
    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let results = index
            .query("memory safety borrow checker", 5)
            .await
            .unwrap();
        assert!(!results.is_empty());
    }
    let elapsed = start.elapsed();
    let avg = elapsed / iterations;

    println!("10k docs query latency ({iterations} iterations):");
    println!("  Total: {elapsed:?}");
    println!("  Average: {avg:?}");
    // In release mode this is <10ms; in debug mode allow up to 500ms
    assert!(
        avg.as_millis() < 500,
        "Average query latency too high: {avg:?}"
    );
}

#[tokio::test]
async fn test_concurrent_indexing() {
    let embedder = Arc::new(DummyEmbedder::new(128));
    let store = Arc::new(MemoryStore::new());
    let index = Arc::new(DocumentIndex::new(store.clone(), embedder));
    let splitter = Arc::new(TextSplitter::new(200, 20));

    let mut handles = vec![];
    for batch in 0..8 {
        let idx = index.clone();
        let sp = splitter.clone();
        handles.push(tokio::spawn(async move {
            let docs: Vec<Document> = (0..100)
                .map(|i| {
                    Document::new(format!(
                        "Batch {batch} document {i} about concurrent programming and data safety."
                    ))
                })
                .collect();
            idx.add_documents(docs, sp.as_ref()).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let count = index.count().await.unwrap();
    println!("Concurrent indexing (8 threads x 100 docs): {count} chunks");
    assert!(count >= 800, "Expected at least 800 chunks, got {count}");
}

#[tokio::test]
async fn test_concurrent_query_while_indexing() {
    let embedder = Arc::new(DummyEmbedder::new(128));
    let store = Arc::new(MemoryStore::new());
    let index = Arc::new(DocumentIndex::new(store, embedder));
    let splitter = Arc::new(TextSplitter::new(200, 20));

    // Pre-populate with some data
    let initial_docs = generate_docs(1000);
    index
        .add_documents(initial_docs, splitter.as_ref())
        .await
        .unwrap();

    // Spawn concurrent queries
    let query_index = index.clone();
    let query_handle = tokio::spawn(async move {
        for _ in 0..50 {
            let results = query_index.query("Rust programming", 5).await;
            assert!(results.is_ok(), "Query failed during concurrent indexing");
        }
    });

    // Simultaneously add more documents
    let index_idx = index.clone();
    let index_sp = splitter.clone();
    let index_handle = tokio::spawn(async move {
        let docs = generate_docs(500);
        index_idx
            .add_documents(docs, index_sp.as_ref())
            .await
            .unwrap();
    });

    // Both should complete without panics or errors
    query_handle.await.unwrap();
    index_handle.await.unwrap();

    let count = index.count().await.unwrap();
    println!("Concurrent query+index: {count} chunks (no errors)");
}

#[tokio::test]
async fn test_bm25_scaling() {
    let mut bm25 = Bm25Index::new();

    // Build BM25 index with 10k chunks
    let chunks: Vec<Chunk> = (0..10_000)
        .map(|i| {
            Chunk::new(
                format!("doc_{}", i / 5),
                format!(
                    "Rust programming language {} memory safety {} concurrency {}",
                    if i % 3 == 0 { "performance" } else { "systems" },
                    if i % 5 == 0 { "ownership" } else { "borrowing" },
                    if i % 7 == 0 { "async" } else { "threads" },
                ),
            )
        })
        .collect();

    let start = Instant::now();
    bm25.add(&chunks);
    let build_time = start.elapsed();
    println!("BM25 build (10k chunks): {build_time:?}");

    // Measure search latency
    let iterations = 1000;
    let start = Instant::now();
    for _ in 0..iterations {
        let results = bm25.search("Rust memory safety", 10);
        assert!(!results.is_empty());
    }
    let elapsed = start.elapsed();
    let avg = elapsed / iterations;
    println!("BM25 search (10k, {iterations} iterations): avg {avg:?}");
    // In release mode this is <1ms; in debug mode allow up to 50ms
    assert!(avg.as_millis() < 50, "BM25 search too slow at 10k: {avg:?}");
}

#[tokio::test]
async fn test_many_small_documents() {
    let embedder = Arc::new(DummyEmbedder::new(128));
    let store = Arc::new(MemoryStore::new());
    let index = DocumentIndex::new(store, embedder);
    let splitter = TextSplitter::new(200, 20);

    // 50k tiny documents (smaller than chunk size, so 1 chunk each)
    let docs: Vec<Document> = (0..50_000)
        .map(|i| Document::new(format!("Short document number {i}.")))
        .collect();

    let start = Instant::now();
    index.add_documents(docs, &splitter).await.unwrap();
    let elapsed = start.elapsed();

    let count = index.count().await.unwrap();
    println!("50k small docs -> {count} chunks in {elapsed:?}");
    println!(
        "  Throughput: {:.0} docs/sec",
        50_000.0 / elapsed.as_secs_f64()
    );
    assert_eq!(count, 50_000);
}

#[tokio::test]
async fn test_large_document() {
    let embedder = Arc::new(DummyEmbedder::new(128));
    let store = Arc::new(MemoryStore::new());
    let index = DocumentIndex::new(store, embedder);
    let splitter = TextSplitter::new(512, 50);

    // Single large document (>500KB)
    let large_text = "Paragraph X: This is a reasonably long sentence about various topics including Rust programming and memory safety concepts. ".repeat(10_000);
    assert!(
        large_text.len() > 500_000,
        "Generated text only {} bytes",
        large_text.len()
    );

    let docs = vec![Document::new(large_text)];
    let start = Instant::now();
    index.add_documents(docs, &splitter).await.unwrap();
    let elapsed = start.elapsed();

    let count = index.count().await.unwrap();
    println!("1MB document -> {count} chunks in {elapsed:?}");
    assert!(count > 1000);
}
