# ADR-0009: Opinion group names + summaries are on-demand generated, never faked

**Status:** Proposal - to be accepted
**Date:** 2026-07-28

## Context

The Polis [Insights](../../CONTEXT.md) report groups participants into opinion groups and, per the
new design, shows each group an AI-generated **name** ("Progressive Digital Advocates") and a short
**summary** of what the group believes, behind an "AI Generated" badge.

Polis provides neither. It gives us the groups and their representative statements, but no names or
prose. So the name/summary has to come from somewhere we build, and the question is what to render
until it exists.

While building the OpinionGroups section we shipped hardcoded placeholder copy (a "Progressive
Digital Advocates" blurb about social media and youth) cycled per group. On a real conversation —
which happened to be about local housing — that placeholder read as a confident, authoritative
insight that was simply *wrong for the topic*. A topically-plausible fake is worse than no data: a
reader can't tell it apart from real analysis.

## Proposal

1. **Never render a fabricated name/summary.** The `OpinionGroupCard` takes an optional `aiSummary`
   prop and hides the entire name/badge/summary block when it is absent. The live report leaves it
   undefined today, so groups show only real data (size, membership share, representative
   statements). Storybook passes `aiSummary` to demo the generated version; production does not.

2. **When we do generate it, generate on demand behind an explicit action**, not on a timer. An
   admin triggers a "Generate group summaries" action that feeds each group's statements to an
   agent (likely RAGFlow, the same stack we use elsewhere) which returns `{ name, summary }` per
   group. The result is stored, not recomputed on every view.

## Rationale

- **On-demand, not auto-refresh.** Group names/summaries get frozen into the report snapshot
  ([ADR-0008](0008-report-pieces-embed-in-tiptap-as-frozen-snapshots.md)); an admin should review
  AI-written prose before it ships under their name. Auto-regeneration would also churn text and
  spend tokens on every refresh for data that barely moves once the clustering settles. A button
  lets the admin re-run deliberately when the groups actually shift.

- **Agent, not another source.** There is no other source. Names/summaries are a reading of each
  group's statements, which is exactly what an LLM step over the representative comments produces.

- **Hidden-until-real, not placeholder.** A missing block is honest; a topical-but-wrong block is
  not. The optional prop keeps the component whole (and demoable) without ever shipping fake copy.

## Consequences

- The live report has no group name/summary until the generation path is built. Accepted: the
  section is useful without it (sizes + representative statements), and it beats shipping wrong data.
- The `aiSummary` seam is the integration point — wiring the agent means populating that prop from
  stored results, no component changes.
- **Revisitable:** if we later want summaries kept continuously fresh, or sourced differently, that
  changes the trigger and this ADR, not the render contract.
