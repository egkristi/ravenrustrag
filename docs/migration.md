# Migration from Python RavenRAG

This guide helps users of the Python [RavenRAG](https://github.com/egkristi/ravenrag) (v0.7.0) migrate to RavenRustRAG.

## Why Migrate?

| Dimension | Python RavenRAG | RavenRustRAG |
|-----------|----------------|--------------|
| Startup time | ~2 seconds | <10 ms |
| Query latency | 50-200 ms | 1-5 ms |
| Memory usage | 200+ MB | 15-30 MB |
| Deployment | Python + venv + ChromaDB | Single static binary |
| Concurrency | GIL-limited | Full multi-threaded |
| Type safety | Runtime errors | Compile-time guarantees |
| Docker image | 800+ MB | <20 MB |

## Key Differences

### Storage Backend

- **Python**: ChromaDB (separate process or embedded)
- **Rust**: SQLite with WAL mode (embedded, no separate process)

There is no ChromaDB support in RavenRustRAG. SQLite is the only persistent backend.

### Embedding Models

Both versions support Ollama with `nomic-embed-text` as the default. The embedding dimensions and similarity scores are compatible.

### Configuration

**Python** (YAML):
```yaml
embedding:
  provider: ollama
  model: nomic-embed-text
chunking:
  chunk_size: 512
  overlap: 50
```

**Rust** (TOML or CLI flags):
```toml
[embedding]
backend = "ollama"
model = "nomic-embed-text"

[pipeline]
chunk_size = 512
chunk_overlap = 50
```

### CLI Commands

| Python | Rust | Notes |
|--------|------|-------|
| `ravenrag index <path>` | `raven index <path>` | Same |
| `ravenrag query <text>` | `raven query <text>` | Same |
| `ravenrag serve` | `raven serve` | Different default port (8484 vs 8000) |
| `ravenrag collections list` | `raven info` | Simplified |
| — | `raven watch <path>` | New: file watching |
| — | `raven graph build` | New: knowledge graph |
| — | `raven mcp` | New: MCP server |
| — | `raven doctor` | New: diagnostics |
| — | `raven benchmark` | New: benchmarking |

### API Changes

The HTTP API is largely compatible but with some differences:

| Python endpoint | Rust endpoint | Changes |
|----------------|---------------|---------|
| `POST /search` | `POST /query` | Renamed, same body format |
| `POST /index` | `POST /index` | Same |
| `GET /health` | `GET /health` | Same |
| `GET /collections` | `GET /stats` | Different response format |

### Features Not Yet Ported

Some Python features are planned but not yet implemented:

- ONNX embedding backends (planned)
- Cross-encoder reranking with ONNX models (planned)
- Multi-collection routing (basic support exists)
- Parent-child retrieval (implemented)

### New Features in Rust

- Hybrid BM25+vector search with RRF fusion
- Knowledge graph retrieval
- MCP server for AI assistants
- File watching with debounce
- Incremental indexing via fingerprinting
- Lock-free concurrent cache
- Memory-mapped SQLite reads
- Prometheus metrics
- Built-in benchmarking
- JSONL export/import

## Migration Steps

### 1. Export from Python

If you have indexed data in ChromaDB, export it:

```bash
# From Python RavenRAG
ravenrag export --output backup.jsonl
```

### 2. Install Rust Version

```bash
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-linux-amd64
chmod +x raven-linux-amd64 && mv raven-linux-amd64 /usr/local/bin/raven
```

### 3. Re-index Documents

Since the storage backends are incompatible (ChromaDB vs SQLite), re-index from source:

```bash
raven index ./your-documents/ --extensions md,txt,html
```

### 4. Verify

```bash
raven info
raven query "test query from your domain"
raven doctor
```

### 5. Update Integrations

- Update API URL and port (default changed to 8484)
- Update endpoint names (`/search` → `/query`)
- MCP integration is new — add to Claude/VS Code config

## Performance Comparison

After migration, run benchmarks to verify improvements:

```bash
raven benchmark --num-docs 500 --iterations 100
```

Typical improvements observed:
- 10-50x faster query latency
- 5-10x faster indexing throughput
- 10x less memory usage
- <1% of Docker image size
