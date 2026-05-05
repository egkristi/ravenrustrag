# RavenRustRAG — Roadmap

> **Status:** v0.1.0-alpha — Phases 1–3 complete, Phase 4 nearly complete
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

For completed work history, see [docs/changelog.md](docs/changelog.md).

---

## Open Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#61](https://github.com/egkristi/ravenrustrag/issues/61) | v1.0 stable release | High | Open (meta) |
| [#53](https://github.com/egkristi/ravenrustrag/issues/53) | 80%+ test coverage target | Medium | Done |
| [#76](https://github.com/egkristi/ravenrustrag/issues/76) | WebSocket streaming endpoint | Medium | Done |
| [#77](https://github.com/egkristi/ravenrustrag/issues/77) | Plugin system for custom embedders | Medium | Done |
| [#55](https://github.com/egkristi/ravenrustrag/issues/55) | Homebrew tap formula | Low | Open |
| [#56](https://github.com/egkristi/ravenrustrag/issues/56) | AUR package | Low | Open |
| [#43](https://github.com/egkristi/ravenrustrag/issues/43) | ONNX Runtime embedding backend | High | Done |
| [#44](https://github.com/egkristi/ravenrustrag/issues/44) | ONNX cross-encoder reranking | Medium | Done |

---

## Phase 4: Polish & Release (in progress)

### Publishing
- [x] crates.io metadata ready — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] Actual crates.io publish — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] Homebrew tap formula — [#55](https://github.com/egkristi/ravenrustrag/issues/55)
- [ ] AUR package — [#56](https://github.com/egkristi/ravenrustrag/issues/56)

### Quality
- [x] 80%+ test coverage — [#53](https://github.com/egkristi/ravenrustrag/issues/53)
- [x] Integration tests for CLI binary (assert_cmd) — [#70](https://github.com/egkristi/ravenrustrag/issues/70)
- [x] Integration tests for HTTP server endpoints — [#70](https://github.com/egkristi/ravenrustrag/issues/70)

### Architecture
- [x] Top-level `ravenrustrag` library crate with builder API — [#75](https://github.com/egkristi/ravenrustrag/issues/75)

### CLI
- [x] `raven init` — config generator — [#72](https://github.com/egkristi/ravenrustrag/issues/72)
- [x] `raven diff` — show changed files since last index — [#78](https://github.com/egkristi/ravenrustrag/issues/78)

### Stability
- [ ] v1.0 stable release — [#61](https://github.com/egkristi/ravenrustrag/issues/61)

---

## Phase 5: Future

Features planned for post-1.0 development:

### LLM Generation
- [ ] `POST /ask` server endpoint with streaming SSE response

### ONNX Runtime
- [x] Local embedding without Ollama — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [x] Cross-encoder reranking — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [ ] Quantized model support (int8/fp16)

### Advanced Features
- [ ] Incremental BM25 updates (avoid full rebuild)
- [ ] Async SQLite backend (tokio-rusqlite)
- [ ] Binary/quantized embedding storage (reduced disk/memory)
- [x] WebSocket streaming endpoint — [#76](https://github.com/egkristi/ravenrustrag/issues/76)
- [ ] Configuration hot-reload for long-running server
- [x] Plugin system for custom embedding backends — [#77](https://github.com/egkristi/ravenrustrag/issues/77)

---

## Known Limitations

_(No current blocking limitations.)_

---

**Last updated:** 2026-05-06
**Next milestone:** v1.0 stable release (#61) — requires #52, #53, #70, #75
