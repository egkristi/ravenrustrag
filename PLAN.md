# RavenRustRAG — Roadmap

> **Status:** v0.1.0-alpha — Phases 1–3 complete, Phase 4 nearly complete  
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

For completed work history, see [docs/changelog.md](docs/changelog.md).

---

## Open Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#52](https://github.com/egkristi/ravenrustrag/issues/52) | Publish to crates.io | High | Open |
| [#61](https://github.com/egkristi/ravenrustrag/issues/61) | v1.0 stable release | High | Open (meta) |
| [#75](https://github.com/egkristi/ravenrustrag/issues/75) | Top-level library crate with clean API | High | Open |
| [#70](https://github.com/egkristi/ravenrustrag/issues/70) | Integration tests for CLI and HTTP server | High | Open |
| [#53](https://github.com/egkristi/ravenrustrag/issues/53) | 80%+ test coverage target | Medium | Open |
| [#72](https://github.com/egkristi/ravenrustrag/issues/72) | raven init — interactive setup | Medium | Open |
| [#76](https://github.com/egkristi/ravenrustrag/issues/76) | WebSocket streaming endpoint | Medium | Open |
| [#77](https://github.com/egkristi/ravenrustrag/issues/77) | Plugin system for custom embedders | Medium | Open |
| [#78](https://github.com/egkristi/ravenrustrag/issues/78) | raven diff — show changes since last index | Medium | Open |
| [#55](https://github.com/egkristi/ravenrustrag/issues/55) | Homebrew tap formula | Low | Open |
| [#56](https://github.com/egkristi/ravenrustrag/issues/56) | AUR package | Low | Open |
| [#43](https://github.com/egkristi/ravenrustrag/issues/43) | ONNX Runtime embedding backend | High | Deferred (MSRV conflict) |
| [#44](https://github.com/egkristi/ravenrustrag/issues/44) | ONNX cross-encoder reranking | Medium | Deferred (blocked by #43) |

---

## Phase 4: Polish & Release (in progress)

### Publishing
- [ ] crates.io publish — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] `cargo install ravenrustrag` — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [ ] Homebrew tap formula — [#55](https://github.com/egkristi/ravenrustrag/issues/55)
- [ ] AUR package — [#56](https://github.com/egkristi/ravenrustrag/issues/56)

### Quality
- [ ] 80%+ test coverage — [#53](https://github.com/egkristi/ravenrustrag/issues/53)
- [ ] Integration tests for CLI binary (assert_cmd) — [#70](https://github.com/egkristi/ravenrustrag/issues/70)
- [ ] Integration tests for HTTP server endpoints — [#70](https://github.com/egkristi/ravenrustrag/issues/70)

### Architecture
- [ ] Top-level `ravenrustrag` library crate with builder API — [#75](https://github.com/egkristi/ravenrustrag/issues/75)

### Stability
- [ ] v1.0 stable release — [#61](https://github.com/egkristi/ravenrustrag/issues/61)

---

## Phase 5: Future

Features planned for post-1.0 development:

### LLM Generation
- [ ] `POST /ask` server endpoint with streaming SSE response

### ONNX Runtime (when ort crate is compatible)
- [ ] Local embedding without Ollama — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [ ] Cross-encoder reranking — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [ ] Quantized model support (int8/fp16)

### Developer Experience
- [ ] `raven init` — interactive config generator — [#72](https://github.com/egkristi/ravenrustrag/issues/72)
- [ ] `raven diff` — show changed files since last index — [#78](https://github.com/egkristi/ravenrustrag/issues/78)

### Advanced Features
- [ ] Incremental BM25 updates (avoid full rebuild)
- [ ] Async SQLite backend (tokio-rusqlite)
- [ ] Binary/quantized embedding storage (reduced disk/memory)
- [ ] WebSocket streaming endpoint — [#76](https://github.com/egkristi/ravenrustrag/issues/76)
- [ ] Configuration hot-reload for long-running server
- [ ] Plugin system for custom embedding backends — [#77](https://github.com/egkristi/ravenrustrag/issues/77)

---

## Known Limitations

1. **ONNX not functional** — Stub exists behind feature flag but `ort` crate has MSRV conflicts. [#43](https://github.com/egkristi/ravenrustrag/issues/43)
2. **No ONNX cross-encoder** — Reranker trait exists, but only keyword-based. Blocked by #43. [#44](https://github.com/egkristi/ravenrustrag/issues/44)

---

**Last updated:** 2026-05-05  
**Next milestone:** v1.0 stable release (#61) — requires #52, #53, #70, #75
