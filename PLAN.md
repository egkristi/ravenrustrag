# RavenRustRAG — Roadmap

> **Status:** v1.0.0 released — All phases complete
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

For completed work history, see [docs/changelog.md](docs/changelog.md).

---

## Open Issues

| Issue | Title | Priority | Status |
|---|---|---|---|
| [#92](https://github.com/egkristi/ravenrustrag/issues/92) | Configure repository secrets for automated publishing | Medium | Open |
| [#94](https://github.com/egkristi/ravenrustrag/issues/94) | Initial winget-pkgs submission | Medium | Done (merged [#95](https://github.com/egkristi/ravenrustrag/pull/95)) |
| [#87](https://github.com/egkristi/ravenrustrag/issues/87) | MCP marketplace listing | Low | Done (merged [#93](https://github.com/egkristi/ravenrustrag/pull/93)) |

---

## Phase 4: Polish & Release

### Publishing
- [x] crates.io metadata ready — [#52](https://github.com/egkristi/ravenrustrag/issues/52)
- [x] crates.io publish workflow — [#84](https://github.com/egkristi/ravenrustrag/issues/84) (workflow ready, needs CARGO_REGISTRY_TOKEN secret)
- [x] Homebrew tap formula — [#85](https://github.com/egkristi/ravenrustrag/issues/85) (template in packaging/homebrew/)
- [x] AUR package — [#86](https://github.com/egkristi/ravenrustrag/issues/86) (PKGBUILD in packaging/aur/)
- [x] MCP marketplace listing (GitHub MCP Registry) — PR [#93](https://github.com/egkristi/ravenrustrag/pull/93)

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
- [x] v1.0 stable release — [#61](https://github.com/egkristi/ravenrustrag/issues/61) — **DONE** (v1.0.0 released)

---

## Phase 5: Future

Features planned for post-1.0 development:

### LLM Generation
- [x] `POST /ask` with streaming citations — `event: token` for text, `event: source` for citations (Perplexity pattern)

### ONNX Runtime
- [x] Local embedding without Ollama — [#43](https://github.com/egkristi/ravenrustrag/issues/43)
- [x] Cross-encoder reranking — [#44](https://github.com/egkristi/ravenrustrag/issues/44)
- [x] Quantized model support (int8/fp16) — OnnxEmbedder/OnnxReranker accept quantized models, with_threads() constructor

### MCP
- [x] MCP resources capability — browse index as `raven://documents/` filesystem
- [x] MCP prompts capability — expose RAG prompt templates
- [x] `raven mcp --filter <expr>` — scoped MCP server per collection/context

### CLI
- [x] `raven query --explain` — show score breakdown (vector vs BM25), fingerprint, position
- [x] `raven backup <file>` — SQLite `.backup` API for O(1) snapshots

### Advanced Features
- [x] Incremental BM25 updates — `remove_by_doc_id()` on BM25Index, wired into DocumentIndex::delete()
- [x] Async SQLite backend — spawn_blocking for heavy operations (add, search, all, load_bm25)
- [x] Binary/quantized embedding storage — EmbeddingFormat (F32/F16/Uint8), encode/decode in raven-core, SqliteStore integration
- [x] WebSocket streaming endpoint — [#76](https://github.com/egkristi/ravenrustrag/issues/76)
- [x] Configuration hot-reload — file watcher on raven.toml, rate limiter hot-reload, change detection logging
- [x] Plugin system for custom embedding backends — [#77](https://github.com/egkristi/ravenrustrag/issues/77)

---

## Phase 6: Distribution & Packaging

All platforms should have native package manager support for frictionless install.

### Windows
- [x] winget (`winget install egkristi.raven`) — automated via release workflow
- [x] Chocolatey (`choco install raven-rag`) — package in packaging/chocolatey/
- [x] Scoop (`scoop install raven`) — manifest in packaging/scoop/
- [ ] MSI installer (WiX Toolset)
- [ ] Standalone `.exe` installer (NSIS or WiX)
- [x] Portable ZIP (no install, just unzip and run) — release workflow creates ZIP for all platforms

### macOS
- [x] Homebrew (`brew install egkristi/tap/raven`) — [#85](https://github.com/egkristi/ravenrustrag/issues/85)
- [ ] DMG disk image (drag-to-Applications)
- [ ] `.pkg` installer (signed)

### Linux
- [x] APT / `.deb` package (Debian, Ubuntu) — cargo-deb + nfpm in CI
- [x] DNF / `.rpm` package (Fedora, RHEL) — nfpm in CI
- [x] Pacman / AUR (Arch Linux) — [#86](https://github.com/egkristi/ravenrustrag/issues/86)
- [x] Zypper / `.rpm` (openSUSE) — shared with DNF rpm
- [x] APK (Alpine Linux) — nfpm in CI
- [x] Snap (`snap install raven-rag`) — snapcraft.yaml in packaging/snap/
- [x] Flatpak — manifest in packaging/flatpak/
- [x] Nix / nixpkgs (`nix-env -iA raven-rag` or flake) — flake.nix in repo root
- [x] AppImage (portable, no install) — build script in packaging/appimage/
- [x] Static binary (musl) — release workflow

### Android
- [ ] APK sideload (standalone Android app)
- [ ] Termux `pkg install raven`
- [ ] F-Droid (open-source app store)

### iOS / iPadOS
- [ ] Apple App Store (requires Swift/UIKit GUI wrapper)
- [ ] TestFlight (beta distribution)

### Generic / Cross-platform
- [x] `cargo install raven-cli` (crates.io) — [#84](https://github.com/egkristi/ravenrustrag/issues/84) (publish workflow ready)
- [x] Pre-built static binaries (GitHub Releases) — release workflow
- [x] Docker (`ghcr.io/egkristi/ravenrustrag`) — multi-arch [#90](https://github.com/egkristi/ravenrustrag/issues/90)
- [x] OCI / generic container image (Podman, containerd, etc.) — Docker image works with any OCI runtime
- [x] Helm chart (Kubernetes deployment) — charts/ravenrustrag/
- [x] `curl` install script — `curl -sSf https://raw.githubusercontent.com/egkristi/ravenrustrag/main/install.sh | sh`

---

## Known Limitations

1. **crates.io publish** — Workflow ready, requires `CARGO_REGISTRY_TOKEN` secret. [#84](https://github.com/egkristi/ravenrustrag/issues/84)
2. **winget publish** — Workflow ready, requires `WINGET_TOKEN` secret and initial winget-pkgs submission. [#94](https://github.com/egkristi/ravenrustrag/issues/94)
3. **Homebrew tap** — Formula template ready, needs `egkristi/homebrew-tap` repository.
4. **AUR submission** — PKGBUILD ready, needs AUR account and initial submit.
5. **Chocolatey** — Package ready, needs Chocolatey API key for publishing.
6. **Snap Store** — snapcraft.yaml ready, needs Snapcraft account.
7. **ONNX requires ONNX Runtime** — The `onnx` feature requires the ONNX Runtime shared library at runtime.
8. **DMG/pkg/MSI/exe installers** — Require code signing certificates.
9. **Android** — Requires cross-compilation to `aarch64-linux-android`, GUI wrapper for APK, Termux package submission.
10. **iOS / iPadOS** — Requires Apple Developer account ($99/yr), Swift/UIKit GUI wrapper, and App Store review.

---

**Last updated:** 2026-05-05
**Next milestone:** Phase 6 secrets configuration — add API tokens to enable automated publishing
