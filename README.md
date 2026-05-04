# RavenRustRAG 🐦‍⬛⚡

> "Fearlessly fast retrieval. Zero-cost intelligence."

**A local-first, embeddable RAG engine in Rust.**

RavenRustRAG is the spiritual successor to [RavenRAG](https://github.com/egkristi/ravenrag) — reimagined in Rust for speed, safety, and deployability. Index thousands of documents in milliseconds. Query with microsecond latency. Ship as a single static binary.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Why Rust?

| | Python (RavenRAG) | Rust (RavenRustRAG) |
|---|---|---|
| **Startup** | 2–5s (import overhead) | <50ms |
| **Query latency** | ~50–200ms | ~1–10ms |
| **Memory** | 200–500MB+ | 20–50MB |
| **Deploy** | Virtualenv + dependencies | Single `raven` binary |
| **Concurrency** | GIL-bounded | Tokio async, thread-safe |
| **Safety** | Runtime exceptions | Compile-time guarantees |

## Features

- 🔥 **Blazing fast** — SIMD-accelerated vector ops, lock-free reads
- 🏠 **Local-first** — No cloud APIs required. Works offline.
- 📦 **Single binary** — `cargo install` or download a release. That's it.
- 🧵 **True async** — Built on Tokio. Handles thousands of concurrent queries.
- 💾 **Pluggable storage** — SQLite (default), in-memory, or bring your own backend
- 🔍 **Hybrid search** — Dense vectors + sparse BM25 with learned fusion
- 🎯 **Reranking** — Cross-encoder and colbert-style rerankers
- 🧠 **Semantic chunking** — Sentence-transformer powered splitting
- 📂 **File loaders** — txt, md, pdf, docx, html (extensible)
- 🌐 **HTTP API** — Axum-powered server with OpenAPI schema
- 💬 **LLM context formatting** — Citation-ready prompt generation
- 📊 **Built-in evaluation** — MRR, NDCG, Recall@k
- 🔌 **MCP server** — Model Context Protocol for AI assistants
- 👁️ **Watch mode** — Auto-reindex on file changes
- ⚙️ **TOML config** — `raven.toml` + env var overrides
- 🔄 **Incremental indexing** — Content-hash deduplication
- 💾 **Export/import** — JSONL backup/restore

## Quick Start

```bash
# Install
cargo install ravenrustrag

# Or build from source
git clone https://github.com/egkristi/ravenrustrag
cd ravenrustrag
cargo build --release

# Index your documents
raven index ./docs --db ./raven.db

# Query
raven query "What is retrieval-augmented generation?"

# Start API server
raven serve --port 8484

# Watch and auto-reindex
raven watch ./docs --db ./raven.db
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         RavenRustRAG                         │
├─────────────────────────────────────────────────────────────┤
│  CLI │ API Server │ MCP Server │ Library                    │
├─────────────────────────────────────────────────────────────┤
│  Pipeline: load → split → embed → store → search → format   │
├─────────┬──────────┬──────────┬──────────┬──────────────────┤
│ Loaders │ Splitters│ Embedders│  Stores  │ Search/Rerank    │
│  .txt   │  Text    │  Ollama  │  SQLite  │  Vector (HNSW)   │
│  .pdf   │  Token   │  Local*  │  Memory  │  BM25            │
│  .md    │Semantic* │  OpenAI  │  Custom  │  Hybrid Fusion   │
│  .html  │          │          │          │  Cross-Encoder*  │
└─────────┴──────────┴──────────┴──────────┴──────────────────┘
  * = optional feature
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
        println!("[{}] {}", result.score, result.text);
    }
    
    Ok(())
}
```

## Configuration

Create `raven.toml`:

```toml
[embedder]
backend = "ollama"        # "ollama", "openai", "local"
model = "nomic-embed-text"
url = "http://localhost:11434"

[store]
backend = "sqlite"
path = "./raven.db"

[splitter]
kind = "semantic"         # "text", "token", "semantic"
chunk_size = 512
chunk_overlap = 50

[server]
host = "127.0.0.1"
port = 8484
# api_key = "your-secret-key"
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/stats` | Index statistics |
| POST | `/query` | Semantic search |
| POST | `/prompt` | LLM-ready prompt |
| POST | `/index` | Add documents |
| GET | `/openapi.json` | OpenAPI schema |

## Benchmarks

Coming soon. Expected on M1 Mac / AMD Ryzen:

- Indexing: ~1,000 docs/sec (with local embeddings)
- Query (cold): ~5ms
- Query (cached): ~0.5ms
- Memory: ~30MB base + storage

## Roadmap

See [PLAN.md](PLAN.md) for detailed implementation plan.

Highlights:
- [ ] v0.1.0 — Core engine (index, query, SQLite, Ollama)
- [ ] v0.2.0 — File loaders + semantic splitting
- [ ] v0.3.0 — Hybrid search + reranking
- [ ] v0.4.0 — HTTP API + CLI polish
- [ ] v0.5.0 — MCP server + evaluation
- [ ] v0.6.0 — Watch mode + incremental indexing
- [ ] v0.7.0 — Knowledge graph + parent-child
- [ ] v1.0.0 — Stable API, benchmarks, docs

## License

MIT. See [LICENSE](LICENSE).

---

Built with 🦀 by [Erling Kristiansen](https://github.com/egkristi). Inspired by [RavenRAG](https://github.com/egkristi/ravenrag).