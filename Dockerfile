# syntax=docker/dockerfile:1
# ──────────────────────────────────────────────
# RavenRustRAG — Fearlessly fast local-first RAG
# ──────────────────────────────────────────────
# Build:  docker build -t ravenrustrag .
# Run:    docker run -p 8484:8484 -v raven-data:/data ravenrustrag
# ──────────────────────────────────────────────

FROM rust:1.75-slim-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for better caching
COPY Cargo.toml Cargo.lock ./
COPY crates/*/Cargo.toml crates/*/
RUN mkdir -p crates/raven-core/src crates/raven-embed/src crates/raven-store/src \
    crates/raven-split/src crates/raven-load/src crates/raven-search/src \
    crates/raven-server/src crates/raven-cli/src crates/raven-mcp/src && \
    echo "fn main() {}" > crates/raven-cli/src/main.rs && \
    for f in crates/*/src/lib.rs; do touch "$f"; done

# Build dependencies only
RUN cargo build --release --bin raven && \
    rm -rf crates/*/src/*.rs

# Copy actual source code
COPY . .

# Build the actual binary
RUN cargo build --release --bin raven

# ──────────────────────────────────────────────
# Runtime stage
# ──────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/raven /usr/local/bin/raven

# Create data directory
RUN mkdir -p /data

ENV RAVEN_DB=/data/raven.db
EXPOSE 8484

ENTRYPOINT ["raven"]
CMD ["serve", "--host", "0.0.0.0", "--port", "8484", "--db", "/data/raven.db"]