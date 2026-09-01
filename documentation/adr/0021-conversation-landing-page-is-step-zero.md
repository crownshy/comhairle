# ADR-0021: The conversation landing page is Step zero

**Status:** proposed
**Date:** 2026-09-01
**Branch:** `mobile-exploration/participant-step-chrome`

## Context

The conversation landing page is where every participant arrives, and on a phone it was the
weakest screen we had. It rendered `ConversationSummary` inside the site chrome: NavBar,
breadcrumbs, a full-width `object-contain` image, the short description, the long
description, and only then the call to action. On a 375pt viewport the button sat roughly
two screens down. Nothing on the page said how many Steps there were or how long they would
take, so the decision to start was made blind.

Meanwhile the Steps themselves had just been rebuilt around `StepChrome` and `StepPager`
(ADR-0017, ADR-0018, ADR-0019): a single viewport, a progress bar, a fixed bottom bar. The
landing page and the flow it led into looked like two different products.

Three things shaped the answer.

**The landing page is already part of the engagement.** A participant who has arrived and is
reading has begun; they simply have not committed. Treating the page as marketing that ends
where the "real" flow starts is a description of our routing, not of their experience.

**The detail has to go somewhere.** The long description, the FAQs and the privacy policy
are real content that some participants do read, and the FAQs and policy are rich text of
unbounded length. Whatever replaced the old layout had to keep them reachable.

**Progress is meaningful before you join.** `StepProgressBar` renders a flexible segment for
the current step and stubs for the rest. Shown on the landing page it says "here is the
shape of the whole thing, and you are at the start of it" without any new mechanism.

## Decision

**1. The landing page renders in participant chrome, as Step zero.** It mounts `StepChrome`
and no NavBar or Footer, exactly like the Step pages, and the progress bar carries an extra
leading segment for the landing itself. A participant sees the journey's full shape before
committing, and the bar does not appear from nowhere on Step one.

Rejected: showing the real Steps with the first one marked current. That would claim the
participant is in Step one when they have not joined, and a `Take Part` they never press
would leave the bar lying.

**2. The intro segment is excluded from "Step N of M".** `StepItem` gains `isIntro`, and
`StepDropdown` numbers only the steps without it. Adding a segment to the bar therefore
cannot change the count a participant is quoted, and the intro row in the dropdown carries a
name with no position line.

Trade-off, accepted: the bar and the numbering now disagree on how many segments there are,
five drawn against "of 4". The bar is a picture of a journey, not a counter, and inflating
the quoted step count to make them agree would be the worse lie.

**3. Detail lives in sections below the fold, not in a modal.** The cover's one affordance
scrolls to a sticky strip of jump links over About / Steps / Questions / Your data, with the
active section lit. Only sections with content appear.

Rejected: a bottom sheet. It would have to hold rich text, a step list and a privacy policy
inside a viewport-height scroll area stacked on the page it came from, and none of it would
be linkable. Sections cost nothing and grow.

**4. The call to action is fixed, not sticky.** It has to survive the scroll through the
detail, not just the cover, so it is the one element that never leaves.

**5. Step zero is the term, not Cover.** `Cover` already names the first slide of a Step
brief (ADR-0017). The component is `StepZeroScreen`.

## Consequences

The landing page fetches the workflow's Steps, which it did not before, in its own
`+page.ts` rather than the conversation layout: the FAQ, privacy and report pages under that
layout do not need them, and the workflow layout fetches its own copy with per-user progress
attached.

`(public)/+layout.svelte` decides to drop the site chrome from a `participantChrome` flag
that the landing load returns, rather than from a pathname match. The `[[preview]]` optional
parameter swallows any single trailing segment, so a regex there would disagree with the
route for URLs like `/conversations/<id>/anything`.

The quoted duration is soft. It sums `TOOL_META.estimatedMinutes`, a hardcoded per-tool
figure shared with the admin design board's "Estimated time" pill, because the data model
has no per-Step estimate. Two Conversations with the same tools quote the same total however
differently they are configured. The number reads more precise than it is; a real
`estimated_minutes` column would fix it, and until then rounding it harder is the cheaper
mitigation.

`ConversationSummary` survives: the invite page still uses it.
