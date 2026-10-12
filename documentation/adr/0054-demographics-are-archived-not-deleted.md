# ADR-0054: Published demographics are archived, not deleted

**Status:** proposed
**Date:** 2026-10-09

## Context

Published demographic questions and forms are referenced by conversation workflows, and by
participant responses. Hard-deleting them would break live conversations, and would remove
answers that participants gave.

## Decision

**1. Published Demographic questions and forms are never hard deleted.** They are archived.

**2. Archived items are hidden from pickers for new use.** Conversations that already use
them keep their pinned versions and continue to work.

**3. Draft items that were never published can be deleted.** Nothing references them.

## Consequences

- Admins cannot free a question's identifier or name. The UI should show archived items
  in a separate list so they can still be found and restored.
- Responses stay linked to their question versions, so reports over old conversations
  still work.
