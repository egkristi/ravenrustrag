# Multi-stage build for RavenRustRAG
# Produces a small static binary (~15MB)

# --- Build stage (Alpine = native musl, no cross-compilation issues) ---
FROM rust:1.86-alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev gcc

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

# Build the actual binary (static musl)
RUN cargo build --release

# --- Runtime stage (scratch — no OS, just the binary) ---
FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/raven /raven

VOLUME /data

ENV RAVEN_DB=/data/raven.db
ENV RAVEN_HOST=0.0.0.0
ENV RAVEN_LOG_FORMAT=json

EXPOSE 8484

USER 65534:65534

ENTRYPOINT ["/raven"]
CMD ["serve"]
