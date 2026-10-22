# ADR-0064: A step ends on a completion screen, and the progress write waits for Proceed

**Status:** proposed
**Date:** 2026-10-22

## Context

Finishing a step is invisible on staging. A tool calls `onDone`, the step page writes
`progressStatus: 'done'` and navigates in the same tick, so the participant's last vote or
survey submit drops them onto the next step with nothing to say it was recorded. The screen
changes under the action they just took, which reads as a misfire.

The tools also handle this on their own or not at all. Prioritization has its own "Thanks
for your answers" card, while Polis, HeyForm and Learn show nothing.

## Decision

**1. Every step ends on a shared completion screen.** The tool unmounts, the progress
segment fills, and the screen shows "Step completed" over a check mark. The pager gives way
to one full-width Proceed button, which is the move in the middle of the bar from
[ADR-0048](0048-the-middle-is-the-move-the-corners-are-navigation.md).

**2. The progress write moves to Proceed.** `stepComplete` only shows the screen.
`SetUserProgress` and the navigation happen in `proceed`, so a tool that finishes on its own
last action can no longer navigate without the participant asking.

The cost is a step that stays `in_progress` if someone closes the tab on the completion
screen. The tool has already saved their answers, so they lose the step boundary, not their
contribution, and press Proceed when they come back. Writing on entry to the screen would
close that gap, but then Proceed would be a lie.

**3. A skip goes straight on.** The screen is for a step the participant did. Skip on an
optional step calls `proceed` directly.

**4. No back arrow.** Polis and HeyForm submissions are final by the time the screen
appears, so a way back would be honest for Learn and misleading for the tools where most
steps end.

**5. Completion green is its own token.** `--step-complete` is shared by every theme,
like `--seed-highlight`. The Proceed button under the mark is already `bg-primary`, and
the same colour on the confirmation and the action would blur them.

## Consequences

- Prioritization shows two completion screens in a row: its own review card, then this one.
  Worth revisiting.
- The chrome's within-step count is hidden on the completion screen, since "Opinion 12 of
  12" describes a tool that is no longer there.
- A slow `SetUserProgress` shows as a spinner on Proceed instead of the participant staring
  at the tool they already finished.
- Revisiting a done step also ends on the screen. Proceed then goes to the next revisitable
  step, as Next did before.
