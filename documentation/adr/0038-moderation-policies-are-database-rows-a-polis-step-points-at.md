# ADR-0038: Moderation policies are database rows that a Polis step points at

**Status:** Proposal - to be discussed with the team
**Date:** 2026-09-15
**Supersedes:** ADR-0037 (moves the policy out of conversation metadata, and a Polis step now
picks one)
**Amends:** ADR-0015 (the reason list comes from a policy row)

## Context

ADR-0037 (#1164) stored a conversation's reject reasons under
`conversation.metadata.moderation_policy`, with the five default reasons as a frontend
constant. #1167 asks for moderation to be modelled in the API instead. A jsonb key on the
conversation has no schema, no server-side validation, and no way to refer to a single
reason.

The team also wants a policy per step, with the option to reuse one policy across steps.

## Decision

- **Two tables.** `moderation_policy` is the policy: a name, owned by one conversation
  (`conversation_id`, deleted with it). `moderation_policy_reason` holds each reason:
  `label`, optional `description`, and `position` for display order.
- **A Polis step points at a policy.** `PolisToolConfig` gets
  `moderation_policy_id: Option<Uuid>`. Steps in the same conversation can share a policy.
  Launch copies the id from the preview config to the live config. Only Polis gets the
  field, since it is the only tool that moderates. ADR-0037 kept the policy off the step so
  two steps wouldn't hold two copies and other tools could share it. Here the step holds
  only an id and the policy belongs to the conversation, so two steps share one row, and a
  tool that moderates later adds the same field and picks from the same policies.
- **The defaults live in the API.** `DEFAULT_REASONS` is defined once in
  `api/src/models/moderation_policy.rs` and served by
  `GET /conversation/{conversation_id}/moderation_policies/default`. Creating a policy
  without reasons copies the defaults in.
- **Routes.** `/conversation/{conversation_id}/moderation_policies` lists and creates.
  `/{moderation_policy_id}` gets, replaces and deletes. Every route checks
  `ConversationUpdate` on the conversation, the same check as moderating a statement, so
  content editors can read and edit policies.
- **Saving replaces the whole list.** `PUT` takes the name and every reason in order. A
  reason sent with its `id` is updated in place, one without an id is added, and one left
  out is deleted. Reason ids stay stable across saves, so a later column on
  `polis_statement_aux` could point at one.
- **The API validates labels.** Labels are trimmed, and a blank label, a label containing
  `": "`, or a label repeated within the policy (ignoring case) gets a 400. These are the
  rules ADR-0037 put in the editor. Rust checks them first so the error is a clear 400. The
  database backs this up with constraints on `(moderation_policy_id, position)` and on
  `(moderation_policy_id, lower(label))`, both deferred to commit so a save that swaps two
  reasons doesn't trip them partway through. The label one is an exclusion constraint,
  because a unique index on `lower(label)` can't be deferred.
- **The step's policy id is checked on save.** The id sits inside the tool config jsonb, so
  no foreign key guards it. Updating a workflow step returns 400 if its
  `moderation_policy_id` isn't one of the policies in the step's own conversation, whatever
  conversation id is in the URL. Deleting a policy returns 409 while any step's preview or
  live config points at it, and 404 for another conversation's policy whether or not it's
  in use. The step save takes `FOR KEY SHARE` on the policy row in the transaction that
  writes the step, and delete takes `FOR UPDATE` before counting steps, so whichever runs
  second waits and a step can't end up pointing at a deleted policy.
- **The admin UI edits one policy per conversation for now.** Configure > Moderation policy
  edits a single policy. The first edit creates it and points every Polis step at it, and
  each later save points any Polis step added since. Reset to default points the steps
  back at nothing and deletes the policy. The API already allows several policies per
  conversation, for a per-step picker later.
- **Where a step's reasons come from.** The Moderation tab and the CSV export use the
  policy the step points at. A step that points at nothing uses the conversation's first
  policy, and a conversation with no policy uses `DEFAULT_REASONS`. The middle rule covers a
  Polis step added after the policy was saved.
- **The stored reason does not change.** A reject still writes `"Label: note"` into
  `polis_statement_aux.moderation_reason` (ADR-0015). Renaming or deleting a reason leaves
  recorded reasons alone.

## Considered options

- **Keep the policy in conversation metadata** (ADR-0037, replaced): no migration, but no
  schema or server-side validation, and one policy per conversation.
- **A policy per Polis step in the admin UI** (deferred): what the tables are shaped for,
  but it needs a picker or editor on each step. One policy edited in Configure keeps the
  #1164 screens as they are.
- **Organisation-owned policies** (deferred): reuse across conversations, but it needs
  organisation-level permission checks and there is no organisation settings screen yet.
- **Create a policy row with every Polis step** (rejected): each step would always point at
  a real row, but existing steps would need a backfill, and most conversations would store
  copies of the same five reasons.
- **Reason ids on `polis_statement_aux`** (still deferred, as in ADR-0015): stable reason
  ids leave the door open.

## Consequences

- The #1164 editor, reject popover and CSV export read and write through these routes.
  `metadata.moderation_policy` is no longer read. The key reached staging but never
  production, and nobody saved a policy with it on staging, so there is no data to migrate.
- The frontend no longer keeps its own copy of the default reasons. The conversation
  layout load fetches them with the policies.
- Pointing steps at the policy is an extra step update per Polis step, but only on the save
  that creates the policy or that finds a newly added step.
- A step save that points at a policy holds a row lock on it until the save commits, so a
  delete of that policy at the same moment waits for the save and then gets a 409.
- After launch, the preview and live configs are saved separately, like the other Polis
  step settings. The editor points both at the policy.
