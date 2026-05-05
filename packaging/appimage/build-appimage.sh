#!/bin/sh
# AppImage build script for RavenRustRAG
# Requires: appimagetool, cargo (with musl target)
set -e

APPDIR="Raven.AppDir"
VERSION="${VERSION:-1.0.0}"

echo "Building raven for musl (static)..."
cargo build --release --target x86_64-unknown-linux-musl -p raven-cli

echo "Creating AppDir..."
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/applications"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"

cp target/x86_64-unknown-linux-musl/release/raven "$APPDIR/usr/bin/raven"
chmod +x "$APPDIR/usr/bin/raven"

cp packaging/appimage/raven.desktop "$APPDIR/raven.desktop"
cp packaging/appimage/raven.desktop "$APPDIR/usr/share/applications/raven.desktop"

# Generate a simple SVG icon if none exists
if [ ! -f packaging/appimage/raven.svg ]; then
    cat > "$APPDIR/raven.svg" <<'SVG'
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="8" fill="#1a1a2e"/>
  <text x="32" y="44" font-family="monospace" font-size="32" fill="#e94560" text-anchor="middle">R</text>
</svg>
SVG
else
    cp packaging/appimage/raven.svg "$APPDIR/raven.svg"
fi
cp "$APPDIR/raven.svg" "$APPDIR/usr/share/icons/hicolor/256x256/apps/raven.svg"

# Create AppRun
cat > "$APPDIR/AppRun" <<'APPRUN'
#!/bin/sh
SELF="$(readlink -f "$0")"
HERE="${SELF%/*}"
exec "${HERE}/usr/bin/raven" "$@"
APPRUN
chmod +x "$APPDIR/AppRun"

echo "Creating AppImage..."
if command -v appimagetool >/dev/null 2>&1; then
    ARCH=x86_64 appimagetool "$APPDIR" "Raven-${VERSION}-x86_64.AppImage"
    echo "Created Raven-${VERSION}-x86_64.AppImage"
else
    echo "appimagetool not found. Install from https://github.com/AppImage/AppImageKit"
    echo "AppDir is ready at: $APPDIR"
fi
