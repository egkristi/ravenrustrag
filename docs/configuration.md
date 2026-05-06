# Configuration

RavenRustRAG can be configured via a TOML config file, environment variables, or CLI flags.

## Config File

Create a `raven.toml` in the project root or pass `--config <path>`:

```toml
[embedder]
backend = "ollama"
model = "nomic-embed-text"
url = "http://localhost:11434"

[splitter]
kind = "text"
chunk_size = 512
chunk_overlap = 50

[pipeline]
embed_batch_size = 64
store_batch_size = 100

[server]
host = "127.0.0.1"
port = 8484
# api_key = "your-secret"
# cors_origins = ["http://localhost:3000"]
request_timeout_secs = 60
rate_limit_per_second = 100
max_query_length = 10000
public_stats = false
```

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `RAVEN_API_KEY` | API authentication key (server) | — (no auth) |
| `RAVEN_DB` | Default database path | `./raven.db` |
| `RAVEN_MODEL` | Default embedding model | `nomic-embed-text` |
| `RAVEN_HOST` | Server bind address | `127.0.0.1` |
| `RAVEN_PORT` | Server port | `8484` |
| `RAVEN_EMBED_BACKEND` | Embedding backend (`ollama`, `openai`, `http`, `onnx`) | `ollama` |
| `RAVEN_EMBED_URL` | Custom embedding service URL | — |
| `RAVEN_CORS_ORIGINS` | Allowed CORS origins (comma-separated) | `*` |
| `RAVEN_RATE_LIMIT` | Rate limit per second | `100` |
| `RAVEN_REQUEST_TIMEOUT` | Request timeout in seconds | `60` |
| `RAVEN_MAX_QUERY_LENGTH` | Max query string length | `10000` |
| `RAVEN_LOG_FORMAT` | Log output format (`text` or `json`) | `text` |

## Precedence

CLI flags > Environment variables > Config file > Defaults

## Embedding Backends

### Ollama (default)

Local inference via [Ollama](https://ollama.com). Requires Ollama to be running with a pulled model.

```bash
OLLAMA_NO_CLOUD=1 ollama serve
```

Recommended models:
- `nomic-embed-text` (768 dimensions, fast, good quality)
- `mxbai-embed-large` (1024 dimensions, higher quality)

### OpenAI

Uses the OpenAI embeddings API. Requires `OPENAI_API_KEY` environment variable.

```bash
ravenrag index ./docs --backend openai --model text-embedding-3-small
```

## Database

RavenRustRAG uses SQLite as its vector store with these optimizations enabled by default:

- **WAL mode** for concurrent read access
- **mmap** (256 MB) for zero-copy reads
- **64 MB page cache**
- **5 second busy timeout** for write contention

The database file is portable and can be copied between machines with the same architecture.
