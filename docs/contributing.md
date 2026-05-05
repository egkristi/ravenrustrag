# Contributing

Contributions to RavenRustRAG are welcome. This guide covers the development workflow.

## Prerequisites

- Rust 1.88+ (MSRV)
- [Ollama](https://ollama.com) (for integration tests with real embeddings)
- Git

## Getting Started

```bash
git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag
cargo build
cargo test
```

## Development Workflow

1. Check [GitHub Issues](https://github.com/egkristi/ravenrustrag/issues) for open work
2. Create a branch from `main`
3. Implement your changes
4. Run the full check suite:

```bash
cargo fmt --all --check
cargo clippy --lib -- -D warnings
cargo test --all
```

5. Commit with a conventional message
6. Open a Pull Request

## Commit Messages

Use conventional commits:

```
feat: add batch embedding support #12
fix: handle empty document gracefully closes #15
refactor: extract BM25 into separate module
docs: add hybrid search documentation
test: add property tests for splitter
perf: switch to DashMap for cache
```

## Code Standards

- **Edition**: Rust 2021
- **Async runtime**: Tokio (full features)
- **Error handling**: `thiserror` for library errors, `anyhow` only in CLI
- **Traits**: Use `async-trait`; all traits must be `Send + Sync`
- **Serialization**: `serde` + `serde_json`
- **Logging**: `tracing` crate (`info!`, `warn!`, `error!` — never `println!` in libraries)
- **No `unwrap()` in library code**: Use `?` and proper error types
- **Tests**: Unit tests in each crate, use `MemoryStore` + `DummyEmbedder` for isolation

## Project Structure

```
ravenrustrag/
├── Cargo.toml          # Workspace root
├── book.toml           # mdBook config
├── mkdocs.yml          # MkDocs config
├── docs/               # Documentation (shared by mkdocs + mdbook)
├── crates/
│   ├── raven-core/     # Foundation types
│   ├── raven-embed/    # Embedding backends
│   ├── raven-split/    # Text chunking
│   ├── raven-load/     # File loading
│   ├── raven-store/    # Vector storage
│   ├── raven-search/   # Pipeline orchestrator
│   ├── raven-server/   # HTTP API
│   ├── raven-mcp/      # MCP server
│   └── raven-cli/      # CLI binary
└── .github/workflows/  # CI/CD
```

## Testing

Run all tests:

```bash
cargo test --all
```

Run tests for a specific crate:

```bash
cargo test -p raven-search
```

For test isolation, use `MemoryStore` (in-memory vector store) and `DummyEmbedder` (deterministic fake embeddings). This avoids needing Ollama running for unit tests.

## Building Documentation

### MkDocs (primary, used for GitHub Pages)

```bash
pip install mkdocs-material mkdocs-minify-plugin
mkdocs serve     # Local preview at http://127.0.0.1:8000
mkdocs build     # Build to site/
```

### mdBook

```bash
cargo install mdbook
mdbook serve     # Local preview at http://127.0.0.1:3000
mdbook build     # Build to book/
```

## Release Process

Releases are automated via GitHub Actions when a tag is pushed:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This triggers cross-compilation for 5 targets and creates a GitHub Release with binaries and checksums.
