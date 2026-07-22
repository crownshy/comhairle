# AGENTS.md

Entry point for LLM agents (and a decent orientation for humans too). This file is
intentionally thin: it points you at the canonical docs and lists the few things you must
not get wrong. **Read the linked docs before writing code.**

## What this repo is

Comhairle is a platform for consultation and deliberation at scale. Admins (policy
makers) create Conversations that route participants (citizens, stakeholders) through a
sequence of engagement tools. Full domain glossary: **[CONTEXT.md](CONTEXT.md)**.

Monorepo layout:

- `api/` — Rust API server (axum, SQLX, sea-query, Postgres).
- `data_model/` — the unified Rust data model ("grammar of participation").
- `adaptors/` — per-tool adaptors (setup, data extraction, login) for the open-source
  tools we integrate.
- `ui/packages/comhairle/` — the SvelteKit frontend (Svelte 5 runes, shadcn-svelte,
  Tailwind v4). This pnpm workspace is where most frontend paths in the docs are rooted.

## Read these before you work

- **[STYLE_GUIDE.md](STYLE_GUIDE.md)** — how we build: general principles, Rust, and the
  full frontend working agreement. Read the relevant section before writing code in that
  layer.
- **[CONTEXT.md](CONTEXT.md)** — domain language. Use these terms exactly; they are load
  bearing (e.g. `Insights` is per-Step, `Report` is conversation-level).
- **[documentation/adr/](documentation/adr)** — architectural decisions and their
  rationale. Check here before reversing a design choice.

## Commands (run from `ui/packages/comhairle`)

- `pnpm test:unit` — Vitest unit tests.
- `pnpm check` — svelte-check (types).
- `pnpm lint` — `prettier --check` + eslint.
- `pnpm prettier --write <files>` — format only the files you touched. **Do not** run the
  repo-wide `pnpm format`; it reformats hundreds of unrelated files and buries your diff.

## Non-negotiables

The short list. Full rationale for each is in [STYLE_GUIDE.md](STYLE_GUIDE.md).

- **Do not `git commit`.** Leave changes for the human to review and commit.
- **Svelte 5 runes only** (`$state` / `$derived` / `$props`). No `export let`, `$:`,
  `$$props`. Never use `$effect` to mirror one piece of state from another; a writable
  `$derived` is the fix.
- **`tryCatchAsync`** for errors, and **never destructure its `Result`** (breaks TS
  narrowing). Branch on `.err`.
- **Colocate by default.** Route-local components live next to the route; promote to
  `src/lib/components/**` only when reused. `src/lib/components/ui/**` is shadcn only.
- **Reuse before you build.** Check `src/lib/components`, `.../ui`, and `src/lib/utils`
  first. Reach for an existing dep over bespoke code.
- **`text-base` is the floor** for content text. Tailwind utilities inline, flat shadcn
  tokens (`bg-card`, `text-muted-foreground`).
- **No em dashes** anywhere in code or prose.
- Before finishing: `pnpm test:unit` and `pnpm check` pass; no `: any` where a real type
  fits; no stray `console.*`.

## Keeping these docs current

When the same review feedback comes up a **second** time, add it to
[STYLE_GUIDE.md](STYLE_GUIDE.md). When a decision reverses or introduces a non-obvious
constraint, write an ADR in [documentation/adr/](documentation/adr). When a domain term
gets coined or redefined, update [CONTEXT.md](CONTEXT.md).
