# RavenRustRAG

> Fearlessly fast retrieval. Zero-cost intelligence.

**A local-first, embeddable RAG engine in Rust** — the successor to [RavenRAG](https://github.com/egkristi/ravenrag), reimagined for speed, safety, and deployability.

Sub-millisecond vector search. Single static binary. No Python. No virtual environments. No GIL.

[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)
[![CI](https://github.com/egkristi/ravenrustrag/actions/workflows/ci.yml/badge.svg)](https://github.com/egkristi/ravenrustrag/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/egkristi/ravenrustrag/graph/badge.svg)](https://codecov.io/gh/egkristi/ravenrustrag)
[![Docs](https://img.shields.io/badge/docs-egkristi.github.io-blueviolet)](https://egkristi.github.io/ravenrustrag/)

## Features

| Feature | Description |
|---------|-------------|
| **Blazing fast** | Compiled native code, zero-copy where possible. Sub-50ms startup, 35 µs queries |
| **Local-first** | No cloud APIs required. Works fully offline with Ollama, vLLM, or any OpenAI-compatible backend |
| **Single binary** | `cargo install ravenrustrag` or download a static musl binary (~9MB) |
| **True async** | Built on Tokio. Thousands of concurrent queries with lock-free reads |
| **Pluggable storage** | SQLite with WAL + mmap (default), in-memory, or custom backend |
| **Hybrid search** | Dense vectors + BM25 keyword matching with Reciprocal Rank Fusion |
| **HNSW search** | O(log n) approximate nearest neighbor via `instant-distance` |
| **Reranking** | Keyword reranker + ONNX cross-encoder backend (behind `onnx` feature) |
| **Knowledge graph** | Entity extraction + graph traversal + graph-vector fusion |
| **Semantic chunking** | Sentence-boundary + embedding cosine similarity splitting |
| **Flexible splitting** | Character, token-aware, and semantic strategies |
| **File loaders** | txt, md, html, csv, json, jsonl, pdf, docx + plugin system |
| **Metadata filtering** | Filter search results by arbitrary key-value metadata |
| **Parent-child** | Search chunks, return full parent documents |
| **Multi-collection** | Route queries across multiple indices with fused top-k |
| **Context formatting** | LLM-ready prompt generation with source citations |
| **Retrieval eval** | Built-in MRR, NDCG, Recall@k, Precision@k metrics |
| **CLI** | 20 commands: index, query, ask, prompt, serve, watch, backup, mcp, doctor, benchmark, graph, etc. |
| **HTTP API** | Axum server with auth, CORS, rate limit, timeout, body limit, OpenAPI |
| **MCP server** | Model Context Protocol for Claude, Copilot, Cursor (tools + resources + prompts) |
| **Embedding backends** | Ollama, OpenAI-compatible, auto-detect |
| **Watch mode** | Auto-reindex on file changes with debounce and delete tracking |
| **Streaming** | `query_stream()` yields results via async channel |
| **Incremental indexing** | SHA-256 fingerprinting, skip unchanged files |
| **Export/import** | JSONL backup and restore with streaming I/O |
| **Lock-free cache** | DashMap + AtomicU64 embedding cache, zero contention |
| **Memory-mapped I/O** | 256 MB mmap for zero-copy SQLite reads |
| **Observability** | Tracing spans, `/metrics` endpoint, OpenTelemetry export |
| **Thread-safe** | All types are `Send + Sync` by default. No data races possible |
| **Config file** | `raven.toml` + env vars + CLI flags, auto-discovery |

## Installation

### Download pre-built binary

Pre-built binaries are available for every release on the [GitHub Releases](https://github.com/egkristi/ravenrustrag/releases) page:

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `raven-linux-amd64` |
| Linux x86_64 (static) | `raven-linux-amd64-musl` |
| Linux ARM64 | `raven-linux-arm64` |
| macOS x86_64 | `raven-darwin-amd64` |
| macOS ARM64 (Apple Silicon) | `raven-darwin-arm64` |
| Windows x86_64 | `raven-windows-amd64.exe` |
| Windows ARM64 | `raven-windows-arm64.exe` |

**Linux / macOS:**

```bash
# Download (replace URL with latest release)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-linux-amd64
chmod +x raven-linux-amd64
sudo mv raven-linux-amd64 /usr/local/bin/raven
```

**Windows (PowerShell):**

```powershell
# Download the latest release
Invoke-WebRequest -Uri "https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-windows-amd64.exe" -OutFile "raven.exe"
# Move to a directory in your PATH, or add the current directory to PATH
Move-Item raven.exe "$env:LOCALAPPDATA\Microsoft\WindowsApps\raven.exe"
```

### Install from source

```bash
git clone https://github.com/egkristi/ravenrustrag
cd ravenrustrag
cargo build --release
```

## Quick Start

```bash
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
raven watch ./docs --extensions md,txt

# Export/import
raven export -o backup.jsonl
raven import backup.jsonl

# Diagnostics
raven doctor

# MCP server (for Claude, Copilot, Cursor)
raven mcp

# MCP with restricted tools
raven mcp --filter search,get_prompt

# Ask a question (RAG + LLM)
raven ask "What is RAG?"

# Create a database backup
raven backup ./raven-backup.db

# Query with detailed scoring
raven query "auth" --explain

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
│  .pdf    │ Sentence │           │          │  Hybrid (RRF)       │
│  .docx   │ Semantic │           │          │  Knowledge graph    │
│  .html   │          │           │          │  Parent-child       │
│  .csv    │          │           │          │  Multi-collection   │
│  .json   │          │           │          │  Streaming          │
│  plugin  │          │           │          │  Reranking          │
└──────────┴──────────┴───────────┴──────────┴─────────────────────┘
```

### Crate Structure

```
ravenrustrag/
├── Cargo.toml           # Workspace
├── mkdocs.yml           # Documentation site config
├── book.toml            # mdBook config (same source)
├── docs/                # User guide (17 pages, published to GitHub Pages)
├── crates/
│   ├── raven-core/      # Document, Chunk, SearchResult, Config, errors
│   ├── raven-embed/     # Embedder trait + Ollama, OpenAI backends + DashMap cache
│   ├── raven-store/     # VectorStore trait + SQLite (WAL, mmap), Memory backends
│   ├── raven-split/     # Splitter trait + Text, Token, Sentence, Semantic
│   ├── raven-load/      # Loader trait + file loaders + plugin registry
│   ├── raven-search/    # DocumentIndex, BM25, HNSW, KnowledgeGraph, Reranker
│   ├── raven-server/    # Axum HTTP API (auth, CORS, rate limit, /metrics, /openapi)
│   ├── raven-cli/       # CLI binary (20 commands)
│   └── raven-mcp/       # MCP server (stdio JSON-RPC, tools + resources + prompts)
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
  index /docs --extensions md,txt
```

## Configuration

Create `raven.toml` in your project root:

```toml
[embedder]
backend = "ollama"           # "ollama" or "openai"
model = "nomic-embed-text"
url = "http://localhost:11434"

[store]
backend = "sqlite"
path = "./raven.db"

[splitter]
kind = "text"                # "text", "token", "semantic"
chunk_size = 512
chunk_overlap = 50

[pipeline]
embed_batch_size = 64
store_batch_size = 100

[server]
host = "127.0.0.1"
port = 8484
# api_key = "your-secret-key"
# cors_origins = ["http://localhost:3000"]
request_timeout_secs = 60
rate_limit_per_second = 100
```

### Environment Variables

| Variable | Overrides | Default |
|----------|-----------|---------|
| `RAVEN_DB` | `store.path` | `./raven.db` |
| `RAVEN_MODEL` | `embedder.model` | `nomic-embed-text` |
| `RAVEN_API_KEY` | `server.api_key` | None |
| `RAVEN_HOST` | `server.host` | `127.0.0.1` |
| `RAVEN_PORT` | `server.port` | `8484` |
| `RAVEN_EMBED_BACKEND` | `embedder.backend` | `ollama` |
| `RAVEN_EMBED_URL` | `embedder.url` | `http://localhost:11434` |
| `RAVEN_CORS_ORIGINS` | `server.cors_origins` | Localhost only |
| `RAVEN_RATE_LIMIT` | `server.rate_limit_per_second` | `100` |
| `RAVEN_REQUEST_TIMEOUT` | `server.request_timeout_secs` | `60` |
| `RAVEN_MAX_QUERY_LENGTH` | `server.max_query_length` | `10000` |
| `RAVEN_LOG_FORMAT` | — | `text` |

CLI flags override env vars. Env vars override config file. Config file overrides defaults.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Liveness probe |
| GET | `/ready` | Readiness probe (checks DB) |
| GET | `/stats` | Index statistics |
| GET | `/collections` | List collections |
| GET | `/metrics` | Request counts, uptime, errors |
| GET | `/openapi.json` | OpenAPI 3.0.3 schema |
| POST | `/query` | Semantic/hybrid/filtered search |
| POST | `/prompt` | LLM-ready formatted prompt with citations |
| POST | `/ask` | RAG Q&A with SSE streaming (source + token + done events) |
| POST | `/index` | Add documents (disabled in read-only mode) |
| DELETE | `/documents/{doc_id}` | Delete a document (disabled in read-only mode) |
| GET | `/ws` | WebSocket (search, prompt, ping) |

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

### ONNX Runtime

Local embedding and cross-encoder reranking without Ollama, using ONNX Runtime:

```bash
# Build with ONNX support (requires Rust 1.88+)
cargo build --release --features onnx
```

> **Note:** The `onnx` feature requires the ONNX Runtime and is optional.
> All features (including `onnx`) work on Rust 1.88+.

## MCP Server

Expose RavenRustRAG as a Model Context Protocol server for AI assistants:

```bash
raven mcp
```

**Capabilities:**

| Capability | Methods |
|---|---|
| Tools | `search`, `get_prompt`, `collection_info`, `index_documents` |
| Resources | `raven://index/stats` — browseable index metadata |
| Prompts | `rag_answer`, `summarize_index` — prompt templates |

Use `--filter` to expose only specific tools:
```bash
raven mcp --filter search,get_prompt
```

Works with Claude Desktop, GitHub Copilot, Cursor, and any MCP-compatible client.

## Benchmarks

Measured on Apple Silicon (M-series), release build, using `DummyEmbedder` (128-dim) to isolate compute from network latency. Run with `cargo bench`. Last verified: 2026-05-05.

### Cosine Similarity (raven-core)

| Dimension | Latency | Throughput |
|-----------|---------|------------|
| 128-d | 39 ns | ~25M ops/s |
| 768-d | 217 ns | ~4.6M ops/s |
| 1536-d | 430 ns | ~2.3M ops/s |

### Search & Indexing (raven-search)

| Operation | Latency |
|-----------|---------|
| Vector query, 100 docs | 35 µs |
| Vector query, 1,000 docs | 378 µs |
| Hybrid query (BM25 + vector), 100 docs | 55 µs |
| BM25 search, 1,000 docs | 58 µs |
| Index 10 docs (split + embed + store) | 41 µs |

Benchmarks depend on hardware, embedding model, and document size. Query latency above excludes embedding time (network-bound for Ollama/OpenAI).

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

#### Resolved Security Issues

All initial security findings ([#1](https://github.com/egkristi/ravenrustrag/issues/1)–[#10](https://github.com/egkristi/ravenrustrag/issues/10)) have been addressed:
Configurable CORS, rate limiting, query length validation, generic error messages, request timeouts, authenticated metrics, MCP access control, SECURITY.md, .dockerignore, and TLS/reverse proxy documentation.

## Documentation

Full documentation is available at **[egkristi.github.io/ravenrustrag](https://egkristi.github.io/ravenrustrag/)**.

Covers: installation, quick start, CLI reference, HTTP API, MCP server, configuration, hybrid search, knowledge graph, Docker deployment, performance tuning, troubleshooting, and migration from Python.

## Roadmap

See [PLAN.md](PLAN.md) for the detailed roadmap. See [docs/changelog.md](docs/changelog.md) for completed work.

- [x] **Phase 1** — Core engine (Document, Chunk, SQLite store, Ollama embeddings, CLI)
- [x] **Phase 2** — HTTP API, MCP server, hybrid search, file loaders, watch mode, export/import, security hardening, BM25 persistence, metadata filtering, input sanitization
- [x] **Phase 3** — HNSW search, knowledge graph, multi-query expansion, lock-free cache, mmap SQLite, CI benchmarks, streaming, multi-collection, parent-child retrieval
- [x] **Phase 4** — Integration tests, top-level library crate, HNSW auto-rebuild, coverage gate, embeddings versioning, read-only mode, MCP validation, stable API surface
- [x] **Phase 5** — ONNX embeddings, ONNX cross-encoder, WebSocket streaming, plugin system, `/ask` SSE streaming, MCP resources/prompts, backup, query explain
- [ ] **Release** — crates.io publish, Homebrew tap, AUR package, v1.0 stable (#61)

## Building from Source

```bash
# Prerequisites
# macOS: xcode-select --install
# Ubuntu: sudo apt install build-essential pkg-config
# Windows: Install Visual Studio Build Tools (C++ workload)
# Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#   (Windows: download rustup-init.exe from https://rustup.rs)

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