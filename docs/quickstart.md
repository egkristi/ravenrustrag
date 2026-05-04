# Quick Start

## Index Documents

Point RavenRustRAG at a directory or file to index:

```bash
# Index a directory of documents
raven index ./docs/

# Index a single file
raven index README.md

# Index with verbose output
raven index ./notes/ -v
```

Supported formats: `.txt`, `.md`, `.csv`, `.json`, `.jsonl`, `.html`

## Query Your Documents

```bash
# Simple semantic search
raven query "how does authentication work"

# Get more results
raven query "error handling patterns" --top-k 10

# Hybrid search (BM25 + vector)
raven query "configuration file" --hybrid
```

## Generate LLM Prompts

The `prompt` command wraps search results in an LLM-ready format:

```bash
raven prompt "explain the build process"
```

Output:

```
Use the following context to answer the question.

Context:
[1] (score: 0.89) The build process uses cargo build...
[2] (score: 0.85) Release builds are created with...

Question: explain the build process
```

## Check Index Status

```bash
raven info
```

## Export and Import

```bash
# Backup your index to JSONL
raven export > backup.jsonl

# Restore from backup
raven import backup.jsonl
```

## Start the HTTP Server

```bash
# Start on default port 8484
raven serve

# With API key authentication
RAVEN_API_KEY=my-secret-key raven serve --port 8484
```

## Run Diagnostics

```bash
raven doctor
```

This checks Ollama connectivity, embedding model availability, and database health.
