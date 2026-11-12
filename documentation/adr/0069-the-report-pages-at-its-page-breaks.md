# ADR-0069: The report pages at its page breaks

**Status:** proposed
**Date:** 2026-11-10
**Supersedes:** decision 1 of [ADR-0058](0058-the-public-report-pages-by-its-own-headings.md),
and its "2 of 3" in decision 3
**Builds on:** [ADR-0066](0066-the-report-is-edited-in-a-reader-view.md)

## Context

ADR-0058 cut the report body into sections and cards from the headings an admin writes: an
H2 started a section and a pill, an H3 started a card inside it, and a horizontal rule
started a card that kept the section's title. A section could hold several cards, and the
pager said "2 of 3".

Once the editor became the report (ADR-0066), that was three tools for one idea, and none of
them looked like what readers got. Making a line bigger moved it into a new tab, and a page
break started a card titled by a heading further up. There are no published reports yet, so
nothing depends on the old split.

## Decision

**1. Only a page break starts a page.** Each page is one card in the published report and
one pill in its header. Headings are text styles and nothing more.

**2. A page's first line of text names its pill.** It can be a heading or a plain
paragraph. A page with no text at all, such as one holding only a chart, is called
"Page n". The card shows the page as written, first line included, with no title added
above it.

**3. Empty pages are dropped,** in the published report and in the editor's pills alike, and
the "Page n" numbers count only the pages that are left.

The split is `splitReportIntoPages` in the public `report/reportPages.ts`, and `reportPages`
in the editor's `report/reportDocument.ts`, which also keeps where each page sits so a pill
can scroll to it or remove it. Both have unit tests for the same rules.

Rejected: H2 also starts a page, so headings and page breaks both cut the report. It kept two
ways to do one thing, which was the confusion this replaces.

## Consequences

- The pager has no "2 of 3" any more: every page is its own pill.
- The report pager now uses the participant step pager's buttons, row and bottom bar from
  `lib/components/pager`, as ADR-0058 planned once the step pager landed. It keeps links for
  `?card=n` and drops the progress line it had; the step pager keeps its Skip, loading and blocked rules.
- A page has no frame around it. It sits in the report column the way it sits in the editor.
- `?card=n` from ADR-0058 still addresses a page.
- The rail calls H2 and H3 Heading and Subheading, and Page break says it starts a new page.
- A long first line makes a long pill label, so pills are capped in width and truncated.
- The editor outline in #1776 was built on sections and cards and needs reworking to list
  pages.
