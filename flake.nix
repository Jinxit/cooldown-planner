{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, fenix, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        lib = nixpkgs.lib;
        stdenv = pkgs.stdenv;
        llvmPackages = pkgs.llvmPackages;

        #pkgs = nixpkgs.legacyPackages.${system};
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit overlays system; };
      in
      rec {
        devShells.default = pkgs.mkShell {
          inherit (fenix.packages.${system}.complete) cargo rustc;
          buildInputs = with pkgs; [
            bashInteractive
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" "cargo" "clippy" "rustc" "rustfmt" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
            binutils
            glibc
            rust-analyzer
            cacert
            openssl
            pkg-config
            sqlx-cli
            cargo-leptos
            nodejs_20
            cargo-watch
            wasm-pack
            binaryen
          ];

          RUST_SRC_PATH = "${fenix.packages.${system}.complete.rust-src}/lib/rustlib/src/rust/library";
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";

          shellHook = ''
            export CURL_CA_BUNDLE=/etc/ssl/certs/ca-bundle.crt
            export PS1="🚦 {\[$(tput sgr0)\]\[\033[38;5;228m\]\w\[$(tput sgr0)\]\[\033[38;5;15m\]}\\$ \[$(tput sgr0)\]"
          '';
        };
      }
    );
}
