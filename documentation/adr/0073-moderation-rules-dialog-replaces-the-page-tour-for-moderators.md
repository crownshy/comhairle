# ADR-0073: The moderation rules dialog replaces the page tour for moderators

**Status:** proposed
**Date:** 2026-11-25

## Context

Every admin page tour plays by itself the first time a browser opens the page. Issue #1934
adds a Moderation rules dialog that opens on a moderator's first visit to a Polis step's
Moderate page. If both played, a moderator's first visit would start with a dialog and then a
tour on top of it. After the dialog closes, a pointer shows where the button that reopens it
is. The pointer is not the page's own tour, so the replay button must not pick it up.

## Decision

**1. A page can turn off its tour's autoplay.** `useTour(getTour, () => ({ autoplay: false }))` still
registers the tour for the replay button but doesn't play it. The Moderate step page turns
autoplay off only for people who can moderate, since only they get the dialog. Anyone else
still gets the tour on their first visit.

**2. `tourStore.play(tour)` runs a one-off tour.** It plays without registering the tour or
marking it seen, so the replay button keeps playing the page's own tour. The rules pointer
uses it, with a `doneLabel` so it shows a single button.

**3. The dialog is marked seen when it opens, not when it closes.** A moderator who leaves
through a link inside the dialog has still seen it. The pointer is tied to that first opening
in memory, so blocked storage can't make it play after every close.

## Consequences

- A page with its own first-visit welcome has to decide who gets the tour instead. Other
  pages are unchanged.
- The seen flag is per browser, not per user or step. A second moderator on the same browser
  never sees the dialog open by itself, but can open it from the button.
