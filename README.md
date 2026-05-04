# RavenRustRAG

> Fearlessly fast retrieval. Zero-cost intelligence.

**A local-first, embeddable RAG engine in Rust** — the successor to [RavenRAG](https://github.com/egkristi/ravenrag), reimagined for speed, safety, and deployability.

Index thousands of documents in milliseconds. Query with microsecond latency. Ship as a single static binary. No Python. No virtual environments. No GIL.

[![Rust](https://img.shields.io/badge/rust-1.86%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)

## Why Rust instead of Python?

RavenRAG (Python) proved the concept. RavenRustRAG delivers on the promise.

| | Python (RavenRAG v0.7.0) | Rust (RavenRustRAG) |
|---|---|---|
| **Startup** | 2–5s (import + model init) | <50ms |
| **Query latency** | 50–200ms | 1–10ms |
| **Memory baseline** | 200–500MB+ | 20–50MB |
| **Deployment** | virtualenv + 4 core deps + optional extras | Single `raven` binary (~15MB) |
| **Concurrency** | GIL-bounded, fake async | Tokio, true async, lock-free reads |
| **Type safety** | Runtime `TypeError`, `AttributeError` | Compile-time `Send + Sync` guarantees |
| **Thread safety** | None — concurrent requests can corrupt state | `Arc<RwLock>`, fearless concurrency |
| **Docker image** | ~1.5GB (Python + model + deps) | ~20MB (static musl binary) |
| **Install** | `pip install ravenrag[all]` + pray | `cargo install ravenrustrag` |

## Features

**Everything RavenRAG can do, plus more — faster, safer, smaller.**

| Feature | Description | vs Python |
|---------|-------------|-----------|
| **Blazing fast** | SIMD vector ops, zero-copy where possible | 10–100x faster |
| **Local-first** | No cloud APIs required. Works fully offline | Parity |
| **Single binary** | `cargo install` or download. That's it | No virtualenv |
| **True async** | Built on Tokio. Thousands of concurrent queries | Not `asyncio.to_thread` wrappers |
| **Pluggable storage** | SQLite (default), in-memory, or custom backend | Parity (no ChromaDB dep) |
| **Hybrid search** | Dense vectors + BM25 keyword matching with RRF fusion | Persistable BM25 index |
| **Reranking** *(planned)* | ONNX-based cross-encoder (no Python runtime needed) | Native, not sentence-transformers |
| **Semantic chunking** *(planned)* | Sentence-boundary + embedding similarity splitting | Parity |
| **Flexible splitting** | Character, token-aware, and semantic strategies | Parity |
| **File loaders** | txt, md, html, csv, json, jsonl + plugin system | pdf, docx planned |
| **Metadata filtering** | Filter search results by arbitrary metadata | Parity |
| **Parent-child** | Search chunks, return full parent documents | Clean trait-based (no abstraction leak) |
| **Context formatting** | LLM-ready prompt generation with citations | Parity |
| **Citations** | Full provenance: source file + chunk reference | Parity |
| **Retrieval eval** | Built-in MRR, NDCG, Recall@k, Precision@k metrics | Parity+ |
| **CLI** | `raven index`, `query`, `serve`, `watch`, `mcp`, `doctor`, `benchmark` | 12 commands |
| **HTTP API** | Axum server with auth, CORS, rate limit, timeout, OpenAPI | Body limit, graceful shutdown |
| **MCP server** | Model Context Protocol for Claude, Copilot, Cursor | Parity |
| **Embedding backends** | Ollama, OpenAI-compatible, auto-detect | ONNX planned |
| **Watch mode** | Auto-reindex on file changes (debounce + delete) | Parity |
| **Config file** | `raven.toml` + env vars, auto-discovery | Parity |
| **Incremental indexing** | SHA-256 fingerprinting, skip unchanged files | Parity |
| **Export/import** | JSONL backup and restore (streaming I/O) | Streaming, not load-all |
| **Multi-collection** | Route queries across multiple indices | Fused top-k |
| **Knowledge graph** *(planned)* | Entity extraction + graph traversal retrieval | Phase 3 |
| **Observability** | Tracing spans, `/metrics` endpoint | Structured logging |
| **Streaming** | `query_stream()` yields results via channel | Phase 3: further streaming |
| **Thread-safe** | All types are `Send + Sync` by default | Python has none |
| **HNSW search** *(planned)* | O(log n) approximate nearest neighbor | Phase 3 |

## Quick Start

```bash
# Install from source
git clone https://github.com/egkristi/ravenrustrag
cd ravenrustrag
cargo build --release

# Or (when published):
cargo install ravenrustrag

# Index your documents
raven index ./docs --db ./raven.db

# Query
raven query "What is retrieval-augmented generation?"

# Hybrid search
raven query "auth flow" --hybrid -k 10

# Get LLM-ready prompt
raven prompt "Explain RAG" -k 3

# Start API server
raven serve --port 8484

# Watch and auto-reindex
raven watch ./docs --extensions ".md,.txt"

# Export/import
raven export -o backup.jsonl
raven import backup.jsonl

# Diagnostics
raven doctor

# MCP server (for Claude, Copilot, Cursor)
raven mcp

# Show index stats
raven info
```

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                          RavenRustRAG                             │
├──────────────────────────────────────────────────────────────────┤
│  CLI │ Axum HTTP Server │ MCP Server (stdio) │ Library (crate)   │
├──────────────────────────────────────────────────────────────────┤
│  Pipeline: load → split → embed → store → search → rerank → fmt │
├──────────┬──────────┬───────────┬──────────┬─────────────────────┤
│ Loaders  │ Splitters│ Embedders │  Stores  │ Search & Retrieval  │
│  .txt    │  Text    │  Ollama   │  SQLite  │  Vector (HNSW)      │
│  .md     │  Token   │  OpenAI   │  Memory  │  BM25 keyword       │
│  .pdf    │ Semantic │  ONNX     │  Custom  │  Hybrid (RRF)       │
│  .docx   │          │  Custom   │          │  Cross-encoder      │
│  .html   │          │           │          │  Graph traversal    │
│  .csv    │          │           │          │  Parent-child       │
│  plugin  │          │           │          │  Multi-collection   │
└──────────┴──────────┴───────────┴──────────┴─────────────────────┘
```

### Crate Structure

```
ravenrustrag/
├── Cargo.toml           # Workspace
├── crates/
│   ├── raven-core/      # Document, Chunk, SearchResult, Config, errors
│   ├── raven-embed/     # Embedder trait + Ollama, OpenAI, ONNX backends
│   ├── raven-store/     # VectorStore trait + SQLite, Memory backends
│   ├── raven-split/     # Splitter trait + Text, Token, Semantic
│   ├── raven-load/      # Loader trait + file loaders + plugin registry
│   ├── raven-search/    # DocumentIndex, HybridSearcher, Reranker, Graph
│   ├── raven-server/    # Axum HTTP API (auth, CORS, /metrics, /openapi.json)
│   ├── raven-cli/       # CLI binary (11 commands)
│   └── raven-mcp/       # MCP server (stdio JSON-RPC)
```

## Library Usage

```rust
use ravenrustrag::{DocumentIndex, TextSplitter, Loader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load documents
    let docs = Loader::from_directory("./docs").await?;
    
    // Split into chunks
    let splitter = TextSplitter::new().with_chunk_size(512);
    let chunks = splitter.split(docs);
    
    // Build index
    let mut index = DocumentIndex::builder()
        .store("./raven.db")
        .embedder("ollama://nomic-embed-text")
        .build().await?;
    
    index.add(chunks).await?;
    
    // Query
    let results = index.query("What is RAG?", 5).await?;
    for result in results {
        println!("[{:.4}] {}", result.score, result.text);
    }

    // Hybrid search
    let results = index.hybrid_query("auth flow", 10, 0.5).await?;

    // LLM-ready prompt with citations
    let prompt = index.query_for_prompt("Explain RAG", 3).await?;
    
    Ok(())
}
```

## Docker

```bash
# Static binary — tiny image (~20MB vs Python's ~1.5GB)
docker pull ghcr.io/egkristi/ravenrustrag:main

# Run with persistent data
docker run -d \
  --name raven \
  -p 8484:8484 \
  -v raven-data:/data \
  ghcr.io/egkristi/ravenrustrag:main

# With API key
docker run -d \
  -p 8484:8484 \
  -v raven-data:/data \
  -e RAVEN_API_KEY=my-secret \
  ghcr.io/egkristi/ravenrustrag:main

# Index local documents
docker run --rm \
  -v raven-data:/data \
  -v ./my-docs:/docs:ro \
  ghcr.io/egkristi/ravenrustrag:main \
  index /docs --glob "**/*.md"
```

## Configuration

Create `raven.toml` in your project root:

```toml
[embedder]
backend = "ollama"           # "ollama", "openai", "onnx"
model = "nomic-embed-text"
url = "http://localhost:11434"

[store]
backend = "sqlite"
path = "./raven.db"

[splitter]
kind = "text"                # "text", "token", "semantic"
chunk_size = 512
chunk_overlap = 50

[search]
top_k = 5
rerank = false
hybrid = false
alpha = 0.5                  # 1.0 = pure vector, 0.0 = pure BM25

[server]
host = "127.0.0.1"
port = 8484
# api_key = "your-secret-key"

[watch]
extensions = [".md", ".txt", ".pdf"]
```

### Environment Variables

| Variable | Overrides | Default |
|----------|-----------|---------|
| `RAVEN_DB` | `store.path` | `./raven.db` |
| `RAVEN_MODEL` | `embedder.model` | `nomic-embed-text` |
| `RAVEN_API_KEY` | `server.api_key` | None |
| `RAVEN_HOST` | `server.host` | `127.0.0.1` |
| `RAVEN_PORT` | `server.port` | `8484` |
| `RAVEN_TOP_K` | `search.top_k` | `5` |

CLI flags override env vars. Env vars override config file. Config file overrides defaults.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/stats` | Index statistics |

| GET | `/metrics` | Timing and cache statistics |
| GET | `/openapi.json` | OpenAPI 3.0 schema |
| POST | `/query` | Semantic/hybrid search |
| POST | `/prompt` | LLM-ready formatted prompt |
| POST | `/index` | Add documents |

```bash
# Search
curl -X POST http://localhost:8484/query \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer my-secret" \
  -d '{"query": "What is RAG?", "top_k": 3, "hybrid": true}'

# LLM prompt
curl -X POST http://localhost:8484/prompt \
  -d '{"query": "Explain embeddings", "top_k": 3}'

# Health
curl http://localhost:8484/health
```

## Embedding Backends

### Ollama (default, local)

```toml
[embedder]
backend = "ollama"
model = "nomic-embed-text"
url = "http://localhost:11434"
```

### OpenAI-compatible (OpenAI, LM Studio, LocalAI, vLLM)

```toml
[embedder]
backend = "openai"
model = "text-embedding-3-small"
url = "https://api.openai.com/v1"
api_key = "sk-..."
```

### ONNX Runtime (planned)

Native ONNX embedding support is planned for a future release — no Python runtime needed.

## MCP Server

Expose RavenRustRAG as a Model Context Protocol server for AI assistants:

```bash
raven mcp
```

**Tools exposed:**
- `search` — Query the index with configurable top_k
- `get_prompt` — Search and format an LLM-ready prompt
- `collection_info` — Index statistics
- `index_documents` — Add documents to the index

Works with Claude Desktop, GitHub Copilot, Cursor, and any MCP-compatible client.

## Benchmarks

Expected on Apple Silicon / AMD Ryzen:

| Operation | RavenRustRAG | RavenRAG (Python) | Speedup |
|-----------|-------------|-------------------|---------|
| Startup | <50ms | 2–5s | ~50x |
| Index 1k docs | ~1s | ~30s | ~30x |
| Query (cold) | ~5ms | ~200ms | ~40x |
| Query (cached) | ~0.5ms | ~50ms | ~100x |
| Memory baseline | ~30MB | ~300MB | ~10x |
| Binary/image size | ~15MB | ~1.5GB | ~100x |

Benchmarks are approximate and depend on hardware, embedding model, and document size.

## Security

- **Thread-safe by default** — All types are `Send + Sync`. No data races possible.
- **Constant-time auth** — Bearer token comparison uses `subtle::ConstantTimeEq` to prevent timing attacks.
- **Auth** — Bearer token authentication via `RAVEN_API_KEY` env var or config.
- **Symlink protection** — `load_directory()` skips symlinks pointing outside target.
- **Request size limit** — Server rejects payloads over 10MB.
- **Parameterized SQL** — All SQLite queries use parameterized statements (no injection).
- **No unsafe code** — `unsafe_code = "forbid"` enforced workspace-wide.
- **Dependency auditing** — `cargo-audit` runs in CI on every push.
- **TLS** — Server does not terminate TLS. Use a reverse proxy (nginx, Caddy) for HTTPS ([#10](https://github.com/egkristi/ravenrustrag/issues/10)).

### Known Security Issues

The following have been identified and tracked. Contributions welcome:

| Issue | Severity | Description |
|-------|----------|-------------|
| [#1](https://github.com/egkristi/ravenrustrag/issues/1) | Medium | CORS is overly permissive (`allow_origin(Any)`) |
| [#2](https://github.com/egkristi/ravenrustrag/issues/2) | Medium | No rate limiting on API endpoints |
| [#3](https://github.com/egkristi/ravenrustrag/issues/3) | Medium | No input validation on query string length |
| [#4](https://github.com/egkristi/ravenrustrag/issues/4) | Low-Medium | Error responses may leak internal details |
| [#5](https://github.com/egkristi/ravenrustrag/issues/5) | Medium | No request timeout on server handlers |
| [#6](https://github.com/egkristi/ravenrustrag/issues/6) | Low | `/metrics` and `/stats` unauthenticated |
| [#7](https://github.com/egkristi/ravenrustrag/issues/7) | Low-Medium | MCP `index_documents` allows unauthenticated writes |
| [#8](https://github.com/egkristi/ravenrustrag/issues/8) | Low | No SECURITY.md with disclosure policy |
| [#9](https://github.com/egkristi/ravenrustrag/issues/9) | Low | `.dockerignore` should exclude more files |
| [#10](https://github.com/egkristi/ravenrustrag/issues/10) | Low–High | No TLS — needs reverse proxy documentation |

## Roadmap

See [PLAN.md](PLAN.md) for the detailed implementation plan.

- [x] **v0.1.0-alpha** — Core engine (Document, Chunk, SQLite store, Ollama embeddings, CLI)
- [ ] **v0.2.0** — HTTP API, MCP server, hybrid search, file loaders, watch mode, export/import, security hardening ([#1](https://github.com/egkristi/ravenrustrag/issues/1)–[#10](https://github.com/egkristi/ravenrustrag/issues/10))
- [ ] **v0.3.0** — Cross-encoder reranking, semantic splitting, PDF/DOCX loaders, ONNX embeddings
- [ ] **v0.4.0** — Knowledge graph, parent-child, multi-collection routing, streaming
- [ ] **v0.5.0** — HNSW search, benchmarks, eval metrics, observability
- [ ] **v1.0.0** — Stable API, crates.io, pre-built binaries, docs, Homebrew

## Building from Source

```bash
# Prerequisites
# macOS: xcode-select --install
# Ubuntu: sudo apt install build-essential pkg-config
# Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag
cargo build --release
cargo test
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **No direct commits to `main`.** All changes must go through a Pull Request.
2. Fork the repository and create a feature branch (`feat/my-feature` or `fix/my-bug`).
3. Ensure your code passes all checks before submitting:
   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets -- -D warnings
   cargo test --all
   ```
4. Write clear commit messages using conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`.
5. Open a PR against `main` with a description of your changes.

## License

Dual licensed: [AGPLv3](LICENSES/AGPLv3.txt) + [Commercial](LICENSES/COMMERCIAL.txt). See [LICENSING.md](LICENSING.md) for details.

---

Built with 🦀 by [Erling Kristiansen](https://github.com/egkristi).  
Successor to [RavenRAG](https://github.com/egkristi/ravenrag) (Python) — same vision, 100x the speed.