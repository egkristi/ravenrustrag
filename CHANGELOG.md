# Changelog

All notable changes to RavenRustRAG will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **CLI**: `raven ask` command for local LLM question-answering via Ollama (#63)
- **Embed**: Generator trait and OllamaGenerator for LLM text generation with streaming
- **CLI**: `raven completions` command for shell completion generation (bash, zsh, fish, elvish, PowerShell) (#62)
- **Store**: Schema migration system with versioned upgrades (#60)
- **Testing**: Property-based tests with proptest for core, split, and search crates (#58)
- **Testing**: Stress tests for concurrent indexing and large document handling (#59)
- **Fuzzing**: Fuzz targets for text splitter, all loaders, and cosine similarity (#57)

## [0.1.0-alpha.1] - 2026-05-04

### Added
- **Core**: Document, Chunk, SearchResult, Config, RavenError, Fingerprint types
- **Embedding**: Embedder trait with Ollama and OpenAI-compatible backends
- **Embedding**: EmbeddingCache with SHA-256 deduplication
- **Storage**: VectorStore trait with SqliteStore and MemoryStore backends
- **Storage**: Fingerprint tracking for incremental indexing
- **Splitting**: TextSplitter, TokenSplitter, SentenceSplitter strategies
- **Loading**: File loaders for txt, md, csv, json, jsonl, html
- **Loading**: JSONL export/import for backup and restore
- **Search**: DocumentIndex pipeline orchestrator (split → embed → store → search)
- **Search**: BM25 hybrid search with Reciprocal Rank Fusion (`--hybrid` flag)
- **Search**: Eval metrics: MRR, NDCG, Recall@k, Precision@k
- **Server**: Axum HTTP API with Bearer auth, CORS, OpenAPI schema
- **Server**: Endpoints: `/health`, `/stats`, `/metrics`, `/query`, `/prompt`, `/index`, `/openapi.json`
- **Server**: Request body limit (10MB), graceful shutdown, request metrics
- **MCP**: JSON-RPC stdio server with `search`, `get_prompt`, `collection_info`, `index_documents` tools
- **CLI**: Commands: `index`, `query`, `prompt`, `info`, `serve`, `clear`, `export`, `import`, `mcp`, `doctor`, `watch`
- **Config**: `raven.toml` with environment variable overrides and auto-discovery
- **Indexing**: Incremental indexing with SHA-256 fingerprints (skip unchanged files)
- **Watch**: Directory watch mode with debounce and delete tracking
- **Observability**: Tracing spans for all pipeline steps
- **Infrastructure**: Dockerfile (multi-stage, non-root, healthcheck)
- **Infrastructure**: GitHub Actions CI (check, fmt, clippy, test, release artifact)
- **Documentation**: Contributing guidelines, PR-only policy for external contributors

[Unreleased]: https://github.com/egkristi/ravenrustrag/compare/v0.1.0-alpha.1...HEAD
[0.1.0-alpha.1]: https://github.com/egkristi/ravenrustrag/releases/tag/v0.1.0-alpha.1
