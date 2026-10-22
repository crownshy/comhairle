# ADR-0063: The step name scrolls with the tool

**Status:** proposed
**Date:** 2026-10-22

## Context

The participant step page has two rows. The top row is pinned: the logo, the step menu, the
progress bar, and under them the step's name and description. The tool scrolls in the row
below.

Two problems with the name and description in the pinned row:

- On a phone they take a fifth of the screen on every step, and the description can run to
  several lines.
- The Polis onboarding spotlight (ADR-0062) dims everything except one card. When that card
  scrolls up under the pinned row, the cutout sits over the step name, so the name shows
  through inside the lit area.

## Decision

The step name and description move into the scrolling row, above the tool. The logo, step
menu and progress bar stay pinned, so the participant can still see where they are and get
to the menu from anywhere in the step.

This applies to every tool, not just Polis.

## Consequences

- Phones get the room back once the participant scrolls past the description.
- The tool column no longer fills the scroll area on its own (`min-h-full`). It grows into
  whatever is left under the step name (`grow`), so a short step still fits without a
  scrollbar.
- Full-height tools (HeyForm, Thinking Space) start lower on first load, by the height of the
  name and description, and need checking on a phone.
