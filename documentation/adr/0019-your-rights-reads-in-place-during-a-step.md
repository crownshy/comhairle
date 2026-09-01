# ADR-0019: Your Rights reads in place during a step, it does not navigate

**Status:** proposed
**Date:** 2026-09-01
**Branch:** `mobile-exploration/participant-step-chrome`

## Context

ADR-0018 removed the site Footer from workflow routes, and its legal links moved into the
step dropdown. See [CONTEXT.md](../../CONTEXT.md) for **Participant step chrome**.

They moved as plain `<a href="/rights/...">`. That is a full navigation out of the
workflow, and it has two failures.

**The participant lands in different chrome with no way back.** `/rights/*` is a global
site route under `(public)`, so it renders the site NavBar and the Your Rights section nav.
There is no step dropdown, no pager, no progress bar, and nothing anywhere on the page
pointing back at the step the participant was halfway through.

**The step's state does not survive the round trip.** Every tool except Learn is an
embedded surface with state the participant built up and we do not persist per keystroke: a
Polis opinion sequence at "Opinion 9 of 16", a partly filled HeyForm. Navigating away
unmounts the iframe. Coming back remounts it cold.

The obvious repair is to give `/rights/*` the step chrome, but `StepChrome` needs step
data (the step list, the current index, the progress fill, the tool's count) that a global
route does not have and cannot get without either duplicating the rights routes under the
workflow path or threading step context through query params. Both are real work, and
neither fixes the second failure: the iframe still reloads.

## Decision

**1. From inside a step, Your Rights opens in a sheet over the step. No navigation.** The
dropdown's legal entries call back into `StepChrome` instead of linking out. The step stays
mounted, tool state is untouched, and closing the sheet is the way back. The chrome problem
dissolves rather than being solved, because the participant never leaves the chrome.

Rejected: rights routes nested under the workflow step path, which is what "give it the
same chrome" literally asks for. It duplicates the route tree and still discards iframe
state on the way back.

Rejected: `target="_blank"`. It preserves the step, but on mobile a new tab is its own kind
of lost, and it makes the legal text feel like an offsite link rather than part of the
service.

**2. The document content is the same component in both places.** The bodies of the ToS and
Cookies pages move to `$lib/components/rights/`, and the routes become title plus
component. Privacy already worked this way via `ComhairlePrivacyPolicy`. There is one copy
of each document, so the sheet and the standalone page cannot drift.

**3. The sheet carries the same section nav as the standalone page, and swallows in-body
cross-links.** ToS links to the Privacy Policy in its own text. Inside the sheet that link
switches the sheet rather than navigating, via an optional `onPrivacyClick` on the content
component. Unset, the anchor is an ordinary link, which is what the standalone route wants.

**4. Accessibility is not in the step dropdown.** The three entries in the dropdown match
what the Footer offered, which is what the dropdown replaced. Accessibility remains
reachable at `/rights/accessibility` and from the sheet's own nav only if it is added to
`legalDocs()` later.

## Consequences

- The legal documents are now rendered in two different widths: a full page and a sheet
  panel. They are prose in a flex column, so they reflow, but a future document with a wide
  table (Cookies already has one) needs to survive the narrower measure. Cookies' table
  keeps its `overflow-x-auto` wrapper for exactly this.
- `StepDropdown` no longer takes a `legalLinks` list. The documents come from
  `legalDocs()`, so the step page no longer constructs them and the dropdown and sheet
  cannot disagree about what is on offer.
- The rights routes still exist and are still linked from the Footer everywhere outside a
  workflow. This decision changes how a participant mid-step reaches them, not where they
  live.
- A participant inside a step cannot deep-link or share a legal document, because the sheet
  has no URL. The standalone route is the shareable form and is one hop away outside the
  workflow.
