# ADR-0049: The step menu is a sheet on a phone

**Status:** accepted
**Date:** 2026-09-05

## Context

The step menu is the participant's map: the workflow's steps with their status and the light
and dark toggle, and later the Learning assistant, the FAQs and the legal documents. Fourteen
rows on a longer conversation once those land.

On the prototype it was one anchored dropdown at every width, `w-72`, hanging off a trigger
in the top right corner. On a phone that is most of the screen, held up by a corner,
scrolling inside itself, with rows sized for a mouse.

## Decision

**1. Below the `md` breakpoint the menu is a bottom sheet; above it, the dropdown stays.**

The sheet is `vaul-svelte`, already in the repo for the support panel. It is capped at
`75svh` with the list scrolling inside, well short of full height because Chrome on iOS can
lay the page out under its toolbars and a taller sheet slides its heading under the address
bar. Rows are `min-h-14`, and it carries a drag handle and a title.

Rejected: **a full-screen overlay.** It wipes the page you are standing on, which is exactly
the context the menu exists to place you in, and it needs an explicit close control because
there is no swipe-down affordance. A sheet is thumb-reachable, dismisses by swipe or
backdrop, and leaves the step visible behind it.

Rejected: **keeping one dropdown and making the rows bigger.** The rows were never the whole
problem. An anchored menu on a small screen is pinned to a corner and cannot use the width.

**2. The rows are shared, not restated.** Step rows render through one snippet used by both
shells. Two shells that could disagree about what is in the menu is the failure this is
guarding against.

**3. On a pointer, the page behind the open menu goes soft.** A `backdrop-blur` veil at
`z-40` under the menu, with the trigger raised above it. The menu overlaps the step's own
text, and without the veil the two read as one surface. The trigger stays sharp because it
is the control you just pressed, and blurring it makes the press look like it missed.

**4. The shell is chosen by `IsMobile`, and the server renders the dropdown.** The media
query has no answer during SSR, so the dropdown is the first paint at every width. The menu
is closed then, and both triggers are the same pill, so the swap on hydration is invisible.

## Consequences

- The theme row keeps the menu open. The mode is a thing you look at, and closing the menu
  to show the result means reopening it to change your mind.
- The legal documents, when they arrive, sit below the fold in the sheet on a short screen.
  They are the quietest rows in the menu and belong last.
