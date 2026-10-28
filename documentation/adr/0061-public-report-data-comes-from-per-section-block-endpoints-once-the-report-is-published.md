# ADR-0061: Public report data comes from per-section-block endpoints once the report is published

**Status:** proposed
**Date:** 2026-09-28
**Amends:** [ADR-0012](0012-report-component-embeds-store-reference-plus-frozen-html.md)
(live embeds now fetch from dedicated public endpoints, not the Insights endpoints)

## Context

ADR-0012 made report embeds live: the published report mounts the real component and it
fetches its own data in the reader's browser. For Polis that works because
`PolisGetReportData` has no auth. #995 asks for embeds from the other tools (HeyForm,
Prioritization, Thinking Space), and their Insights endpoints are either admin-only or, in
HeyForm's case, open when they shouldn't be (#1251).

Reusing the Insights endpoints for public readers would mean opening up endpoints built to
show admins everything. An audit of `PolisGetReportData` (#1253) found what that leads to: no
check that anyone meant the data to be public, per-participant pids and PCA positions that no
component reads, and representative comments that may carry unmoderated text.

## Decision

- **Public report data only comes from dedicated report-data endpoints.** The Insights
  endpoints stay admin-only and are never called by a public surface.
- **One endpoint per section block, to start.** Each embeddable section block gets its own
  endpoint that returns only the fields it draws. What goes public is then easy to review one
  section block at a time, and each response caches on its own. If the count gets out of
  hand, or several section blocks keep fetching the same thing, we revisit and merge them.
- **Anonymised and moderated only.** No user ids, pids or per-person rows. No rejected or
  pending content, whatever a tool's own moderation setting shows participants. A future
  moderation report is the one expected exception, and it gets its own ADR.
- **Publishing the report is the switch.** Every public endpoint goes through
  `tools::public_report_data::public_report_tool_config`, which returns 404 unless the step's
  conversation has a published report (`report.is_public`), and 404 when the step runs another
  tool. Both look the same as a missing step. Unpublishing the report turns the data off again.
- **Frontend registry.** `lib/reports/embeds.ts` lists each tool's section blocks (metadata
  only) and `lib/reports/ReportEmbedLive.svelte` maps a tool to the component that loads and
  renders them. A tool PR adds one entry to each.

## Consequences

- Each tool's embeds ship as their own PR with their endpoints: Polis #1253, HeyForm #1259,
  Prioritization #1260, Thinking Space #1261.
- Until #1253 lands, the existing Polis embeds keep reading `PolisGetReportData` and ignore
  the published state. The room display reads it too (#1254). Locking it down to admins
  (#1255) waits on both moving.
- Once the report is published, every step in the conversation serves its anonymised totals,
  including steps the report doesn't embed. If that turns out to matter, the guard can check
  the step is embedded in the published report instead.
- More endpoints to maintain, each small. Several section blocks from one step can repeat an
  upstream fetch; the per-step cache from #1243 absorbs that for Polis.

## Considered options

- **A per-step opt-in flag** (`workflow_step.report_data_public`), switched on from the step's
  settings or the embed dialog. It let a facilitator publish one step's results and not
  another's, but it added a second "is this public?" decision next to publishing the report,
  in a place facilitators don't think about reporting. Dropped before merge.
