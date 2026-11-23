# ADR-0072: Hosts edit only upcoming agenda items during a call

**Status:** proposed
**Date:** 2026-11-17

## Context

Hosts (moderators and facilitators) wanted to change an event's agenda while the call is
running: add a breakout on the spot, or build the rest of the agenda with the room
(issue #611). The agenda could only be edited from the admin event page, which needs a
platform admin and knows nothing about where the call has got to. The call's progress (the
current agenda step, who is a host) lives in memory in the API instance holding the hosts'
websocket connections, not in the database.

## Decision

**1. Only items after the current step can change.** The current item and everything before
it are locked. The server keeps those items exactly as they are in the database and replaces
only the rest, so a running breakout or a finished item can't be edited or removed, whatever
the client sends.

**2. The edit goes over the call's websocket, not the events API.** A host sends
`video_call:update_upcoming_agenda` with the items to put after the current step and an
`edit_id`. The handler checks the sender is a host on the call, reads the current step from the
call state, writes the event's agenda, and broadcasts `video_call:agenda_updated` to everyone on
the call, which clients already handle. Any host can edit, platform admin or not.

The sending host always gets one `video_call:agenda_update_result` with their `edit_id` and an
outcome: `saved` (with the new agenda), `step_moved`, `save_failed` or `not_host`. The broadcast
can't say whose edit landed, since another host's save looks the same, so the sidebar waits for
this reply instead.

**3. An edit made from an out-of-date step is refused.** The message carries the step the host
was editing from. If the call has moved on, the server answers `step_moved` and saves nothing,
so an edit can't rewrite an item that started while the host was typing. Edits and step changes
share a lock on the API instance, so the step can't move between the check and the save. The
sidebar keeps the host's changes when an edit is refused or fails, including while a breakout
panel covers it.

**4. The sidebar edits titles, breakout questions, duration and room size.** Descriptions and
breakout instructions are kept as saved; they're still edited on the admin event page.

## Consequences

- Two hosts editing at once: the last save wins for the upcoming items. Nothing merges.
- The admin event page doesn't hear about live edits. Saving its agenda during a call
  overwrites them (and can change locked items, since that route doesn't know the step).
- Room size for pre-assigned breakouts still comes from the first breakout item, so adding or
  editing an earlier breakout can change it.
- If no reply arrives within ten seconds, the sidebar reports a failed save even though the
  edit may still land.
- The lock is one per API instance, not per event. Saves are short and rare, so a step change
  on another call waiting behind one is acceptable.
- After `step_moved` the host can only start over from the new step. Their edits stay on
  screen to copy from, but can't be saved as they are.
- Editing is in the desktop sidebar only. The mobile facilitator sheet still just steps through
  the agenda.
