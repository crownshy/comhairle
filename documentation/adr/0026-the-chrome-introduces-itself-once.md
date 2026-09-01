# ADR-0026: The chrome introduces itself once, on the first step

**Status:** proposed
**Date:** 2026-09-01
**Branch:** participant chrome demo branch

## Context

The participant chrome now has four fixed places: the mark that goes back to Before you start
(ADR-0024), the brief chip that reopens the step's instructions (ADR-0025), and back and
forward in the corners of the bottom bar. Every one of them is learnable in a second, and
none of them is discoverable, which is a different thing.

The costly one is the brief. A participant reads the cover, presses Start, and the
instructions are gone. If they did not take them in, nothing on the screen says they can have
them back, so they answer the step guessing at what it wanted.

## Decision

**1. The first time through, the chrome introduces itself, one place at a time.** A layer over
the step circles a single control, says what it is for, and moves on when the participant
presses Next. Four short beats, with Skip and Escape ending it at any point.

Rejected: naming all four at once around the edges of the screen. It was the first thing we
built and it read as clutter: four labels landing on top of the step's own text, none of them
attached to anything, and nothing saying which was which.

**2. It waits for the step body.** The cover has no pager and no brief chip, so a tour shown
there would point at controls that are not on the screen yet. The body is the first moment
the chrome is complete, and it is also the moment the instructions have just disappeared,
which is the thing the tour most needs to answer.

**3. First run is read from progress, not from a flag of our own.** No step finished means
first run. A participant who returns to an unfinished conversation in another browser is
offered it again, which is right, and one who has completed a step never sees it, which is
also right. Dismissal is kept in `localStorage`, keyed by conversation, so it does not come
back within the session or on the next step.

Rejected: a server-side "seen" flag. It would be a new field on participation for something
whose worst failure is showing a dismissible screen twice.

**4. It circles the real control, measured.** Each place carries a `data-tour` name, and the
layer finds it, measures it and draws the ring around it. Positions guessed from the corners
were close but never right, and would have gone wrong the first time a control moved.

The darkening is the ring's own box shadow, so the hole is exactly the control and nothing
is drawn over the thing being pointed at. It is also a plain black shadow rather than a
themed one: an overlay built from the foreground token inverts in dark mode, which is what
the first version did.

**5. The page underneath is blocked while the tour is up.** The circled control looks
reachable and is not. The alternative is a participant pressing the Next it has just been
shown, navigating to the following step, and leaving the tour pointing at controls that have
moved on without it.

## Consequences

- An admin previewing a conversation sees it once too. Preview has no progress, so it is
  always a first run there. That is the same screen a participant gets, which is the point of
  preview.
- Storage that refuses to be written (private browsing, blocked site data) means the tour can
  appear again on the next visit. Shown twice was chosen over never shown at all.
- Each caption names its place in words ("Top left: ...") rather than relying on the ring, so
  it carries the same meaning read aloud. The card is a labelled dialog and focus moves to it
  on every beat, which is what announces the new place; the ring is decorative and hidden.
- The four captions are copy that has to stay true to the chrome. If a control moves, the
  caption moves with it, and CONTEXT.md's Hint entry is where the wording is settled.
- A control that is not on the screen is not in the tour, so a step with no brief gets three
  beats rather than a ring around an empty corner.
