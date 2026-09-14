# ADR-0038: Back closes the document viewer

**Status:** proposed
**Date:** 2026-09-14
**Branch:** demo branch

## Context

Sources in a Learning assistant answer, and source-document badges in Learn content, open in
`PdfDocumentDialog`. On a phone the dialog fills the screen, so it looks like a page, and
people press back (or swipe back) to leave it. Opening the dialog added nothing to browser
history, so back took them off the step and away from the answer they were reading.

The way out in the header was an outline button labelled "Close".

## Decision

**1. Opening the viewer pushes a shallow history entry.** `pushState('', { documentViewerOpen:
true })` keeps the URL and runs no `load`. Back pops that entry and the dialog closes. Closing
from inside (the X, Escape, the overlay) calls `history.back()`, so the entry does not stay
behind to swallow the next back press.

SvelteKit treats a pop onto a shallow entry as a state change rather than a navigation, so
`beforeNavigate` guards (the embed param in the conversation layout, `unsavedChangesGuard`)
do not run.

Rejected: **a query param such as `?doc=`.** A param promises that the URL can be shared or
reloaded into the open viewer, and it cannot: the page and highlights come from the answer's
retrieved chunk, which only exists in memory. Setting it also goes through `goto`, a real
navigation that runs the guards and any `load` reading the URL.

Rejected: **handling it in `LearningAssistant` only.** Learn content badges open the same
dialog as the same full-screen sheet.

**2. The handling lives in the dialog, and only the instance that pushed reacts.** A Learn
page can mount two of these dialogs, the content renderer's and the assistant's. Both read
`page.state`, so each tracks whether it pushed the entry and ignores it otherwise.

**3. The header close is an icon.** A ghost `X` button at `size-10` with `aria-label="Close"`,
beside Download. It leaves the document name more room on a narrow screen.

## Consequences

- This is the first use of `pushState` and `App.PageState` in the repo. If back should also
  close the step menu sheet or the support drawer, the same pattern applies.
- After closing, browser forward lands on the viewer's entry with no viewer open, so forward
  then back looks like nothing happened.
- Reloading with the viewer open keeps its history entry, so the first back press after the
  reload lands on the same step.
