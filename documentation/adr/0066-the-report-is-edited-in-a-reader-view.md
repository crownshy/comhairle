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
a pill per page and charts at reading size. Charts in the editor are kept out
of the editor's text styles so they look the same as on the public page. The conversation
title isn't editable here; clicking it says where to change it. The editor and the published report share
one column, up to 56rem wide, so three stat cards fit in a row and a line of text stays
readable.

**2. One toolbar across the top holds every tool, like a document editor.** Under the bar
that says what publishing shares sits a single horizontal toolbar: Add a chart; text,
heading and subheading; bold, italic, underline and strikethrough; lists and quote;
alignment; link; then page break, image, video, audio and table. A page break is a
horizontal rule and starts a new page (ADR-0069); the editor shows it as a labelled dashed
line. The toolbar acts on whichever editor the cursor is in. Charts and page breaks only go
in the body, so they are greyed out while the cursor is in the executive summary, and the
summary turns away one typed (`---`), pasted or dropped there. Media and tables work in both. Add a chart opens a dialog: pick the step, then pick the chart from live previews.
Selecting an image shows a small bar for its alignment (left, centre, right) and width (25%
to 100% of the column), ported from the Learn editor on `demo/young-scot`. Both are stored
as keys on the image node and laid out by the shared editor stylesheet, so the published
report places images the same way.
The page's section header uses its one-row compact form here, so the report keeps its
height.

Rejected: a block rail on the left with formatting in a bubble over the selection. Two
places to look for tools, and the bubble had to be kept from stretching the page's scroll
area.

**3. An unwritten report opens empty.** A summary or body counts as unwritten when it is
empty or still holds the API's placeholder sentence. The editor opens it empty, with a short
hint as placeholder text inside the empty editor: it can't be selected, goes the moment you
type and is never saved. Neither the summary nor a page has a frame around it, in the editor or the published
report; in the editor a rule under the summary marks where the opening screen ends. An empty
summary shows only to someone who can edit it, since readers don't see an empty summary at
all. The published report applies the same rule, so an untouched report never shows readers
the placeholder sentence. On the published report Read more is pinned to the bottom of the opening screen, like
the participant step bar.

Rejected: a starter draft with a section per step, its main charts embedded under italic
prompts. It made a new report look busy before anyone had written a word, and every prompt
left in had to be caught at publish time.

**4. Sharing is always on screen.** The top bar says what publishing shares, or what is
public now, and the Add a chart dialog says when a chart becomes public. Publishing is a
review: what anyone with the link can read and one line on what stays private. Publishing
saves what's on screen first. Charts carry no label of their own: one on every chart
repeated the top bar and crowded the report.

**5. The editor is its own small TipTap wrapper, not `RichTextEditor`.** It shares the
schema (`getBaseExtensions`) and the formatting commands (`formattingTools.ts`) with every
other rich text editor. `RichTextEditor` brings its own toolbar, a bordered scroll box and a
`value` that it writes back into the editor whenever it changes. Here one toolbar serves two
editors, the editor is the page, and the text on screen is the only copy, so none of that
fits.

**6. Translations move into a dialog.** The page edits the main language. Translate opens
the existing translation editor for the summary and the body.

**7. Wide screens list the pages on the left, like tabs in a document editor.** From `xl` up
a Pages panel sits left of the report. It names each page as the published report does
(ADR-0069), highlights the page scrolled to, jumps to a page when clicked, adds a page with
+ (a page break at the end) and deletes one from its menu. It takes over from the pills
above the report, which stay on narrower screens where the panel doesn't fit. An empty page,
such as one just added with +, is listed straight away as "Untitled page" in grey, though the
published report leaves it out until something is on it. The page's section header, share
bar and toolbar share the page's left edge, and the admin sidebar keeps no gap on its right
so the white report meets it.

## Consequences

- The publish switch, which wasn't wired to anything, is gone. Publish and unpublish now
  call `UpdateReport`.
- Impacts and facilitator notes stay on the page, below the report, marked as not part of
  the public report.
- On narrow screens the toolbar scrolls sideways rather than wrapping.
- The editor and the published report treat the API's placeholder text as unwritten by
  matching the sentence. If the API's wording changes, `$lib/reports/reportDocument.ts` has
  to follow.
- Issue #1269 also asks for reordering pages by dragging and for dragging charts in from
  the list. Neither is built yet. Authors see the public result through the editor's own
  styling and a Preview link, not side by side.
