# ADR-0003: Replace the Pol.is admin iframe with native UI backed by a server-side config-write proxy

**Status:** accepted
**Date:** 2026-07-10

## Context

The Pol.is Step's admin **Setup** surface embeds Pol.is's own admin console in an **iframe** ([PolisManage.svelte](../../ui/packages/comhairle/src/lib/tools/polis/PolisManage.svelte)), auto-logging in via a `POLIS_LOGIN` `postMessage`. It's clunky, visually inconsistent with the comhairle admin, and pushes Pol.is authentication into the browser.

The equivalent management surfaces (statement **Moderation**, **Insights**) were already built natively in the civic_os admin (`bloom/civic_os/packages/admin`), so the functionality and analytics exist as a reference implementation. The remaining question was *how* to write Pol.is conversation-level config (topic, description, `strict_moderation`, `is_active`) without the iframe.

Key facts that made this feasible:
- The backend already logs into Pol.is as the admin **server-side** (`polis_service.rs::login`) and can `update_poll` (PUT `/api/v3/conversations`) and `post_seed_comment` with that cookie. These were internal-only.
- `WikiPollReport` (the `report_data` export) already derives `JsonSchema`; the client method was just untyped (`z.void()`) because the route never declared its response.

## Decision

**Rip out the iframe and drive Pol.is entirely through the comhairle backend.** No client-side Pol.is auth.

- **Pol.is-proxied config** (`is_active`, `topic`, `description`, `strict_moderation`) is written through a **new public backend route** that widens `UpdatePollRequest` and reuses the existing server-side `login()` + `update_poll()`. A generated `apiClient.PolisUpdateConfig` method fronts it.
- **Seed authoring** (Setup textarea + Moderation "add statement" / CSV import) posts through a **new backend seed route** wrapping `post_seed_comment` against the active poll (preview while draft, live after launch) — not the browser-side cross-origin POST that civic_os used.
- **Insights `report_data`** is typed by adding `.response::<200, Json<WikiPollReport>>()` to the route and regenerating the client — a typed api-client call, not a raw `fetch`.
- **comhairle display flags** (`required_votes`, `show_remaining_statements`, "label seeds as conversation starter") stay in `tool_config` via the existing `UpdateConversationWorkflowStep`.

## Considered options

- **Keep the iframe** (rejected): least work, but keeps the clunky UX and client-side Pol.is auth the rework exists to remove.
- **Store everything in `tool_config`** (rejected): Pol.is would never learn about topic/description/moderation changes — those must reach the Pol.is conversation to take effect.
- **Server-side config proxy** (chosen): consistent with the existing api-client, keeps Pol.is credentials on the server, and the backend already had the auth + `update_poll`/`post_seed_comment` plumbing.

## Consequences

- **New backend surface to review.** Widening `UpdatePollRequest`, two new routes, and one response annotation. Shipped as a dedicated endpoints PR ahead of the UI PRs.
- **The "show visualization" flag is deferred.** The custom participant embed renders no opinion-map, so the flag would control nothing. Building a participant PCA/opinion-map component (data via `get_math_pca`/`report_data`) is a follow-up, bundled with retiring the separate `PolisReport.svelte` iframe.
- **Pre-launch state is a seed-staging screen, not a moderation workspace.** Real participant voting is post-launch only. `launch` mints a fresh live poll and migrates only seed **text** — aux metadata (`moderation_status`, `themes`) does not carry over, and rejected seeds are not yet filtered (`// TODO: filter seed statements`). Honouring pre-launch moderation at launch is a separate follow-up.
- **civic_os is treated as the source of truth** for the Moderation/Insights glossary and analytics; its raw-`fetch` calls can later migrate onto the now-typed client.

## Amendment (Setup pre-fill: mirror config into `tool_config`)

Polis exposes no read path for a conversation's `topic` / `description` / `is_active` / `strict_moderation`, and `PolisToolConfig` (the stored tool config) drops unknown fields. So the native Setup form could not pre-fill current values from the write-only `PolisUpdateConfig` alone.

**Decision:** the Polis-proxied config is **mirrored into `tool_config`**. `PolisToolConfig` gains `topic`, `description`, `is_active`, `strict_moderation` (plus the comhairle-only display flag `label_seeds_as_conversation_starter`). Saving a config field writes to **Polis first** (`PolisUpdateConfig`, enforcement) and then to `tool_config` (the value the Setup form reads back). This matches how `required_votes` / `show_remaining_statements` already work, and needs no new read endpoint.

**Consequence:** `tool_config` is comhairle's copy of the config, Polis is the enforcement side — they can drift if a Polis-side write succeeds but the mirror write fails (or if Polis config is changed out-of-band). The Polis-first ordering keeps `tool_config` from ever claiming a value Polis rejected. Considered alternative: a live GET-config endpoint (single source of truth, extra round-trip and more Polis-fetch plumbing) — deferred; the mirror is enough for now.
