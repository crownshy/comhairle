# ADR-0046: Public report data comes from per-widget endpoints behind a step opt-in

**Status:** Proposal - to be discussed with the team
**Date:** 2026-09-28
**Amends:** ADR-0012 (live embeds now fetch from dedicated public endpoints, not the Insights
endpoints)

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
- **One endpoint per widget, to start.** Each embeddable widget gets its own endpoint that
  returns only the fields it draws. What goes public is then easy to review one widget at a
  time, and each response caches on its own. If the count gets out of hand, or several widgets
  keep fetching the same thing, we revisit and merge them.
- **Anonymised and moderated only.** No user ids, pids or per-person rows. No rejected or
  pending content, whatever a tool's own moderation setting shows participants. A future
  moderation report is the one expected exception, and it gets its own ADR.
- **A step opts in explicitly.** `workflow_step.report_data_public` defaults to off. Every
  public endpoint goes through `tools::public_report_data::public_report_tool_config`, which
  returns 404 when the flag is off, the same as for a missing step. Admins turn it on from the
  step's settings or from the embed dialog. Embedding a widget never turns it on by itself.
- **Frontend registry.** `lib/reports/embeds.ts` lists each tool's widgets (metadata only) and
  `lib/reports/ReportEmbedLive.svelte` maps a tool to the component that loads and renders
  them. A tool PR adds one entry to each.

## Consequences

- Each tool's embeds ship as their own PR with their endpoints: Polis #1253, HeyForm #1259,
  Prioritization #1260, Thinking Space #1261.
- Until #1253 lands, the existing Polis embeds keep reading `PolisGetReportData` and ignore
  the flag. The room display reads it too (#1254). Locking it down to admins (#1255) waits on
  both moving.
- Public readers see a placeholder for a widget whose step hasn't opted in. An admin can turn
  the flag off after publishing, which blanks those widgets on the live report.
- More endpoints to maintain, each small. Several widgets from one step can repeat an
  upstream fetch; the per-step cache from #1243 absorbs that for Polis.
