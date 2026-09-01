# ADR-0025: In the step's bottom bar, the middle is the move and the corners are navigation

**Status:** proposed
**Date:** 2026-09-01
**Branch:** participant chrome demo branch
**Refines:** part 3 of
[ADR-0018](0018-one-pager-innermost-first-navigation.md) and part 1 of
[ADR-0023](0023-the-step-brief-ends-in-a-button.md)

## Context

The bottom bar changes shape between the three phases of a step, and until now it changed
meaning in the same place.

On the cover the bar is slide dots over one full-width button: Next, then Start. Press it and
the tool loads, the bar becomes the pager, and what lands in the middle of that bar, under
the thumb that was pressing Start a second ago, is the Hint chip. It is not a move. It opens
the step brief over the step.

Two things follow. A participant who taps where the button was gets the brief back, which
looks like the screen they just left, and taps forward again: the flow reads as a loop with
no way on. And in a tool that carries its own Next and Skip inside the frame, a bare chevron
in the corner of our bar says nothing about which control leaves the step.

## Decision

**1. The middle of the bar is the move.** A full-width button in the middle means the main
way forward and nothing else: Start on the cover, Proceed on the completion screen. Nothing
else may sit there.

**2. The bottom bar is navigation only.** Back on the left, forward on the right, in the same
corners in every phase that has them, and nothing between them.

**3. The brief chip moves to the header, next to the step's name.** It is not a move, so it
does not belong in the bar at all. What it answers is "what is this step", which is what that
corner is already about, and it opens a take-over whose Close sits in the same corner it was
pressed from. It appears only while the tool is up: on the cover the brief is the screen, and
a step with no description has no brief to reopen.

**4. Both directions carry their label.** The forward slot says Next or Skip rather than
showing a bare chevron, including while it is disabled. A dimmed "Next" says "not yet, and
this is the one that leaves the step", which is the question a participant has when the tool
in front of them has a Next of its own.

Rejected: giving the body phase its own full-width forward button. In most steps the tool
owns the forward action, and a second full-width button next to the tool's would compete
with it rather than clarify it.

Rejected: keeping the chip in the bar but off to one side. It still put a control that is not
a move in the row a participant scans for the way forward.

## Consequences

- The primary action moves between the middle (cover, completion) and the right corner
  (body). That is the point: it is a different kind of action in each, and the shape says so.
- ADR-0024's cover with no back and no skip is the same rule seen from the other side. The
  cover's only decision is whether to go on, so its bar has a middle and no corners.
- The step header now carries the step's name, the within-step count and the brief chip. On a
  narrow screen the name truncates sooner. It was already the row that truncates, and the chip
  is one short word.
- `StepPager` no longer knows about the brief. `StepChrome` takes `briefOpen` and `onBrief`,
  and omitting `onBrief` is how a page says it has no brief, which is what the landing page
  and a description-less step do.
