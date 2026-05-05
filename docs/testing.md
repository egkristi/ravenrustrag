# Testing

RavenRustRAG uses a multi-layered testing strategy to ensure correctness and performance.

## Running Tests

```bash
# All tests
cargo test --all

# Specific crate
cargo test -p raven-search

# With output
cargo test --all -- --nocapture

# Single test
cargo test -p raven-store test_sqlite_store
```

## Test Architecture

### Unit Tests

Each crate contains unit tests in `#[cfg(test)]` modules. Tests use:

- **`MemoryStore`**: In-memory vector store that requires no SQLite
- **`DummyEmbedder`**: Deterministic embeddings based on text hash (no Ollama needed)
- **`TextSplitter`**: Simple character-based splitting for predictable chunk boundaries

This ensures tests run fast, in parallel, and without external dependencies.

### Integration Tests

Tests that exercise the full pipeline (index → query) use `DocumentIndex` with real store backends:

```rust
#[tokio::test]
async fn test_document_index() {
    let store = MemoryStore::new(4);
    let embedder = DummyEmbedder::new(4);
    let splitter = TextSplitter::new(100, 20);

    let index = DocumentIndex::builder()
        .store(Arc::new(store))
        .embedder(Arc::new(embedder))
        .splitter(Arc::new(splitter))
        .build()
        .unwrap();

    let docs = vec![Document::new("Test content here")];
    index.add_documents(&docs).await.unwrap();

    let results = index.query("test", 5).await.unwrap();
    assert!(!results.is_empty());
}
```

### Server Tests

The HTTP server tests use `axum::test` helpers to send requests without starting a real TCP listener:

```rust
#[tokio::test]
async fn test_health() {
    let app = create_app(/* ... */);
    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

### MCP Tests

MCP tests simulate JSON-RPC message exchanges over an in-memory channel, verifying protocol compliance and tool execution.

## Test Coverage

Run coverage with `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all --out html
# Report at tarpaulin-report.html
```

## Benchmarks

Performance benchmarks use Criterion:

```bash
cargo bench
```

Benchmarks measure:
- Embedding cache lookup (hit/miss)
- SQLite insert/query at various scales
- Cosine similarity computation
- Text splitting throughput
- Full pipeline (index + query) end-to-end

Results are tracked in CI via `benchmark-action/github-action-benchmark` and published to the `gh-pages` branch.

## Linting

```bash
# Clippy (strict mode for library code)
cargo clippy --lib -- -D warnings

# Format check
cargo fmt --all --check
```

## Property Testing

For critical algorithms (cosine similarity, text splitting, BM25 scoring), property-based tests using `proptest` verify invariants:

- Cosine similarity is always in [-1, 1]
- Splitting never loses text content
- BM25 scores are non-negative
- RRF fusion preserves result count

## Fuzz Testing

Security-sensitive parsers (JSON-RPC, HTTP request bodies) can be fuzz-tested:

```bash
cargo install cargo-fuzz
cargo fuzz run mcp_parse
```

## CI Integration

All tests run on every push and PR via GitHub Actions:

1. `cargo fmt --all --check`
2. `cargo clippy --lib -- -D warnings`
3. `cargo test --all`
4. MSRV check (Rust 1.88)
5. Benchmarks (on main branch pushes)
