#!/usr/bin/env bash
# Raspberry Pi camera setup (Pi 4 and Pi 5). Runs after the camera probe and
# bootstrap install. Drivers (libcamera, picamera2, pigpio) are installed by the
# bootstrap install via apt; this only wires up the camerad service and pigpiod.

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
    exec sudo -s "$0" "$@"
fi

STATE_FILE=/var/local/LRIMa-central/camera_probe.state
STATE="$(cat "$STATE_FILE" 2>/dev/null || echo none)"
echo "Camera probe result: $STATE"

if [ "$STATE" == "found" ]; then
    if [ -f "/var/local/LRIMa-central/camerad.service" ]; then
        echo "Installing camerad.service..."
        cp "/var/local/LRIMa-central/camerad.service" /etc/systemd/system/camerad.service
        systemctl daemon-reload
        systemctl enable --now camerad.service || true
    else
        echo "Warning: camerad.service not found, skipping service install"
    fi
else
    echo "No camera detected. Skipping camerad.service."
fi

echo "Installing pigpio..."
systemctl enable --now pigpiod

echo "Starting LRIMa-central..."
systemctl enable --now LRIMa-central.service

systemctl disable LRIMa-camera-probe.service LRIMa-bootstrap-install.service LRIMa-camera-setup.service
echo "Bootstrap complete!"