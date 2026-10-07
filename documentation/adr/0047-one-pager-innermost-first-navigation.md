# ADR-0047: One pager, and arrows that traverse the innermost sequence first

**Status:** accepted
**Date:** 2026-08-31
**Scope:** parts 1 and 4 land with the step shell (#1317). Parts 2, 3 and 5 are the tool
sequence contract and land one tool at a time (#1318).

## Context

Participant navigation moves out of `StepHeader` and into a persistent **pager** at the
bottom of the viewport: back on the left, forward or Skip on the right. See
[CONTEXT.md](../../CONTEXT.md) for **Pager**.

The problem is that one pair of arrows has to serve nested sequences at once.

**Tool-internal sequences.** Learn paginates. `LearnUI` already hands the step page a
`currentNextAction` and `currentPrevAction`, and `StepHeader` wired its chevrons to them,
falling through with `onNext={currentNextAction ?? stepComplete}`. So the arrows already
mean two different things depending on which tool is mounted. Prioritization has stages and
Thinking Space has rounds, neither of which is exposed today.

**Steps.** Forward at the end of everything means complete the step and move on.

Before this change the ambiguity was survivable because the arrows lived in a header that
scrolled away and tools mostly shipped their own in-body next buttons. A persistent pager
that is always on screen cannot be ambiguous.

## Decision

**1. Arrows always move through the innermost open sequence, and running off its end pops
out to the next level.** Tool-internal sequence first, then the step boundary. Backward is
the mirror: the first page's back arrow means the previous step.

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
and can implement neither on its own, so its segment does not move until the fork reports.

**4. The forward slot states one thing at a time.** Next when the step can advance, Skip
when it cannot and the step is optional, and a disabled Next when it cannot and the step is
required. The free-floating "Skip this step" button in the step body is removed, because a
second forward affordance is the thing this decision exists to prevent.

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
  it. The boundaries are announced by the progress bar moving to the next segment.
- A tool that reports progress wrongly now corrupts a chrome-level element rather than a
  local one. The bar is clamped to its segment so a bad fraction cannot bleed into
  neighbouring steps.
- HeyForm remains the known exception at every level: no internal sequence the pager can
  drive, and no restyle. Its interior lives in the fork at `../heyform` and any change
  there is a paired change plus a deploy.

  *Update, 2026-09-09.* The progress gap is closed on the prototype. The fork includes the
  active question's index and the field count in the `FORM_STEP_CHANGE` message it already
  posts to the parent, and `HeyFormEmbed` reports `index / total` through the contract
  above. The sequence limit stands: the pager still cannot drive the form's own Next and
  Back, and the bar holds at the handover point against a fork build without the payload.
