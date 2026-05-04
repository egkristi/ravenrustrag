# Troubleshooting

Common issues and solutions when using RavenRustRAG.

## Ollama Connection Errors

### "Failed to connect to Ollama"

**Symptoms**: Indexing or querying fails with a connection error.

**Solutions**:

1. Verify Ollama is running:
   ```bash
   curl http://localhost:11434/api/tags
   ```

2. Start Ollama if not running:
   ```bash
   OLLAMA_NO_CLOUD=1 ollama serve
   ```

3. If using a non-default URL, pass `--url`:
   ```bash
   raven query "test" --url http://my-host:11434
   ```

4. Run diagnostics:
   ```bash
   raven doctor
   ```

### "Model not found"

**Symptoms**: Embedding fails with a model error.

**Solutions**:

1. Pull the model:
   ```bash
   OLLAMA_NO_CLOUD=1 ollama pull nomic-embed-text
   ```

2. Verify it's available:
   ```bash
   ollama list
   ```

3. If using a different model, specify it:
   ```bash
   raven index ./docs --model mxbai-embed-large
   ```

## Database Errors

### "Database is locked"

**Symptoms**: Operations fail with SQLite busy/locked errors.

**Solutions**:

1. This usually resolves within 5 seconds (busy_timeout). If persistent:
2. Check for zombie processes holding the lock:
   ```bash
   fuser raven.db  # Linux
   lsof raven.db   # macOS
   ```
3. Remove stale WAL files (only if no process is using the DB):
   ```bash
   rm raven.db-wal raven.db-shm
   ```

### "Dimension mismatch"

**Symptoms**: Query fails because the embedding dimension doesn't match stored vectors.

**Cause**: You indexed with one model (e.g., 768-dim `nomic-embed-text`) but are querying with a different model (e.g., 1536-dim `text-embedding-3-small`).

**Solution**: Use the same model for indexing and querying, or clear and re-index:

```bash
raven clear
raven index ./docs --model nomic-embed-text
raven query "test" --model nomic-embed-text
```

## Indexing Issues

### "No documents found"

**Symptoms**: `raven index` reports 0 documents.

**Solutions**:

1. Check file extensions match. Default is `txt,md`:
   ```bash
   raven index ./docs --extensions md,txt,html,json
   ```

2. Verify the path contains files:
   ```bash
   find ./docs -type f \( -name "*.md" -o -name "*.txt" \)
   ```

### "Duplicate indexing / growing database"

**Symptoms**: Database grows on every re-index even without content changes.

**Solution**: RavenRustRAG uses fingerprinting for incremental indexing. If your files haven't changed, re-indexing should be a no-op. If the database grows unexpectedly:

1. Check if file modification times are changing (backups, syncing tools)
2. Clear and re-index from scratch:
   ```bash
   raven clear && raven index ./docs
   ```

## Server Issues

### "Address already in use"

**Symptoms**: `raven serve` fails to bind.

**Solutions**:

1. Use a different port:
   ```bash
   raven serve --port 8485
   ```

2. Find and kill the existing process:
   ```bash
   lsof -i :8484
   kill <PID>
   ```

### "401 Unauthorized"

**Symptoms**: API calls return 401.

**Solution**: Include the Bearer token:

```bash
curl -H "Authorization: Bearer $RAVEN_API_KEY" http://localhost:8484/query \
  -d '{"query": "test"}'
```

## Performance Issues

### Slow Indexing

1. Check Ollama is running on the same machine (network latency)
2. Consider a faster model (nomic-embed-text is fast; larger models are slower)
3. Reduce chunk overlap to generate fewer chunks
4. Use SSD storage for the database

### Slow Queries

1. Check database size — very large indexes benefit from mmap (enabled by default)
2. Reduce `top_k` if you don't need many results
3. Avoid hybrid search if BM25 isn't needed for your use case
4. Run `raven benchmark` to baseline your system

## Docker Issues

### "Permission denied" on volume

**Symptoms**: Container can't write to mounted volume.

**Solution**: The container runs as UID 65534. Ensure the host directory is writable:

```bash
chmod 777 ./data  # or chown to 65534
docker run -v ./data:/data ghcr.io/egkristi/ravenrustrag:latest info --db /data/raven.db
```

### Container can't reach Ollama

**Solution**: Use host networking or the host's IP:

```bash
# Linux
docker run --network host ghcr.io/egkristi/ravenrustrag:latest \
  query "test" --url http://localhost:11434

# macOS/Windows (Docker Desktop)
docker run ghcr.io/egkristi/ravenrustrag:latest \
  query "test" --url http://host.docker.internal:11434
```

## Getting Help

1. Run `raven doctor` for automated diagnostics
2. Use `--verbose` flag for detailed logging
3. Use `--json` for machine-parseable output
4. Check [GitHub Issues](https://github.com/egkristi/ravenrustrag/issues)
