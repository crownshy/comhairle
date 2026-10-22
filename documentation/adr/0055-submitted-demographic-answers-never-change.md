# ADR-0055: Submitted demographic answers never change

**Status:** proposed
**Date:** 2026-10-16

## Context

Reports and exports count the demographic answers given in a conversation. If a participant
could overwrite or delete an answer later, the results of a finished conversation would
change after the fact.

## Decision

**1. A submitted answer is never edited or deleted.** The database refuses updates to its
value and question version.

**2. An answer belongs to the conversation it was given in.** A user has one answer per
question version and conversation. Answering again in another conversation, including by
confirming an auto-filled value, stores a new answer there. Reports and exports read only the
answers given in their own conversation.

**3. Submitting the same answer again is accepted, a different one is refused.**

**4. Users may anonymise an answer.** This clears its user, so it is no longer linked to
them or suggested as an auto-fill. The value and conversation stay.

## Consequences

- The auto-fill value for a question is the user's latest answer to that exact version.
- Anonymised answers no longer match a participant, so they drop out of reports and exports
  that are built per participant.
- Answers without a conversation (signup defaults, migrated answers) feed auto-fill but do
  not appear in any conversation's report.
