#!/usr/bin/env bash

set -euo pipefail

if [ "$EUID" -ne 0 ]
then
    exec sudo -s "$0" "$@"
fi

rm -f 2026-04-21-raspios-trixie-arm64-lite.img
cp 2026-04-21-raspios-trixie-arm64-lite.img.xz.bck 2026-04-21-raspios-trixie-arm64-lite.img.xz; xz -d 2026-04-21-raspios-trixie-arm64-lite.img.xz;
echo "decompressed, will create good image"
cargo run -- --raspberry-pi-image-file-path 2026-04-21-raspios-trixie-arm64-lite.img --config-file-path config.json
echo "created, will flash"
sudo dd if=2026-04-21-raspios-trixie-arm64-lite.img of=/dev/sdb bs=4M conv=fsync status=progress
echo "flashed, will fill"
sudo parted -s -a opt /dev/sdb "resizepart 2 100%"
sudo e2fsck -f /dev/sdb2
sudo resize2fs /dev/sdb2
echo "did everything, will eject and sync"
sudo sync
sudo eject /dev/sdb

echo "you are good to go!"
sudo -k
