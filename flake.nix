{
  description = "Build deps for libcamera-rs";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell rec {
            buildInputs = [
              clang
              cmake
              libcamera
              pkg-config
              rust-bin.stable.latest.default
              rust-bindgen
            ];

            LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages_latest.libclang.lib];
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };
        }
    );
}
