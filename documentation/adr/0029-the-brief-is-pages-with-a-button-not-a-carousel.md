# ADR-0029: The brief is pages with a button, not a carousel

**Status:** proposed
**Date:** 2026-09-02
**Branch:** `mobile-exploration/participant-step-chrome`
**Amends:** part 1 and part 3 of
[ADR-0023](0023-the-step-brief-ends-in-a-button.md), the swipe in part 1 of
[ADR-0024](0024-before-you-start-is-a-deck-you-can-come-back-to.md)

## Context

The step brief was built as a carousel: dots under the content, a swipe handler on the
slide area, and arrow keys in the hint. Before you start borrowed the same handler, where a
horizontal drag scrolled to the next page.

The gesture is the part that keeps costing. It sits on top of a scrolling page and a
tool body that has its own drag targets, it has no visible affordance, and every screen it
touches has to decide what a drag means before the content does. Nothing about the brief
needs it: the reader is being told something and then asked whether they are ready.

## Decision

**1. No swipe on any of the brief's screens.** `carouselSwipe` is deleted, along with its
tests and the pointer handlers on the cover, the hint and Before you start. Forward is the
button, back is the pager or the browser.

**2. No dots.** `StepBriefBar` is a button and nothing else. The screen already says which
step this is, in the header and the progress bar; the reader of a two-page brief does not
need a second, smaller count under it.

**3. The hint keeps Escape and loses the arrow keys.** Escape is what a dialog answers to.
Left and right were the keyboard half of the gesture, invisible in the same way.

The brief is still pages split at horizontal rules (ADR-0017), still shown one at a time,
and the label still progresses to Start or Close on the last one. What changes is that the
only way through is the control the reader can see.

## Consequences

- The cover has no back control at all on a phone, where before it had an invisible one.
  ADR-0023 already accepted that back was invisible there; this makes it absent. The
  browser's own back gesture still leaves the step.
- Once past the first page of a multi-page brief, there is no way back to it short of
  leaving the step and returning. Worth a visible back control on the bar if briefs get
  long enough for anyone to want one.
- The brief bar is now the height of one button, so the cover's content area is taller than
  it was.
- `SlideDots` is unused by the brief. It stays for the lived experience tool, which is its
  only remaining caller.
- Before you start keeps its scroll snapping and its chips. Only the horizontal drag is
  gone; scrolling down is unaffected.
