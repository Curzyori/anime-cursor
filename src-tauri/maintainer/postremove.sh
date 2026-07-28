#!/bin/bash
# postrm: purge user app data on dpkg -P only.
# Remove/upgrade/deconfigure: do nothing.
set -e

case "$1" in
    purge) ;;
    *) exit 0 ;;
esac

TARGET_USER="${SUDO_USER:-$(logname 2>/dev/null || true)}"
[ -n "$TARGET_USER" ] || exit 0

HOME_DIR=$(getent passwd "$TARGET_USER" | cut -d: -f6)
[ -n "$HOME_DIR" ] || exit 0

rm -rf "$HOME_DIR/.anime-cursor"

exit 0
