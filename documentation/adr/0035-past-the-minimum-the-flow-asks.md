# ADR-0035: Past the follow-up minimum, the flow asks instead of assuming

**Status:** proposed
**Date:** 2026-09-06
**Branch:** participant chrome demo branch

## Context

A Thinking Space question is a root answer followed by `followUpCount` follow-ups. The count
is a floor, not a ceiling: someone with more to say should be able to keep going.

Until now that was expressed by never closing the picker. Every submitted follow-up fetched
another round of three, and once the minimum was met an outline "Next question" button
appeared under them. The screen a participant met after their last required follow-up was
therefore identical to the one they met after their first: same heading, same three cards,
same shape. The only thing that had changed was a quiet button in the bottom right, sitting
directly above the pager's Skip in the step's bottom bar.

Two problems. The way out was hidden, in the sense that nothing on the screen said a decision
had been reached: the picker reads as an instruction, and the instruction was still "pick
one". And the fetch happened before the decision, so a participant who was done waited
through a round of generated follow-ups they never wanted.

## Decision

**1. The fork is a screen of its own.** Once the follow-up minimum for a question is met,
the flow shows a card naming the two ways on: another follow-up on this question, or the
next question (the summary, on the last one; back to the root picker, in extension mode).
Both are cards of the same size, because past the minimum neither is the expected answer.

**2. The follow-up round is fetched after the choice, not before it.** `goDeeper()` is what
kicks off `loadPicker`, so nobody waits on questions they were about to walk away from.

**3. The picker past the minimum can only be reached by choosing it,** so its backwards move
is the fork rather than nothing. That is the innermost-first rule of
[ADR-0018](0018-one-pager-innermost-first-navigation.md) applied one level deeper.

Rejected: a modal over the loaded picker. It is a thing you must dismiss before you can read
what is behind it, and it keeps the wasted fetch.

Rejected: leaving the flow alone and making the outline button full width. It still puts the
decision at the bottom of a screen whose heading is telling you to do something else, and it
still competes with the pager.

## Consequences

- Resuming onto a question whose follow-ups are already done lands on the fork, not on a
  picker the participant never asked for.
- The picker's own copy past the minimum is now "pick another follow-up", with going back as
  the way to change your mind. It no longer has to double as an exit.
- Extension mode loses its "Done with this question" footer button; the fork carries that
  move instead, so both modes make the keep-going decision in the same place.
- One more screen between answering and the next question. That is the cost of asking, and
  it is the same shape as the handoff card between questions, which participants have
  already met.
