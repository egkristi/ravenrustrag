use criterion::{criterion_group, criterion_main, Criterion};
use raven_core::Document;
use raven_embed::DummyEmbedder;
use raven_search::DocumentIndex;
use raven_split::TextSplitter;
use raven_store::MemoryStore;
use std::sync::Arc;

fn create_index(num_docs: usize) -> (tokio::runtime::Runtime, DocumentIndex) {
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let store = Arc::new(MemoryStore::new());
    let embedder = Arc::new(DummyEmbedder::new(128));
    let index = DocumentIndex::new(store, embedder);
    let splitter = TextSplitter::new(200, 20);

    let docs: Vec<Document> = (0..num_docs)
        .map(|i| {
            Document::new(format!(
                "Document {i} about Rust programming, memory safety, and performance. \
                 Rust is a systems programming language focused on safety, speed, and concurrency."
            ))
        })
        .collect();

    rt.block_on(index.add_documents(docs, &splitter))
        .expect("add documents");

    (rt, index)
}

fn bench_query(c: &mut Criterion) {
    let (rt, index) = create_index(100);

    c.bench_function("query_100docs", |b| {
        b.iter(|| {
            rt.block_on(index.query("Rust programming", 5))
                .expect("query");
        });
    });
}

fn bench_query_1000(c: &mut Criterion) {
    let (rt, index) = create_index(1000);

    c.bench_function("query_1000docs", |b| {
        b.iter(|| {
            rt.block_on(index.query("Rust programming", 5))
                .expect("query");
        });
    });
}

fn bench_hybrid_query(c: &mut Criterion) {
    let (rt, index) = create_index(100);

    c.bench_function("hybrid_query_100docs", |b| {
        b.iter(|| {
            rt.block_on(index.query_hybrid("Rust programming", 5, 0.5))
                .expect("hybrid query");
        });
    });
}

fn bench_index(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let embedder = Arc::new(DummyEmbedder::new(128));
    let splitter = TextSplitter::new(200, 20);

    c.bench_function("index_10docs", |b| {
        b.iter(|| {
            let store = Arc::new(MemoryStore::new());
            let index = DocumentIndex::new(store, embedder.clone());
            let docs: Vec<Document> = (0..10)
                .map(|i| Document::new(format!("Document {i} about benchmarking")))
                .collect();
            rt.block_on(index.add_documents(docs, &splitter))
                .expect("add documents");
        });
    });
}

fn bench_bm25(c: &mut Criterion) {
    let mut bm25 = raven_search::Bm25Index::new();
    let chunks: Vec<raven_core::Chunk> = (0..1000)
        .map(|i| {
            raven_core::Chunk::new(
                &format!("doc_{i}"),
                &format!("Rust programming language number {i} with safety and performance"),
            )
        })
        .collect();
    bm25.add(&chunks);

    c.bench_function("bm25_search_1000", |b| {
        b.iter(|| {
            bm25.search("Rust programming", 10);
        });
    });
}

criterion_group!(
    benches,
    bench_query,
    bench_query_1000,
    bench_hybrid_query,
    bench_index,
    bench_bm25
);
criterion_main!(benches);
