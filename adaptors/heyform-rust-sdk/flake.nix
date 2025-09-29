{
  description = "HeyForm Rust SDK development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            # Additional tools for development
            cargo-edit
            cargo-watch
            cargo-expand
          ];

          shellHook = ''
            echo "🦀 HeyForm Rust SDK development environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available tools:"
            echo "  - rust-analyzer (LSP)"
            echo "  - clippy (linter)"
            echo "  - rustfmt (formatter)"
            echo "  - cargo-edit (dependency management)"
            echo "  - cargo-watch (auto-rebuild)"
            echo "  - cargo-expand (macro expansion)"
            echo ""
            echo "Run 'cargo build' to build the project"
            echo "Run 'cargo test' to run tests"
            echo "Run 'cargo clippy' to lint code"
            echo "Run 'cargo fmt' to format code"
          '';

          # Set environment variables for OpenSSL
          OPENSSL_DEV = pkgs.openssl.dev;
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      });
}