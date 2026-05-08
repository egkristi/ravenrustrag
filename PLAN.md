# RavenRustRAG — Roadmap

> **Status:** v1.0.2 released — All phases complete
> **Goal:** Functionally superior to the Python version (RavenRAG v0.7.0) with orders-of-magnitude better performance.

For completed work history, see [docs/changelog.md](docs/changelog.md).
For the full feature set, see [README.md](README.md).

---

## Open Issues

| Issue | Title | Priority |
|---|---|---|
| [#92](https://github.com/egkristi/ravenrustrag/issues/92) | Configure repository secrets for automated publishing | Medium |

---

## Pending: Distribution & Packaging

Items that require external accounts, certificates, or GUI wrappers:

### Windows
- [ ] MSI installer (WiX Toolset) — requires code signing certificate
- [ ] Standalone `.exe` installer (NSIS or WiX) — requires code signing certificate

### macOS
- [ ] DMG disk image (drag-to-Applications) — requires Apple Developer ID
- [ ] `.pkg` installer (signed) — requires Apple Developer ID

### Android
- [ ] APK sideload (standalone Android app) — requires NDK cross-compilation + GUI wrapper
- [ ] Termux `pkg install raven` — requires Termux package repository submission
- [ ] F-Droid (open-source app store) — requires Android app + F-Droid submission

### iOS / iPadOS
- [ ] Apple App Store — requires Apple Developer account ($99/yr) + Swift/UIKit GUI wrapper
- [ ] TestFlight (beta distribution) — same requirements as App Store

---

## Pending: Publishing Activation

These packaging formats are ready but require secrets or account setup ([#92](https://github.com/egkristi/ravenrustrag/issues/92)):

| Package Manager | Status | Blocker |
|---|---|---|
| crates.io | Workflow ready | `CARGO_REGISTRY_TOKEN` secret |
| winget | Manifest + workflow ready | `WINGET_TOKEN` secret + initial winget-pkgs PR |
| Homebrew | **Live** (`brew install egkristi/tap/ravenrag`) | — |
| AUR | PKGBUILD ready | AUR account + initial submission |
| Chocolatey | Package ready | Chocolatey API key |
| Snap Store | snapcraft.yaml ready | Snapcraft account |
| Flatpak/Flathub | Manifest ready | Flathub submission |

---

## Known Limitations

1. **ONNX requires ONNX Runtime** — The `onnx` feature requires the ONNX Runtime shared library at runtime.
2. **TLS** — Server does not terminate TLS. Use a reverse proxy (nginx, Caddy) for HTTPS.
3. **DMG/pkg/MSI/exe installers** — Require code signing certificates.
4. **Android/iOS** — Require platform-specific GUI wrappers and developer accounts.

---

**Last updated:** 2026-05-08
**Next milestone:** Configure repository secrets (#92) to enable automated publishing
