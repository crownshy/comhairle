{
  description = "Comhairle development environment (Rust API + SvelteKit UI)";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  # Flake outputs
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
          (final: prev: {
            rustToolchain = prev.rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "clippy" "rust-src" "rustfmt" ];
            };
          })
        ];

        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default = pkgs.mkShell {
          env = {
            # Build-time toolchain env only.
            # Runtime/app config (DATABASE_URL, MAILER__*, secrets, etc.)
            # lives in `.env` (gitignored) — see `.env.example` for the full
            # list. `.envrc` loads `.env` via direnv, and the API binary
            # itself loads it via `dotenvy::dotenv()`.
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          };

          packages = with pkgs; [
            # ---- Rust toolchain ----
            rustToolchain
            sqlx-cli
            cargo-watch
            cargo-nextest
            bacon

            # ---- Native build deps ----
            openssl
            pkg-config
            cmake
            clang
            libiconv

            # ---- Database / infra ----
            postgresql_16   # provides psql client
            redis           # provides redis-cli; runtime redis can also come from docker

            # ---- Task runner / misc ----
            just
            watchexec
            atac
            jq
            git

            # ---- Frontend ----
            nodejs_22
            pnpm
            corepack
          ] ++ lib.optionals stdenv.isDarwin [
            # macOS-specific: frameworks the aws-* and openssl crates can need
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.apple_sdk.frameworks.CoreFoundation
          ] ++ lib.optionals stdenv.isLinux [
            # Linux-only: minikube/kubectl for local k8s work
            minikube
            kubectl
          ];

          shellHook = ''
            echo ""
            echo "Comhairle dev shell ready"
            echo "  rust:    $(rustc --version 2>/dev/null)"
            echo "  node:    $(node --version 2>/dev/null)"
            echo "  pnpm:    $(pnpm --version 2>/dev/null)"
            echo ""
            echo "Quickstart:"
            echo "  just pg            # start Postgres in Docker (needs Docker Desktop/Colima)"
            echo "  just api-dev       # run API"
            echo "  cd ui/packages && pnpm comhairle   # run frontend"
            echo ""
          '';
        };

        # `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      });
}
