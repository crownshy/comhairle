# ADR-0012: Report component embeds are live, referenced by Step

**Status:** accepted (supersedes the frozen-snapshot direction of [ADR-0008](0008-report-pieces-embed-in-tiptap-as-frozen-snapshots.md) for these embeds)
**Date:** 2026-08-10

## Context

The End-of-engagement [Report view](../../CONTEXT.md) is a human-authored TipTap document
(the `Report.summary` field). A facilitator pulls real report components (Polis areas of
consensus, opinion groups, the consensus continuum, etc.) into that document from a button
in the editor and places them where they want in the narrative.

ADR-0008 framed the storage choice as frozen-vs-live and chose **frozen HTML snapshots**.
We started there. In practice the report components are *interactive* (expandable statement
lists, a hover-driven beeswarm, group selectors), and freezing them to static HTML turned
every control into a dead element that looked broken. Reviving those controls on a frozen
blob would have meant re-implementing each one as ambient JS on baked markup. At that point
the honest, simpler thing is to render the real component. So we reversed: **these embeds
are live.** ADR-0008's frozen model was considered and is recorded here as the rejected
alternative.

Two facts made live viable without backend work:

- `PolisGetReportData` (the core report export) has **no auth** — a public/anonymous report
  page can call it.
- `PolisListStatementAux` requires an authenticated user, so it is best-effort: present for
  logged-in viewers (themes, moderation counts), gracefully empty for anonymous ones. The
  components render from `reportData` alone.

## Decision

**1. One authored surface.** The `summary` TipTap document is the single composition
surface. `section_configs` is the picker's registry (which Steps are report-capable), not a
second render path.

**2. The embeddable unit is a section block** from an explicit allow-list (Polis: *Key
stats*, *Areas of consensus*, *Areas of disagreement*, *Consensus continuum*, *Opinion
groups*). Not the sub-primitives they compose from; not the whole `PolisInsights` page.

**3. The node stores only a reference** — `{ toolStepId, componentType, config }`. There is
no baked HTML. Every render surface mounts the real Svelte component against current data:

- **Editor**: the node view mounts the live component (full interactivity while composing).
- **Published web page**: the report body is rendered by walking the document and
  interleaving the live component at each embed node, between the normally-rendered rich
  text.

**4. Data loading is per-embed and client-side.** A shared `ReportEmbedLive` component loads
`reportData` (required) and `statementAux` (best-effort via `tryCatchAsync`) by
`toolStepId`, and renders the section. It owns its loading / empty / error states.

**5. Deleted-Step / no-data fallback.** If the Step or its data no longer resolves, the
embed shows an inline "this component's data is no longer available" state instead of
breaking the page. The reference is all that is stored, so there is nothing stale to show.

## Alternatives considered

- **Frozen HTML snapshots** (ADR-0008; our first cut). At freeze time the component renders
  to HTML baked into the node; every surface shows that static HTML. Portable to no-JS paths
  (email, print, PDF) and needs no live data access. **Rejected** because the components are
  interactive and a frozen snapshot makes every control dead; a report that a viewer expects
  to explore (expand statements, hover the beeswarm) is the product, and freezing removes it.
  Reconsider if/when a no-JS export (emailed or PDF report) becomes a hard requirement — that
  path will need a frozen or server-rendered fallback.
- **Whole-tool embed** (drop the entire `PolisInsights` page). All-or-nothing, no curation.
- **Lean into `section_configs`** as the real structure. More rigid; fights "embed anywhere".

## Consequences

- **No-JS surfaces don't render the embeds.** Email / print / a static PDF export will show
  the surrounding prose but not live components. If we want "download the report" later, it
  needs a server-side render or a frozen fallback keyed off the same reference — revisit then.
- **Embeds paint after hydration.** The report body SSRs its text; each live component loads
  its data client-side and pops in. Acceptable for a report; could be pre-loaded in the page
  loader later.
- **Anonymous viewers get reduced fidelity** on anything sourced from `statementAux` (themes,
  approved/pending counts) until/unless that endpoint is opened up.
- **`PolisGetReportData` is unauthenticated.** This is what makes the public live report work;
  noting it here because it is load-bearing and a future auth change to that endpoint would
  break public reports.

## Open questions (deferred, for team discussion)

Tracked on the issue tracker (spike #839), not decided here: multi-page reports + page
breaks · what participant data the public report shows · full presentation rebuild (retire
the tabs + hardcoded iframe) · no-JS export / "download the report" (needs the frozen or
server-rendered fallback above) · rich-text-ifying the Impacts / feedback sections ·
translatable embeds.
