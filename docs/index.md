# RavenRustRAG

RavenRustRAG is a local-first RAG (Retrieval-Augmented Generation) engine written in Rust. It provides millisecond document indexing, microsecond query latency, and ships as a single static binary with zero runtime dependencies.

## What is RAG?

Retrieval-Augmented Generation combines document retrieval with LLM prompting. Instead of feeding an entire corpus to a language model, RAG retrieves the most relevant chunks and provides them as context. This reduces hallucinations, keeps responses grounded in your data, and works within token limits.

## Key Features

- **Vector search** with cosine similarity over local embeddings
- **Hybrid search** combining BM25 keyword matching with vector retrieval (RRF fusion)
- **Knowledge graph** for entity-relationship traversal
- **HTTP API** (Axum) with Bearer auth and OpenAPI schema
- **MCP server** for AI assistant integration (Claude, etc.)
- **CLI** for scripting and automation
- **Docker** support with minimal scratch-based images

## Architecture

RavenRustRAG is a Cargo workspace with 9 crates:

| Crate | Purpose |
|-------|---------|
| `raven-core` | Document, Chunk, SearchResult, Config, RavenError |
| `raven-embed` | Embedder trait + Ollama/OpenAI/Dummy backends + cache |
| `raven-store` | VectorStore trait + SQLite/Memory stores |
| `raven-split` | Splitter trait + text/sentence/token splitters |
| `raven-load` | File loaders (txt, md, csv, json, html) + JSONL export |
| `raven-search` | DocumentIndex pipeline orchestrator |
| `raven-server` | Axum HTTP API server |
| `raven-mcp` | MCP server (stdio JSON-RPC) |
| `raven-cli` | CLI binary |

## Getting Started

See [Installation](./installation.md) to get the binary, then [Quick Start](./quickstart.md) to index your first documents.
