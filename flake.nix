{
  description = "Raspberry Pi OS image customizer for LRIMa Central";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        nightlyRust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rustfmt" ];
        });

        rustPlatform = pkgs.makeRustPlatform {
          rustc = nightlyRust;
          cargo = nightlyRust;
        };
      in {
        packages.default = rustPlatform.buildRustPackage {
          pname = "culture-raspimage-convert";
          version = "1.0.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ libguestfs ];
          doCheck = false;
          meta.platforms = pkgs.lib.platforms.linux;
        };

        devShells.default = pkgs.mkShell {
          packages = [ pkgs.nightlyRust
		  pkgs.pkg-config
		  pkgs.libguestfs
		  pkgs.qemu ];
        };
      });
}
