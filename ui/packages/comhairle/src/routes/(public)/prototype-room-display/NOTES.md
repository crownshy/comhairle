# Room display prototype: which direction?

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

_Not decided yet._ Fill in which variant won and why, then delete the losers, the
switcher, and this file.
