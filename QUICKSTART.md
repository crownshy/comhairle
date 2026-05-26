# Quickstart

## Prerequisites

You only need two things on the host. Everything else comes from Nix.

- **Nix** with flakes enabled (recommended installer: <https://zero-to-nix.com/start/install> or [Determinate Nix Installer](https://install.determinate.systems/)).
- **Docker daemon** Docker Desktop, Colima, or OrbStack. The Nix shell ships the `psql` client but not the Docker daemon.
  - macOS: `brew install --cask docker` _or_ `brew install colima && colima start`.

## Setup

```bash
git clone <repo> comhairle
cd comhairle
nix develop          # first run downloads the toolchain (a few minutes)
cp .env.example .env
```
Once inside the shell you have: `cargo`, `rustc`, `clippy`, `rust-analyzer`, `rustfmt`, `sqlx`, `bacon`, `cargo-watch`, `just`, `watchexec`, `node`, `pnpm`, `psql`, `redis-cli`, `atac`, plus OpenSSL / pkg-config / cmake / clang wired up via env vars.

## Run DB

```bash
just pg
```

This runs `postgres:16` on `localhost:5434` with user/password/db all set to `comhairle`. Data is persisted to `./pg_data` in the repo (already gitignored).

## Run API

In a second shell, start the API (migrations run automatically on boot):

```bash
just api-dev
```

### Seed DB

Run `just seed` to populate the database with initial data including:
- **Default admin login:** `admin@crown-shy.com` / `adminPassword123!`


## Run UI

In a third shell:

```bash
cd ui/packages && pnpm comhairle
```

That's it.

| What         | URL                                |
| ------------ | ---------------------------------- |
| Frontend     | <http://localhost:5173>            |
| API          | <http://localhost:3000>            |
| Postgres     | `localhost:5434` (user/db: `comhairle`) |


## Optional services

`.env` is gitignored. Copy from `.env.example` and uncomment the sections you need. See the comments inline in `.env.example` for what each block unlocks (Mailer, Translator, Bot service, Video calls, etc.).

