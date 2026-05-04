# RavenRustRAG — Implementation Plan

> **Status:** v0.1.0-alpha — Phases 1–3 complete, Phase 4 in progress, 13 open issues  
> **Motto:** *Make it work, make it right, make it fast — in that order.*  
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

---

## Reference: RavenRAG v0.7.0 (Python)

Complete feature list of the Python version as of 2026-05-04 (~4,200 lines, 24 modules):

| Category | Python Features | Rust Status |
|---|---|---|
| **Core** | Document, QueryResult (citation), DocumentIndex, async (aadd/aquery) | ✅ Phase 1 |
| **Embedding** | sentence-transformers, Ollama, OpenAI, vLLM, custom protocol | ✅ Ollama + OpenAI + auto-detect |
| **Storage** | ChromaDB, FAISS, SQLite-vec, VectorStoreBackend protocol | ✅ SQLite + Memory |
| **Splitting** | TextSplitter, TokenSplitter, SemanticSplitter | ✅ Text + Token + Sentence + Semantic |
| **Loaders** | .txt .md .pdf .docx .pptx .xlsx .csv .rtf .html + plugin system | ✅ txt,md,csv,json,jsonl,html,pdf,docx |
| **Search** | Vector, BM25 hybrid (RRF), cross-encoder reranking, streaming | ✅ Vector + BM25 hybrid (RRF) + HNSW + streaming |
| **Graph** | KnowledgeGraph, GraphRetriever, entity extraction, RRF fusion | ✅ Complete |
| **Server** | HTTP (stdlib), auth, CORS, /metrics, /openapi.json, 7 endpoints | ✅ Axum, 9 endpoints, full OpenAPI 3.0 |
| **MCP** | stdio JSON-RPC, 3 tools (search, get_prompt, collection_info) | ✅ 4 tools |
| **CLI** | 11 commands (index, query, prompt, serve, watch, info, export, import, doctor, mcp, benchmark) | ✅ 12 commands |
| **Pipeline** | Pipeline class, run/query/stream, error strategies | ✅ DocumentIndex pipeline |
| **Config** | TOML + pyproject.toml + env vars, auto-discovery | ✅ Base |
| **Cache** | Thread-safe LRU embedding cache | ✅ |
| **Eval** | MRR, NDCG, Recall@k | ✅ MRR, NDCG, Recall@k, Precision@k |
| **Watch** | File watcher with debounce + delete tracking | ✅ notify crate |
| **Export** | JSONL backup/restore | ✅ export/import |
| **Fingerprint** | SHA-256 incremental indexing | ✅ |
| **Observability** | @timed decorator, /metrics, raven benchmark | ✅ tracing spans, /metrics |
| **Multi-collection** | MultiCollectionRouter, cross-index query | ✅ MultiCollectionRouter |
| **Parent-child** | query_parent() — search chunks, return parents | ✅ query_parent() |
| **Context** | ContextFormatter, templates, citations in prompts | ✅ Base |
| **Docker** | Multi-stage, model pre-download, non-root, healthcheck | ✅ Dockerfile |
| **CI** | GitHub Actions, lint, test (75% coverage), container build | ✅ GitHub Actions |

### Known Weaknesses in the Python Version

These must **not** be reproduced in Rust:

1. **No thread safety** — concurrent requests can corrupt state
2. **Sync-first** — async is `asyncio.to_thread` wrappers, not real async
3. **ChromaDB leakage** — `query_parent()` breaks the VectorStoreBackend abstraction
4. **Minimal TOML parser** — regex-based, does not handle arrays/escaped quotes
5. **No rate limiting** — server is DoS-vulnerable
6. **No request timeout** — slow query blocks thread forever
7. **BM25 not persisted** — rebuilt in memory on change
8. **Flat vector search in SQLite backend** — O(n), no index
9. **Slow startup** — 2-5s due to Python import + model loading
10. **High memory usage** — 200-500MB+ baseline

### Rust Advantages That Make RavenRustRAG Superior

| Dimension | Python | Rust |
|---|---|---|
| **Startup** | 2–5s | <50ms |
| **Query latency** | 50–200ms | 35 µs (100 docs, measured) |
| **Memory** | 200–500MB+ | 20–50MB |
| **Deploy** | virtualenv + deps | Single static binary (8.7 MB) |
| **Concurrency** | GIL-bound | Lock-free reads, Tokio async |
| **Safety** | Runtime exceptions | Compile-time guarantees |
| **Thread safety** | None | Send + Sync, Arc<RwLock> |

---

## 1. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                          RavenRustRAG                             │
├──────────────────────────────────────────────────────────────────┤
│  CLI │ Axum HTTP Server │ MCP Server (stdio) │ Library (crate)   │
├──────────────────────────────────────────────────────────────────┤
│  Pipeline: load → split → embed → store → search → rerank → fmt │
├──────────┬──────────┬───────────┬──────────┬─────────────────────┤
│ Loaders  │ Splitters│ Embedders │  Stores  │ Search & Retrieval  │
│  .txt    │  Text    │  Ollama   │  SQLite  │  Vector (flat/HNSW) │
│  .md     │  Token   │  OpenAI   │  Memory  │  BM25 keyword       │
│  .pdf    │ Semantic │  ONNX     │  Custom  │  Hybrid (RRF)       │
│  .docx   │          │  Custom   │          │  Cross-encoder      │
│  .html   │          │           │          │  Graph traversal    │
│  .csv    │          │           │          │  Parent-child       │
│  .json   │          │           │          │  Multi-collection   │
│  plugin  │          │           │          │  Streaming          │
└──────────┴──────────┴───────────┴──────────┴─────────────────────┘
```

## 2. Crate Structure

```
ravenrustrag/
├── Cargo.toml                  # Workspace root
├── crates/
│   ├── raven-core/             # Document, Chunk, SearchResult, Config, errors, fingerprint
│   ├── raven-embed/            # Embedder trait + Ollama, OpenAI, ONNX backends + cache
│   ├── raven-store/            # VectorStore trait + SQLite, Memory backends
│   ├── raven-split/            # Splitter trait + Text, Token, Semantic splitters
│   ├── raven-load/             # Loader trait + file loaders + plugin registry
│   ├── raven-search/           # DocumentIndex, Pipeline, HybridSearcher, Reranker, Graph
│   ├── raven-server/           # Axum HTTP API (auth, CORS, /metrics, /openapi.json)
│   ├── raven-cli/              # CLI binary: 11+ commands
│   └── raven-mcp/              # MCP server (stdio JSON-RPC)
├── raven.toml                  # Default config
├── Dockerfile                  # Multi-stage, static binary, scratch/alpine
└── .github/workflows/          # CI: test, lint, release, container
```

---

## 3. Phase 1: Foundation ✅ COMPLETE

### 3.1 raven-core ✅
- [x] `Document` — with metadata, id (SHA-256 fallback)
- [x] `Chunk` — doc_id, text, metadata, embedding
- [x] `SearchResult` — chunk, score, distance, citation
- [x] `RavenError` — thiserror-based enum
- [x] `Config` — TOML + env var support
- [x] Fingerprint (SHA-256 content hash)

### 3.2 raven-embed ✅
- [x] `Embedder` trait (async)
- [x] `OllamaBackend` — HTTP client to Ollama /api/embed
- [x] `EmbeddingCache` — LRU in-memory cache
- [x] `CachedEmbedder` — transparent cache wrapper

### 3.3 raven-store ✅
- [x] `VectorStore` trait (async)
- [x] `SqliteStore` — rusqlite + cosine similarity
- [x] `MemoryStore` — for testing
- [x] Metadata-filtering
- [x] Fingerprint table for incremental indexing

### 3.4 raven-split ✅
- [x] `Splitter` trait
- [x] `TextSplitter` — character-based with configurable overlap
- [x] `TokenSplitter` — word-boundary-aware splitting
- [x] `SentenceSplitter` — sentence-boundary splitting

### 3.5 raven-load ✅
- [x] `Loader` — from_file, from_directory
- [x] Extension-filtering
- [x] Recursive directory walking

### 3.6 raven-search ✅
- [x] `DocumentIndex` — pipeline orchestrator
- [x] Builder pattern
- [x] `add_documents()` — split → embed → store
- [x] `query()` — embed → search
- [x] `query_for_prompt()` — LLM-ready context with citations

### 3.7 raven-cli ✅
- [x] `raven index <path>` — index with progress bar
- [x] `raven query "tekst"` — search with scoring
- [x] `raven info` — statistics
- [x] `raven clear` — clear index
- [x] `raven serve` — placeholder

---

## 4. Phase 2: Feature Parity with Python

**Goal:** Match all features in RavenRAG v0.7.0, but with better design.

### 4.1 HTTP API Server (raven-server)
- [x] Axum-based server with Tokio
- [x] `GET /health` — health check
- [x] `GET /stats` — index statistics
- [x] `GET /collections` — list collections
- [x] `GET /metrics` — timing and cache stats
- [x] `GET /openapi.json` — OpenAPI 3.0 schema
- [x] `POST /query` — search (top_k, where, rerank, hybrid, alpha)
- [x] `POST /prompt` — LLM-ready prompt
- [x] `POST /index` — add documents
- [x] Bearer token auth (via header + config/env)
- [x] CORS configuration (tower-http)
- [x] Request size limit (10MB)
- [x] Request timeout (configurable) — [#5](https://github.com/egkristi/ravenrustrag/issues/5)
- [x] Rate limiting (token-bucket middleware) — [#2](https://github.com/egkristi/ravenrustrag/issues/2) — **better than Python**
- [x] Graceful shutdown

### 4.2 MCP Server (raven-mcp)
- [x] JSON-RPC over stdio (MCP 2024-11-05)
- [x] Tool: `search` — query with top_k
- [x] Tool: `get_prompt` — search + format LLM prompt
- [x] Tool: `collection_info` — index statistics
- [x] Tool: `index_documents` — add documents **new vs Python**
- [x] Proper error codes and schema validation (JSON-RPC named constants, top_k range check)

### 4.3 Additional Embedding Backends
- [x] `OpenAIBackend` — OpenAI-compatible API (OpenAI, LM Studio, LocalAI, vLLM)
- [ ] ONNX Runtime local embeddings — deferred (ort crate MSRV conflict) — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [x] Backend auto-detection via `create_embedder()` / `create_cached_embedder()` factory functions

### 4.4 Splitter Extensions
- [x] `TokenSplitter` — tokenizer-aware splitting
- [x] `SentenceSplitter` — sentence-boundary splitting
- [x] `SemanticSplitter` — sentence-boundary + embedding cosine similarity (raven-search/src/semantic_split.rs)
- [x] Metadata preservation (chunk_index, source_id) through entire pipeline

### 4.5 File Loaders
- [x] Markdown with frontmatter parsing (YAML metadata → doc metadata)
- [x] PDF loader (pdf-extract, behind `pdf` feature flag)
- [x] HTML loader (strip tags, remove scripts/styles)
- [x] CSV loader (csv crate)
- [x] JSON/JSONL loader
- [x] DOCX loader (zip-based, behind `docx` feature flag)
- [x] Plugin system: `register_loader` for custom file types
- [x] Auto-detect file type and select loader

### 4.6 Hybrid Search
- [x] BM25 index (custom Okapi BM25)
- [x] `HybridSearcher` — vector + BM25 with Reciprocal Rank Fusion
- [x] Configurable alpha (0.0 = pure BM25, 1.0 = pure vector)
- [x] Metadata filtering on search results -- [#35](https://github.com/egkristi/ravenrustrag/issues/35)
- [x] BM25 persistence in SQLite -- [#37](https://github.com/egkristi/ravenrustrag/issues/37)

### 4.7 Cross-encoder Reranking
- [ ] ONNX-based cross-encoder (local, no Python) — deferred (blocked by #43) — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [x] Rerank trait with pluggable backends (`Reranker` trait + `KeywordReranker`)
- [x] Fetch 4x → rerank → return top_k (`rerank()` function in raven-search/src/rerank.rs)

### 4.8 Watch Mode
- [x] `notify` crate for filesystem events
- [x] Debounce with configurable delay
- [x] Delete support (remove documents when files are deleted)
- [x] Extension-filtering
- [x] CLI: `raven watch ./docs --extensions "md,txt"`

### 4.9 Export/Import
- [x] JSONL export (`raven export -o backup.jsonl`)
- [x] JSONL import (`raven import backup.jsonl`)
- [x] Skip invalid/empty rows on import
- [x] Streaming I/O for large files (export_jsonl_streaming, import_jsonl_streaming)

### 4.10 Context Formatting
- [x] `ContextFormatter` with templates ({context}, {query}, {sources})
- [x] Citation insertion in formatted output
- [x] Configurable templates via raven.toml (ContextConfig)

### 4.11 CLI Extensions
- [x] `raven serve` — start HTTP server
- [x] `raven prompt "tekst"` — formatted LLM prompt
- [x] `raven watch <path>` — auto-reindex
- [x] `raven export` / `raven import` — JSONL backup/restore
- [x] `raven doctor` — diagnostics (check Ollama, db, config)
- [x] `raven mcp` — start MCP server
- [x] `raven benchmark` — performance measurement (index, query, hybrid, BM25)
- [x] `--hybrid`, `--verbose` flags on query

### 4.12 Configuration
- [x] `raven.toml` auto-discovery (walk up from cwd)
- [x] Env var overrides (RAVEN_DB, RAVEN_MODEL, RAVEN_API_KEY, etc.)
- [x] Unknown key warnings (typo protection)
- [x] Full config validation at startup

### 4.13 Docker & CI
- [x] Multi-stage Dockerfile (builder → debian-slim)
- [x] Static binary (`musl` target) for direct download — [#54](https://github.com/egkristi/ravenrustrag/issues/54)
- [x] GitHub Actions: test, lint (clippy), format (rustfmt), release
- [x] Container build and push to GHCR
- [x] Cross-compile for linux/amd64, linux/amd64-musl, linux/arm64 -- [#40](https://github.com/egkristi/ravenrustrag/issues/40)

### 4.14 Security Hardening

Findings from the security audit ([#1](https://github.com/egkristi/ravenrustrag/issues/1)–[#10](https://github.com/egkristi/ravenrustrag/issues/10)):

- [x] Configurable CORS origins (default to localhost) — [#1](https://github.com/egkristi/ravenrustrag/issues/1)
- [x] Rate limiting via tower middleware — [#2](https://github.com/egkristi/ravenrustrag/issues/2)
- [x] Query string length validation — [#3](https://github.com/egkristi/ravenrustrag/issues/3)
- [x] Generic error messages to clients (no internal leaks) — [#4](https://github.com/egkristi/ravenrustrag/issues/4)
- [x] Per-request timeout — [#5](https://github.com/egkristi/ravenrustrag/issues/5)
- [x] Option to put `/metrics` and `/stats` behind auth — [#6](https://github.com/egkristi/ravenrustrag/issues/6)
- [x] MCP write-operation access control — [#7](https://github.com/egkristi/ravenrustrag/issues/7)
- [x] Add SECURITY.md with vulnerability disclosure policy — [#8](https://github.com/egkristi/ravenrustrag/issues/8)
- [x] Expand `.dockerignore` — [#9](https://github.com/egkristi/ravenrustrag/issues/9)
- [x] Document TLS / reverse proxy requirement — [#10](https://github.com/egkristi/ravenrustrag/issues/10)

Already mitigated:
- [x] Constant-time auth comparison (`subtle::ConstantTimeEq`)
- [x] `unsafe_code = "forbid"` workspace-wide
- [x] Parameterized SQL (no injection)
- [x] Symlink traversal protection in file loader
- [x] `cargo-audit` in CI pipeline
- [x] 10MB request body limit

---

## 5. Phase 3: Rust Superiority

Features that make the Rust version **strictly better** than Python:

### 5.1 Advanced Retrieval
- [x] Parent-child retrieval (`query_parent()` — via VectorStore trait, no abstraction leaks)
- [x] Multi-collection routing (`MultiCollectionRouter`)
- [x] Streaming results (`query_stream()` — channel-based)
- [x] Multi-query expansion (`expand_query()` in raven-search/src/multi_query.rs)

### 5.2 Knowledge Graph
- [x] Entity extraction (regex-based NER in raven-search/src/graph.rs)
- [x] In-memory graph with JSON persistence (`KnowledgeGraph`)
- [x] Graph traversal (BFS with max_hops)
- [x] `GraphRetriever` — RRF fusion between graph and vector
- [x] `raven graph build` / `raven graph query` CLI commands — [#45](https://github.com/egkristi/ravenrustrag/issues/45)

### 5.3 Eval & Benchmarking
- [x] `evaluate()` — MRR, NDCG, Recall@k, Precision@k against ground truth
- [x] Criterion-based micro-benchmarks (crates/raven-search/benches/)
- [x] `raven benchmark` with detailed report (index speed, query latency, BM25)
- [x] CI-driven performance regression — [#46](https://github.com/egkristi/ravenrustrag/issues/46)

### 5.4 Observability
- [x] Tracing with `tracing` crate (structured logging)
- [x] Timing spans for all pipeline steps
- [x] `/metrics` endpoint with request counters
- [x] OpenTelemetry OTLP export (behind `otel` feature flag in raven-server)

### 5.5 HNSW Vector Search
- [x] HNSW via `instant-distance` (behind `hnsw` feature flag, default enabled)
- [x] O(log n) search instead of O(n)
- [x] Scalable to millions of documents — **much better than Python**

### 5.6 Performance Advantages
- [x] SIMD-friendly cosine similarity (auto-vectorized loop in raven-core)
- [x] Lock-free concurrent reads (DashMap + AtomicU64) — [#47](https://github.com/egkristi/ravenrustrag/issues/47)
- [x] Zero-copy memory-mapped SQLite (PRAGMA mmap_size=256MB) — [#48](https://github.com/egkristi/ravenrustrag/issues/48)
- [x] Batch embedding with parallelism (semaphore-limited concurrency)

---

## 6. Phase 4: Polish & Release

### 6.1 Documentation
- [x] rustdoc for all public items (crate-level docs + key types)
- [x] mdBook + MkDocs user guide (17 pages, GitHub Pages deployment) — [#49](https://github.com/egkristi/ravenrustrag/issues/49)
- [x] Migration guide from Python RavenRAG — [#50](https://github.com/egkristi/ravenrustrag/issues/50)
- [x] Performance comparisons vs Python version — [#51](https://github.com/egkristi/ravenrustrag/issues/51)
- [x] Troubleshooting section — [#49](https://github.com/egkristi/ravenrustrag/issues/49)

#### Measured Performance (Apple Silicon, release build)

| Benchmark | Result |
|-----------|--------|
| Cosine similarity 128-d | 39 ns |
| Cosine similarity 768-d | 220 ns |
| Cosine similarity 1536-d | 434 ns |
| Vector query (100 docs) | 35 µs |
| Vector query (1,000 docs) | 370 µs |
| Hybrid query (100 docs) | 55 µs |
| Index 10 docs | 41 µs |
| Release binary size | 8.7 MB |
| Lines of Rust | ~9,100 |
| Tests | 123 |

### 6.2 Publishing
- [ ] crates.io publish — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] `cargo install ravenrustrag` — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [x] GitHub Releases with pre-built binaries (linux, macos) — release.yml workflow
- [ ] Homebrew tap formula — [#55](https://github.com/egkristi/ravenrustrag/issues/55)
- [ ] AUR package — [#56](https://github.com/egkristi/ravenrustrag/issues/56)
- [ ] Shell completions (bash, zsh, fish) — [#62](https://github.com/egkristi/ravenrustrag/issues/62)

### 6.3 Quality
- [ ] 80%+ test coverage — [#53](https://github.com/egkristi/ravenrustrag/issues/53)
- [ ] Property-based testing (proptest) for splitters and search — [#58](https://github.com/egkristi/ravenrustrag/issues/58)
- [ ] Fuzzing for parsers and input handling — [#57](https://github.com/egkristi/ravenrustrag/issues/57)
- [ ] Concurrent stress tests — [#59](https://github.com/egkristi/ravenrustrag/issues/59)
- [ ] 10k+ document scaling test — [#59](https://github.com/egkristi/ravenrustrag/issues/59)

### 6.4 Stability
- [ ] SQLite schema versioning and automatic migrations — [#60](https://github.com/egkristi/ravenrustrag/issues/60)
- [ ] v1.0 stable release — [#61](https://github.com/egkristi/ravenrustrag/issues/61)

---

## Phase 5: Future

Features planned for post-1.0 development:

### 5F.1 LLM Generation
- [ ] Generator trait + Ollama/OpenAI backends — [#63](https://github.com/egkristi/ravenrustrag/issues/63)
- [ ] `raven ask <question>` — full RAG pipeline with answer generation
- [ ] `POST /ask` server endpoint with streaming SSE response
- [ ] Configurable system prompts and temperature

### 5F.2 ONNX Runtime (when ort crate is compatible)
- [ ] Local embedding without Ollama — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [ ] Cross-encoder reranking — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [ ] Quantized model support (int8/fp16)

### 5F.3 Advanced Features
- [ ] Incremental BM25 updates (avoid full rebuild)
- [ ] Async SQLite backend (tokio-rusqlite)
- [ ] Binary/quantized embedding storage (reduced disk/memory)
- [ ] WebSocket streaming endpoint
- [ ] Configuration hot-reload for long-running server

---

## 7. Known Limitations (Current)

1. **ONNX not functional** — Stub exists behind feature flag but `ort` crate has MSRV conflicts (requires reqwest 0.12+). [#43](https://github.com/egkristi/ravenrustrag/issues/43)
2. **No ONNX cross-encoder** — Reranker trait exists, but only keyword-based. Blocked by #43. [#44](https://github.com/egkristi/ravenrustrag/issues/44)
3. **No schema migrations** — Database schema changes require manual re-indexing. [#60](https://github.com/egkristi/ravenrustrag/issues/60)
4. **No LLM generation** — System formats prompts but cannot call LLMs directly. [#63](https://github.com/egkristi/ravenrustrag/issues/63)

## 7.1 Open Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#43](https://github.com/egkristi/ravenrustrag/issues/43) | ONNX Runtime embedding backend | High | Deferred (MSRV conflict) |
| [#44](https://github.com/egkristi/ravenrustrag/issues/44) | ONNX cross-encoder reranking | Medium | Deferred (blocked by #43) |
| [#52](https://github.com/egkristi/ravenrustrag/issues/52) | Publish to crates.io | High | Open |
| [#53](https://github.com/egkristi/ravenrustrag/issues/53) | 80%+ test coverage target | Medium | Open |
| [#55](https://github.com/egkristi/ravenrustrag/issues/55) | Homebrew tap for macOS | Medium | Open |
| [#56](https://github.com/egkristi/ravenrustrag/issues/56) | AUR package for Arch Linux | Low | Open |
| [#57](https://github.com/egkristi/ravenrustrag/issues/57) | Fuzz testing (cargo-fuzz) | Medium | Open |
| [#58](https://github.com/egkristi/ravenrustrag/issues/58) | Property-based testing (proptest) | Medium | Open |
| [#59](https://github.com/egkristi/ravenrustrag/issues/59) | Concurrent stress tests + 10k scaling | Medium | Open |
| [#60](https://github.com/egkristi/ravenrustrag/issues/60) | SQLite schema migrations | High | Open |
| [#61](https://github.com/egkristi/ravenrustrag/issues/61) | v1.0 stable release | High | Open (meta) |
| [#62](https://github.com/egkristi/ravenrustrag/issues/62) | Shell completions (bash/zsh/fish) | Low | Open |
| [#63](https://github.com/egkristi/ravenrustrag/issues/63) | LLM generation integration | Medium | Open |

### Resolved Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#11](https://github.com/egkristi/ravenrustrag/issues/11) | DELETE /documents/{doc_id} endpoint | High | Resolved |
| [#12](https://github.com/egkristi/ravenrustrag/issues/12) | Server and MCP integration tests | High | Resolved |
| [#13](https://github.com/egkristi/ravenrustrag/issues/13) | CLI --json output flag | Medium | Resolved |
| [#14](https://github.com/egkristi/ravenrustrag/issues/14) | SQLite WAL mode optimization | Medium | Resolved |
| [#15](https://github.com/egkristi/ravenrustrag/issues/15) | Complete OpenAPI 3.0 schema | Medium | Resolved |
| [#16](https://github.com/egkristi/ravenrustrag/issues/16) | PDF file loader support | Low | Resolved |
| [#32](https://github.com/egkristi/ravenrustrag/issues/32) | Remove unused ndarray dependency | Low | Resolved |
| [#33](https://github.com/egkristi/ravenrustrag/issues/33) | Fix misleading SIMD claims in README | Medium | Resolved |
| [#34](https://github.com/egkristi/ravenrustrag/issues/34) | Improve SentenceSplitter abbreviation handling | Medium | Resolved |
| [#35](https://github.com/egkristi/ravenrustrag/issues/35) | Add metadata filtering on vector search | High | Resolved |
| [#36](https://github.com/egkristi/ravenrustrag/issues/36) | Make batch sizes configurable | Medium | Resolved |
| [#37](https://github.com/egkristi/ravenrustrag/issues/37) | Persist BM25 index in SQLite | High | Resolved |
| [#38](https://github.com/egkristi/ravenrustrag/issues/38) | Add input sanitization for queries | Medium | Resolved |
| [#39](https://github.com/egkristi/ravenrustrag/issues/39) | Expand test coverage across crates | Medium | Resolved |
| [#40](https://github.com/egkristi/ravenrustrag/issues/40) | Musl static binary and arm64 CI | Low | Resolved |
| [#45](https://github.com/egkristi/ravenrustrag/issues/45) | Knowledge Graph CLI commands | Medium | Resolved |
| [#46](https://github.com/egkristi/ravenrustrag/issues/46) | CI-driven performance regression | Low | Resolved |
| [#47](https://github.com/egkristi/ravenrustrag/issues/47) | Lock-free concurrent reads (DashMap) | Low | Resolved |
| [#48](https://github.com/egkristi/ravenrustrag/issues/48) | Zero-copy deserialization + mmap SQLite | Low | Resolved |
| [#49](https://github.com/egkristi/ravenrustrag/issues/49) | mdBook + MkDocs user guide | Medium | Resolved |
| [#50](https://github.com/egkristi/ravenrustrag/issues/50) | Migration guide from Python | Low | Resolved |
| [#51](https://github.com/egkristi/ravenrustrag/issues/51) | Performance comparison docs | Low | Resolved |
| [#54](https://github.com/egkristi/ravenrustrag/issues/54) | Static musl binary distribution | Low | Resolved |

## 8. Build Instructions

```bash
# Clone
git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag

# Prerequisites
# macOS: xcode-select --install
# Ubuntu: sudo apt install build-essential pkg-config
# Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Test
cargo test

# Run CLI
./target/release/raven index ./docs --db ./raven.db
./target/release/raven query "What is RAG?"

# With Ollama
raven index ./docs --url http://localhost:11434 --model nomic-embed-text
```

---

**Last updated:** 2026-05-05  
**Next milestone:** v1.0 stable release (#61) — requires #52, #53, #55–#60