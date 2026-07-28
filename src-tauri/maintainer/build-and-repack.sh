#!/bin/bash
# Inject maintainer scripts into a Tauri-built .deb via dpkg-deb extract/inject/repack.
# No hard-coded project path or version — resolves everything from this script's location.
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEB_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle/deb"

# Build
cd "$PROJECT_ROOT"
npm run tauri build -- --bundles deb 2>&1

# Find the freshly built package, never a prior repacked artifact.
DEB_FILE=$(find "$DEB_DIR" -maxdepth 1 -name '*.deb' -type f ! -name '*-repacked*.deb' | head -n1)
[ -n "$DEB_FILE" ] || { echo "ERROR: no .deb found in $DEB_DIR" >&2; exit 1; }
echo "Found: $DEB_FILE"

# Extract control + data into temp dir
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

mkdir -p "$WORK/DEBIAN"
dpkg-deb -e "$DEB_FILE" "$WORK/DEBIAN"
dpkg-deb -x "$DEB_FILE" "$WORK"

# Inject maintainer scripts
cp "$SCRIPT_DIR/preremove.sh" "$WORK/DEBIAN/prerm"
cp "$SCRIPT_DIR/postremove.sh" "$WORK/DEBIAN/postrm"
chmod 755 "$WORK/DEBIAN/prerm" "$WORK/DEBIAN/postrm"

# Atomic repack: write to sibling .tmp, then mv
OUTPUT="${DEB_FILE%.deb}-repacked.deb"
dpkg-deb -Zxz --root-owner-group -b "$WORK" "$OUTPUT.tmp"
mv "$OUTPUT.tmp" "$OUTPUT"

echo "=== Repacked: $OUTPUT ==="
echo "Verifying scripts:"
VERIFY=$(mktemp -d)
trap 'rm -rf "$WORK" "$VERIFY"' EXIT
mkdir -p "$VERIFY/DEBIAN"
dpkg-deb -e "$OUTPUT" "$VERIFY/DEBIAN"
for script in prerm postrm; do
    [ -f "$VERIFY/DEBIAN/$script" ] && echo "$script: present" || { echo "$script: MISSING" >&2; exit 1; }
done
echo "control: OK"
echo "=== Done ==="
