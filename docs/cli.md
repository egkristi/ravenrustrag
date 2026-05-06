# CLI Reference

## Global Flags

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--config` | `-c` | — | Path to config file |
| `--verbose` | `-v` | `false` | Enable verbose logging |
| `--json` | — | `false` | Output as JSON (for scripting) |
| `--log-format` | — | `text` | Log format: `text` or `json` (env: `RAVEN_LOG_FORMAT`) |

## Commands

### `ravenrag index <path>`

Index documents from a directory or file.

```bash
ravenrag index ./documents/ --extensions txt,md,html --chunk-size 1024
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend (`ollama` or `openai`) |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |
| `--chunk-size` | — | `512` | Chunk size in characters |
| `--chunk-overlap` | — | `50` | Overlap between chunks |
| `--extensions` | — | `txt,md` | File extensions to include (comma-separated) |

### `ravenrag query <query>`

Search the index.

```bash
ravenrag query "how does authentication work" --top-k 10 --hybrid
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |
| `--top-k` | `-k` | `5` | Number of results |
| `--hybrid` | — | `false` | Use hybrid BM25 + vector search with RRF |
| `--alpha` | — | `0.5` | Hybrid blend (1.0 = pure vector, 0.0 = pure BM25) |
| `--explain` | — | `false` | Show detailed scoring (distance, metadata, doc_id) |

### `ravenrag prompt <query>`

Generate an LLM-ready prompt with retrieved context.

```bash
ravenrag prompt "explain the architecture" --top-k 5
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |
| `--top-k` | `-k` | `3` | Number of context chunks |

### `ravenrag serve`

Start the HTTP API server.

```bash
RAVEN_API_KEY=secret ravenrag serve --port 8484
```

| Flag | Short | Default | Env Var | Description |
|------|-------|---------|---------|-------------|
| `--host` | — | `127.0.0.1` | `RAVEN_HOST` | Bind address |
| `--port` | `-p` | `8484` | `RAVEN_PORT` | Port |
| `--db` | `-d` | `./raven.db` | — | Database path |
| `--backend` | `-b` | `ollama` | — | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | — | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | — | Embedding model |
| `--api-key` | — | — | `RAVEN_API_KEY` | API key for auth (optional) |
| `--read-only` | — | `false` | — | Disable write endpoints (index, delete) |

### `ravenrag watch <path>`

Watch a directory and auto-index on file changes.

```bash
ravenrag watch ./notes/ --debounce 1000 --extensions md,txt
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |
| `--extensions` | — | `txt,md` | File extensions to watch |
| `--debounce` | — | `500` | Debounce interval in milliseconds |

### `ravenrag graph <subcommand>`

Build or query the knowledge graph.

#### `ravenrag graph build`

```bash
ravenrag graph build --output ./my-graph.json
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--output` | `-o` | `./raven-graph.json` | Graph output file |

#### `ravenrag graph query <query>`

```bash
ravenrag graph query "Rust async patterns" --max-hops 3
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--graph` | `-g` | `./raven-graph.json` | Graph file path |
| `--max-hops` | — | `2` | Max traversal hops |
| `--top-k` | `-k` | `5` | Number of results |

### `ravenrag info`

Show index statistics (document count, chunk count, database size).

```bash
ravenrag info --db ./raven.db
```

### `ravenrag clear`

Clear all indexed data.

```bash
ravenrag clear --db ./raven.db
```

### `ravenrag export`

Export the index to JSONL format for backup.

```bash
ravenrag export --output backup.jsonl
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--output` | `-o` | `raven-export.jsonl` | Output file path |
| `--db` | `-d` | `./raven.db` | Database path |

### `ravenrag import <file>`

Import documents from a JSONL file.

```bash
ravenrag import backup.jsonl
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |

### `ravenrag mcp`

Start the MCP (Model Context Protocol) server on stdio for AI assistant integration.

```bash
ravenrag mcp --db ./raven.db
ravenrag mcp --filter search,get_prompt
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |
| `--filter` | `-f` | — | Restrict tools (comma-separated names) |

### `ravenrag ask <query>`

Full RAG pipeline: retrieve context, generate an answer via local LLM (Ollama).

```bash
ravenrag ask "What is retrieval-augmented generation?"
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--backend` | `-b` | `ollama` | Embedding backend |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |
| `--model` | `-m` | `nomic-embed-text` | Embedding model |
| `--llm-model` | `-l` | `llama3` | LLM model for generation |
| `--top-k` | `-k` | `5` | Number of context chunks |
| `--temperature` | — | `0.7` | Generation temperature |

### `ravenrag backup <output>`

Create a consistent SQLite backup using the backup API.

```bash
ravenrag backup ./raven-backup.db
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Source database |

### `ravenrag init`

Generate a default `raven.toml` configuration file.

```bash
ravenrag init
ravenrag init --output ./custom-config.toml
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--output` | `-o` | `./raven.toml` | Output path |
| `--force` | — | `false` | Overwrite existing file |

### `ravenrag diff <path>`

Show files changed since last index.

```bash
ravenrag diff ./docs/
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--extensions` | — | `txt,md` | File extensions to check |

### `ravenrag status`

Show index health at a glance (chunk count, DB size, connectivity).

```bash
ravenrag status
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--db` | `-d` | `./raven.db` | Database path |
| `--url` | `-u` | `http://localhost:11434` | Ollama URL |

### `ravenrag completions <shell>`

Generate shell completion scripts.

```bash
ravenrag completions bash > /etc/bash_completion.d/raven
ravenrag completions zsh > ~/.zfunc/_raven
ravenrag completions fish > ~/.config/fish/completions/raven.fish
```

Supported shells: `bash`, `zsh`, `fish`, `elvish`, `powershell`.

### `ravenrag doctor`

Run system diagnostics (Ollama connectivity, model availability, database health).

```bash
ravenrag doctor
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--url` | `-u` | `http://localhost:11434` | Ollama URL to check |
| `--db` | `-d` | `./raven.db` | Database path to check |

### `ravenrag benchmark`

Run performance benchmarks.

```bash
ravenrag benchmark --num-docs 500 --iterations 100
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--num-docs` | `-n` | `100` | Number of documents to generate |
| `--iterations` | `-i` | `50` | Number of query iterations |
