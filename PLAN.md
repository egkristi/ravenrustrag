# RavenRustRAG — Implementation Plan

> **Status:** Planning  
> **Target:** v0.1.0-alpha within 2 weeks  
> **Motto:** *Make it work, make it right, make it fast — in that order.*

## 1. Philosophy

1. **Local-first** — Works without internet, cloud APIs optional
2. **Single binary** — `cargo install` and go
3. **Async by default** — Tokio throughout, zero blocking I/O
4. **Modular** — Use as CLI, server, library, or MCP tool
5. **Ergonomic** — Good defaults, minimal configuration

## 2. Architecture

### 2.1 Crate Structure

```
ravenrustrag/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── raven-core/         # Core types, Document, Chunk, errors
│   ├── raven-embed/        # Embedding backends (trait + impls)
│   ├── raven-store/        # Vector stores (SQLite, memory, trait)
│   ├── raven-split/        # Text splitting strategies
│   ├── raven-load/         # File loaders
│   ├── raven-search/       # Search, hybrid, reranking
│   ├── raven-server/       # Axum HTTP API
│   ├── raven-cli/          # CLI binary
│   └── raven-mcp/          # MCP protocol server
├── raven.toml              # Example config
└── benches/                # Criterion benchmarks
```

### 2.2 Core Types

```rust
// crates/raven-core/src/lib.rs
pub struct Document {
    pub id: String,
    pub text: String,
    pub metadata: HashMap<String, String>,
}

pub struct Chunk {
    pub id: String,
    pub doc_id: String,
    pub text: String,
    pub metadata: HashMap<String, String>,
    pub embedding: Option<Vec<f32>>,
}

pub struct SearchResult {
    pub chunk: Chunk,
    pub score: f32,
    pub distance: f32,
}

#[derive(Debug, Error)]
pub enum RavenError {
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("embedder error: {0}")]
    Embed(#[from] EmbedError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## 3. Phase 1: Foundation (Week 1)

### 3.1 raven-core
- [ ] Define `Document`, `Chunk`, `SearchResult`
- [ ] Error types with `thiserror`
- [ ] Content fingerprinting (SHA-256)
- [ ] Config types (TOML deserialization)

### 3.2 raven-embed
- [ ] `Embedder` trait: `async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>`
- [ ] `OllamaBackend` — HTTP client to Ollama API
- [ ] `OpenAIBackend` — OpenAI-compatible (optional feature)
- [ ] `LocalBackend` — ONNX Runtime for local models (optional)
- [ ] Embedding cache (LRU in-memory)

### 3.3 raven-store
- [ ] `VectorStore` trait:
  ```rust
  pub trait VectorStore: Send + Sync {
      async fn add(&self, chunks: &[Chunk]) -> Result<()>;
      async fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>>;
      async fn delete(&self, doc_id: &str) -> Result<()>;
      async fn count(&self) -> Result<usize>;
  }
  ```
- [ ] `SqliteStore` — rusqlite + custom vector ops
  - Flat brute-force search first (cosine similarity)
  - HNSW via `sqlite-vec` or custom implementation
- [ ] `MemoryStore` — in-memory, for testing
- [ ] Metadata filtering

### 3.4 raven-split
- [ ] `Splitter` trait
- [ ] `TextSplitter` — character-based with overlap
- [ ] `TokenSplitter` — tiktoken-rs approximation
- [ ] `SemanticSplitter` — sentence boundaries + embedder (optional)

### 3.5 raven-load
- [ ] `Loader` trait
- [ ] `TextLoader` — .txt, .md
- [ ] `PdfLoader` — pdf-extract (optional)
- [ ] `HtmlLoader` — html5ever (optional)
- [ ] `DirectoryLoader` — glob + recursive

### 3.6 Integration: DocumentIndex
- [ ] Builder pattern:
  ```rust
  let index = DocumentIndex::builder()
      .store(SqliteStore::new("./raven.db"))
      .embedder(OllamaBackend::new("http://localhost:11434", "nomic-embed-text"))
      .splitter(TextSplitter::new(512, 50))
      .build()?;
  ```
- [ ] `add(documents)` → split → embed → store
- [ ] `query(text)` → embed → search → return results
- [ ] `query_for_prompt(text)` → query → format with citations

### 3.7 raven-cli (basic)
- [ ] `raven index <path> --db <db>`
- [ ] `raven query "text" --db <db>`
- [ ] `raven info --db <db>`
- [ ] clap for argument parsing
- [ ] tokio runtime

## 4. Phase 2: Advanced Features (Week 2)

### 4.1 Hybrid Search
- [ ] BM25 index (tantivy-lite or custom)
- [ ] Reciprocal Rank Fusion (RRF)
- [ ] Configurable alpha (vector vs sparse weight)

### 4.2 Reranking
- [ ] `Reranker` trait
- [ ] Cross-encoder via ONNX (optional)
- [ ] ColBERT-style late interaction (stretch)

### 4.3 HTTP API (raven-server)
- [ ] Axum routes:
  - `GET /health`
  - `GET /stats`
  - `POST /query` (JSON body)
  - `POST /prompt`
  - `POST /index`
  - `GET /openapi.json`
- [ ] Bearer auth (optional)
- [ ] CORS support
- [ ] Graceful shutdown

### 4.4 MCP Server (raven-mcp)
- [ ] stdio transport
- [ ] Tools: `rag_search`, `rag_index`, `rag_stats`
- [ ] Resources: `rag://{collection}/stats`

### 4.5 CLI Polish
- [ ] `raven serve` — start API server
- [ ] `raven watch` — file watcher (notify-rs)
- [ ] `raven export` — JSONL dump
- [ ] `raven import` — JSONL restore
- [ ] `raven benchmark` — built-in benchmarks
- [ ] `raven doctor` — diagnostics
- [ ] Progress bars (indicatif)
- [ ] Colored output

### 4.6 Incremental Indexing
- [ ] File fingerprinting (mtime + content hash)
- [ ] Skip unchanged files
- [ ] Detect deletions
- [ ] Fingerprint persistence (SQLite table)

## 5. Phase 3: Production Readiness

### 5.1 Testing
- [ ] Unit tests (80%+ coverage)
- [ ] Integration tests (full pipeline)
- [ ] Property-based tests (proptest)
- [ ] Benchmarks (Criterion)

### 5.2 Documentation
- [ ] rustdoc for all public APIs
- [ ] User guide (mdBook)
- [ ] Architecture decision records (ADRs)

### 5.3 CI/CD
- [ ] GitHub Actions: test, clippy, fmt, audit
- [ ] Cross-compilation releases (Linux, macOS, Windows)
- [ ] crates.io publish
- [ ] Docker image

### 5.4 Performance
- [ ] SIMD vector ops (ndarray + packed_simd)
- [ ] Connection pooling (embedder HTTP client)
- [ ] Streaming index (process large files in chunks)
- [ ] Memory-mapped storage option

## 6. Key Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `axum` | HTTP server |
| `serde` + `toml` | Config, serialization |
| `clap` | CLI parsing |
| `reqwest` | HTTP client (embedders) |
| `rusqlite` | SQLite storage |
| `ndarray` | Vector math |
| `thiserror` + `anyhow` | Error handling |
| `tracing` | Logging |
| `notify` | File watching |
| `indicatif` | Progress bars |
| `tiktoken-rs` | Token counting |
| `serde_json` | JSON handling |
| `utoipa` | OpenAPI generation |
| `tower` | Middleware (auth, CORS) |
| `sha2` | Content hashing |
| `walkdir` | Directory traversal |

## 7. Design Decisions

### 7.1 SQLite as Default Store
- **Pro:** Single file, zero setup, ACID, cross-platform
- **Con:** No native vector search (need custom or extension)
- **Mitigation:** Flat search for small datasets, HNSW for large

### 7.2 Async Throughout
- **Pro:** Concurrent embedding requests, non-blocking I/O
- **Con:** Slightly more complex code
- **Mitigation:** Good abstractions, `.await` is ergonomic in Rust

### 7.3 Modular Crate Structure
- **Pro:** Users only pull what they need, faster compile times
- **Con:** More workspace complexity
- **Mitigation:** Workspace-level Cargo.toml with shared deps

## 8. Success Criteria

- [ ] `cargo install ravenrustrag` → single binary works
- [ ] Index 1000 documents in <5s (with local embedder)
- [ ] Query latency <10ms (cold), <1ms (cached)
- [ ] Memory footprint <50MB for 10k documents
- [ ] All RavenRAG Python features replicated
- [ ] Faster in every benchmark

## 9. Immediate Next Steps

1. Initialize workspace: `cargo new --lib ravenrustrag`
2. Set up workspace Cargo.toml with all crates
3. Implement `raven-core` types
4. Implement `OllamaBackend` embedder
5. Implement `SqliteStore` with flat search
6. Wire up basic `DocumentIndex`
7. CLI skeleton with `index` and `query` commands
8. Test end-to-end with sample documents

---

**Last updated:** 2026-05-04  
**Next review:** After v0.1.0-alpha