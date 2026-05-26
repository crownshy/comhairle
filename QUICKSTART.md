# Quickstart

## Prerequisites

You only need two things on the host. Everything else comes from Nix.

- **Nix** with flakes enabled (recommended installer: <https://zero-to-nix.com/start/install> or [Determinate Nix Installer](https://install.determinate.systems/)).
- **Docker daemon** Docker Desktop, Colima, or OrbStack. The Nix shell ships the `psql` client but not the Docker daemon.
  - macOS: `brew install --cask docker` _or_ `brew install colima && colima start`.

## Setup

Clone the repository:
```bash
git clone <repo> comhairle
cd comhairle
```

Set up environment variables:
```bash
cp .env.example .env
```

Enter the Nix development shell. First run downloads the toolchain (a few minutes)
```bash
nix develop 
```
Once inside the shell you have: `cargo`, `rustc`, `clippy`, `rust-analyzer`, `rustfmt`, `sqlx`, `bacon`, `cargo-watch`, `just`, `watchexec`, `node`, `pnpm`, `psql`, `redis-cli`, `atac`, plus OpenSSL / pkg-config / cmake / clang wired up via env vars.

## Fast path (tmux)

If you just want everything running in one go:

```bash
nix develop -c just all
```

This spins up a `comhairle` tmux session with four windows: `postgres`, `api`, `ui`, and a `seed` shell. Once the API is up, switch to the `seed` window (`Ctrl-b 3`) and run `just seed`.

## 1. Run DB
```bash
nix develop
just pg
```

This runs `postgres:16` on `localhost:5434` with user/password/db all set to `comhairle`. Data is persisted to `./pg_data` in the repo (already gitignored).

## 2. Run API

In a second shell, start the API (migrations run automatically on boot):

```bash
nix develop
just api-dev
```

### 3. Seed DB

Run `just seed` to populate the database with initial data including:
- **Default admin login:** `admin@crown-shy.com` / `adminPassword123!`


## 4. Run UI

In a third shell:

```bash
nix develop
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

