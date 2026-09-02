#!/usr/bin/env bash
# Raspberry Pi camera setup (Pi 4 and Pi 5).
# Runs only if a camera is detected on the CSI port.
# Detects the connected camera, installs the required drivers (libcamera)
# and updates the boot configuration (/boot/firmware/config.txt).

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
    exec sudo -s "$0" "$@"
fi

RASP_MODEL="$(cat /sys/firmware/devicetree/base/model 2>/dev/null || true)"
echo "Raspberry Pi model: $RASP_MODEL"

IS_PI5=false
if [[ "$RASP_MODEL" == *"Raspberry Pi 5"* ]]; then
    IS_PI5=true
fi

# ---------------------------------------------------------------------------
# Camera detection
# ---------------------------------------------------------------------------
camera_detected=false
camera_model=""

detect_via_libcamera() {
    if command -v libcamera-hello >/dev/null 2>&1; then
        libcamera-hello --list-cameras 2>/dev/null | grep -qE "Available cameras|imx[0-9]+|ov[0-9]+|sc[0-9]+"
    else
        return 1
    fi
}

detect_via_vcgencmd() {
    if command -v vcgencmd >/dev/null 2>&1; then
        vcgencmd get_camera 2>/dev/null | grep -q "detected=1"
    else
        return 1
    fi
}

device_tree_sensors() {
    find /proc/device-tree -type f -name compatible -exec cat {} + 2>/dev/null \
        | tr -d '\0' \
        | grep -oE "imx[0-9]+|ov[0-9]+" || true
}

if detect_via_libcamera; then
    camera_detected=true
    camera_model="$(libcamera-hello --list-cameras 2>/dev/null \
        | grep -oE "imx[0-9]+|ov[0-9]+|sc[0-9]+" | head -n1 | tr '[:upper:]' '[:lower:]' || true)"
elif detect_via_vcgencmd; then
    camera_detected=true
elif [[ -n "$(device_tree_sensors)" ]]; then
    camera_detected=true
    camera_model="$(device_tree_sensors | head -n1)"
fi

if [[ "$camera_detected" != true ]]; then
    echo "No camera detected. Skipping camera setup."
    exit 0
fi

echo "Camera detected${camera_model:+: $camera_model}"

# ---------------------------------------------------------------------------
# Boot configuration
# ---------------------------------------------------------------------------
CONFIG_FILE="/boot/firmware/config.txt"
if [ ! -f "$CONFIG_FILE" ]; then
    CONFIG_FILE="/boot/config.txt"
fi

if [ -f "$CONFIG_FILE" ]; then
    cp "$CONFIG_FILE" "$CONFIG_FILE.bak"
    sed -i -E '/^camera_auto_detect=/d; /^dtoverlay=(imx|ov)/d' "$CONFIG_FILE"
    if [[ -n "$camera_model" ]]; then
        printf 'camera_auto_detect=0\ndtoverlay=%s\n' "$camera_model" >> "$CONFIG_FILE"
    else
        printf 'camera_auto_detect=1\n' >> "$CONFIG_FILE"
    fi
    echo "Updated $CONFIG_FILE (backup: $CONFIG_FILE.bak)"
else
    echo "Warning: config.txt not found at $CONFIG_FILE, skipping boot config update"
fi

# ---------------------------------------------------------------------------
# Drivers (libcamera)
# ---------------------------------------------------------------------------
echo "Installing camera drivers..."
apt-get update -q
apt-get install -y \
    libboost-dev \
    libgnutls28-dev openssl libtiff-dev pybind11-dev \
    qtbase5-dev libqt5core5a libqt5widgets5 \
    meson cmake \
    python3-yaml python3-ply \
    libglib2.0-dev libgstreamer-plugins-base1.0-dev

# Install libcamera to interface with ArduCam camera modules
if [ ! -d "/tmp/libcamera/.git" ]; then
    git clone https://github.com/raspberrypi/libcamera.git /tmp/libcamera
fi
cd /tmp/libcamera
if [ ! -d "build" ]; then
    meson setup build --buildtype=release -Dgstreamer=enabled -Dpycamera=enabled
fi
ninja -C build install

# ---------------------------------------------------------------------------
# pigpio (GPIO control)
# ---------------------------------------------------------------------------
echo "Installing pigpio..."
apt-get install -y pigpio python3-pigpio
systemctl enable --now pigpiod

# ---------------------------------------------------------------------------
# Camera systemd service
# ---------------------------------------------------------------------------
if [ -f "/var/local/LRIMa-central/camerad.service" ]; then
    echo "Installing camerad.service..."
    cp "/var/local/LRIMa-central/camerad.service" /etc/systemd/system/camerad.service
    systemctl daemon-reload
    systemctl enable camerad.service
    systemctl restart camerad.service || true
else
    echo "Warning: camerad.service not found, skipping service install"
fi

echo "Camera setup complete! Reboot for changes to fully apply."