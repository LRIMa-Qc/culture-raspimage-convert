#!/usr/bin/env bash
# First bootstrap script: probes every available ArduCam dtoverlay, one per boot,
# until a camera is detected (or all overlays are exhausted). Then hands off to
# the next script in the chain.

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
    exec sudo -s "$0" "$@"
fi

STATE_DIR=/var/local/LRIMa-central
STATE_FILE="$STATE_DIR/camera_probe.state"
CONFIG_FILE=/boot/firmware/config.txt
[ -f "$CONFIG_FILE" ] || CONFIG_FILE=/boot/config.txt
NEXT_SERVICE=LRIMa-bootstrap-install.service

camera_present() {
    if find /proc/device-tree -name compatible -exec cat {} + 2>/dev/null \
        | tr -d '\0' \
        | grep -qE "imx[0-9]+|ov[0-9]+|sc[0-9]+"; then
        return 0
    fi
    if command -v vcgencmd >/dev/null 2>&1 \
        && vcgencmd get_camera 2>/dev/null | grep -q "detected=1"; then
        return 0
    fi
    return 1
}

finish() {
    echo "$1" > "$STATE_FILE"
    systemctl disable LRIMa-camera-probe.service
    systemctl enable --now "$NEXT_SERVICE"
    exit 0
}

mapfile -t OVERLAYS < <(compgen -G '/boot/firmware/overlays/arducam-*.dtbo' 2>/dev/null || true)
if [ ${#OVERLAYS[@]} -eq 0 ]; then
    mapfile -t OVERLAYS < <(compgen -G '/boot/overlays/arducam-*.dtbo' 2>/dev/null || true)
fi

IDX=0
if [ -f "$STATE_FILE" ]; then
    case "$(cat "$STATE_FILE")" in
        found|none) echo "Camera probe already resolved."; exit 0 ;;
        *) IDX="$(cat "$STATE_FILE")" ;;
    esac
fi

if [ "$IDX" -gt 0 ] && camera_present; then
    echo "Camera detected with ${OVERLAYS[$((IDX-1))]}"
    finish found
fi

if [ "$IDX" -ge "${#OVERLAYS[@]}" ]; then
    echo "No camera found with any ArduCam overlay."
    finish none
fi

name="$(basename "${OVERLAYS[$IDX]}" .dtbo)"
echo "Applying dtoverlay=$name (attempt $((IDX+1))/${#OVERLAYS[@]})"
sed -i -E '/^dtoverlay=arducam-/d' "$CONFIG_FILE"
echo "dtoverlay=$name" >> "$CONFIG_FILE"
echo "$((IDX+1))" > "$STATE_FILE"
reboot