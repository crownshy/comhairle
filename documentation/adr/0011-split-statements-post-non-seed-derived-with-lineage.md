# ADR-0011: Splitting a participant statement posts non-seed, auto-accepted, derived statements with lineage

**Status:** Proposal - to be discussed with the team
**Date:** 2026-08-07

## Context

When a policy team moderates a Pol.is poll, a participant sometimes submits a **composite
statement** that bundles two ideas, or an ambiguously worded one. Standard practice is to **split**
it into clean, separately votable statements, or to **reword** a single ambiguous one. We have no
way to do either today.

The only "admin adds a statement" path is `post_seed_comment`
([polis_service.rs](../../api/src/wiki_poll_service/polis_service.rs)), which hardcodes
`is_seed: true`. So the naive split (add the replacements through existing tooling) lands them as
**seed statements**, misrepresenting participant-derived content as host-authored. That is the exact
thing we want to avoid: it is a provenance lie to participants and to anyone reading the data later.

We probed the live Pol.is (`polis.comhairle.scot`) to settle what was untested. Findings:

- Pol.is **honours `is_seed: false`** on a host-posted comment. A host can author a genuine
  non-seed statement. Our transparency preference is achievable natively, no workaround needed.
- The host post resolves `"pid": "mypid"` to the admin's participant id (0 in a fresh poll). Not a
  magic "pid 0" convention, just the admin's participant slot.
- **`is_seed` silently controls auto-approval.** A seed posts as `mod: 1` (accepted, live
  immediately); a non-seed host post lands `mod: 0` (**pending**). So posting a proper non-seed
  replacement drops it into the moderation queue instead of making it votable.
- Pol.is has **no edit-in-place**. A "reword" is unavoidably a new statement plus a rejection of the
  old one, not a mutation.

`polis_statement_aux` already carries the admin-only sidecar metadata (`is_seed`,
`moderation_status`, `moderation_reason`, `themes`, `visible_statement_when_submitted`) but has **no
lineage column**, so a derived statement cannot currently be told apart from a raw participant one.

## Decision

A **split** (one operation, whether it produces one replacement or several) does, server-side, in
this order:

1. **Post** each replacement to the active poll with `is_seed: false`. They are real, votable, and
   never flagged as host seeds.
2. **Auto-accept** each replacement (`moderate_comment(Accept)`, `mod: 1`) so they are immediately
   votable rather than left dangling in the pending queue. The admin authored them deliberately;
   making them re-approve their own text is busywork.
3. **Reject** the original composite statement (`moderate_comment(Reject)`, `mod: -1`) and set its
   `moderation_reason` to a system note recording the split.
4. **Record lineage.** Each replacement's aux row gets a new self-referential
   `original_statement_id UUID REFERENCES polis_statement_aux(id)` pointing at the original. The
   discriminator is: `is_seed: false` **and** `original_statement_id` set means **derived**;
   `is_seed: true` means **seed**; neither means **raw participant**.

Because lineage cannot round-trip through Pol.is (Pol.is has no such concept), the split endpoint
**writes the derived aux rows itself** from the tids Pol.is returns, rather than posting and then
re-syncing the way the seed dialog does. Sync preserves `original_statement_id` as a comhairle-only
field, the same way it already preserves `themes` and `moderation_reason`.

To participants a derived statement is indistinguishable from any other accepted statement, which is
correct: it is a real statement now, not a host seed, and it carries no "conversation starter" label.

## Considered options

- **Seed the replacements** (rejected): simplest (auto-approved, one call each), but it is the
  provenance lie the whole feature exists to prevent.
- **Post non-seed and leave them pending** (rejected): honest, but every split litters the mod queue
  with statements the admin already approved by authoring them. Reads as a bug.
- **Edit the statement in place** (impossible): Pol.is exposes no edit. Reword is modelled as
  split-of-one for this reason.
- **Point lineage at the Pol.is tid instead of our aux UUID** (rejected): tids are only unique per
  conversation and carry no lineage; the aux UUID is our stable key and lets the UI self-join to
  show the original's text and its `visible_statement_when_submitted` context for free.

## Consequences

- **The split is not atomic** (Pol.is has no transactions). We sequence to fail safe: post and accept
  all replacements first, reject the original **last**, so we never destroy the source before its
  replacements exist. On partial failure we surface what completed rather than fake a rollback;
  reject is idempotent, so retry is safe. Worst residual case is an orphan accepted replacement plus
  a still-live original, both visible and fixable, never a lost statement.
- **One migration** adds `original_statement_id`; the model plumbing (`to_values`,
  `upsert_from_polis` preservation) follows the existing partial-update convention.
- **Provisional.** This is an exploratory PR off the spike (#806) to give the team something
  concrete to react to before the design is settled. The admin-facing labelling ("Edited" badge,
  original linked to its replacements) and the separate question of an optional reason on a plain
  reject (#834) may move.
