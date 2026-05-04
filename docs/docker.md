# Docker

RavenRustRAG is available as a minimal Docker image based on `scratch` (no OS, just the binary).

## Pulling the Image

```bash
docker pull ghcr.io/egkristi/ravenrustrag:latest
```

Tags:
- `latest` — latest release from main
- `vX.Y.Z` — specific version
- `sha-<commit>` — specific commit

## Running

```bash
# Basic usage
docker run --rm -v ./data:/data ghcr.io/egkristi/ravenrustrag:latest info --db /data/raven.db

# Start server
docker run -d \
  --name ravenrag \
  -p 8484:8484 \
  -v ./data:/data \
  -e RAVEN_API_KEY=my-secret \
  ghcr.io/egkristi/ravenrustrag:latest \
  serve --host 0.0.0.0 --db /data/raven.db
```

## Docker Compose

```yaml
services:
  ravenrag:
    image: ghcr.io/egkristi/ravenrustrag:latest
    ports:
      - "8484:8484"
    volumes:
      - ./data:/data
    environment:
      - RAVEN_API_KEY=${RAVEN_API_KEY}
    command: serve --host 0.0.0.0 --db /data/raven.db
    restart: unless-stopped

  ollama:
    image: ollama/ollama
    ports:
      - "11434:11434"
    volumes:
      - ollama-data:/root/.ollama
    environment:
      - OLLAMA_NO_CLOUD=1

volumes:
  ollama-data:
```

## Building Locally

```bash
docker build -t ravenrustrag .
```

The Dockerfile uses a multi-stage build:
1. Alpine-based Rust builder (musl for static linking)
2. `scratch` runtime (binary + ca-certificates only)

The resulting image is typically under 20 MB.

## Security

- Runs as non-root user (UID 65534)
- No shell, no OS utilities (scratch base)
- TLS certificates bundled for HTTPS outbound connections
- API key authentication for protected endpoints
