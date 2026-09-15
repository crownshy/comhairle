# ADR-0038: Moderation policies are database rows that a Polis step points at

**Status:** Proposal - to be discussed with the team
**Date:** 2026-09-15
**Amends:** ADR-0037 (replaces its storage in conversation metadata), ADR-0015

## Context

ADR-0037 (#1164) stores a conversation's reject reasons under
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
  Launch copies the id from the preview config to the live config.
- **No policy means the defaults.** A step with no `moderation_policy_id` uses
  `DEFAULT_REASONS`, defined once in `api/src/models/moderation_policy.rs` and served by
  `GET /conversation/{conversation_id}/moderation_policies/default`. Creating a policy
  without reasons copies the defaults in. Clearing the step's `moderation_policy_id` goes
  back to the defaults.
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
  rules ADR-0037 put in the editor. Uniqueness is checked in Rust, not with a unique index,
  because a save that swaps two labels would trip the index partway through.
- **The step's policy id is checked on save.** The id sits inside the tool config jsonb, so
  no foreign key guards it. Updating a workflow step returns 400 if its
  `moderation_policy_id` isn't one of the conversation's policies. Deleting a policy
  returns 409 while any step's preview or live config points at it.
- **The stored reason does not change.** A reject still writes `"Label: note"` into
  `polis_statement_aux.moderation_reason` (ADR-0015). Renaming or deleting a reason leaves
  recorded reasons alone.

## Considered options

- **Keep the policy in conversation metadata** (ADR-0037, replaced): no migration, but no
  schema or server-side validation, and one policy per conversation.
- **Organisation-owned policies** (deferred): reuse across conversations, but it needs
  organisation-level permission checks and there is no organisation settings screen yet.
- **Create a policy row with every Polis step** (rejected): each step would always point at
  a real row, but existing steps would need a backfill, and most conversations would store
  copies of the same five reasons.
- **Reason ids on `polis_statement_aux`** (still deferred, as in ADR-0015): stable reason
  ids leave the door open.

## Consequences

- #1164 has to switch to these routes, drop `metadata.moderation_policy`, and drop its copy
  of the default reasons. The metadata key never shipped, so there is no data to migrate.
- The Configure editor now edits a policy that a step points at. Picking or sharing a
  policy per step has no UI yet.
- The delete guard and the step check are separate queries, so a step saved during a
  delete can end up pointing at a missing policy. A client that gets a 404 for a step's
  policy should fall back to the defaults.
- After launch, the preview and live configs are saved separately, like the other Polis
  step settings. Pointing a live step at a different policy updates `tool_config`.
