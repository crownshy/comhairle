# ADR-0037: Controls draw their own press state

**Status:** proposed
**Date:** 2026-09-14
**Branch:** demo branch

## Context

On a phone, tapping a button gave almost no sign that the tap landed. Tailwind v4 compiles
`hover:` inside `@media (hover: hover)`, so no hover style applies on a touch screen, and
the shadcn `Button` had no `active:` styles. The only response was the browser's tap
highlight, which on iOS is a grey rectangle that ignores the pill shape of our buttons.
Page loads already show `RouteProgress`, but a tap that opens a dialog, casts a vote or
flips a switch showed nothing between the finger going down and the result.

There are around 150 hand-rolled `<button>` elements outside `Button`, plus bits-ui
triggers (tabs, switch, checkbox, select, accordion) that render their own `<button>`.
Fixing `Button` alone would leave most of the participant chrome silent.

## Decision

**1. One press tint for every control, in `app.css`.** While a `button`, `summary`,
`[role='button']` or `[data-slot='button']` is `:active` and not disabled, it gets a 12%
tint of its own text colour laid over its fill as a `background-image`. Because the tint
follows `currentColor`, white text on a primary fill lightens it, dark text on a pale
secondary fill darkens it, and outline and ghost controls pick up their text colour. That
holds in all six themes and in dark mode with no colour per variant. Material's pressed
"state layer" works the same way.

Rejected: **an `active:bg-*` colour per `Button` variant.** Every variant needs its own
step past its hover colour in light and dark, and the pale fills have no lighter step that
shows (`secondary` is `#f3f4f6` in two themes). It also does nothing for hand-rolled
buttons.

Rejected: **a pseudo-element overlay.** It needs `position: relative` on every button,
which breaks buttons positioned against an outer ancestor, and it collides with components
that already use `::before` or `::after`.

**2. `Button` also shrinks to 97% while pressed.** The scale goes in instantly
(`active:duration-0`) and eases back over the default 150ms when the finger lifts, so a
quick tap still shows as a flash. It is off under `prefers-reduced-motion`. Hand-rolled
buttons get the tint only, because a scale on a full-width row or list item looks like the
layout jumping.

**3. The browser's tap highlight is off on the same controls.** They also get
`touch-action: manipulation`, so a quick second tap does not zoom the page on iOS.

**4. The root layout adds an empty, passive `touchstart` listener.** iOS Safari does not
apply `:active` on tap until the page has a touch listener. A passive listener cannot delay
scrolling.

Hover stays as it is. On a touch screen a hover style sticks after the tap, which is why
Tailwind gates it, and the press tint gives a phone the feedback hover gives a mouse.

## Consequences

- A hand-rolled `<button>` gets press feedback with no classes. Do not add an `active:`
  tint or `-webkit-tap-highlight-color` to one.
- A control that must not tint opts out with `active:bg-none`. A control with a
  `bg-linear-*` or image background loses the tint, because the utility beats the
  base-layer `background-image`.
- The tint switches on and off instantly. `background-image` cannot transition, so only
  `Button`'s scale eases out.
- Plain links keep the browser's own tap highlight.
- Point 4 rests on Apple's archived Safari Web Content Guide and has not been checked on a
  current iPhone.
