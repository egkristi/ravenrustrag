# RavenRustRAG — AI Agent Instructions

## Language

All code, comments, documentation, commit messages, and plan files must be written in **English only**.

## Writing Style

- Be conservative with emoji/emoticon use in documentation and README files. Avoid emoji-per-line patterns (e.g. feature tables with an emoji on every row). A project icon in the title is fine; emoji walls are not.
- Write in a clear, professional tone. Let the content speak for itself.

## Ollama Policy

- **No cloud models.** Ollama must run with `OLLAMA_NO_CLOUD=1` at all times.
- Only use local models (pulled and stored on disk). Never configure or invoke remote/cloud inference through Ollama.
- The embedding model used by default is `nomic-embed-text` (local).

## Project Overview

RavenRustRAG is a local-first RAG (Retrieval-Augmented Generation) engine written in Rust.
It is the successor to the Python [RavenRAG](https://github.com/egkristi/ravenrag) project
and must be **strictly superior** in every dimension: faster, safer, smaller, and feature-complete.

## Architecture

Cargo workspace with 10 crates (~14,600 lines of Rust, 273 tests):

| Crate | Purpose |
|---|---|
| `raven-core` | Document, Chunk, SearchResult, Config, RavenError, fingerprint, cosine_similarity |
| `raven-embed` | Embedder + Generator traits, Ollama, OpenAI, vLLM, LiteLLM, Http, ONNX, DummyEmbedder backends + DashMap cache |
| `raven-store` | VectorStore trait + SqliteStore (WAL, HNSW, backup), MemoryStore, MetadataFilter |
| `raven-split` | Splitter trait + Text, Token, Sentence, Semantic splitters |
| `raven-load` | Loader + file format support (txt, md, csv, json, jsonl, html, pdf, docx) + plugin registry |
| `raven-search` | DocumentIndex, BM25, KnowledgeGraph, MultiCollectionRouter, Reranker, eval metrics |
| `raven-server` | Axum HTTP API (auth, CORS, rate limit, SSE streaming /ask, WebSocket, OpenAPI) |
| `raven-mcp` | MCP server (stdio JSON-RPC, tools + resources + prompts) |
| `raven-cli` | CLI binary: 20 commands |
| `ravenrustrag` | Top-level library crate with stable public API re-exports |

## Dependency Flow

```
raven-core  (no internal deps)
  ↑
raven-embed (depends on raven-core)
raven-split (depends on raven-core)
raven-load  (depends on raven-core)
raven-store (depends on raven-core)
  ↑
raven-search (depends on core, embed, store, split)
  ↑
raven-server (depends on core, embed, store, split, search)
raven-mcp    (depends on core, search, split)
  ↑
raven-cli    (depends on all)
```

## Coding Standards

- **Edition**: Rust 2021, MSRV 1.88
- **Async runtime**: Tokio (full features)
- **Error handling**: `thiserror` for library errors (`RavenError`), `anyhow` only in CLI
- **Traits**: Use `async-trait` for async trait methods. All traits must be `Send + Sync`
- **Serialization**: `serde` + `serde_json` for all data types
- **HTTP**: `reqwest` for client, `axum` for server
- **Logging**: `tracing` crate. Use `info!`, `warn!`, `error!` — never `println!` in libraries
- **Tests**: Unit tests in each crate. Use `MemoryStore` + `DummyEmbedder` for test isolation

## Key Design Principles

1. **Thread-safe by default**: All public types must be `Send + Sync`
2. **No unwrap in library code**: Use `?` and proper error types
3. **Batch operations**: Embedding and storage should work in batches
4. **Zero-copy where possible**: Use `&str` over `String` in function parameters
5. **Builder pattern**: For complex types (DocumentIndex, etc.)
6. **Feature flags**: Optional dependencies behind cargo features
7. **No ChromaDB**: SQLite is the primary store (unlike Python version)

## Build & Test

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo test               # Run all tests
cargo clippy             # Lint
cargo fmt --check        # Format check
```

## Git Workflow

- **Push to git for each completed working feature or resolved issue**
- **No direct commits to `main` from external contributors.** All external changes must go through a Pull Request.
- All planned changes and issues must be tracked as GitHub Issues before work begins
- Commit messages: `feat: <description>`, `fix: <description>`, `refactor: <description>`
- Reference GitHub Issues in commits (e.g. `feat: add batch embedding #12`, `fix: musl build failure closes #15`)
- Always run `cargo test` before committing
- Always run `cargo clippy` before pushing
- Format: `git add -A && git commit -m "<message>" && git push`
- **After every push**: Check GitHub Actions at https://github.com/egkristi/ravenrustrag/actions for pipeline failures. If any workflow (CI, Docker, CodeQL) fails, diagnose the error and create a GitHub Issue at https://github.com/egkristi/ravenrustrag/issues describing the problem and proposed fix.

## File Formats Supported

| Extension | Loader | Notes |
|---|---|---|
| `.txt` | Text (fallback) | Plain text, any extension not matched |
| `.md`, `.markdown` | Markdown | YAML frontmatter parsed into metadata |
| `.csv` | CSV | Headers become key-value pairs |
| `.json` | JSON | Pretty-printed for chunking |
| `.jsonl`, `.ndjson` | JSONL | Each line as separate record |
| `.html`, `.htm` | HTML | Tags stripped, scripts/styles removed |
| `.docx` | DOCX | ZIP-based XML extraction (behind `docx` feature, default on) |
| `.pdf` | PDF | Text extraction via pdf-extract (behind `pdf` feature, default on) |
| Custom | Plugin | Register via `LoaderRegistry::register()` |

## API Endpoints (raven-server)

| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/health` | No | Liveness probe |
| GET | `/ready` | No | Readiness probe (checks DB) |
| GET | `/stats` | Conditional | Index statistics |
| GET | `/collections` | No | List collections |
| GET | `/metrics` | Conditional | Server metrics |
| GET | `/openapi.json` | No | OpenAPI 3.0.3 schema |
| POST | `/query` | Yes | Search documents (vector/hybrid/filtered) |
| POST | `/prompt` | Yes | LLM-ready prompt with citations |
| POST | `/ask` | Yes | RAG Q&A with SSE streaming (source/token/done) |
| POST | `/index` | Yes | Add documents (disabled in read-only mode) |
| DELETE | `/documents/:id` | Yes | Delete document (disabled in read-only mode) |
| GET | `/ws` | No | WebSocket (search, prompt, ping) |

Auth is Bearer token via `Authorization` header, only required when `RAVEN_API_KEY` is set.
Server supports `--read-only` mode which returns 403 for write endpoints.

## MCP Capabilities (raven-mcp)

### Tools
| Tool | Description |
|---|---|
| `search` | Query the index with top_k |
| `get_prompt` | Search + format LLM prompt |
| `collection_info` | Index statistics |
| `index_documents` | Add documents |

### Resources
| URI | Description |
|---|---|
| `raven://index/stats` | Index statistics (chunk count, model) |

### Prompts
| Name | Description |
|---|---|
| `rag_answer` | Generate answer from retrieved context |
| `summarize_index` | Summarize index contents |

Supports `--filter` to restrict exposed tools.

## CLI Commands

```
ravenrag index <path>       # Index documents
ravenrag query <text>       # Search (--hybrid, --explain)
ravenrag ask <text>         # RAG Q&A with local LLM
ravenrag prompt <text>      # LLM-formatted prompt
ravenrag serve              # Start HTTP server (--read-only)
ravenrag info               # Show stats
ravenrag status             # Health dashboard
ravenrag clear              # Clear index
ravenrag export             # JSONL backup
ravenrag import <file>      # JSONL restore
ravenrag backup <file>      # SQLite backup API
ravenrag mcp                # Start MCP server (--filter)
ravenrag doctor             # Diagnostics
ravenrag watch <path>       # Auto-reindex on changes
ravenrag benchmark          # Performance benchmarks
ravenrag graph build        # Build knowledge graph
ravenrag graph query <text> # Query knowledge graph
ravenrag init               # Generate raven.toml
ravenrag diff <path>        # Show changed files
ravenrag completions <sh>   # Shell completions
```

## Environment Variables

| Variable | Purpose |
|---|---|
| `RAVEN_API_KEY` | API authentication key |
| `RAVEN_DB` | Default database path |
| `RAVEN_MODEL` | Default embedding model |
| `RAVEN_HOST` | Server bind host |
| `RAVEN_PORT` | Server bind port |
| `RAVEN_EMBED_BACKEND` | Embedding backend selection |
| `RAVEN_EMBED_URL` | Embedding service URL |
| `RAVEN_CORS_ORIGINS` | Allowed CORS origins |
| `RAVEN_RATE_LIMIT` | Rate limit per second |
| `RAVEN_REQUEST_TIMEOUT` | Request timeout in seconds |
| `RAVEN_MAX_QUERY_LENGTH` | Max query string length |
| `RAVEN_LOG_FORMAT` | Log format (text/json) |

## Reference: Python RavenRAG v0.7.0

The Python version has ~4,200 lines across 24 modules. All key features have been matched or exceeded:
- Knowledge graph retrieval — Done (Phase 3)
- BM25 hybrid search with RRF — Done (Phase 2)
- Cross-encoder reranking — Done (ONNX, Phase 5)
- Watch mode with debounce — Done (Phase 2)
- Multi-collection routing — Done (Phase 3)
- Parent-child retrieval — Done (Phase 3)
- Eval metrics: MRR, NDCG, Recall@k — Done (Phase 3)

RavenRustRAG is now ~14,600 lines across 10 crates with 273 tests.

See PLAN.md for the full roadmap.
