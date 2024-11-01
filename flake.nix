{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs
    , rust-overlay
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustpkg = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "rust-src" "rustfmt" "clippy" "rust-analyzer" ]; # rust-src for rust-analyzer
            targets = [ "x86_64-unknown-linux-gnu" ];
          });
      in
      with pkgs; {
        devShells = {
          default =
            mkShell {
              name = "slint-dev";
              LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs;
                lib.makeLibraryPath [
                  wayland
                  libxkbcommon
                  fontconfig
                  vulkan-loader 
                  libGL
                ]}";
              buildInputs = [
                pkg-config
                openssl
                rustpkg
              ];
            };
        };
      }
    );
}