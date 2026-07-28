#!/bin/bash
# prerm: restore default cursor on remove/purge only.
# Never act on upgrade/deconfigure — theme dir must survive upgrades.
set -e

case "$1" in
    remove|purge) ;;
    *) exit 0 ;;
esac

TARGET_USER="${SUDO_USER:-$(logname 2>/dev/null || true)}"
[ -n "$TARGET_USER" ] || exit 0

HOME_DIR=$(getent passwd "$TARGET_USER" | cut -d: -f6)
[ -n "$HOME_DIR" ] || exit 0

# Explicit D-Bus session env for the target user
UID_VAL=$(id -u "$TARGET_USER")
DBUS_ENV="DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/${UID_VAL}/bus"
XDG_ENV="XDG_RUNTIME_DIR=/run/user/${UID_VAL}"

# Reset cursor before deleting theme dir. Only delete on success.
if sudo -u "$TARGET_USER" "$DBUS_ENV" "$XDG_ENV" \
        gsettings set org.gnome.desktop.interface cursor-theme 'default' 2>/dev/null; then
    sudo -u "$TARGET_USER" "$DBUS_ENV" "$XDG_ENV" \
        gsettings set org.gnome.desktop.interface cursor-size 24 2>/dev/null || true
    rm -rf "$HOME_DIR/.icons/anime-cursor"
fi
# else: gsettings failed (no GNOME session, unsupported DE) — keep theme in place

exit 0
