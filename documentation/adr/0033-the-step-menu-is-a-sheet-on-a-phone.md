# ADR-0033: The step menu is a sheet on a phone

**Status:** proposed
**Date:** 2026-09-05
**Branch:** participant chrome demo branch

## Context

The step menu is the participant's map: five or six steps, the Learning assistant, the FAQs,
the light and dark toggle, and three legal documents. Fourteen rows on a longer conversation.

It was one anchored dropdown at every width, `w-72`, hanging off a trigger in the top right
corner. On a phone that is most of the screen, held up by a corner, scrolling inside itself,
with rows sized for a mouse. It is also the way into everything the tour is about to point
at, so it is worth fixing before the tour names it.

## Decision

**1. Below the `md` breakpoint the menu is a bottom sheet; above it, the dropdown stays.**

The sheet is `vaul-svelte`, already in the repo for the "Find out more" panel. It is capped
at `85vh` with the list scrolling inside, rows at `min-h-14`, and it carries a drag handle,
a title and a close control.

Rejected: **a full-screen overlay.** It wipes the page you are standing on, which is exactly
the context the menu exists to place you in, and it needs an explicit close control because
there is no swipe-down affordance. A sheet is thumb-reachable, dismisses by swipe or
backdrop, and leaves the step visible behind it.

Rejected: **keeping one dropdown and making the rows bigger.** The rows were never the whole
problem. An anchored menu on a small screen is pinned to a corner and cannot use the width.

**2. The rows are shared, not restated.** Step rows render through one snippet used by both
shells. Everything under them (assistant, FAQs, theme, legal) is uniform enough to be data,
so each shell maps the same list rather than repeating its contents. Two shells that could
disagree about what is in the menu is the failure this is guarding against.

**3. On a pointer, the page behind the open menu goes soft.** A `backdrop-blur` veil at
`z-40` under the menu, with the trigger raised above it. The menu overlaps the step's own
text, and without the veil the two read as one surface. The trigger stays sharp because it
is the control you just pressed, and blurring it makes the press look like it missed.

**4. The shell is chosen by `IsMobile`, and the server renders the dropdown.** The media
query has no answer during SSR, so the dropdown is the first paint at every width. The menu
is closed then, and both triggers are the same pill, so the swap on hydration is invisible.

## Consequences

- `StepDropdown.svelte` became `StepMenu.svelte`. The name was going to be wrong either way
  once half of it stopped being a dropdown.
- Opening the assistant from the sheet is one drawer closing while another opens. The sheet
  is told to close before the panel opens, but vaul's exit runs for about a second, so the
  two overlap. The support panel is full width on a phone and covers the sheet, so it reads
  as one sheet replacing another rather than as a stack.
- The legal documents sit below the fold in the sheet on a short screen. They are the
  quietest rows in the menu and were already last in the dropdown.
- `CircleQuestionMark` now types its `class` prop, which it needed to be usable as a value
  in the shared row data rather than only as a tag.
