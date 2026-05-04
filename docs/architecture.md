# Architecture

RavenRustRAG is a Cargo workspace with 9 crates organized in a layered dependency hierarchy.

## Crate Overview

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

## Crate Responsibilities

### raven-core

Foundation types shared by all crates:

- `Document` — input document with content and metadata
- `Chunk` — text segment with embedding vector and metadata
- `SearchResult` — ranked result with score and source info
- `Config` — TOML-based configuration
- `RavenError` — unified error type using `thiserror`
- `fingerprint()` — content hashing for incremental indexing
- `cosine_similarity()` — optimized vector comparison

### raven-embed

Embedding abstraction layer:

- `Embedder` trait — async interface for text-to-vector conversion
- `OllamaEmbedder` — local inference via Ollama HTTP API
- `OpenAIEmbedder` — cloud inference via OpenAI API
- `DummyEmbedder` — deterministic fake embeddings for testing
- `EmbeddingCache` — lock-free DashMap cache with LRU-style eviction
- `CachedEmbedder` — transparent caching wrapper for any Embedder

### raven-split

Text chunking strategies:

- `Splitter` trait — async interface for text splitting
- `TextSplitter` — character-based with configurable overlap
- `SentenceSplitter` — respects sentence boundaries
- `TokenSplitter` — approximate token-based splitting
- `SemanticSplitter` — groups sentences by embedding similarity

### raven-load

Document ingestion:

- `Loader` — detects format by extension and loads documents
- Format handlers: text, markdown (with frontmatter), CSV, JSON, JSONL, HTML
- `export_jsonl()` / `import_jsonl()` — backup and restore
- Directory traversal with extension filtering

### raven-store

Vector storage backends:

- `VectorStore` trait — async CRUD + similarity search
- `SqliteStore` — persistent storage with WAL, mmap, cosine similarity
- `MemoryStore` — in-memory store for testing
- `HnswIndex` — approximate nearest neighbor index
- `MetadataFilter` — key-value filtering on search results
- Fingerprint tracking for incremental indexing

### raven-search

Pipeline orchestrator:

- `DocumentIndex` — builder-pattern coordinator (split → embed → store → search)
- `BM25Index` — keyword search with TF-IDF scoring
- `KnowledgeGraph` / `GraphRetriever` — entity extraction and graph traversal
- `MultiQueryExpander` — query expansion via keyword extraction
- `Reranker` trait + `KeywordReranker` — result re-scoring
- `EvalMetrics` — MRR, NDCG, Precision@k, Recall@k
- Watch mode with debounce for live re-indexing

### raven-server

HTTP API:

- Axum-based server with Tower middleware
- Bearer token authentication
- CORS configuration
- Rate limiting
- OpenAPI 3.0 schema generation
- Prometheus metrics endpoint
- Structured JSON responses

### raven-mcp

AI assistant integration:

- MCP (Model Context Protocol) server over stdio
- JSON-RPC 2.0 message handling
- Tool definitions: search, get_prompt, collection_info, index_documents
- Input validation and error reporting

### raven-cli

User-facing binary:

- Clap-derived command parser
- All commands: index, query, prompt, serve, watch, graph, info, clear, export, import, mcp, doctor, benchmark
- Structured and JSON output modes
- Config file loading

## Design Principles

1. **Thread-safe by default**: All public types are `Send + Sync`
2. **No unwrap in library code**: Proper error propagation with `?`
3. **Batch operations**: Embedding and storage work in batches for throughput
4. **Zero-copy where possible**: mmap for SQLite, `&str` parameters
5. **Builder pattern**: Complex types use builders (DocumentIndex)
6. **Trait-based abstraction**: All major components are behind traits for testability
7. **Lock-free concurrency**: DashMap + atomics where possible, Mutex only when necessary

## Data Flow

### Indexing Pipeline

```
Files → Loader → Documents → Splitter → Chunks → Embedder → Vectors
                                                                  ↓
                                                            VectorStore (SQLite)
                                                                  ↓
                                                            BM25 terms stored
```

### Query Pipeline

```
Query → Embedder → Query Vector → VectorStore.search() → Results
                                                              ↓
                  (optional) → BM25.search() → RRF fusion → Reranker → Final Results
```

### Knowledge Graph

```
Chunks → Entity Extraction → Graph Nodes + Edges → JSON file
                                                        ↓
Query → Entity Extraction → Graph Traversal → Related Chunks → Fusion with vector results
```
