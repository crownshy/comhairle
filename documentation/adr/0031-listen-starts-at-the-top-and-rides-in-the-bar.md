# ADR-0031: Listen starts at the top of the page and rides in the bar

**Status:** proposed
**Date:** 2026-09-04
**Branch:** `demo/young-scot`
**Refines:** part 2 of
[ADR-0025](0025-the-middle-is-the-move-the-corners-are-navigation.md)

## Context

Listen speaks a participant page's text aloud with the browser's speech synthesiser. The
first build gave it two pieces: a wide "Listen to this page" pill in the middle of the
content, and a player that appeared above the bottom bar once something was playing.

Both pieces cost the reading. The pill sat between the title and the first paragraph, which
is exactly where a reader's eye lands, and it pushed the content down on a screen that is
already only one viewport. The player at the bottom split the control in two, so starting
and pausing happened at opposite ends of the screen, and it competed with the pager for the
one row the bottom bar has.

Three shapes were built on the live step route and compared: the control as an icon in the
chrome's right-hand cluster, as a podcast-style card at the top of the content, and as a
sticky band under the chrome that was its own progress track.

## Decision

**The offer is a button at the top of the page. The controls ride in the bottom bar.**

The two halves of Listen are two different kinds of thing, and they get two different places.

**Starting is a decision made once, at the top.** The button sits above the article, where
the reading begins, and it quotes the cost before it asks: "Listen to this page · 4 min". It
is not sticky. A reader decides whether to listen at the point they decide whether to read.
The first build's pill was a problem because it scrolled away and took the only control with
it; once the controls live in the bar, the offer can afford to stay put.

**Pause and speed stay on screen.** Once a page is playing, the pager's middle, between Back
and Next, carries a play/pause button and a speed chip. That is the one row that is always
in reach, and the controls a listener actually needs while reading along are those two.
Nothing else goes there: the button at the top already says what is playing.

**The button becomes the progress track.** Playing, it fills left to right as the voice moves
through the blocks, and its label changes to Pause with a "3 of 12" count. Progress is
counted in blocks rather than seconds, because the voice reports where it is by block and
any time we showed would be a guess. The block being read is marked in the article and
scrolled into view only when it is off screen, so a participant reading ahead is not pulled
back on every sentence.

**Pause is a cancel that remembers the block.** The synthesiser's own pause is unreliable on
Android and resumes from nowhere after a few seconds on some builds. Restarting the current
block is the version that works everywhere, and losing a sentence's worth of position is a
fair trade for a control that does what it says.

**No duration means no button.** The estimate is costed at attach time, off the rendered
text, at a middling read-aloud pace. Zero minutes is how a surface with nothing readable is
told apart from a short one, so a tool body with no prose (Polis, a survey) gets no control
at all rather than one that does nothing. The same goes for a browser with no voice in the
participant's language: no offer is better than a silent one.

**It is offered on Learn pages and nowhere else.** Learn is the only place a participant is
handed more than a screenful of prose to get through, which is the whole case for reading
it aloud. The short screens are not worth the offer: a step brief slide, step zero and
Before you start are each about a screen, and a control that reads two sentences aloud costs
more attention to notice and decide about than the sentences do to read.

## Consequences

ADR-0025's "the bottom bar is navigation only" gains one exception: Listen's transport may
sit in the pager's middle while a page is being read aloud. It is not a move, and it is
there only in that state. The rest of the time the middle stays empty. `StepPager` takes an
optional `middle` snippet, and the step page hands it the transport only while Listen is
not idle.

The state is a module (`listen.svelte.ts`), not a context, because its two controls live in
components that do not share a parent: the Learn body attaches its article and renders the
offer, and the step page renders the transport into the pager. The synthesiser is one
global per window anyway.

Turning a Learn page, or leaving the step, stops playback. The Learn body re-attaches on
every content change, and detaching is what stops the voice, so no phase or route needs to
know about Listen.

The admin participant view renders the button because it renders the real Learn body
(ADR-0030). It is inert there, like everything else on that screen.

Any surface that hosts Listen later needs a top to put the offer at and a bar to put the
transport in. The step page has both. The landing page and the step brief take-over would
each need a place for the transport before they could offer it.

## Alternatives considered

Rejected: an icon in the chrome's right-hand cluster. It is the cheapest thing on the page,
but it is also the least legible: an unlabelled speaker icon among navigation controls reads
as a mute toggle, and the duration has nowhere to go. Expanding it into a transport in place
crowded a cluster that already carries the step name, the hint chip and the menu.

Rejected: a podcast-style card at the top of the content, with artwork and a scrubber. It
reads well and it is the shape people already know, but it takes a third of the first
viewport, and the artwork implies a produced recording rather than a synthesiser reading
the page.

Rejected: a sticky band under the chrome carrying the offer, the transport and the progress
track in one strip. It kept the whole control in one place and in reach the whole way down,
but it was a second sticky row on a screen that already has the chrome and the bar, and it
made the offer permanent on a page where most participants will read rather than listen. The
button-plus-transport shape spends sticky space only on the participants who chose to
listen.
