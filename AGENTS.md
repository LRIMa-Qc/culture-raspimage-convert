# culture-raspimage-convert

Raspberry Pi OS image customizer. Patches a raw `.img` with LRIMa Central config (WiFi, Bluetooth, hostname, systemd services, bootstrap install script) offline via `guestfs` (libguestfs). Two binaries: `image-core` (main), `image-installer` (unfinished stdin helper).

## Build & run

```bash
cargo build --release
cargo run --release -- --raspberry-pi-image-file-path <path> --config-file-path <path>
```

Config can also be passed as inline JSON: `--config-json '{"working_directory": "...", ...}'`.

Config file is JSON, fields match `src/config_commons/mod.rs` (17 fields). See `config.example.json`.

## Architecture

- `src/bin/image-core/` — main binary. Mounts RPi image via `guestfs`, renders Jinja templates from `config_file/`, writes them to the image filesystem.
- `src/bin/image-installer/` — stub, reads one line from stdin. Not used in production.
- `src/config_commons/` — shared `Config` struct (serde Deserialize).
- `config_file/` — Jinja2 templates (`{{VARIABLE}}` syntax via `minijinja`). Each `handle_*` function in `handle.rs` reads a template, substitutes keys, writes to target path.
- `useful_scripts/` — shell wrappers for the full workflow: decompress xz → run image-core → `dd` to SD card → resize partition.

## Key operations (handle.rs)

| Function | Template | Target path |
|---|---|---|
| `handle_systemd_boot_services` | `LRIMa-central.service` | `/etc/systemd/system/LRIMa-central.service` + symlink |
| `handle_bluetooth_services` | `bluetooth.conf` | `/etc/bluetooth/main.conf` |
| `handle_config_file` | `config.ini` | `/var/local/LRIMa-central/config.ini` |
| `handle_wifi_configuration` | `networkmanager.nmconnection` | `/etc/NetworkManager/system-connections/LRIMa.nmconnection` (chmod 600) |
| `handle_wifi_country` | `cfg80211.conf` | `/etc/modprobe.d/cfg80211.conf` |
| `handle_bootstrap_install_script` | `install.sh` | `/var/local/LRIMa-central/install.sh` (chmod 700) |
| `handle_bootstrap_install_service` | `LRIMa-centrale-install-runonce.service` | systemd oneshot + symlink to `cloud-init.target.wants/` |
| `handle_hostname` | `hostname` | `/etc/hostname` |
| `handle_sudoers_deploy` | `sudoers_deploy` | `/etc/sudoers.d/deployer` |
| `handle_poppup_raspos` | (none) | `rm /usr/lib/systemd/system/userconfig.service` |

## Validation (validation.rs)

Config validated before any writes. Checks:
- `wifi_country`: exactly 2 uppercase chars
- `wifi_ssid`/`wifi_password`: validated via `wpa-psk` crate
- `hostname`, `account_name`, `bluetooth_controller_name`, `filename_of_repo`: must match `^[a-zA-Z][-a-z0-9A-Z_]*`
- `standard_logs`, `error_logs`: must be valid paths; must share same parent directory

## Constraints

- **Rust edition 2024** — requires nightly Rust. `rustup default nightly` or `cargo +nightly build`.
- Requires `libguestfs` (crate `guestfs`). Install system package: `brew install libguestfs` (macOS) or `apt install libguestfs-dev` (Debian).
- `guestfs` needs `qemu` and `supermin`; on macOS also needs `qemu` installed.
- Image file must be writable (not read-only). Scripts decompress `.xz` to raw `.img` first.
- `cargo run` must be run from repo root (templates loaded as `config_file/*` relative paths).
- `.gitignore` ignores `*.img`, `*.xz`, `/target`.

## Image creation workflow

1. Download Raspberry Pi OS Lite image (`.img.xz`)
2. Create `config.json` from `config.example.json`
3. `xz -d image.img.xz`
4. `cargo run --release -- --raspberry-pi-image-file-path image.img --config-file-path config.json`
5. `dd if=image.img of=/dev/sdX bs=4M conv=fsync status=progress`
6. `parted /dev/sdX resizepart 2 100%` + `e2fsck` + `resize2fs`

`useful_scripts/create_iso_and_flash.sh` automates steps 3-6 (requires sudo). Assumes device `/dev/sdb`.

## Notes

- `image-installer` binary is incomplete — prompts for stdin but does nothing with it.
- `config_file/install.sh` is a Jinja template that becomes a first-boot bootstrap script. Clones `iot_obj-sicro-sensor` repo, sets up Python venv, installs deps per Pi 4/5.
- Bluetooth adapter required (USB dongle). WiFi recommended on 2.4GHz only.