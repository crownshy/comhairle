# ADR-0012: Report component embeds store a reference plus frozen HTML

**Status:** accepted (MVP scope; several parts explicitly deferred, see Open questions)
**Date:** 2026-08-10
**Builds on:** [ADR-0008](0008-report-pieces-embed-in-tiptap-as-frozen-snapshots.md)

## Context

The End-of-engagement [Report view](../../CONTEXT.md) is a human-authored TipTap document
(the `Report.summary` field). We want a facilitator to pull real report components (Polis
areas of consensus, opinion groups, the consensus continuum, etc.) into that document from
a button in the editor, place them wherever they want in the narrative, and have them show
up on the published report.

ADR-0008 already decided the storage question at a high level: report pieces embed as
**frozen HTML snapshots**, not live bindings. This ADR pins down the concrete shape of the
embed node, the authoring flow, and what is deliberately out of scope for the first cut.

Two things were true when we started and shaped the decisions below:

- The published report page was rendering `summary` through `marked.parse` (treating a
  TipTap ProseMirror JSON document as markdown), so it printed raw JSON. Rich text did not
  render at all. Fixing that is a prerequisite, not a nice-to-have.
- The report already had two half-built models of "report content": the `summary` TipTap
  field, and a dormant `section_configs` list (seeded per Step at report generation, tagged
  with each Step's tool + `ai_generated`/`verified` flags, consumed by nothing).

## Decision

**1. One authored surface.** The `summary` TipTap document is the single composition
surface for the report. Components are embedded inline as nodes; there is no competing
structured renderer. `section_configs` is repurposed as the *picker's registry* (which
Steps in this conversation have a report-capable tool), not a second render path.

**2. The embeddable unit is a section block.** The dialog offers self-contained section
components (Polis: *Key stats*, *Areas of consensus*, *Areas of disagreement*, *Consensus
continuum*, *Opinion groups*) from an explicit allow-list. Sub-primitives that only make
sense inside a section (`VoteBar`, `OpinionGroupCard`, `StatementVoteBlock`) are not
offered, and the whole-page `PolisInsights` composition is not offered either (embedding
the entire thing defeats curation).

Each component gets a `frozen` mode used for the snapshot: interactive controls (filter
chips, "See all" toggles, group selectors, hover-driven detail) are dropped and everything
is shown. The beeswarm (`ConsensusContinuum`) additionally runs its d3-force layout to
completion *synchronously* at a fixed viewBox width in `frozen` mode, because its normal
animated, width-measured layout cannot be captured by an innerHTML read.

**3. The node stores a reference *and* the frozen HTML.**

- `reference`: `{ toolStepId, componentType, config }` — the recipe the snapshot was built
  from.
- `frozenHtml`: the rendered-to-HTML snapshot of the component.

The frozen HTML is what renders, everywhere (editor preview, published web view, and any
future print/download path). It slots straight into the published page's existing
string-based render pipeline (`renderRichTextToHtml` + `{@html}`), so "render correctly"
is the smallest possible change. The reference is cheap insurance: it is what a future
"refresh snapshot" action re-freezes from, what interim-vs-final re-freezes need, and the
exact hook a future live-component upgrade would attach to.

**4. Deleted-Step fallback.** Because the HTML is baked in, deleting the source Step does
**not** blank the report — the snapshot keeps rendering. A dangling reference only disables
*refresh*: the editor shows the last snapshot with a "source Step was deleted, can't
refresh" badge and a remove option.

**5. Insert with defaults.** No per-component config stage (filters, top-N) in the first
cut; components insert with sensible defaults.

## Alternatives considered

- **Live components** (store only the reference, mount the real Svelte component against
  current data on every view). More truthful, interactive (beeswarm hover, expandable
  tables), always fresh. Rejected for now: it needs a node-view bridge and a rework of the
  published render path, it breaks print/email/download (no JS, no DOM), and a report is a
  snapshot by definition once the conversation is closed. This is a real open question with
  team-level implications (a whole "live report" concept) — see Open questions. Storing the
  reference now keeps the door open to add it without a migration.
- **Whole-tool embed** (drop the entire `PolisInsights` page as one block). Faster, but
  all-or-nothing — no curation, which is the point of the feature.
- **Lean into `section_configs`** as the real structure (author toggles/verifies
  pre-seeded per-Step sections; prose is just glue). More rigid, and it fights the "embed
  anywhere in the flow" model.
- **HTML-only node** (drop the reference). Marginally simpler now, but a frozen block with
  no recipe is a dead end: no refresh, no re-freeze, no upgrade path.

## Consequences

- Embedded components are **static** in the published report: the beeswarm and expandable
  tables become pictures, not widgets. Acceptable for a snapshot; revisit with live
  components if needed.
- We can freeze **different snapshots at different stages** (interim reports), which is a
  genuine upside of the frozen model, not just a limitation.
- A frozen HTML blob is baked from our own components over our own data (not author markup),
  but it is still emitted via `{@html}`; the render path must keep treating only our schema
  as trusted and must not become a general raw-HTML sink.

## Open questions (deferred, for team discussion)

Tracked on the issue tracker, not decided here:

- Single-page vs **multi-page** report and page breaks.
- What **data** the public report shows (real participant stats, demographics, methodology)
  instead of today's hardcoded values.
- Full **presentation rebuild**: retire the per-Step tabs and the hardcoded Polis iframe.
- **Live components** / the "live report" concept (the alternative above).
- Making the other report sections (Impacts, Facilitator/Participant feedback) rich text.
- **Translatable** embeds.
- **Preview + download** (PDF) of the whole report.
