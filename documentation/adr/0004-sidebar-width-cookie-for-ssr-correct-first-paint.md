# ADR-0004: Persist admin sidebar width in a cookie for an SSR-correct first paint

**Status:** accepted
**Date:** 2026-07-17

## Context

The admin sidebar is resizable. On refresh it visibly **jumped width**: the first
painted frame showed the hardcoded `DEFAULT_WIDTH` (288px), then snapped to the
user's saved width.

Root cause: the saved width lived in **`localStorage`**
([sidebarWidth.svelte.ts](../../ui/packages/comhairle/src/lib/components/sidebarWidth.svelte.ts)),
which neither the server nor the first client paint can read. The width was held in a
**module-level singleton** whose `set` / `hydrate` / `persist` are all client-guarded,
so it was *never* mutated during SSR. The provider bound `--sidebar-width` to that
singleton ([+layout.svelte](../../ui/packages/comhairle/src/routes/(admin)/+layout.svelte)),
so SSR always emitted 288 and the client snapped to the real value inside an `$effect`.
An `initializing` flag disabled the CSS transition during that snap, so the artifact was
a hard *jump*, not a slide.

This is the same class of bug as dark-mode FOUC. The app already solves *that* with a
blocking inline script in [app.html](../../ui/packages/comhairle/src/app.html). Separately,
the shadcn sidebar already persists its **open/collapsed** state in a **cookie**
(`sidebar:state`, [constants.ts](../../ui/packages/comhairle/src/lib/components/ui/sidebar/constants.ts)),
precisely so the correct state is known server-side. Width was the odd one out.

Key facts that made the fix straightforward:
- SSR is on for the admin group (no `ssr = false`), and the root layout already reads a
  cookie server-side (`auth-token`,
  [+layout.server.ts](../../ui/packages/comhairle/src/routes/+layout.server.ts)).
- The theme's *server* side is a deploy-time env var (`PUBLIC_THEME`), not a per-user
  cookie, so there was no per-user cookie-injection pattern to copy for width.
- The `(admin)` group had only a **universal** `+layout.ts`, and universal loads cannot
  read cookies. A **server** load is required.

## Decision

**Make the correct width known at first paint by storing it in a cookie the server
reads, and move the width value out of the module singleton into layout-owned reactive
state.** No animation (a slide-on-every-refresh masks the jump, it does not remove it).

- **Cookie, not localStorage.** `sidebar:width` (parallels `sidebar:state`), 7-day
  max-age (reuse `SIDEBAR_COOKIE_MAX_AGE`), `path=/`, `SameSite=Lax`. Written client-side
  via `document.cookie` on drag-end / expand-click, the same way `sidebar:state` is
  written. The value is clamped to `[MIN_WIDTH, MAX_WIDTH]` before writing **and**
  re-clamped on read so a tampered cookie cannot produce a silly width.
- **Read server-side** in a new `(admin)/+layout.server.ts` via `cookies.get('sidebar:width')`,
  clamped, returned as `data.sidebarWidth`.
- **Layout owns the width** as a writable derived seeded from load data:
  `let width = $derived(data.sidebarWidth)`. This is the "seed from a prop, re-seed on
  change" pattern the working agreement holds up as canonical (a `$derived`, not an
  `$effect`-sync). Drag handlers write `width` directly; it re-seeds from the cookie on
  the next refresh. `width` + a `persist` are exposed via a small context so
  `SidebarResizeHandle` and `AdminNav` consume them instead of the singleton.
- **The module singleton's *state* is deleted.** `sidebarWidth.svelte.ts` is kept only
  for the pure constants / clamp / cookie helpers.

Because the SSR value and the live value are one reactive expression seeded from the same
per-request load data, there is structurally no hydration mismatch and no flash.

## Considered options

- **Animate the jump** (rejected): turns the snap into a slide on every refresh, which is
  arguably worse and does not meet the goal of "no jump."
- **Keep localStorage + a blocking inline script in `app.html`** (rejected): mirrors the
  theme FOUC fix, but the provider sets `--sidebar-width` as an inline style on the
  wrapper, which beats a `:root` var a head script would set, forcing a restructure of
  where the var is applied. The server HTML would also still say 288, only masked by a
  pre-paint DOM mutation.
- **Keep the singleton, gate the style on an "interacted" flag** (rejected as the lesser
  shape): smallest diff, but keeps the root-cause singleton and needs a hand-rolled gate
  to switch the style from `data` to the store.
- **Cookie + layout-owned writable derived** (chosen): SSR-correct output rather than a
  client-side patch, no server-state leak, no `$effect`, and it matches the house
  writable-derived-from-load pattern.

## Consequences

- **One-time width reset on deploy.** Existing widths live in the retired
  `comhairle:sidebarWidth` localStorage key; the `sidebar:width` cookie is absent on first
  load, so anyone who had customized their width falls back to 288 once and re-drags. No
  migration code is carried for a single reset in an internal admin tool.
- **New scoped server load.** `(admin)/+layout.server.ts` is added; the existing universal
  `+layout.ts` continues to run and its `data` merges with the server load's.
- **Width now rides on admin requests** as a ~10-byte cookie (negligible), the same as the
  existing `sidebar:state` cookie.
- **The `initializing` flag and its transition-disable path go away** (no more localStorage
  hydrate snap); `resizing` still disables the transition during a live drag.
