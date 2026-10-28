# ADR-0061: Public report data is served only for section blocks the published report embeds

**Status:** proposed
**Date:** 2026-10-28
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

The public endpoints answer anyone who calls them, not only the report page. So we need a
rule for when a step's data is public that facilitators can see and predict. Facilitators
running the same kind of step don't always want the same results shared: one may publish a
Thinking Space summary, another may have told participants it stays with the organisers.

## Decision

- **Embedding a section block in the published report is what makes it public.** Each public
  endpoint serves a step's section block only when the conversation's report is published
  (`report.is_public`) and its summary or body, in any language, embeds that section block for
  that step. Everything else returns 404, the same as a missing step. Removing the embed or
  unpublishing the report turns the data off again. There is no separate toggle.
- **The report text is the record.** The gate reads the embeds out of the stored report
  documents on each request (`tools::public_report_data::embeds`), so it can't drift from what
  readers see. The per-step caches absorb the extra query.
- **Facilitators are told where they act.** Every embed in the report editor says "Public when
  published", the embed dialog says embedded results become public, and the step's Configure
  page lists which of its section blocks are public in the report, with a link to edit them.
- **Public report data only comes from dedicated report-data endpoints.** The Insights
  endpoints stay admin-only and are never called by a public surface.
- **One endpoint per section block, to start.** Each embeddable section block gets its own
  endpoint that returns only the fields it draws. What goes public is then easy to review one
  section block at a time, and each response caches on its own. If the count gets out of
  hand, or several section blocks keep fetching the same thing, we revisit and merge them.
- **Anonymised and moderated by construction.** Every public endpoint goes through
  `serve_public_report_data`, which only returns types declared with the `public_report_data!`
  macro. A field that isn't one of the approved building blocks (`Count`, `Share`,
  `ModeratedText`, booleans, fixed labels, and lists or options of them) fails to compile, so
  ids, pids, emails and raw strings can't reach a public response. Polis added three:
  `StatementId` and `OpinionGroupId` (Polis's own numbering for statements and groups, which
  says nothing about who wrote or joined them) and `ThemeLabel` (themes facilitators tag
  statements with). Participant text can only
  go out as `ModeratedText`, which is built from accepted content only. Adding a building block
  needs a reviewer to agree it can't identify a person. A future moderation report is the one
  expected exception, and it gets its own ADR.
- **Facilitators preview the same data.** Each tool's public endpoints have preview twins that
  skip the published and embedded checks but need permission to update the conversation.
  They run the same builders and return the same types, so the report editor, the add-chart
  dialog and the report preview show what readers will get before the report is published.
- **Frontend registry.** `lib/reports/embeds.ts` lists each tool's section blocks (metadata
  only) and `lib/reports/ReportEmbedLive.svelte` maps a tool to the component that loads and
  renders them. A tool PR adds one entry to each.

## Consequences

- Each tool's embeds ship as their own PR with their endpoints: Polis #1253, HeyForm #1259,
  Prioritization #1260, Thinking Space #1261. Each declares its responses with
  `public_report_data!` and calls `serve_public_report_data` with its section block type.
- The Polis embeds read `/tools/polis/public_report/` and `/tools/polis/report_preview/`
  (#1253), accepted statements only, whatever `strict_moderation` says. Divisiveness goes out
  as a share of the most divisive statement's, since the continuum only needs it relative. The
  room display still reads `PolisGetReportData` (#1254). Locking that down to admins (#1255)
  waits on the display moving.
- Types stop identifiers and unmoderated text, but they can't stop small numbers from pointing
  at a person, such as an opinion group of one. Whether to round or hide small counts is open
  and belongs with the first tool that shows group sizes.
- A published report's embed shows "unavailable" only when its step or data is gone, not
  because a separate switch is off.
- More endpoints to maintain, each small. Several section blocks from one step can repeat an
  upstream fetch; the per-step cache from #1243 absorbs that for Polis.

## Considered options

- **A per-step opt-in flag** (`workflow_step.report_data_public`), switched on from the step's
  settings or the embed dialog. It let a facilitator publish one step's results and not
  another's, but it added a second "is this public?" decision next to the report, in a place
  facilitators don't think about reporting. An embed with the flag off showed "unavailable" on
  the published report with no hint why. Dropped before merge.
- **Publishing the report makes every step public.** One switch and nothing to explain, but a
  report that only shows a Polis chart would still serve the Thinking Space totals to anyone
  who called the endpoint. Dropped because what goes public should match what the report
  shows.
- **Record the embeds in a table when the report is saved.** Cheaper lookups, but the report
  text saves through the generic translations endpoint, so the hook would sit in shared
  translation code and could drift if a save path skipped it.
