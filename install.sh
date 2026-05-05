#!/bin/sh
# RavenRustRAG installer
# Usage: curl -sSf https://raw.githubusercontent.com/egkristi/ravenrustrag/main/install.sh | sh
set -e

REPO="egkristi/ravenrustrag"
BINARY="raven"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64|amd64)   ARTIFACT="raven-linux-amd64" ;;
                aarch64|arm64)  ARTIFACT="raven-linux-arm64" ;;
                *)              echo "Error: unsupported architecture: $ARCH"; exit 1 ;;
            esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64|amd64)   ARTIFACT="raven-darwin-amd64" ;;
                aarch64|arm64)  ARTIFACT="raven-darwin-arm64" ;;
                *)              echo "Error: unsupported architecture: $ARCH"; exit 1 ;;
            esac
            ;;
        *)
            echo "Error: unsupported OS: $OS (use Windows binaries or cargo install)"
            exit 1
            ;;
    esac
}

# Get latest release tag
get_latest_version() {
    VERSION="$(curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' \
        | head -1 \
        | sed 's/.*"tag_name": *"//;s/".*//')"

    if [ -z "$VERSION" ]; then
        echo "Error: could not determine latest version"
        exit 1
    fi
}

main() {
    detect_platform
    get_latest_version

    URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}"

    echo "Installing ${BINARY} ${VERSION} (${ARTIFACT})..."
    echo "  From: ${URL}"
    echo "  To:   ${INSTALL_DIR}/${BINARY}"

    TMPFILE="$(mktemp)"
    trap 'rm -f "$TMPFILE"' EXIT

    curl -sSfL "$URL" -o "$TMPFILE"
    chmod +x "$TMPFILE"

    if [ -w "$INSTALL_DIR" ]; then
        mv "$TMPFILE" "${INSTALL_DIR}/${BINARY}"
    else
        echo "  (requires sudo)"
        sudo mv "$TMPFILE" "${INSTALL_DIR}/${BINARY}"
    fi

    echo "Installed ${BINARY} ${VERSION} to ${INSTALL_DIR}/${BINARY}"
    "${INSTALL_DIR}/${BINARY}" --version 2>/dev/null || true
}

main
