# Changelog

All notable changes to RavenRustRAG are documented here.

This project follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- `raven ask` command for local LLM question-answering via Ollama (#63)
- Generator trait and OllamaGenerator for LLM text generation with streaming
- `raven completions` command for shell completion generation (bash, zsh, fish, elvish, PowerShell) (#62)
- Schema migration system with versioned upgrades for SqliteStore (#60)
- Property-based tests with proptest for core, split, and search crates (#58)
- Stress tests for concurrent indexing and large document handling (#59)
- Fuzz targets for text splitter, all loaders, and cosine similarity (#57)
- mdBook + MkDocs documentation site
- DashMap lock-free embedding cache (#47)
- Memory-mapped I/O for SQLite (256 MB mmap) (#48)
- CI concurrency groups to prevent queue flooding (#46)
- Knowledge graph CLI commands (`raven graph build`, `raven graph query`) (#45)
- Multi-target release binaries (linux-amd64, linux-musl, linux-arm64, darwin-amd64, darwin-arm64) (#54)
- GitHub Actions CI with fmt, clippy, test, MSRV, bench stages
- Docker workflow with GHCR publishing
- CodeQL security scanning

### Changed
- EmbeddingCache internals: Mutex<HashMap> replaced with DashMap + AtomicU64
- Semaphore error in batch embedding now returns proper RavenError instead of panic

### Fixed
- Clippy compliance: removed unwrap in library code

## [0.1.0-alpha.1] — Initial Release

### Added
- Cargo workspace with 9 crates
- Core types: Document, Chunk, SearchResult, Config, RavenError
- Embedding backends: Ollama, OpenAI, DummyEmbedder
- Embedding cache with configurable max size
- CachedEmbedder wrapper
- Text splitters: TextSplitter, SentenceSplitter, TokenSplitter, SemanticSplitter
- File loaders: txt, md (with frontmatter), csv, json, jsonl, html
- JSONL export/import
- Vector stores: SqliteStore (WAL, persistent), MemoryStore (testing)
- HNSW approximate nearest neighbor index
- Metadata filtering on search results
- DocumentIndex builder pattern pipeline orchestrator
- BM25 keyword search with persistent term storage
- Hybrid search with RRF (Reciprocal Rank Fusion)
- Knowledge graph: entity extraction, graph building, traversal, graph-vector fusion
- Multi-query expansion via keyword extraction
- Reranker trait with KeywordReranker
- Evaluation metrics: MRR, NDCG, Precision@k, Recall@k
- Watch mode with debounce for live re-indexing
- Multi-collection router
- Parent-child retrieval
- Axum HTTP server with auth, CORS, rate limiting, OpenAPI schema, metrics
- MCP server (stdio JSON-RPC) with search, prompt, info, index tools
- CLI: index, query, prompt, serve, watch, graph, info, clear, export, import, mcp, doctor, benchmark
- Incremental indexing via content fingerprinting
- Criterion benchmarks
- Pre-commit hooks (fmt, clippy, test)
- Docker multi-stage build (Alpine builder, scratch runtime)

[Unreleased]: https://github.com/egkristi/ravenrustrag/compare/v0.1.0-alpha.1...HEAD
[0.1.0-alpha.1]: https://github.com/egkristi/ravenrustrag/releases/tag/v0.1.0-alpha.1
