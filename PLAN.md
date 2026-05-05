# RavenRustRAG — Roadmap

> **Status:** v0.1.0-alpha — Phases 1–4 complete, Phase 5 features implemented
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

For completed work history, see [docs/changelog.md](docs/changelog.md).

---

## Open Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#79](https://github.com/egkristi/ravenrustrag/issues/79) | HNSW: replace flat O(n) scan in SqliteStore | **Critical** | Done |
| [#83](https://github.com/egkristi/ravenrustrag/issues/83) | Define stable public API surface (`#[doc(hidden)]`) | High | Done |
| [#80](https://github.com/egkristi/ravenrustrag/issues/80) | ONNX MSRV split documentation + CI gate | High | Done |
| [#81](https://github.com/egkristi/ravenrustrag/issues/81) | Publish actual test coverage percentage | Medium | Done |
| [#82](https://github.com/egkristi/ravenrustrag/issues/82) | Verify and update benchmark numbers in README | Medium | Done |
| [#61](https://github.com/egkristi/ravenrustrag/issues/61) | v1.0 stable release | High | Open (meta) |
| [#55](https://github.com/egkristi/ravenrustrag/issues/55) | Homebrew tap formula | Low | Open |
| [#56](https://github.com/egkristi/ravenrustrag/issues/56) | AUR package | Low | Open |
| [#53](https://github.com/egkristi/ravenrustrag/issues/53) | 80%+ test coverage target | Medium | Done |
| [#76](https://github.com/egkristi/ravenrustrag/issues/76) | WebSocket streaming endpoint | Medium | Done |
| [#77](https://github.com/egkristi/ravenrustrag/issues/77) | Plugin system for custom embedders | Medium | Done |
| [#43](https://github.com/egkristi/ravenrustrag/issues/43) | ONNX Runtime embedding backend | High | Done |
| [#44](https://github.com/egkristi/ravenrustrag/issues/44) | ONNX cross-encoder reranking | Medium | Done |

---

## Phase 4: Polish & Release (in progress)

### Publishing
- [x] crates.io metadata ready — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] Actual crates.io publish — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] Homebrew tap formula — [#55](https://github.com/egkristi/ravenrustrag/issues/55)
- [ ] AUR package — [#56](https://github.com/egkristi/ravenrustrag/issues/56)
- [ ] MCP marketplace listing (GitHub MCP Registry)

### Quality
- [x] 80%+ test coverage — [#53](https://github.com/egkristi/ravenrustrag/issues/53)
- [x] Publish actual coverage percentage and Codecov badge — [#81](https://github.com/egkristi/ravenrustrag/issues/81)
- [x] Integration tests for CLI binary (assert_cmd) — [#70](https://github.com/egkristi/ravenrustrag/issues/70)
- [x] Integration tests for HTTP server endpoints — [#70](https://github.com/egkristi/ravenrustrag/issues/70)
- [x] Verify and update benchmark numbers in README — [#82](https://github.com/egkristi/ravenrustrag/issues/82)

### Architecture
- [x] Top-level `ravenrustrag` library crate with builder API — [#75](https://github.com/egkristi/ravenrustrag/issues/75)

### CLI
- [x] `raven init` — config generator — [#72](https://github.com/egkristi/ravenrustrag/issues/72)
- [x] `raven diff` — show changed files since last index — [#78](https://github.com/egkristi/ravenrustrag/issues/78)

### Robustness
- [x] Embeddings versioning — store model name + dimensions in fingerprint table, reject mismatched queries
- [x] `raven serve --read-only` — disable write endpoints for production deployments
- [x] JSON Schema validation on MCP tools/list response (improves Claude/Cursor integration)

### Performance
- [x] HNSW: replace flat O(n) vector scan in SqliteStore — [#79](https://github.com/egkristi/ravenrustrag/issues/79)

### API Stability
- [x] Define stable public API surface with `#[doc(hidden)]` on internals — [#83](https://github.com/egkristi/ravenrustrag/issues/83)
- [x] ONNX MSRV split documentation + CI gate — [#80](https://github.com/egkristi/ravenrustrag/issues/80)
- [ ] v1.0 stable release — [#61](https://github.com/egkristi/ravenrustrag/issues/61)

---

## Phase 5: Future

Features planned for post-1.0 development:

### LLM Generation
- [x] `POST /ask` with streaming citations — `event: token` for text, `event: source` for citations (Perplexity pattern)

### ONNX Runtime
- [x] Local embedding without Ollama — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [x] Cross-encoder reranking — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [ ] Quantized model support (int8/fp16)

### MCP
- [x] MCP resources capability — browse index as `raven://documents/` filesystem
- [x] MCP prompts capability — expose RAG prompt templates
- [x] `raven mcp --filter <expr>` — scoped MCP server per collection/context

### CLI
- [x] `raven query --explain` — show score breakdown (vector vs BM25), fingerprint, position
- [x] `raven backup <file>` — SQLite `.backup` API for O(1) snapshots

### Advanced Features
- [ ] Incremental BM25 updates (avoid full rebuild)
- [ ] Async SQLite backend (tokio-rusqlite)
- [ ] Binary/quantized embedding storage (reduced disk/memory)
- [x] WebSocket streaming endpoint — [#76](https://github.com/egkristi/ravenrustrag/issues/76)
- [ ] Configuration hot-reload for long-running server
- [x] Plugin system for custom embedding backends — [#77](https://github.com/egkristi/ravenrustrag/issues/77)

---

## Known Limitations

1. **Actual crates.io publish** — Metadata ready but not yet published. [#52](https://github.com/egkristi/ravenrustrag/issues/52)
2. **Homebrew/AUR packages** — Not yet published. [#55](https://github.com/egkristi/ravenrustrag/issues/55), [#56](https://github.com/egkristi/ravenrustrag/issues/56)
3. **ONNX requires Rust 1.88+** — The `onnx` feature requires a newer compiler than the base MSRV (1.86). Documented and gated in CI.

---

**Last updated:** 2026-05-05
**Next milestone:** v1.0 stable release (#61) — all code features complete, remaining work is packaging and publishing
