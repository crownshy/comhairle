# ADR-0040: The live opinion map deals out votes it cannot look up

**Status:** proposed
**Date:** 2026-09-23
**Branch:** demo branch

## Context

Cross-highlight recolours every dot on the opinion map by how that participant voted on the
selected statement. The scripted scenario carries a vote per participant per statement, so
the prototype has the matrix. A live Polis step does not: `report_data` gives a participant
a `pid`, a `group_id` and a PCA position, and nothing else. Vote counts arrive per statement
per group.

Two ways to get the real matrix:

- `/api/v3/votes?conversation_id=X&pid=N`, one request per participant. The display polls
  every 8 seconds, so a 100-person room is 100 requests every 8 seconds, against a Polis we
  run ourselves and have watched fall over.
- The participant-votes CSV export, one request for the whole matrix. Polis generates it from
  the same math job that feeds the report, it is sized for a download rather than a poll, and
  it would need a new backend route to proxy.

The room does not read individual dots anyway. From eight metres a projected map is a shape:
which cluster went green, how much of it, how the other cluster answered. That is a
proportion, and the report already carries every proportion the map can show.

## Decision

**The live map deals each group's real counts across that group's dots.** Group B cast 9
agrees, 2 disagrees and 1 pass, so 9, 2 and 1 of Group B's dots take those colours, and the
rest stay in the not-voted grey. Dots outside any group take whatever `overall_votes` has
left after the groups, which is also what the whole map runs on before Polis has clustered.
`apportionedVotes.ts` builds the matrix; `liveReport.ts` puts it where the scripted matrix
used to go, so the components take it unchanged.

Exact per group, exact between groups, exact overall. Arbitrary in one place only: which dot
took which colour.

Largest-remainder apportionment, so the colours sum to the dots present. When Polis has
placed every member the seats are the counts themselves; when it has placed fewer, the group
keeps its proportions and the dots round.

The deal order is an integer hash of `(tid, pid)`, not a random draw, which buys two things a
polled wall needs. A poll that changes nothing repaints the identical map. And because the
colours are handed out in blocks along a stable ranking, one new vote nudges each boundary
along by one dot, so at most one dot per colour changes hands however big the group.

Keying the order on the statement as well as the participant means a dot is not the same
voter from one statement to the next. A dot that agreed with everything would look like a
person who agrees with everything, and no such person is in the data.

**The numbers beside the map stay exact.** `voteBarsFor` reads the vote bars off the report
comment, not off the dots, so nothing a facilitator can quote comes from a rounded dot.

**`RoomDisplaySource.perParticipantVotes` becomes `voteMatrix: 'per-participant' |
'apportioned'`.** The old boolean gated three unrelated things behind "live cannot do this".
Two of them still need gating and the third does not: cross-highlight now works on both
sources, the settle animation still needs real per-person vote counts, and the bars still
come from the payload when the matrix is apportioned.

Rejected: **per-pid vote requests.** 100 participants times a poll every 8 seconds, for a
wall that shows a shape.

Rejected: **the CSV export.** A new backend route, a parser, and a second data path to keep
in step with the report, to draw the same shape the report already describes.

## Consequences

- A dot's colour is not that person's ballot. Copy around the map says how the room split,
  never how anyone voted. The deck slide that reads "Every dot is a person, coloured by how
  they voted on this one" is gated to the scripted source and must stay gated.
- Nobody in the room can find their own vote on the wall, which is the better default at a
  28-person table: a real vote pinned to a fixed dot is something a neighbour can watch.
- The countdown to the next reveal stage counts voters, which an apportioned matrix cannot
  tell you. It stays off for a live source.
- If a per-participant endpoint ever lands, it fills `votesByTid` with real votes and sets
  `voteMatrix: 'per-participant'`. Nothing else changes.
