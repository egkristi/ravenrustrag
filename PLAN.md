# RavenRustRAG — Roadmap

> **Status:** v0.1.0-alpha — Phases 1–3 complete, Phase 4 nearly complete
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

For completed work history, see [docs/changelog.md](docs/changelog.md).

---

## Open Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#79](https://github.com/egkristi/ravenrustrag/issues/79) | HNSW: replace flat O(n) scan in SqliteStore | **Critical** | Open |
| [#83](https://github.com/egkristi/ravenrustrag/issues/83) | Define stable public API surface (`#[doc(hidden)]`) | High | Open |
| [#80](https://github.com/egkristi/ravenrustrag/issues/80) | ONNX MSRV split documentation + CI gate | High | Open |
| [#81](https://github.com/egkristi/ravenrustrag/issues/81) | Publish actual test coverage percentage | Medium | Open |
| [#82](https://github.com/egkristi/ravenrustrag/issues/82) | Verify and update benchmark numbers in README | Medium | Open |
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
- [ ] Publish actual coverage percentage and Codecov badge — [#81](https://github.com/egkristi/ravenrustrag/issues/81)
- [x] Integration tests for CLI binary (assert_cmd) — [#70](https://github.com/egkristi/ravenrustrag/issues/70)
- [x] Integration tests for HTTP server endpoints — [#70](https://github.com/egkristi/ravenrustrag/issues/70)
- [ ] Verify and update benchmark numbers in README — [#82](https://github.com/egkristi/ravenrustrag/issues/82)

### Architecture
- [x] Top-level `ravenrustrag` library crate with builder API — [#75](https://github.com/egkristi/ravenrustrag/issues/75)

### CLI
- [x] `raven init` — config generator — [#72](https://github.com/egkristi/ravenrustrag/issues/72)
- [x] `raven diff` — show changed files since last index — [#78](https://github.com/egkristi/ravenrustrag/issues/78)

### Robustness
- [ ] Embeddings versioning — store model name + dimensions in fingerprint table, reject mismatched queries
- [ ] `raven serve --read-only` — disable write endpoints for production deployments
- [ ] JSON Schema validation on MCP tools/list response (improves Claude/Cursor integration)

### Performance
- [ ] HNSW: replace flat O(n) vector scan in SqliteStore — [#79](https://github.com/egkristi/ravenrustrag/issues/79)

### API Stability
- [ ] Define stable public API surface with `#[doc(hidden)]` on internals — [#83](https://github.com/egkristi/ravenrustrag/issues/83)
- [ ] ONNX MSRV split documentation + CI gate — [#80](https://github.com/egkristi/ravenrustrag/issues/80)
- [ ] v1.0 stable release — [#61](https://github.com/egkristi/ravenrustrag/issues/61)

---

## Phase 5: Future

Features planned for post-1.0 development:

### LLM Generation
- [ ] `POST /ask` with streaming citations — `event: token` for text, `event: source` for citations (Perplexity pattern)

### ONNX Runtime
- [x] Local embedding without Ollama — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [x] Cross-encoder reranking — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [ ] Quantized model support (int8/fp16)

### MCP
- [ ] MCP resources capability — browse index as `raven://documents/` filesystem
- [ ] MCP prompts capability — expose RAG prompt templates
- [ ] `raven mcp --filter <expr>` — scoped MCP server per collection/context

### CLI
- [ ] `raven query --explain` — show score breakdown (vector vs BM25), fingerprint, position
- [ ] `raven backup <file>` — SQLite `.backup` API for O(1) snapshots

### Advanced Features
- [ ] Incremental BM25 updates (avoid full rebuild)
- [ ] Async SQLite backend (tokio-rusqlite)
- [ ] Binary/quantized embedding storage (reduced disk/memory)
- [x] WebSocket streaming endpoint — [#76](https://github.com/egkristi/ravenrustrag/issues/76)
- [ ] Configuration hot-reload for long-running server
- [x] Plugin system for custom embedding backends — [#77](https://github.com/egkristi/ravenrustrag/issues/77)

---

## Known Limitations

1. **Flat O(n) vector search** — SqliteStore::search() falls back to brute-force cosine scan. HNSW index exists behind feature flag but is not auto-maintained on insert. This is the largest performance bottleneck at scale. [#79](https://github.com/egkristi/ravenrustrag/issues/79)
2. **ONNX requires Rust 1.88+** — The `onnx` feature compiles and works, but requires a higher MSRV (1.88) than the default (1.86). This split is not documented. [#80](https://github.com/egkristi/ravenrustrag/issues/80)
3. **Coverage percentage unknown** — CI runs tarpaulin but the actual percentage is not published or verified against the 80% target. [#81](https://github.com/egkristi/ravenrustrag/issues/81)
4. **No stable API contract** — All `pub` types across 10 crates are technically public. Without `#[doc(hidden)]` or visibility reduction, semver is unenforceable after v1.0. [#83](https://github.com/egkristi/ravenrustrag/issues/83)

---

**Last updated:** 2026-05-05
**Next milestone:** v1.0 stable release (#61) — requires #79, #83, #80, #81, #82
