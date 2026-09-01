# ADR-0023: The step brief ends in a button, and the hint takes the whole screen

**Status:** proposed
**Date:** 2026-09-01
**Branch:** `mobile-exploration/participant-step-chrome`
**Supersedes:** the Start label in part 4 of
[ADR-0018](0018-one-pager-innermost-first-navigation.md)

## Context

[ADR-0018](0018-one-pager-innermost-first-navigation.md) gave the whole participant flow one
pager and made its forward slot say Start on the cover's last slide. Two things about that
did not survive contact with the mobile design.

The cover has exactly one thing to do, and a chevron in the corner is a small target for it.
Every screen in the flow that is only an instruction (the cover's slides) ends in the same
question: are you ready. That reads as a button, not an arrow.

And the hint, which is the same brief reopened mid-step, was a card floating on a scrim.
Same slides, same illustration, different frame, so the two surfaces of one brief looked
like two different things.

Three variants were prototyped behind a `?variant=` switch and thrown away. The one below
won.

## Decision

**1. The step brief gets its own bottom bar.** Slide dots and a single full-width forward
button, in place of the pager. The label progresses: Next while slides remain, then Start on
the cover's last slide and Close on the hint's, which is the only place either surface has
to send the reader. `StepBriefBar` is shared by both, so the two surfaces cannot drift.

**2. The hint is a full screen take-over shaped like the cover.** Same slide layout, same
bar, a close control in the header row's place. It is still `role="dialog"`.

**3. Step-level controls on the cover are desktop only.** Back, and Skip for an optional
step, sit in a row above the slide at `md` and up, where there is room for them. On a phone
the cover carries the button and nothing else, and back is a swipe. The bar is at the bottom
at every breakpoint: that is where a thumb is.

**4. The pager's forward slot loses Start.** It never renders during the cover phase now, so
Next, Skip and a disabled chevron are the whole vocabulary. One thing at a time still holds:
no screen shows both a pager arrow and a brief button.

## Consequences

- On a phone, the cover has no visible back control. Swipe and the browser's own back
  gesture are the ways out of it. This is the cost of a single forward affordance, and it is
  the only screen in the flow where back is invisible.
- Skip is likewise invisible on a phone until the tool body mounts and the pager appears. An
  optional step therefore cannot be skipped from its cover on mobile without reading it
  first. Worth revisiting if participants complain.
- The brief bar is taller than the pager (dots plus a 48px button), so the cover's slide area
  is shorter than the tool body's. Nothing in the cover scrolls today.
- A step with a single slide gets no dots, and its button reads Start immediately.
