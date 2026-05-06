# Quick Start

## Index Documents

Point RavenRustRAG at a directory or file to index:

```bash
# Index a directory of documents
ravenrag index ./docs/

# Index a single file
ravenrag index README.md

# Index with verbose output
ravenrag index ./notes/ -v
```

Supported formats: `.txt`, `.md`, `.csv`, `.json`, `.jsonl`, `.html`

## Query Your Documents

```bash
# Simple semantic search
ravenrag query "how does authentication work"

# Get more results
ravenrag query "error handling patterns" --top-k 10

# Hybrid search (BM25 + vector)
ravenrag query "configuration file" --hybrid
```

## Generate LLM Prompts

The `prompt` command wraps search results in an LLM-ready format:

```bash
ravenrag prompt "explain the build process"
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
ravenrag info
```

## Export and Import

```bash
# Backup your index to JSONL
ravenrag export > backup.jsonl

# Restore from backup
ravenrag import backup.jsonl
```

## Start the HTTP Server

```bash
# Start on default port 8484
ravenrag serve

# With API key authentication
RAVEN_API_KEY=my-secret-key ravenrag serve --port 8484
```

## Run Diagnostics

```bash
ravenrag doctor
```

This checks Ollama connectivity, embedding model availability, and database health.
