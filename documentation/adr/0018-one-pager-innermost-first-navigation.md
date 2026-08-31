# ADR-0018: One pager, and arrows that traverse the innermost sequence first

**Status:** proposed
**Date:** 2026-08-31
**Branch:** `mobile-exploration/participant-step-chrome`

## Context

The mobile exploration moves participant navigation out of `StepHeader` and into a
persistent **pager** at the bottom of the viewport: back on the left, the Hint pill in the
middle, forward or Skip on the right. See [CONTEXT.md](../../CONTEXT.md) for **Pager**.

The problem is that one pair of arrows has to serve three nested sequences at once.

**Slides.** The cover is a deck. Forward means next slide until the deck runs out.

**Tool-internal sequences.** Learn paginates. `LearnUI` already hands the step page a
`currentNextAction` and `currentPrevAction`, and `StepHeader` wires its chevrons to them,
falling through with `onNext={currentNextAction ?? stepComplete}`. So the arrows already
mean two different things depending on which tool is mounted. Prioritization has stages and
Thinking Space has rounds, neither of which is exposed today.

**Steps.** Forward at the end of everything means complete the step and move on.

Before this change the ambiguity was survivable because the arrows lived in a header that
scrolled away and tools mostly shipped their own in-body next buttons. A persistent pager
that is always on screen cannot be ambiguous.

## Decision

**1. Arrows always move through the innermost open sequence, and running off its end pops
out to the next level.** Cover slides, then tool-internal sequence, then the step boundary.
Backward is the mirror: slide one's back arrow means the previous step.

Rejected: step-level-only arrows, with tools keeping their own in-body next buttons. It
puts two forward affordances on screen at once and leaves the pager lying about what
forward does inside Learn, which is where the ambiguity already exists.

**2. Tool-internal sequences are declared, not inferred.** Learn's ad hoc
`onNextAction` / `onPrevAction` pair is generalised into an optional contract any tool
`UserUI` may implement. A tool that declares nothing is a single page and the arrows act at
the step boundary, which is today's behaviour for six of the seven tools.

**3. Progress is reported through the same contract.** A tool that can say "page 2 of 5"
for the arrows can say how full its progress segment is, from the same state. Both are
optional, and both fall back to step-granular behaviour. HeyForm is a cross-origin iframe
and can implement neither, so its segment does not move. That is a real gap and it is
visible: one step in seven whose bar sits still.

**4. The forward slot states one thing at a time.** Next when the step can advance, Skip
when it cannot and the step is optional, a disabled chevron when it cannot and the step is
required, and Start on the cover's last slide. The free-floating "Skip this step" button
in the step body is removed, because a second forward affordance is the thing this decision
exists to prevent.

**5. One progress bar in the chrome, none in the bodies.** `PolisEmbed` draws its own
opinion counter and fill bar today. Two bars describing the same step, ten pixels apart and
free to disagree, is worse than either alone. Polis reports to the chrome instead, and its
`show_remaining_statements` toggle is rerouted to control the chrome's count so the admin
setting keeps working rather than becoming dead config.

## Consequences

- Every tool that wants pager integration implements optional props. The blast radius is
  each tool's `UserUI` and the step page that mounts them, not a shared base class.
- Learn's existing behaviour is preserved exactly. Its hooks are renamed and formalised,
  not rewritten.
- Forward changes meaning at sequence boundaries, which is discoverable only by pressing
  it. The Start label on the cover's last slide announces the one boundary where the change
  is largest. The others are announced by the progress bar moving to the next segment.
- A tool that reports progress wrongly now corrupts a chrome-level element rather than a
  local one. The bar is clamped to its segment so a bad fraction cannot bleed into
  neighbouring steps.
- HeyForm remains the known exception at every level: no internal sequence, no progress, no
  restyle. Its interior lives in the fork at `../heyform` and any change there is a paired
  change plus a deploy.
