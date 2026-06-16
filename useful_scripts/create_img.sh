#!/usr/bin/env bash

set -euo pipefail

cd /home/lrima/Documents/prog/image-culture/culture-raspimage-convert
rm -f 2026-04-21-raspios-trixie-arm64-lite.img
cp 2026-04-21-raspios-trixie-arm64-lite.img.xz.bck 2026-04-21-raspios-trixie-arm64-lite.img.xz
xz -d 2026-04-21-raspios-trixie-arm64-lite.img.xz;
cargo run --release -- --raspberry-pi-image-file-path 2026-04-21-raspios-trixie-arm64-lite.img --config-file-path config.json
