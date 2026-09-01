# ADR-0027: The end of the flow counts what you did

**Status:** proposed
**Date:** 2026-09-01
**Branch:** participant chrome demo branch

## Context

Every screen of the participant flow now runs in one chrome: a header with the progress bar,
a body, and navigation in the corners (ADR-0024, ADR-0025). The thank-you page did not. It
dropped out of that chrome onto a bare page and stacked everything it had to say in one
column: a paragraph of thanks, a note about results, a heading, a row of buttons back into
the steps, a feedback button, another heading, and a preferences form. Nothing on it marked
the moment the participant had just finished, and the first thing under the fold was
administration.

## Decision

**1. It is the last screen of the flow, in the flow's chrome.** Same header, same progress
bar with its last segment full, same menu. Someone who finishes has not left the
conversation, so the page should not look like a different site.

The bar gets no segment of its own for the thank-you screen. The count every step quotes is
"Step N of M", and adding a segment would change M.

**2. It opens on one line and a few numbers that count themselves up.** The counting is the
animation: reduced motion, and a tab that is not being looked at, get the finished number
rather than a shortened count.

**3. Every number is measured.** The steps and the percentage come from this participant's
own progress rows. The minutes come from a clock the step pages stamp in `localStorage`,
because `user_progress` carries a status and no timestamps, and participation's `created_at`
spans every visit: a participant who came back a week later would be told they had spent a
week on it.

Rejected: adding up the tools' hardcoded `estimatedMinutes` and calling it time spent. It is
the figure the landing page quotes as an estimate before anyone starts, and two participants
who took very different amounts of time would be told the same number.

**4. The clock is a sitting, and it declines to answer rather than guess.** A gap of more
than half an hour starts it again, so a participant who returns the next day is timed from
when they returned. No stamps, a sitting longer than four hours, or a clock that moved
under us, and the figure is dropped: the other two numbers stand on their own.

**5. Nothing finished means no numbers.** Someone who arrives here without doing the flow is
not congratulated with a row of zeroes. An admin's preview is the exception: preview records
no progress at all, and what it is for is showing what a participant sees, so it shows full
marks.

**6. Everything else folds away.** Going back to a step, giving feedback and opting into
updates sit behind two disclosures rather than down the page. They are all things a
participant might want; none of them is what the screen is for.

## Consequences

- The time is per browser. Blocked storage, or finishing in a different browser from the one
  the steps were done in, drops the tile. Two tiles is a fine screen; a wrong number is not.
- The stamp is written by the step pages, so a flow that gains a participant-facing page
  outside `/s/` will need to stamp it too, or that time is not counted.
- The copy is English only for now, like the rest of this branch's chrome.
- A conversation with an admin-authored thank-you message still renders it, under the
  numbers rather than instead of them.
