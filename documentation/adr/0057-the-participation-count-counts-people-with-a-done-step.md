# ADR-0057: The participation count counts people with a done step

**Status:** proposed
**Date:** 2026-10-20

## Context

Issue #1434 shows participants "X people have taken part so far" on the thank-you page, or an
early-participant note below 15 people.

The first version counted distinct people in `user_participation`, which gets a row when
someone registers for a workflow. Registration happens before any step, so someone who signed
up and left without doing anything was counted as having taken part.

## Decision

**1. Someone has taken part once they have a `done` progress row for at least one step in any
workflow of the conversation.** Each person counts once, however many steps or workflows they
have done. `user_progress::count_people_who_have_taken_part` is the only place this is
computed.

**2. Registering alone does not count.**

Rejected: **counting only people who have finished.** Finished is derived from every step in a
workflow (see ADR-0016), so adding a step to a live workflow would drop everyone who had
finished out of the count until they did the new step.

## Consequences

- "Participant" now means two things in the API. The participation count uses taken part. The
  notification recipients, the email-updates gate and document access still use
  `user_participation::get_participant_user_ids_for_conversation`, which is everyone who
  registered. This decision leaves them as they are.
- Skipping an optional step writes `done`, so a person who only skipped counts as taken part.
- Anyone who reaches the thank-you page has finished, so they are always in the count, and
  "including you" stays true.
