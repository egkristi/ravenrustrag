# Changelog

All notable changes to RavenRustRAG are documented here.

This project follows [Semantic Versioning](https://semver.org/).

## [1.0.0] — 2026-05-05

### Completed Phases

All development phases are complete. RavenRustRAG is functionally superior to the Python RavenRAG v0.7.0.

**Phase 4 — Polish & Release:**
Publishing (crates.io metadata, publish workflow, Homebrew formula, AUR PKGBUILD, MCP marketplace listing), Quality (80%+ test coverage, Codecov badge, CLI integration tests, HTTP server integration tests, verified benchmarks), Architecture (top-level library crate with builder API), CLI (`ravenrag init`, `ravenrag diff`), Robustness (embeddings versioning, `--read-only` mode, MCP JSON Schema validation), Performance (HNSW index replacing O(n) scan), API Stability (stable public API surface, ONNX MSRV split, v1.0 release).

**Phase 5 — Advanced Features:**
LLM Generation (`POST /ask` with SSE streaming citations), ONNX Runtime (local embedding, cross-encoder reranking, quantized model support), MCP (resources capability, prompts capability, `--filter` scoping), CLI (`--explain` scoring, `ravenrag backup`), Advanced (incremental BM25, async SQLite, binary/quantized embedding storage, WebSocket streaming, config hot-reload, plugin system).

**Phase 6 — Distribution & Packaging:**
Windows (winget, Chocolatey, Scoop, portable ZIP), macOS (Homebrew), Linux (APT/deb, RPM, AUR, Zypper, APK, Snap, Flatpak, Nix, AppImage, static musl binary), Cross-platform (crates.io, GitHub Releases, Docker multi-arch, OCI, Helm chart, curl install script).

### Added
- `POST /ask` SSE streaming citations with typed events: source, token, error, done
- `ravenrag serve --read-only` mode for production deployments
- MCP resources/list + resources/read capabilities (raven://index/stats)
- MCP prompts/list + prompts/get capabilities (rag_answer, summarize_index)
- `ravenrag mcp --filter <expr>` to restrict exposed tools
- MCP JSON Schema constraints on tools/list (additionalProperties, min/max bounds)
- `ravenrag query --explain` for detailed scoring breakdown
- `ravenrag backup <file>` via SQLite backup API
- Embeddings versioning — model + dimensions stored, dimension mismatch rejected
- HNSW index auto-rebuilds at store open, eliminates O(n) flat scan (#79)
- Stable public API surface via ravenrustrag crate re-exports (#83)
- CI coverage percentage threshold gate at 70% (#81)
- CI ONNX feature gate job (#80)
- Verified benchmark numbers in README (#82)
- HttpEmbedder for custom embedding backends via generic HTTP API (plugin system) (#77)
- WebSocket endpoint `/ws` for real-time streaming search and prompt (#76)
- 37 new tests across all crates, including Unicode edge cases (#53)
- `ravenrag diff` command to show changes since last index (#78)
- `ravenrag ask` command for local LLM question-answering via Ollama (#63)
- Generator trait and OllamaGenerator for LLM text generation with streaming
- `ravenrag completions` command for shell completion generation (bash, zsh, fish, elvish, PowerShell) (#62)
- `ravenrag status` command for rich index health dashboard (#74)
- `--dry-run` mode for `ravenrag index` (#71)
- Colored CLI output with term highlighting (#73)
- Schema migration system with versioned upgrades for SqliteStore (#60)
- Property-based tests with proptest for core, split, and search crates (#58)
- Stress tests for concurrent indexing and large document handling (#59)
- Fuzz targets for text splitter, all loaders, and cosine similarity (#57)

### Changed
- `create_embedder` and `create_cached_embedder` now support "http" backend (#77)
- All Cargo.toml files now include crates.io metadata (homepage, repository, keywords, categories) (#52)

### Fixed
- Unicode text splitter bug where multi-byte chars at chunk boundaries produced empty chunks (#53)
- `ravenrag diff` macOS path canonicalization issue with `/var/folders` vs `/private/var/folders` (#78)
- mdBook + MkDocs documentation site
- HNSW integration in SqliteStore for O(log n) vector search (#64)
- `VectorStore::get_by_doc_id()` for efficient parent-child retrieval (#65)
- Split read/write connections in SqliteStore for concurrent reads (#69)
- LRU cache eviction via moka crate (#68)
- Auto-detect embedding dimension from model (#67)
- Full `raven.toml` config wiring in CLI (#66)
- DashMap lock-free embedding cache (#47)
- Memory-mapped I/O for SQLite (256 MB mmap) (#48)
- CI concurrency groups to prevent queue flooding (#46)
- Knowledge graph CLI commands (`ravenrag graph build`, `ravenrag graph query`) (#45)
- Multi-target release binaries (linux-amd64, linux-musl, linux-arm64, darwin-amd64, darwin-arm64) (#54)
- GitHub Actions CI with fmt, clippy, test, MSRV, bench stages
- Docker workflow with GHCR publishing
- CodeQL security scanning

### Changed
- Default `--extensions` for CLI index/watch commands now includes all supported formats (txt,md,csv,json,html,pdf,docx)
- EmbeddingCache internals: Mutex<HashMap> replaced with DashMap + AtomicU64
- SqliteStore uses separate read/write connections for better concurrency
- Semaphore error in batch embedding now returns proper RavenError instead of panic

### Fixed
- SqliteStore search now uses HNSW for O(log n) instead of O(n) brute-force (#64)
- `query_parent()` no longer loads entire database (#65)
- Embedding dimension no longer hardcoded to 768 (#67)
- CLI now reads and applies raven.toml configuration (#66)
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

[1.0.0]: https://github.com/egkristi/ravenrustrag/compare/v0.1.0-alpha.1...v1.0.0
[0.1.0-alpha.1]: https://github.com/egkristi/ravenrustrag/releases/tag/v0.1.0-alpha.1
