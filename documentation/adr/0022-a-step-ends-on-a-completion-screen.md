# ADR-0022: A step ends on a completion screen, and the progress write waits for Proceed

**Status:** proposed
**Date:** 2026-09-01
**Branch:** `mobile-exploration/participant-step-chrome`

## Context

Finishing a step used to be invisible. A tool called `onDone`, the step page wrote
`progressStatus: 'done'` and navigated in the same tick, and the participant's last vote or
survey submit dropped them onto the next step's cover with no acknowledgement that anything
had been recorded.

Two problems. The screen the participant is looking at changes out from under the action
they took, which reads as a misfire rather than a success. And the tools each solved it
locally or not at all: Prioritization shows its own "Thanks for your answers" card and a
"Response submitted" overlay, Polis and HeyForm show nothing, Learn shows nothing.

## Decision

**1. Every step ends on a shared completion screen.** A third phase, `done`, joins `cover`
and `body`. The tool unmounts, the progress segment fills, and the screen carries the title
and a check mark. The pager's arrows and Hint pill give way to a single full-width Proceed
button, which follows the one-thing-at-a-time rule from
[ADR-0018](0018-one-pager-innermost-first-navigation.md).

**2. The progress write moves from "the tool says done" to "the participant presses
Proceed."** `stepComplete` now only switches the phase. The API call and the navigation
happen in `proceed`. A tool that finishes on its own last action can no longer navigate
without the participant asking for it.

The cost is a step that stays `in_progress` if someone closes the tab on the completion
screen. Their answers are already saved by the tool itself, so what they lose is the step
boundary, not their contribution: they come back to a step whose work is done and press
Proceed. Writing on entry to the screen instead would remove that gap, but then Proceed is
a lie, because leaving was already recorded as finishing.

**3. The screen is a dead end by design.** No back arrow. Polis and HeyForm submissions are
final by the time this screen appears, so an arrow that promises a way back would be honest
for Learn and misleading for the tools where most steps end.

**4. Completion green is a token, not the theme's primary.** `--step-complete` is a
code-only token shared by every theme, alongside `--seed-highlight` and the `--chat-*`
family. The Proceed button directly under the check mark is already `bg-primary`, so
reusing primary for the mark would put the same colour on the confirmation and the action.

## Consequences

- Prioritization now shows two completion screens in a row: its own card, then this one.
  Its card is the review gate, which is a different job, but the pair is worth revisiting
  once the shared screen is on the branch.
- `segmentFill` returns 1 for the `done` phase regardless of what the tool last reported.
  HeyForm's segment, stuck at the handover point for the whole step, finally moves.
- The chrome's within-step count is suppressed on the completion screen. Polis's
  "Opinion 12 of 12" describes a tool that is no longer on screen.
- A slow `SetUserProgress` is now visible: Proceed shows a spinner rather than the
  participant staring at the tool they already finished.
