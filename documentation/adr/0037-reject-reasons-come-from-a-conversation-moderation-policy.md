# ADR-0037: Reject reasons come from a per-conversation moderation policy in conversation metadata

**Status:** Proposal - to be discussed with the team
**Date:** 2026-09-14
**Amends:** ADR-0015 (builds the per-conversation reason list it deferred)

## Context

ADR-0015 made the reject reason an optional preset label from a hardcoded list of five,
stored as text in `polis_statement_aux.moderation_reason`, and deferred a per-conversation
list until a second client needed a different set.

#1080 asks for the policy team to write that list, so reasons follow the engagement's own
moderation policy and can be counted in the CSV export (#892, #899). The glossary already
has a pattern for per-conversation settings that need no schema change: a key in the
conversation's `metadata` jsonb, edited from a Configure sub-tab and written with
`PatchConversationMetadata`.

## Decision

Each conversation has a **moderation policy**: a list of reject reasons, each a label plus
an optional description of what counts under it.

- **Storage.** `metadata.moderation_policy = { reject_reasons: [{ label, description? }] }`.
  No migration. It sits on the conversation, not the step, so every moderated step shares
  one list. Polis is the only reader today.
- **The default is a template.** With nothing stored, the conversation uses
  `DEFAULT_REJECT_REASONS` (the ADR-0015 five, now with descriptions). The first edit saves
  the whole list onto the conversation. "Reset to default" stores `null`, so the
  conversation follows the code default again.
- **An empty list is allowed.** Moderators then only get the free-text note.
- **Editing** happens in Configure > Moderation policy and autosaves like the glossary
  (ADR-0006). Labels are trimmed and de-duplicated case-insensitively on save, because the
  label is both the key in the reason picker and the stored value.
- **The stored reason does not change.** A reject still writes `"Label: note"` into
  `moderation_reason`. Renaming or deleting a reason leaves recorded reasons alone.
- **Export splits it back out.** The Moderation tab's Download menu builds a CSV in the
  browser from the loaded rows: all, accepted, rejected or pending. `reject_reason` and
  `reject_note` come from splitting `moderation_reason` against the current labels plus the
  defaults. Text that doesn't start with a known label, such as the split flow's sentence,
  goes whole into `reject_note`.
- **Descriptions are for moderators.** The reason picker in the reject popover is a
  searchable list. Each reason shows its description under the label, and search matches
  descriptions as well as labels. Once a reason is picked, the field shows only the label.
  Participants see neither reasons nor descriptions.

## Considered options

- **Organisation-level list** (deferred): would suit a client running many engagements, but
  there is no organisation settings surface yet. The code default fills that role for now.
- **Policy on the Polis step's `tool_config`** (rejected): a conversation with two Polis
  steps would keep two copies, and other moderated tools couldn't share it.
- **Reason ids on the aux row** (rejected, as in ADR-0015): exact counts across renames, at
  the cost of a migration and a second shape for a column the split flow writes prose into.
- **Required before launch** (asked for in #1080, not built): ADR-0001 removed state-based
  gating from the admin UI, and blocking launch on this would add a new gate. Better as a
  warning in a pre-launch checklist alongside #1042, which is a product call.
- **Server-side export endpoint** (suggested in #892, not built): the Moderation tab already
  holds every statement for the step. Revisit if that list gets paginated.

## Consequences

- Counting by reason works while labels stay stable. After a rename, older rejections show
  the old wording in `reject_note` with an empty `reject_reason`.
- The conversation layout load reads the policy once, for both the editor and the
  Moderation tab. The editor invalidates `conversation:meta` when it unmounts, after any
  save still in flight has landed, so moving to the Moderation tab shows the new list
  without a reload.
- The glossary and moderation policy editors share one autosave helper. It runs saves one
  at a time, because each PATCH replaces the whole key and an older save landing last would
  win.
- `src/lib/utils/csv.ts` is now shared with the Insights export. It prefixes cells starting
  with `=`, `+`, `-` or `@` with an apostrophe so spreadsheets don't run participant text as
  formulas, and it writes a UTF-8 byte order mark so Excel reads accented text correctly.
- Still open: whether a reason is mandatory, a launch check, and whether reasons ever reach
  the participant (#1046, #1052).
