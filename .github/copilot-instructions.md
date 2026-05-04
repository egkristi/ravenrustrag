# RavenRustRAG — AI Agent Instructions

## Language

All code, comments, documentation, commit messages, and plan files must be written in **English only**.

## Project Overview

RavenRustRAG is a local-first RAG (Retrieval-Augmented Generation) engine written in Rust.
It is the successor to the Python [RavenRAG](https://github.com/egkristi/ravenrag) project
and must be **strictly superior** in every dimension: faster, safer, smaller, and feature-complete.

## Architecture

Cargo workspace with 9 crates:

| Crate | Purpose |
|---|---|
| `raven-core` | Document, Chunk, SearchResult, Config, RavenError, fingerprint |
| `raven-embed` | Embedder trait + Ollama, OpenAI, DummyEmbedder backends + cache |
| `raven-store` | VectorStore trait + SqliteStore, MemoryStore |
| `raven-split` | Splitter trait + TextSplitter |
| `raven-load` | Loader + file format support (txt, md, csv, json, html) + JSONL export/import |
| `raven-search` | DocumentIndex — pipeline orchestrator (split → embed → store → search) |
| `raven-server` | Axum HTTP API server (auth, CORS, /query, /prompt, /index, /openapi.json) |
| `raven-mcp` | MCP server (stdio JSON-RPC) for AI assistants |
| `raven-cli` | CLI binary: index, query, prompt, serve, info, clear, export, import, mcp, doctor |

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

- **Edition**: Rust 2021, MSRV 1.75
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

- **Push to git for each completed working feature**
- Commit messages: `feat: <description>`, `fix: <description>`, `refactor: <description>`
- Always run `cargo test` before committing
- Always run `cargo clippy` before pushing
- Format: `git add -A && git commit -m "<message>" && git push`

## File Formats Supported

| Extension | Loader | Notes |
|---|---|---|
| `.txt` | Text (fallback) | Plain text, any extension not matched |
| `.md` | Markdown | YAML frontmatter parsed into metadata |
| `.csv` | CSV | Headers become key-value pairs |
| `.json` | JSON | Pretty-printed for chunking |
| `.jsonl` | JSONL | Each line as separate record |
| `.html` | HTML | Tags stripped, scripts/styles removed |

## API Endpoints (raven-server)

| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/health` | No | Health check |
| GET | `/stats` | No | Index statistics |
| GET | `/openapi.json` | No | OpenAPI 3.0 schema |
| POST | `/query` | Yes | Search documents |
| POST | `/prompt` | Yes | LLM-ready prompt |
| POST | `/index` | Yes | Add documents |

Auth is Bearer token via `Authorization` header, only required when `RAVEN_API_KEY` is set.

## MCP Tools (raven-mcp)

| Tool | Description |
|---|---|
| `search` | Query the index with top_k |
| `get_prompt` | Search + format LLM prompt |
| `collection_info` | Index statistics |
| `index_documents` | Add documents |

## CLI Commands

```
raven index <path>     # Index documents
raven query <text>     # Search
raven prompt <text>    # LLM-formatted prompt
raven serve            # Start HTTP server
raven info             # Show stats
raven clear            # Clear index
raven export           # JSONL backup
raven import <file>    # JSONL restore
raven mcp              # Start MCP server
raven doctor           # Diagnostics
```

## Environment Variables

| Variable | Purpose |
|---|---|
| `RAVEN_API_KEY` | API authentication key |
| `RAVEN_DB` | Default database path |
| `RAVEN_MODEL` | Default embedding model |

## Reference: Python RavenRAG v0.7.0

The Python version has ~4,200 lines across 24 modules. Key features to match or exceed:
- Knowledge graph retrieval (Fase 3)
- BM25 hybrid search with RRF (Fase 2)
- Cross-encoder reranking (Fase 2)
- Watch mode with debounce (Fase 2)
- Multi-collection routing (Fase 3)
- Parent-child retrieval (Fase 3)
- Eval metrics: MRR, NDCG, Recall@k (Fase 3)

See PLAN.md for the full roadmap.
