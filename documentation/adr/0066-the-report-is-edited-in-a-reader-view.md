# ADR-0066: The report is edited in a reader view

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

**2. The rail adds blocks; a bubble formats text.** The rail on the left holds Add a chart,
text, section (H2), card (H3), page break, image, video, audio and table. A page break is the
horizontal rule that already starts a new card (ADR-0058); the editor shows it as a labelled
dashed line. Selecting text opens a bubble with bold, italic, underline, strikethrough, lists,
quote, alignment and link. The rail only acts on the body, so the executive summary gets the
bubble and no blocks. Add a chart opens a dialog: pick the step, then pick the chart from live
previews.

Rejected: every tool in the rail (too long to scan) and the old toolbar across the top (it
breaks the editor looking like the page). Splitting blocks from formatting keeps everything
the Learn editor's toolbar can do, apart from the Learn-only source document and glossary
tools.

**3. An unwritten report opens empty.** A summary or body counts as unwritten when it is
empty or still holds the API's placeholder sentence. The editor opens it empty with a short
hint, which is never saved.

Rejected: a starter draft with a section per step, its main charts embedded under italic
prompts. It made a new report look busy before anyone had written a word, and every prompt
left in had to be caught at publish time.

**4. Sharing is always on screen.** The top bar says what publishing shares, or what is
public now, and the Add a chart dialog says when a chart becomes public. Publishing is a
review: what anyone with the link can read and one line on what stays private. Publishing
saves what's on screen first. Charts carry no label of their own: one on every chart
repeated the top bar and crowded the report.

**5. An outline on wide screens.** At `xl` and up an outline sits right of the report: each
section, with its cards under it, and clicking one scrolls the editor there. The editor
scrolls as one document; only readers page through cards.

**6. Translations move into a dialog.** The page edits the main language. Translate opens
the existing translation editor for the summary and the body.

## Consequences

- The publish switch, which wasn't wired to anything, is gone. Publish and unpublish now
  call `UpdateReport`.
- Impacts and facilitator notes stay on the page, below the report, marked as not part of
  the public report.
- The rail is hidden on small screens, so blocks can't be added there. The bubble and
  keyboard shortcuts still work.
- The editor treats the API's placeholder text as unwritten by matching the sentence. If the
  API's wording changes, `reportDocument.ts` has to follow.
