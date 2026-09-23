# Room display: notes

## Where it lives

`/conversations/<conversation_id>/room-display/<workflow_step_id>`, public and
chrome-free (the `(public)` layout drops the nav and footer for any `room-display`
path). Linked from the step's admin Insights tab. Two sources behind one display:

- default: **live**. Polls the step's `PolisGetReportData` every few seconds
  (`liveSource.svelte.ts`). Participants come with a PCA position and a group id, so
  the map, group labels, counts, strip and group statements all work. What the report
  does _not_ carry is a per-participant vote matrix, so a live wall cannot colour
  individual dots by one statement; it shows that statement's group bars instead.
  That endpoint is the one piece of backend work this needs (CONTEXT.md,
  "Cross-highlight").
- `?mode=demo`: the scripted scenario with animated joins and votes, for showing the
  thing off without a room. Transport controls appear in dev builds.

## The board

What the display shows is a **board**: a layout plus a set of blocks (CONTEXT.md,
"Block / Layout / Board"). `blocks.ts` holds the vocabulary and the precedence rule;
`BoardSettings.svelte` is the panel on the display that edits it.

- `?variant=console|marquee|deck` names a board. Console is the default and is the
  direction the team picked below.
- `?layout=split|console|deck` and `?blocks=a,b,c` say one exactly. Either of them
  present pins the board, so a link reproduces the sender's screen.
- With no board parameter at all, the display comes back to whatever this machine was
  last left showing (`storedBoard.ts`). A projector that reboots ten minutes before
  the room arrives should not come back to the factory default.
- The settings panel rewrites the URL as you toggle, and drops `variant`: once a block
  has been touched by hand the preset name is no longer true.

Why the panel is on the display rather than in admin: the question "is the map landing
with this group?" is asked in the room, with the room watching. A round trip through a
settings page means the answer is always "leave it".

The deck is the loosest fit for the vocabulary. Its five slides are five blocks, so
switching one off removes a slide, but it has no facilitator controls for `groups` to
address and its question and counts live inside the join slide rather than being
separately switchable. The settings panel says so rather than showing a switch that
does nothing.

The recruitment screen is not the board: before Polis clusters there is nothing to
arrange, so `WarmingScreen` ignores the block set.

## Latest statements: marquee or still?

**Question:** the `marquee` block started as a horizontal ticker, statements scrolling
continuously along the bottom of the wall. It looked alive and read badly.

**The argument against scrolling:** you cannot read moving text across a room. The eye
has to acquire a moving target and then track it, and at eight metres it loses before
the sentence finishes. That is the same failure the whole display was built to avoid,
reintroduced at the bottom of the screen. Slowing it down does not fix it, because the
problem is the motion rather than the speed: a slower scroll just means fewer
statements go past while still being unreadable.

**What replaced it:** motion on arrival and nowhere else. A new statement animates in
at the head, the rest slide along, the oldest drops off, and then everything is
completely still until the next one. This is exactly the **accent** CONTEXT.md already
describes: in-place, non-blocking, never taking the screen. Between arrivals the room
is reading type that is not moving, which is the only way it gets read at all.

Recency is carried by an opacity ramp rather than by movement. Oldest is faintest, and
that says "this is a running list, newest first" without anything having to move.

**Four presentations, one block.** `?latest=` picks:

| value     | what it is                                  | where it suits                 |
| --------- | ------------------------------------------- | ------------------------------ |
| `row`     | still, newest leftmost, 4 across (default)  | the wide band along the bottom |
| `column`  | still, newest at top, 3 down                | the bottom band, given height  |
| `aside`   | still, in the column under the strip        | `split` only                   |
| `marquee` | the original continuous scroll, much slower | the thing being argued against |

`aside` says _where_ rather than _how_, which is why it is a fourth value of the same
setting rather than a second setting: a row of four in a third of the wall's width
would be four slivers, so what goes beside the strip can only be a column. It exists
because the strip is a fixed-height plot, so in a full-height column it leaves dead
space under itself, and that space is the right size for what was just said. Only
`split` has that column; elsewhere `aside` falls back to a column along the bottom and
the settings panel does not offer it.

How many fit there depends on the screen: two on a projector, one on a laptop window,
because a second cut off halfway down its box reads as broken rather than as "there is
more". That is the one place the display measures the viewport.

The marquee is kept rather than deleted so the claim above can lose in an actual room
instead of winning in a review. It is slowed right down, which is the most generous
version of the idea. If it still loses in front of an audience, delete
`LatestStatementsMarquee.svelte`.

**Known cost of `column`:** in the `split` layout it competes with the opinion map for
height, and three statements is enough to squash the map noticeably. `row` is the right
default there. Column is waiting for a layout that gives the block a real column.

**A second trap:** the statement strip's dots are SVG `<circle>`s with `tabindex`, and
Chrome draws a focus ring on an SVG shape as its _bounding box_, so a focused dot got a
square around a circle. `focus-visible:outline-none` did not catch it, because a click
counts as `:focus` without counting as `:focus-visible`. It is now `outline-none`
outright, which costs the keyboard nothing: focus already moves the selection, so the
focused dot grows and turns primary of its own accord.

**A trap worth remembering:** the list uses `animate:flip`, and Svelte drives that with
a generated keyframe on the element's own `animation` property. Putting our own CSS
`animation` on that same element left permanent stuck transforms (items frozen at
`scale(0.24, 2.12)`) whenever the layout changed under it. The arrival animation lives
on a box inside the flipped element for that reason. The list is also keyed on the
direction, so switching presentation rebuilds rather than flying every item from its
old position to its new one.

## Lighting

`?theme=light|dark`, or `auto` to leave the app's own setting alone. The display cannot
work this out for itself: the same projector is unreadable dark in a bright hall and
glaring light in a dim one, and that is a property of the room rather than of the
viewer.

It drives the app-wide `themeStore` rather than scoping itself, because dark is a
`.dark` class on `<html>` and themed deployments key off `[data-theme=x].dark` on that
same element. There is no way to scope it to this page without breaking the theme on
every branded deployment. On a projector that is the right trade. `auto` therefore does
nothing rather than restoring a previous mode, so merely opening this page never
overrides someone's own preference.

Below is the record of how the direction was chosen.

## Which direction?

**Question:** the one-screen board is too busy. Three regions (opinion map, consensus
continuum with its vote block, and a twelve-item statement ticker) compete for
attention, at desk scale, on a surface meant to be read from across a room. What should
the Room display actually be?

**Shape:** four variants on this route, switchable with `?variant=`. Same scripted
scenario behind all of them, so they are judged on arrangement rather than on data. The
driver controls live in the floating bar so no variant has to carry them.

| `?variant=` | Direction | Who interprets  | The bet it makes                                              |
| ----------- | --------- | --------------- | ------------------------------------------------------------- |
| `board`     | Board     | The room        | The board works once it is cut to two regions and scaled up.  |
| `deck`      | Deck      | The facilitator | Five fixed slides with live content, advanced by hand.        |
| `narrator`  | Narrator  | The machine     | The same slides, advanced by a timer, so nobody has to drive. |
| `console`   | (new)     | Split           | The busy-ness belongs on a laptop, not on the wall.           |

(`board` and `narrator` are gone; `board` came back later as the `split` layout, which
is the one-screen bet again with the statement marquee and the controls moved to the
bottom.)

`board`, `deck` and `narrator` are the three directions CONTEXT.md already names.
`console` is a fourth: it says the board is busy because one surface is doing two jobs,
and splits it into a calm wall and a dense facilitator panel.

## What each variant throws away

- **board** loses the ticker column and the continuum's card chrome. It keeps
  cross-highlight as the one thing a facilitator touches.
- **deck** loses ambient operation entirely. Nothing moves unless someone advances it,
  which is a real cost when the facilitator is talking rather than clicking.
- **narrator** loses facilitator control. The rotation is `ambientFocus.ts`, which was
  already written and tested but unused.
- **console** loses the single-surface property. Two surfaces means the wall and the
  laptop have to share focus state, which is a build problem the prototype ducks by
  showing them side by side in one page.

## Open questions the variants surface

- The report's `ConsensusContinuum` is a desk component: card, heading, subtitle, 5px
  dots, 12px axis labels. `StatementStrip.svelte` here is the room-scale answer. Does
  the real continuum grow a room mode, or does the Room display own a separate strip?
- Moments (`moments.ts`) are still computed and still not rendered anywhere. Deck and
  Narrator both have an obvious slot for them; Board does not.
- The Deck's arrow keys are the affordance under test (a presentation clicker sends
  them), which is why the prototype bar cycles variants on `[` and `]` instead.

## Verdict

**Console won.** Board and Narrator are deleted (in git history if wanted). Deck stays
as the alternative for a facilitator who would rather present than drive, and because
its arrow-key advance is still the affordance a clicker sends.

Folded in from the review of the videos:

- The strip shows one statement at a time: hover a dot and it appears under the strip
  on the console, and the wall recolours the map by it.
- Opinion groups are listed on the console with live head counts. Picking one swaps
  the wall's map for that group's key statements with live vote bars
  (`liveVotes.ts`, folded from the vote stream, not read off the report comment).
  Picking it again brings the map back. "Consensus statements" does the same with the
  statements ranked by `group_informed_consensus`.
- Group labels sit under each cluster on the map and follow it (`groupCentroids`).
- The QR code stays in the wall's corner all session for latecomers.
- Text stepped up throughout: the wall was unreadable from two metres on a TV.
- The console collapses so the wall can be judged on its own.

## Still open

- **Which blocks anyone actually turns off.** The panel exists partly to find out.
  If the answer after a few rooms is "nobody touches it", the board belongs in the
  step's config where it is set once, not on the display.
- **Density.** Hundreds or thousands of participants will overlap. Polis caps base
  clusters at 100 and `memberCount` is already carried, so the next step is sizing
  dots by members and drawing a split (pie or ring) for the cross-highlight, not a
  second component. Deferred until the roundtable-scale version has run.
- **Phone view.** Scan the QR, vote from the phone, and get a participant-facing live
  view at the end. "Follow presenter" versus "explore on my own" is the switch to
  design. Not started.
- **Two real surfaces.** Console and wall are one page here. Real use is one laptop
  driving one projector, so focus and wall view need to travel between two windows
  (BroadcastChannel is the cheap answer for one machine, a socket for two).
- **Per-participant votes endpoint.** Without it the live wall has no cross-highlight
  and no "N more voters" countdown. The raw material (`/api/v3/votes?pid=`, base
  clusters) is already reachable server-side.
- **Join URL.** The QR defaults to the conversation page; pass `?join=` with an open
  invite URL until the display can look one up itself.
- **Events product.** Whether this is a separate Slido-style product is a positioning
  question, not a build one. Build the basic version first.
- `moments.ts` is still computed and rendered nowhere. The console has a slot for it
  (the statement box); the wall does not, deliberately. It is the obvious next block.
