# ADR-0058: The public report pages through cards cut from its own headings

**Status:** proposed
**Date:** 2026-10-21
**Builds on:** [ADR-0047](0047-one-pager-innermost-first-navigation.md),
[ADR-0048](0048-the-middle-is-the-move-the-corners-are-navigation.md)

## Context

The new final report design is a phone-first sequence of screens. It opens on the
conversation title, the executive summary and a Read more button. Then come section pills
and one card at a time with a pager underneath, and it ends on a thank-you screen.

The report body is one TipTap document that an admin writes in the report editor. It has no
fields for sections or cards. The design needs both: a pill per section, and sometimes
several cards in one section ("Overview 1 of 2", or "What emerged" followed by "Common
ground" and "Where opinions differ").

## Decision

**1. The headings an admin already writes decide the structure.**

- An H2 starts a section. Its text is the pill label and the first card's title.
- An H3 starts a new card in the current section, titled by the H3.
- A horizontal rule starts a new card that keeps the section title.
- Anything before the first H2 goes in a section called Introduction.
- Cards with no text and no embedded component are dropped, and so are sections left
  empty.

The split is `splitReportIntoSections` in `report/reportPages.ts`, with unit tests next to
it.

Rejected: a structured report model with section and card rows. It means a migration, a
new editor and a new API, all for a design we have only just seen. Headings cost nothing,
read correctly in the editor, and older reports already use H2s for their sections.

**2. The position lives in the URL as `?card=n`.** No `card` is the opening screen,
1 to n are the cards in reading order, and n + 1 is the thank-you screen. The pager and
pills are plain links, so browser Back steps back a card and a link can point at one card.
A number out of range falls back to the opening screen.

**3. The pager follows ADR-0047 and ADR-0048.** Next moves through the cards of a section
before moving to the next section. Back sits in the left corner and a labelled Next in the
right. The middle shows "2 of 3" when a section has more than one card, as text and not as
a control.

**4. Long cards are cut with Show more rather than scrolling inside the card.** A card is
cut at a fixed height from the first render, so content does not jump once the page
measures it. The Show more button appears only when something is hidden.

## Consequences

- Admins shape the report by choosing H2, H3 or a rule in the editor. The editor does not
  say that yet, so the report page in the admin needs a line of help text.
- The scroll navigation from #1225 (sticky pills that follow the scroll position, and Back
  to top) is gone, along with its IntersectionObserver.
- The Polis iframe toggle from #1225 is gone. The plan was to retire it, and it had no place
  in the paged design.
- The pager here and the step pager in #1322 look alike but stay separate, because the step
  pager carries step rules such as Skip and a blocked reason. Once #1322 lands, the shared
  parts can move to `lib/components`.
- The organiser block and the Sign up and Email us buttons in the design are left out.
  The organisation endpoint needs a signed-in user and has no logo, and the two buttons
  have nowhere to point yet.
