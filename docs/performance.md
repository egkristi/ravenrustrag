# Performance Tuning

This guide covers techniques to maximize RavenRustRAG throughput and minimize latency.

## Embedding Cache

RavenRustRAG uses a lock-free in-memory embedding cache (DashMap) that avoids redundant embedding API calls. The cache is especially effective when:

- Re-indexing documents with overlapping content
- Running multiple queries with similar phrasing
- Using the `watch` command with incremental updates

Cache stats are available via:

```bash
ravenrag info --verbose
```

## SQLite Optimizations

The following PRAGMAs are applied automatically:

| PRAGMA | Value | Effect |
|--------|-------|--------|
| `journal_mode` | WAL | Concurrent reads during writes |
| `synchronous` | NORMAL | Faster writes (safe with WAL) |
| `cache_size` | -64000 | 64 MB page cache |
| `mmap_size` | 268435456 | 256 MB memory-mapped I/O |
| `busy_timeout` | 5000 | 5s retry on lock contention |

### WAL Mode

Write-Ahead Logging allows readers to proceed without blocking on writes. This is critical for the server where queries and indexing happen concurrently.

### Memory-Mapped I/O

With mmap enabled, SQLite reads bypass the userspace buffer and read directly from the OS page cache. This eliminates syscall overhead for hot data and is especially beneficial for large indexes.

## Batch Sizes

Embedding and storage operations are batched. Tune batch sizes based on your hardware:

- **Embedding batch**: Number of texts sent per API call (default varies by backend)
- **Concurrent batches**: Controlled by semaphore (default: 4 concurrent embedding calls)

Larger batches reduce API overhead but increase memory usage and latency per batch.

## Chunk Size Tuning

| Chunk Size | Pros | Cons |
|------------|------|------|
| 256 chars | Fine-grained retrieval, precise matches | More chunks to store/search, fragmented context |
| 512 chars (default) | Good balance of precision and context | — |
| 1024 chars | More context per result, fewer chunks | Less precise matches, may include irrelevant text |

For technical documentation, 512 is usually optimal. For narrative content, 1024 may work better.

## Hybrid Search Performance

BM25 search adds minimal overhead since terms are pre-computed during indexing. The RRF fusion step is O(n) in the number of results. For most use cases, hybrid search adds less than 1ms to query time.

## Hardware Recommendations

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| RAM | 512 MB | 2+ GB (for large indexes with mmap) |
| Storage | HDD works | SSD strongly recommended |
| CPU | Any x86_64/arm64 | Multi-core for concurrent embedding |

## Benchmarking

Use the built-in benchmark command to measure your system's performance:

```bash
ravenrag benchmark --num-docs 1000 --iterations 100
```

This generates synthetic documents, indexes them, and measures query latency percentiles.

## Monitoring

The HTTP server exposes `/metrics` in Prometheus format with:

- Request count by endpoint
- Latency histograms
- Active connections
- Cache hit/miss rates
