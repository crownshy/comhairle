# ADR-0045: Room display surfaces share state over a BroadcastChannel

**Status:** proposed
**Date:** 2026-09-25
**Branch:** demo branch

## Context

The "Wall and laptop" layout of the Room display (ADR-0040 for the map it draws) is two
surfaces: a calm wall the room reads from eight metres, and a dense console the
facilitator drives from half a metre. They were built as two halves of one page so the
pair could be judged in one screenshot, with a "Hide console" button so the wall could
be judged alone.

That does not survive contact with a room. A facilitator mirrors a laptop to a projector
and the audience sees the console. The alternative, two browser windows of the same
page, meant the wall did not know which statement the console had hovered or which group
it had picked, because that state lived inside one component.

Real use is one laptop with a projector as its second screen. Two computers driving one
wall is not a case anyone has asked for.

## Decision

The console layout takes a `?surface=` parameter. `wall` renders only the wall, with no
frame or caption, and a faint control to front the console window. `console` renders only
the console, with a button to open the wall in a new window. Absent, both halves share the
page as before, and the console header gains "Open in new window", which opens the console
and turns the current page into the wall.

The two windows stay in step over a `BroadcastChannel` named for the workflow step
(`src/lib/room-display/surfaces.ts`). What travels is the console state (focused statement,
what the wall's main area shows) and the board itself, so a size or block changed on the
laptop lands on the wall. A window that opens says hello; any window holding a console
answers with its state and board, so the wall can be opened after the console is set up.
The state moved out of the layout component into the page, which owns the wire.

Windows are opened by name, so asking twice fronts the existing window. The board's URL
rewrite keeps `surface`, so a reload of either window comes back as the same half.

## Consequences

- No server involvement. A `BroadcastChannel` reaches every window of this origin on
  the same machine and nothing else, which is exactly the two-window case.
- Two machines cannot pair. That would need a socket through the API and is a separate
  decision if it is ever wanted; nothing here would be thrown away for it.
- A board that arrives over the channel is applied without being sent back, or two
  windows would echo it at each other. Any window can still edit the board from its
  settings panel; the edit reaches the other.
- Where `BroadcastChannel` is missing (server render, a locked-down kiosk browser) the
  link is inert and the one-page layout works as it did.
- In demo mode each window runs its own scripted clock, so a wall opened a minute after
  the console is a minute behind it in the scenario. Live mode polls the same report
  and does not have this problem.
