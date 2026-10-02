# ADR-0048: In the step's bottom bar, the middle is the move and the corners are navigation

**Status:** accepted
**Date:** 2026-09-01
**Refines:** part 4 of [ADR-0047](0047-one-pager-innermost-first-navigation.md)

## Context

On the prototype the bottom bar changed shape between the phases of a step, and until this
decision it changed meaning in the same place. The step brief ended in one full-width button
in the middle of the bar; press it and the tool loaded, the bar became the pager, and what
landed in the middle of that bar, under the thumb that had just pressed Start, was a chip
that opened the brief again. It is not a move. A participant who tapped where the button was
got the brief back and the flow read as a loop with no way on.

And in a tool that carries its own Next and Skip inside the frame, a bare chevron in the
corner of our bar says nothing about which control leaves the step.

Staging has the pager and not yet the brief, so the parts that matter here are 2 and 4.

## Decision

**1. The middle of the bar is the move.** A full-width button in the middle means the main
way forward and nothing else. Nothing else may sit there.

**2. The bottom bar is navigation only.** Back on the left, forward on the right, in the same
corners in every phase that has them, and nothing between them.

**3. Anything that is not a move belongs in the header, not the bar.** What a brief chip
answers is "what is this step", which is what the header is already about.

**4. Both directions carry their label.** The forward slot says Next or Skip rather than
showing a bare chevron, including while it is disabled. A dimmed "Next" says "not yet, and
this is the one that leaves the step", which is the question a participant has when the tool
in front of them has a Next of its own.

Rejected: giving the body phase its own full-width forward button. In most steps the tool
owns the forward action, and a second full-width button next to the tool's would compete
with it rather than clarify it.

Rejected: keeping a non-navigation control in the bar but off to one side. It still put a
control that is not a move in the row a participant scans for the way forward.

## Consequences

- The primary action moves between the middle (a screen whose only decision is whether to go
  on) and the right corner (the tool body). That is the point: it is a different kind of
  action in each, and the shape says so.
- `StepPager` knows nothing but back and forward. Anything a later screen wants in the bar
  is a different bar, not an extra slot on this one.
