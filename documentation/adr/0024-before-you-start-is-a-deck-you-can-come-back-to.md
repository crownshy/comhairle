# ADR-0024: Before you start is a deck of pages, and a place you can come back to

**Status:** proposed
**Date:** 2026-09-01
**Branch:** participant chrome demo branch
**Supersedes:** part 3 of
[ADR-0021](0021-conversation-landing-page-is-step-zero.md),
[ADR-0020](0020-the-logo-asks-before-it-leaves-a-step.md) in full, part 3 of
[ADR-0023](0023-the-step-brief-ends-in-a-button.md), and the word "every" in part 1 of
[ADR-0022](0022-a-step-ends-on-a-completion-screen.md)

## Context

ADR-0021 put the conversation's detail below the cover as four sections under a sticky strip
of jump links: About, Steps, Questions, Your data. Two things about that have not held up.

**The strip lies about what it is.** It looks like four tabs and behaves like four anchors,
so tapping a chip starts a scroll that carries the reader through the other sections on the
way. On a phone the reader ends up somewhere between two headings with no sense of which
chip they are on. The strip also fixes the shape of the content at four sections, which is
the shape of our fields rather than the shape of anything an admin wants to say.

**Before you start is not reachable once you have joined.** It is the only screen that says
what the conversation is, how long it takes and what happens to your answers, and after the
first tap of the call to action there is no way back to it. ADR-0020 made the logo inside a
step open a dialog about leaving, because at that point leaving the site was the only thing
the logo could do. That framed a participant's question ("what was this about again?") as a
decision about abandoning the conversation.

## Decision

**1. Before you start is a deck. One chip is one page, and one page is one screen.** Each
page takes at least a viewport, so a chip lands on a screen rather than partway through a
wall of text, and the reader always knows which page they are on.

The pages stay in the ordinary scroll rather than being mounted one at a time. Scrolling
down walks them in order, the lit chip follows the reader, and swiping is the same move as
scrolling to the next page (ADR-0018). The strip is the shortcut, not the only way through.

Rejected: showing one page at a time and nothing else. It makes the deck a dead end for a
reader who simply keeps scrolling, and it throws away the navigation the phone already has.

**2. The pages are authored, not fixed.** The description splits at horizontal rules into as
many pages as it needs, the same break the step brief splits slides at (ADR-0017), so this
costs no new field and no new editor. A page that opens with a heading is named by that
heading, in the chip and on the page. A page without one falls back to the section's default
name, which is what every conversation written before this change gets.

The FAQs and the privacy policy stay one page each. They are separate fields with their own
pages elsewhere in the product, and a rule inside a privacy policy is a divider rather than
a page break.

**3. The step list is generated, so it goes last.** It is the one page nobody writes: names
come from the workflow, durations from `TOOL_META.estimatedMinutes` (see CONTEXT.md,
Estimated time). Everything an admin wrote is read before the page written for them.

**4. Inside a step, the logo goes back to Before you start.** It is a move within the
conversation, not a way off the site, so there is nothing to confirm and no dialog. Before
you start also keeps its row in the step menu and its segment in the progress bar once a
participant has joined, so the bar does not lose a segment at Step one and the way back is
in the menu as well as under the mark.

Rejected: keeping the dialog and pointing it at Before you start. A confirmation earns its
place by protecting something losable, and this destination loses nothing.

**5. The step cover carries no back and no skip.** It is an instruction screen: it says what
the step is and asks for one decision, which is whether to go on. The desktop row of controls
ADR-0023 put above it competed with that, and offered a way out of a step to a participant
who had not yet seen what they were leaving. Both moves are still on the pager once the step
is open, where they act on a step the participant has actually met.

**6. A skipped step does not end on the completion screen.** Skip goes straight to the next
step. A check mark and "Step complete" over a step the participant chose not to do is a
congratulation for nothing, and it makes skipping cost the same two taps as finishing. The
progress write is unchanged: skipping still marks the step done, which is what it did when
the same button led through the completion screen.

## Consequences

- `StepLeaveDialog` is deleted. `StepChrome` takes `introUrl` in place of `returnUrl` and
  `anonymousId`.
- The link back into the workflow and, for an anonymous participant, the anonymous id they
  sign back in with, are no longer offered anywhere in the step chrome. That was the honest
  half of ADR-0020 and it now has no home. The landing page offers "Jump back in" to a
  participant in the same browser, which is not the same thing as being told how to come
  back from another one.
- The description is rendered through `ContentRenderer` rather than as a single paragraph,
  so its markdown is now live: headings, lists and rules mean what they say. Existing plain
  prose is unaffected.
- Two chips can end up with the same name when several pages are written without headings.
  Repeats are numbered rather than left ambiguous.
- The completion screen is now the end of a step that was done, not of every step. The
  `done` phase is reached from a tool finishing, never from Skip.
- `StepCoverNav` is deleted. An optional step can no longer be skipped from its cover, only
  from inside it.
- The conversation cover is unchanged. It is still one viewport ending in the call to
  action, and its one affordance now scrolls to the first page of the deck rather than to a
  wall of sections.
