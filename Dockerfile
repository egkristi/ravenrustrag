# Multi-stage build for RavenRustRAG
# Produces a small static binary (~15MB)

FROM rust:1.75-bookworm AS builder

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY crates/raven-core/Cargo.toml crates/raven-core/Cargo.toml
COPY crates/raven-embed/Cargo.toml crates/raven-embed/Cargo.toml
COPY crates/raven-store/Cargo.toml crates/raven-store/Cargo.toml
COPY crates/raven-split/Cargo.toml crates/raven-split/Cargo.toml
COPY crates/raven-load/Cargo.toml crates/raven-load/Cargo.toml
COPY crates/raven-search/Cargo.toml crates/raven-search/Cargo.toml
COPY crates/raven-server/Cargo.toml crates/raven-server/Cargo.toml
COPY crates/raven-mcp/Cargo.toml crates/raven-mcp/Cargo.toml
COPY crates/raven-cli/Cargo.toml crates/raven-cli/Cargo.toml

# Create dummy source files for dependency caching
RUN mkdir -p crates/raven-core/src && echo "pub fn dummy() {}" > crates/raven-core/src/lib.rs && \
    mkdir -p crates/raven-embed/src && echo "pub fn dummy() {}" > crates/raven-embed/src/lib.rs && \
    mkdir -p crates/raven-store/src && echo "pub fn dummy() {}" > crates/raven-store/src/lib.rs && \
    mkdir -p crates/raven-split/src && echo "pub fn dummy() {}" > crates/raven-split/src/lib.rs && \
    mkdir -p crates/raven-load/src && echo "pub fn dummy() {}" > crates/raven-load/src/lib.rs && \
    mkdir -p crates/raven-search/src && echo "pub fn dummy() {}" > crates/raven-search/src/lib.rs && \
    mkdir -p crates/raven-server/src && echo "pub fn dummy() {}" > crates/raven-server/src/lib.rs && \
    mkdir -p crates/raven-mcp/src && echo "pub fn dummy() {}" > crates/raven-mcp/src/lib.rs && \
    mkdir -p crates/raven-cli/src && echo "fn main() {}" > crates/raven-cli/src/main.rs

# Build dependencies only (cached layer)
RUN cargo build --release 2>/dev/null || true

# Copy real source code
COPY crates/ crates/

# Touch source files to invalidate cache for actual code
RUN find crates -name "*.rs" -exec touch {} +

# Build the actual binary
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN groupadd -r raven && useradd -r -g raven -d /home/raven -m raven

COPY --from=builder /app/target/release/raven /usr/local/bin/raven

# Default data directory
RUN mkdir -p /data && chown raven:raven /data
VOLUME /data

USER raven
WORKDIR /data

ENV RAVEN_DB=/data/raven.db

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD raven doctor || exit 1

ENTRYPOINT ["raven"]
CMD ["serve"]
