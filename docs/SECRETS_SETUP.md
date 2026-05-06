# Repository Secrets Setup Guide

This document describes the secrets that must be configured manually in the repository settings.

## Required Secrets

### 1. `CARGO_REGISTRY_TOKEN`

**Purpose:** Publish crates to [crates.io](https://crates.io)

**Workflow:** `publish.yml`

**How to create:**
1. Log in to [crates.io](https://crates.io) with your GitHub account
2. Go to **Account Settings** → **API Tokens**
3. Click **New Token**
4. Name: `GitHub Actions Publish`
5. Scopes: `publish-new`
6. Copy the token

**How to add to repository:**
1. Go to https://github.com/egkristi/ravenrustrag/settings/secrets/actions
2. Click **New repository secret**
3. Name: `CARGO_REGISTRY_TOKEN`
4. Value: (paste token from crates.io)
5. Click **Add secret**

---

### 2. `WINGET_TOKEN`

**Purpose:** Submit package updates to [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs)

**Workflow:** `release.yml`

**How to create:**
1. Go to https://github.com/settings/tokens
2. Click **Generate new token (classic)**
3. Note: `winget-pkgs submission`
4. Expiration: (your preference)
5. Scopes: `public_repo` (minimum), or `repo` if submitting to private repos
6. Click **Generate token**
7. Copy the token

**How to add to repository:**
1. Go to https://github.com/egkristi/ravenrustrag/settings/secrets/actions
2. Click **New repository secret**
3. Name: `WINGET_TOKEN`
4. Value: (paste token from GitHub)
5. Click **Add secret**

---

## Manual One-Time Setup Tasks

These tasks cannot be automated via CI and require manual action:

### Homebrew Tap
- [ ] Create repository `egkristi/homebrew-tap`
- [ ] Add formula for `raven` (see `packaging/homebrew/`)

### AUR (Arch Linux)
- [ ] Create account on https://aur.archlinux.org
- [ ] Submit PKGBUILD (see `packaging/aur/`)

### Chocolatey (Windows)
- [ ] Create account on https://chocolatey.org
- [ ] Submit package (see `packaging/chocolatey/`)

### Snapcraft (Ubuntu)
- [ ] Create account on https://snapcraft.io
- [ ] Register package name `raven-rag`
- [ ] Upload snap (see `packaging/snap/`)

### Flathub (Linux)
- [ ] Fork https://github.com/flathub/flathub
- [ ] Submit Flatpak manifest (see `packaging/flatpak/`)

## Winget Initial Submission

After adding `WINGET_TOKEN`:
1. Fork https://github.com/microsoft/winget-pkgs
2. Copy manifest from `packaging/winget/egkristi.raven/1.0.0/`
3. Submit PR to `microsoft/winget-pkgs`
4. Future releases will be auto-submitted by the release workflow
