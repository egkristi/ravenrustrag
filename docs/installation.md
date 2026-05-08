# Installation

## Homebrew (macOS — recommended)

```bash
brew tap egkristi/tap
brew install ravenrag

# Or install directly without tapping:
brew install egkristi/tap/ravenrag
```

## From GitHub Releases

Download the latest pre-built binary from [GitHub Releases](https://github.com/egkristi/ravenrustrag/releases):

```bash
# Linux (glibc)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-linux-amd64
chmod +x raven-linux-amd64 && mv raven-linux-amd64 /usr/local/bin/ravenrag

# Linux (musl, fully static)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-linux-amd64-musl
chmod +x raven-linux-amd64-musl && mv raven-linux-amd64-musl /usr/local/bin/ravenrag

# macOS (Apple Silicon)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-darwin-arm64
chmod +x raven-darwin-arm64 && mv raven-darwin-arm64 /usr/local/bin/ravenrag
```

## From Source

Requires Rust 1.88+ (MSRV):

```bash
git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag
cargo build --release
# Binary at target/release/ravenrag
```

## Docker

```bash
# Available tags:
#   latest           — most recent release
#   1.0.2            — specific version
#   1.0              — latest patch in 1.0.x
#   sha-aa5b66d      — specific commit
docker pull ghcr.io/egkristi/ravenrustrag:latest
docker pull ghcr.io/egkristi/ravenrustrag:1.0.2

docker run --rm -v ./data:/data ghcr.io/egkristi/ravenrustrag:1.0.2 --help
```

## Prerequisites

For embedding generation, you need [Ollama](https://ollama.com) running locally:

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull the default embedding model
OLLAMA_NO_CLOUD=1 ollama pull nomic-embed-text

# Start Ollama (if not running as a service)
OLLAMA_NO_CLOUD=1 ollama serve
```

> **Important**: Always run Ollama with `OLLAMA_NO_CLOUD=1` to ensure no cloud inference is used.
