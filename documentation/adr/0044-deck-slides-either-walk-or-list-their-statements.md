# ADR-0044: Deck slides with several statements either walk them or list them

**Status:** proposed
**Date:** 2026-09-25
**Branch:** demo branch (Room display, deck layout)

## Context

The Room display's deck layout showed one statement on "What we agree on" and one on
"Where we split". Both were the single best-ranked statement, re-resolved live, so as
votes arrived the split slide swapped statements under the facilitator with no warning
and no way to stop it. A facilitator mid-sentence about one split would watch it vanish.

One statement per slide is also too little for a session. The room reaches agreement on
several things, and a slide that names one of them undersells the result.

Two presentations were built and both were wanted. A list of five reads as a result and
is what a room takes a photo of. Showing one at a time is what lets the map be coloured
by the statement, and is the one that walks a room through an argument. Neither is
right for both slides in every room.

The deck's notes record that it "loses ambient operation entirely: nothing moves unless
someone advances it". That was true and was a cost, not a goal. The room display's rule
on motion (CONTEXT.md, "accent") is that things move on arrival and sit still between
events, not that nothing ever changes.

## Decision

Both slides draw from the top five statements for their intent, through a ranked
variant of the ambient resolver (`rankIntent` in `ambientFocus.ts`) so the deck and the
ambient rotation cannot disagree about what "most divisive" means. Statements with a
zero score are left out rather than padding the five.

**Both slides share one style, `walk` or `list`.** It is a board field (`slides` in
`blocks.ts`, `?slides=walk` or `?slides=list` in the URL) so a link carries it and a
projector remembers it, like every other board choice. It was first built as a choice
per slide, and that was two switches for one decision: a facilitator is either walking
the room through the statements or showing them the result, and does not do one on
agreement and the other on disagreement. The panel offers it once, as "Statement slides
show", and only on the deck, since only the deck has the slides. Every template walks.

**A list** puts up to five rows on screen, strongest first, each with the same overall
and per-group bars the single statement had. Every row is the same size; the order says
which is strongest. The split slide's list drops the map, which can only be coloured by
one statement, because five rows of per-group bars already show where each one splits.

**A walk** shows one statement at a time, moving through the five on the ambient dwell
(twelve seconds), swapping on arrival and still in between. Both slides walk the same
way: the statement and its bars on the left, the map coloured by it on the right. The
bars say how the room split on it and the map says who, and one without the other was
half the picture. The facilitator can:

- **hold** the statement on screen with the space bar or the pill under the statement,
  and release it the same way;
- **step** to the next or previous one with the up and down arrows, which also holds.

The hold is by statement id, not by position in the list. The ranking keeps moving with
the votes while a statement is held, and a held statement stays on screen even if it
drops out of the top five. Releasing resumes the walk from wherever the held statement
now sits, or from the start if it has fallen out. Each walking slide keeps its own place
and its own hold, so holding an agreement does not freeze the split. Switching a slide
to the list drops its hold: a hold is "stay on this one while I talk", and once the
slides have been changed out from under it that sentence is over. A walk that came
back still held would look like a walk that had stopped.

**On the deck the panel lists slides, not blocks.** Its rows are the five slides by the
name on the wall ("Join in", "Where we split"), each with its switch and size. Blocks
with no slide (question, counts, group controls)
and the latest-statements style are not shown, because a switch that does nothing on
this layout is worse than no switch. `DECK_SLIDES` in `blocks.ts` is the one list both
the deck and the panel read.

The walk is the one thing on the deck that moves without a hand on it. Left and right
arrows, PageUp and PageDown and click still change slides and nothing else, so a
presentation clicker behaves as before.

## Consequences

- The deck is no longer entirely hand-driven. A facilitator who wants the old behaviour
  holds the statement, or lists the slide; there is no separate "never move" setting
  because "stay on this one" is a decision made in the moment, not in setup.
- Hold and walk position live in the component and are lost on reload, like the slide
  position. The style is remembered, because it is part of the board.
- A board remembered before `slides` existed, or while it was still per slide, fails
  `isRoomBoard` and is rebuilt from the preset, which is how every added board field
  has been handled. The old `?slides=statement:list` form is noise, not translated.
- Five is a constant in the deck, not a board size. If a room wants three or seven the
  number becomes a per-block size like the others.
- The walk's timer is the only `setInterval` in the deck. Anything else that wants to
  move on its own needs its own argument against the motion rule.
