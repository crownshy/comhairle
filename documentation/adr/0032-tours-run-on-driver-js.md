# ADR-0032: Tours run on driver.js, behind a config of our own

**Status:** proposed
**Date:** 2026-09-05
**Branch:** participant chrome demo branch

## Context

ADR-0026 introduced a one-time tour of the participant chrome and built it by hand:
`StepTour.svelte`, about 190 lines, with the spotlight cut as a `box-shadow: 0 0 0 9999px`
on a ring positioned from `getBoundingClientRect()`, plus a focus trap, Escape handling and
resize remeasuring.

It worked, and it was four hardcoded beats. Two things pushed past what it could do:

1. **The Learning assistant beat scrolls.** The assistant sits below the fold on a Learn
   step. Pointing at it means scrolling it into view first and then measuring, which the
   hand-rolled version had no notion of.
2. **We want tours on other routes.** A route should declare a tour by naming elements and
   writing captions, not by editing a component with a constant array in it.

Both are solved problems. The question was whether to keep growing ours or take a library.

## Decision

**Use driver.js for the mechanics, and keep the tour's shape as our own config.**

`driver.js` (~5kb, ISC, actively maintained) gives us the spotlight, popover positioning,
`smoothScroll` into view, keyboard control, and `waitForElement` for a target that mounts
late. It is framework agnostic, so nothing about it is tied to a Svelte version.

What stays ours, in `src/lib/tours/`:

- **`types.ts`** — a `Tour` is an id and a list of stops. A stop names its target
  (`target: 'brief'`, matched against `data-tour="brief"` in the markup), carries a caption
  as a function so paraglide resolves it at run time, can wait for a control that mounts late
  (`waitMs`), and can open whatever it points inside (`before`).
- **`Tour.svelte`** — the runner. Drops a stop whose control is not on this screen, builds
  the driver config, applies our copy, and writes the dismissal down when the tour ends
  however it ends.
- **`seen.ts`** — dismissal in `localStorage`, keyed by tour id and scope.
- **`tour.css`** — driver.js's white card repainted in the theme tokens.

A route now adds a tour by exporting a `Tour` and putting `data-tour` attributes on the
controls. `stepTour.ts` is the first one.

### What driver.js costs, and what we did about each

- **Its popover is its own DOM.** No Svelte components inside a caption, and rich content
  would mean HTML strings. Accepted: every caption we have is one sentence. If a beat ever
  needs a component, that beat is the argument for reversing this.
- **Its dismissal is an unlabelled × in the corner.** The runner relabels it "Skip" in
  `onPopoverRender` and moves it into the footer row beside the count, which is where the
  hand-rolled version had it.
- **Its progress text templates `{{current}} of {{total}}` itself**, which cannot carry a
  translated ordering. Each stop gets a `progressText` resolved through paraglide instead.
- **`nextBtnText` set per step defeats the Done label.** driver.js swaps Next for Done by
  overwriting the step's `nextBtnText`, then spreads the step's own popover over that, so a
  step that sets one says "Next" on the last beat. Both labels live at config level.
- **Its stylesheet is fixed colours.** `tour.css` overrides it under the `popoverClass` we
  set, so a driver.js instance started anywhere else keeps its own look.

Rejected:

- **Keeping the hand-rolled tour.** It would have grown scroll-into-view, deferred targets,
  and a config layer, which is most of what driver.js already is.
- **Shepherd.js.** Heavier, and its theming is a stylesheet you override rather than a class
  hook.
- **intro.js.** AGPL or a commercial licence.
- **A Svelte-specific tour package.** The ones that exist are Svelte 3/4 era and unmaintained.

## Consequences

- The spotlight is now an SVG path with a `stageRadius`, not a ring with a huge box shadow.
  It is still a plain black overlay rather than a themed one, for the reason ADR-0026 gives:
  an overlay built from the foreground token inverts in dark mode.
- `stageRadius` is one number for a whole driver.js instance, and the tour points at both
  pill chips and wide panels. The runner sets it per beat from the target's own computed
  corner radius instead, so a pill stays a pill and a panel does not come out as a lozenge.
- ADR-0026's point 4 ("it circles the real control, measured") is still true, but the
  measuring is driver.js's now. Points 1, 2, 3 and 5 are unchanged; the page underneath is
  blocked with `disableActiveInteraction` rather than a catcher div of our own.
- The storage key changed from `comhairle-step-tour-<conversation>` to
  `comhairle-tour-participant-step-<conversation>`, so anyone who has already dismissed the
  step tour is offered it once more. One tap, and only on this branch.
- `before` is in the type but no tour uses it yet. It exists for a beat that has to open
  whatever it points inside, and it is untested until one does.
- Deciding a tour's beats means waiting for controls that mount late, which driver.js's
  `waitForElement` cannot help with: it waits per step, after the count is already fixed.
  The runner resolves every stop up front instead (see ADR-0034).
- Adding driver.js re-normalised peer suffixes throughout `pnpm-lock.yaml`. The lockfile
  diff is much larger than one dependency, and `@zodios/core` needs a `pnpm install` and a
  dev-server restart after the change to pick up its new store path.
