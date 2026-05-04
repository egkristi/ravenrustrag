# Installation

## From GitHub Releases (recommended)

Download the latest pre-built binary from [GitHub Releases](https://github.com/egkristi/ravenrustrag/releases):

```bash
# Linux (glibc)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-linux-amd64
chmod +x raven-linux-amd64 && mv raven-linux-amd64 /usr/local/bin/raven

# Linux (musl, fully static)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-linux-amd64-musl
chmod +x raven-linux-amd64-musl && mv raven-linux-amd64-musl /usr/local/bin/raven

# macOS (Apple Silicon)
curl -LO https://github.com/egkristi/ravenrustrag/releases/latest/download/raven-darwin-arm64
chmod +x raven-darwin-arm64 && mv raven-darwin-arm64 /usr/local/bin/raven
```

## From Source

Requires Rust 1.86+ (MSRV):

```bash
git clone https://github.com/egkristi/ravenrustrag.git
cd ravenrustrag
cargo build --release
# Binary at target/release/raven
```

## Docker

```bash
docker pull ghcr.io/egkristi/ravenrustrag:latest
docker run --rm -v ./data:/data ghcr.io/egkristi/ravenrustrag:latest --help
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
