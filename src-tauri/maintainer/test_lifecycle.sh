#!/bin/bash
# Minimal shell tests for maintainer scripts.
# No external test framework — just assert + demo pattern.
# Does NOT install/remove any package.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PASS=0
FAIL=0

_pass() { PASS=$((PASS + 1)); echo "  OK: $1"; }
_fail() { FAIL=$((FAIL + 1)); echo "  FAIL: $1"; }

assert_exit0() {
    local label="$1"; shift
    if "$@" >/dev/null 2>&1; then _pass "$label"; else _fail "$label (exit=$?)"; fi
}

assert_exit1() {
    local label="$1"; shift
    if "$@" >/dev/null 2>&1; then _fail "$label (expected exit!=0)"; else _pass "$label"; fi
}

# Tests use subshells with unset SUDO_USER and failing logname to simulate no target user.

echo "=== prerm tests ==="

# 1. Upgrade: must exit 0, no action
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/preremove.sh" upgrade
) >/dev/null 2>&1 && _pass "prerm: upgrade → exit 0" || _fail "prerm: upgrade"

# 2. Deconfigure: must exit 0, no action
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/preremove.sh" deconfigure
) >/dev/null 2>&1 && _pass "prerm: deconfigure → exit 0" || _fail "prerm: deconfigure"

# 3. Remove: runs cleanup path (may fail gsettings but still exit 0)
(
    unset SUDO_USER
    logname() { echo "testuser"; }
    export -f logname
    bash "$SCRIPT_DIR/preremove.sh" remove
) >/dev/null 2>&1 && _pass "prerm: remove → exit 0" || _fail "prerm: remove"

# 4. Purge: runs cleanup path
(
    unset SUDO_USER
    logname() { echo "testuser"; }
    export -f logname
    bash "$SCRIPT_DIR/preremove.sh" purge
) >/dev/null 2>&1 && _pass "prerm: purge → exit 0" || _fail "prerm: purge"

# 5. No target user: exit 0
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/preremove.sh" remove
) >/dev/null 2>&1 && _pass "prerm: no user → exit 0" || _fail "prerm: no user"

# 6. No arg: exit 0 (acts like upgrade)
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/preremove.sh"
) >/dev/null 2>&1 && _pass "prerm: no arg → exit 0" || _fail "prerm: no arg"

echo "=== postrm tests ==="

# 7. Purge: runs cleanup path
(
    unset SUDO_USER
    logname() { echo "testuser"; }
    export -f logname
    bash "$SCRIPT_DIR/postremove.sh" purge
) >/dev/null 2>&1 && _pass "postrm: purge → exit 0" || _fail "postrm: purge"

# 8. Remove: no action, exit 0
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/postremove.sh" remove
) >/dev/null 2>&1 && _pass "postrm: remove → exit 0" || _fail "postrm: remove"

# 9. Upgrade: no action, exit 0
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/postremove.sh" upgrade
) >/dev/null 2>&1 && _pass "postrm: upgrade → exit 0" || _fail "postrm: upgrade"

# 10. No arg: exit 0
(
    unset SUDO_USER
    bash "$SCRIPT_DIR/postremove.sh"
) >/dev/null 2>&1 && _pass "postrm: no arg → exit 0" || _fail "postrm: no arg"

echo "=== repack tests ==="

# Build a dummy .deb, then test the extract/inject/repack cycle
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

# Create minimal fake package tree
mkdir -p "$WORK/pkg/DEBIAN"
cat > "$WORK/pkg/DEBIAN/control" <<EOF
Package: anime-cursor
Version: 0.0.0-test
Architecture: amd64
Maintainer: test
Description: test package
EOF

mkdir -p "$WORK/pkg/usr/share/test"
echo "payload" > "$WORK/pkg/usr/share/test/file.txt"

# Build dummy .deb
FAKE_DEB="$WORK/fake.deb"
dpkg-deb -Zxz -b "$WORK/pkg" "$FAKE_DEB" 2>/dev/null || dpkg-deb -b "$WORK/pkg" "$FAKE_DEB"
[ -f "$FAKE_DEB" ] || { echo "FAIL: could not create dummy .deb"; exit 1; }

# Test: extract → inject → repack → verify scripts present
REPACK_WORK=$(mktemp -d)
mkdir -p "$REPACK_WORK/DEBIAN"
dpkg-deb -e "$FAKE_DEB" "$REPACK_WORK/DEBIAN"
dpkg-deb -x "$FAKE_DEB" "$REPACK_WORK"

cp "$SCRIPT_DIR/preremove.sh" "$REPACK_WORK/DEBIAN/prerm"
cp "$SCRIPT_DIR/postremove.sh" "$REPACK_WORK/DEBIAN/postrm"
chmod 755 "$REPACK_WORK/DEBIAN/prerm" "$REPACK_WORK/DEBIAN/postrm"

REPACKED="$WORK/repacked.deb"
dpkg-deb -Zxz --root-owner-group -b "$REPACK_WORK" "$REPACKED" 2>/dev/null || \
    dpkg-deb -b "$REPACK_WORK" "$REPACKED"

# Verify
VERIFY=$(mktemp -d)
mkdir -p "$VERIFY/DEBIAN"
dpkg-deb -e "$REPACKED" "$VERIFY/DEBIAN"

[ -f "$VERIFY/DEBIAN/prerm" ] && _pass "repack: prerm present" || _fail "repack: prerm missing"
[ -f "$VERIFY/DEBIAN/postrm" ] && _pass "repack: postrm present" || _fail "repack: postrm missing"
[ -f "$VERIFY/DEBIAN/control" ] && _pass "repack: control present" || _fail "repack: control missing"

# Verify data survived
dpkg-deb -x "$REPACKED" "$VERIFY/data"
[ -f "$VERIFY/data/usr/share/test/file.txt" ] && _pass "repack: data preserved" || _fail "repack: data lost"

# Verify script content matches source
diff -q "$SCRIPT_DIR/preremove.sh" "$VERIFY/DEBIAN/prerm" >/dev/null 2>&1 && \
    _pass "repack: prerm content matches" || _fail "repack: prerm content mismatch"
diff -q "$SCRIPT_DIR/postremove.sh" "$VERIFY/DEBIAN/postrm" >/dev/null 2>&1 && \
    _pass "repack: postrm content matches" || _fail "repack: postrm content mismatch"

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ] || exit 1
