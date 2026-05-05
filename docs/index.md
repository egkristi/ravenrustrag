# RavenRustRAG

RavenRustRAG is a local-first RAG (Retrieval-Augmented Generation) engine written in Rust. It provides millisecond document indexing, microsecond query latency, and ships as a single static binary with zero runtime dependencies.

## What is RAG?

Retrieval-Augmented Generation combines document retrieval with LLM prompting. Instead of feeding an entire corpus to a language model, RAG retrieves the most relevant chunks and provides them as context. This reduces hallucinations, keeps responses grounded in your data, and works within token limits.

## Key Features

- **Vector search** with cosine similarity over local embeddings
- **HNSW index** for O(log n) approximate nearest neighbor search
- **Hybrid search** combining BM25 keyword matching with vector retrieval (RRF fusion)
- **Knowledge graph** for entity-relationship traversal and graph-vector fusion
- **Semantic splitting** via embedding cosine similarity between sentences
- **Lock-free caching** with DashMap for concurrent embedding cache access
- **Memory-mapped SQLite** with 256 MB mmap for zero-copy reads
- **HTTP API** (Axum) with Bearer auth, rate limiting, CORS, and OpenAPI schema
- **MCP server** for AI assistant integration (Claude, Copilot, Cursor)
- **CLI** with 20 commands for scripting and automation
- **File watching** with debounce for automatic re-indexing
- **Incremental indexing** via SHA-256 content fingerprinting
- **Docker** support with minimal scratch-based images (~15 MB)

## Architecture

RavenRustRAG is a Cargo workspace with 10 crates:

| Crate | Purpose |
|-------|---------|
| `raven-core` | Document, Chunk, SearchResult, Config, RavenError, cosine similarity |
| `raven-embed` | Embedder trait + Ollama/OpenAI/Dummy backends + DashMap cache |
| `raven-store` | VectorStore trait + SQLite (WAL, mmap) / Memory / HNSW stores |
| `raven-split` | Splitter trait + Text/Sentence/Token/Semantic splitters |
| `raven-load` | File loaders (txt, md, csv, json, html, pdf, docx) + JSONL export |
| `raven-search` | DocumentIndex, BM25, KnowledgeGraph, Reranker, MultiQuery, Eval |
| `raven-server` | Axum HTTP API (auth, CORS, rate limit, SSE streaming, WebSocket, metrics, OpenAPI) |
| `raven-mcp` | MCP server (stdio JSON-RPC, tools + resources + prompts) |
| `raven-cli` | CLI binary (20 commands) |
| `ravenrustrag` | Top-level library crate with stable public API re-exports |

## Getting Started

See [Installation](./installation.md) to get the binary, then [Quick Start](./quickstart.md) to index your first documents.
