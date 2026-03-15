#!/bin/bash
# Build AppImage for Luzumi manually (workaround for Tauri CLI linuxdeploy bug)
set -e

ARCH=$(uname -m)
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
RELEASE_DIR="$PROJECT_DIR/src-tauri/target/release"
APPDIR="$RELEASE_DIR/bundle/appimage/luzumi.AppDir"
OUTPUT="$RELEASE_DIR/bundle/appimage/luzumi_$(grep '"version"' "$PROJECT_DIR/src-tauri/tauri.conf.json" | head -1 | sed 's/.*: *"\(.*\)".*/\1/')_amd64.AppImage"

# Download tools if needed
TOOLS_DIR="$PROJECT_DIR/scripts/.appimage-tools"
mkdir -p "$TOOLS_DIR"

LINUXDEPLOY="$TOOLS_DIR/linuxdeploy-$ARCH.AppImage"
APPIMAGETOOL="$TOOLS_DIR/appimagetool-$ARCH.AppImage"

if [ ! -f "$LINUXDEPLOY" ]; then
    echo "Downloading linuxdeploy..."
    curl -L -o "$LINUXDEPLOY" "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-$ARCH.AppImage"
    chmod +x "$LINUXDEPLOY"
fi

if [ ! -f "$APPIMAGETOOL" ]; then
    echo "Downloading appimagetool..."
    curl -L -o "$APPIMAGETOOL" "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-$ARCH.AppImage"
    chmod +x "$APPIMAGETOOL"
fi

# Build the project first (deb/rpm only via Tauri)
echo "Building project..."
cd "$PROJECT_DIR"
npx tauri build

# Create AppDir structure
echo "Creating AppImage..."
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/applications"
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"

cp "$RELEASE_DIR/luzumi" "$APPDIR/usr/bin/"
cp "$PROJECT_DIR/src-tauri/icons/archive.png" "$APPDIR/usr/share/icons/hicolor/256x256/apps/luzumi.png"
cp "$PROJECT_DIR/src-tauri/icons/archive.png" "$APPDIR/luzumi.png"

cat > "$APPDIR/usr/share/applications/luzumi.desktop" << 'EOF'
[Desktop Entry]
Name=Luzumi
Exec=luzumi %u
Icon=luzumi
Type=Application
MimeType=inode/directory;
Categories=System;FileTools;FileManager;
Terminal=false
StartupNotify=true
EOF

cp "$APPDIR/usr/share/applications/luzumi.desktop" "$APPDIR/luzumi.desktop"

# Create AppRun
cat > "$APPDIR/AppRun" << 'APPRUN'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/luzumi" "$@"
APPRUN
chmod +x "$APPDIR/AppRun"

# Build AppImage
ARCH=$ARCH APPIMAGE_EXTRACT_AND_RUN=1 "$APPIMAGETOOL" "$APPDIR" "$OUTPUT"

echo ""
echo "AppImage created: $OUTPUT"
