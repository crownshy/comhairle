# ADR-0034: The Learning assistant is introduced where it sits

**Status:** proposed
**Date:** 2026-09-05
**Branch:** participant chrome demo branch

Amends ADR-0028.

## Context

ADR-0028 gave the assistant an intro card on first arrival: a title, three points, the
privacy line, "Got it", and everything else behind "Learn more". It was the right content
and the wrong shape. On a phone it is a full screen of blue above an input the participant
has not seen yet, and it explains the thing while sitting on top of it.

Meanwhile ADR-0032 turned the tour into something any route can declare. A tour beat says
the same three sentences while pointing at the actual ask bar, and costs no vertical space
at all.

## Decision

**1. The assistant is introduced by the tour, not a card.** It gets two beats: the assistant
itself, scrolled into view and circled, and the step menu, where it also lives once the
participant has moved off a Learn step. The card stays in the component but is closed on
arrival, and "What is this?" opens it with Learn more already expanded.

**2. It is two more beats on the one tour, not a second tour.** Six beats in the order a
thumb finds them: the two corners at the top, the assistant in the body, the menu it also
lives in, then the two corners at the bottom. One count, one "Got it", one dismissal.

Rejected: a separate assistant tour keyed on its own. It was built that way first, and it
meant a participant on a Learn step got "1 of 4", then "1 of 2" a moment later, which reads
as two interruptions rather than one introduction.

**3. The privacy line moves out from under the fold.** It sits under the ask bar, always. It
is the one line that has to be read before the first question rather than after it, and a
notice about not typing your name is not something to hide behind a disclosure. Everything
else that was in the card is in the tour or behind "What is this?".

**4. Which beats exist is resolved before anything is drawn.** The count a participant reads
is of the beats that survive, so the runner cannot start and then discover a seventh. A step
transition takes about a second to put its tool up, so the assistant beat carries a
`waitMs`: the runner keeps looking for that long before deciding it is not on this screen.

**5. A tour with nothing to show is not recorded as seen.** Otherwise a tour whose screen has
not come up yet would be quietly spent on the wrong step.

## Consequences

- `hasSeenIntro` / `markIntroSeen` and the `comhairle-assistant-intro` storage key are gone.
  Dismissal is the step tour's, and there is only one of those.
- **A conversation whose first step is not a Learn page never shows the assistant beat.** The
  tour runs once, at the start, and a beat whose control is not on that screen is dropped;
  those participants get five beats and meet the assistant through the menu beat and "What is
  this?". If that turns out to matter, the fix is a second tour, which is what this decision
  rejected, so it would be a reversal rather than an addition.
- The support drawer has no tour, so the assistant there is introduced only by "What is
  this?". That is the same control it has always had, and the drawer's tab already names it.
- The tour reaches the assistant by scrolling the step's `<main>`. driver.js repositions its
  spotlight on window scroll only, so the runner listens for scroll on the document in the
  capture phase and refreshes on any scroller. Without that the hole stays where the target
  used to be, which is what the first attempt did.
- Centring the target is ours rather than driver.js's `smoothScroll`, which nudges a target
  just inside the edge and measures before the scroll has landed.
- `learning_assistant_intro_title` and the three intro points are still used, now only behind
  "What is this?". If that control is ever dropped, they go with it.
