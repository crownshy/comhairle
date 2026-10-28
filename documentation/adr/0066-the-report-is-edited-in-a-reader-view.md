# ADR-0066: The report is edited in a reader view, from a starter draft

**Status:** proposed
**Date:** 2026-10-28
**Builds on:** [ADR-0058](0058-the-public-report-pages-by-its-own-headings.md) and
[ADR-0061](0061-public-report-data-is-served-only-for-section-blocks-the-published-report-embeds.md)

## Context

The report page was a settings form: two small cards, each with a cramped rich-text box for
the summary and the body, a publish switch in the corner, and a two-stage dialog for adding
charts. Facilitators couldn't see what readers would get, and a new report opened on the
API's placeholder sentences.

Since ADR-0061, embedding a chart is what makes that step's results public, so whatever
replaces the form has to keep that obvious.

A throwaway prototype on `prototype/report-editor` tried four directions. This one won.

## Decision

**1. The editor is the report.** The page uses the full content width, like the design
board. It shows the report with the published page's own type, executive summary box,
section pills (the body's H2s) and charts at reading size. Charts in the editor are kept out
of the editor's text styles so they look the same as on the public page. The conversation
title isn't editable here; clicking it says where to change it.

**2. A short rail holds the tools.** Add a chart, text, section (H2), card (H3), bold,
italic, lists, quote and link. Alignment, tables and media are left out to keep it short.
Add a chart opens a dialog: pick the step, then pick the chart from live previews.

**3. An unwritten report opens on a starter draft.** An introduction prompt, one section
per step that has charts with its main charts embedded under a prompt, and a closing
section. For Polis that is Key stats and Areas of consensus. The draft is only shown. The
first edit saves it, so opening the page never writes anything. A summary or body counts as
unwritten when it is empty or still holds the API's placeholder sentence.

**4. Sharing is always on screen.** The top bar says what publishing shares, or what is
public now, and every embedded chart says "Public when published" or "Public now".
Publishing is a review: what anyone with the link can read, one line on what stays private,
and a warning if starter prompts are still in the text. Publishing saves what's on screen
first, including an untouched draft.

**5. Translations move into a dialog.** The page edits the main language. Translate opens
the existing translation editor for the summary and the body.

## Consequences

- The publish switch, which wasn't wired to anything, is gone. Publish and unpublish now
  call `UpdateReport`.
- Impacts and facilitator notes stay on the page, below the report, marked as not part of
  the public report.
- The rail is hidden on small screens. Keyboard shortcuts still work there.
- The editor treats the API's placeholder text as unwritten by matching the sentence. If the
  API's wording changes, `reportDocument.ts` has to follow.
