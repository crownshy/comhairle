# ADR-0015: A reject reason is an optional preset label stored as free text, not a structured code

**Status:** Proposal - to be discussed with the team
**Date:** 2026-08-13

## Context

When a moderator rejects a Pol.is statement there is nowhere to record *why* (#834). The
backend already carries the storage: `polis_statement_aux.moderation_reason` is an
`Option<String>` free-text column, and the single moderate endpoint already accepts an
optional `moderation_reason`. The #806 split flow also writes this column, storing a
human-readable sentence (`"Reworded/split by moderator into N statement(s)"`). So the
column exists and is already treated as free-text prose.

The originating client runs a moderation policy organised around seven rejection
categories they code A to G, kept as a hand-maintained rejection log in an external Google
Doc. The open question was what shape the reason should take in the product:

- free text only,
- a preset list of reasons, or
- a per-conversation configurable list.

And, if preset, whether to bake in the client's A to G letter codes or store readable
labels.

## Decision

The reject reason is an **optional preset label**, chosen from a small hardcoded list,
stored as a **human-readable string** in the existing `moderation_reason` free-text
column. No new column, no enum, no structured code.

- **Preset list (five, labels only):** Off-topic or unclear; Harmful or abusive;
  Advertising or campaigning; Privacy or personal info; Duplicate. These are the client's
  categories generalised, minus the ones that do not belong (see below). The list lives as
  a shared frontend constant.
- **Optional note.** A moderator may add a free-text note alongside a chosen label, or
  instead of one. When both are present they serialise into the one column as
  `"Label: note"`; chip-only stores the label; note-only stores the note.
- **Optional overall.** Rejecting with no reason is allowed; the capture UI (a popover on
  the reject control) never blocks a moderator who just wants the statement gone.
- **Reject only.** Accepts are not justified; a moderation log is about what is removed.
- **Accept clears the reason.** Because `update` treats `moderation_reason: None` as
  "leave unchanged", accepting a previously-rejected statement explicitly sets the reason
  to null, preserving the invariant "a reason describes why this statement is *currently*
  rejected."

Two categories from the client's policy are deliberately **not** in the list:

- **Multiple themes** is handled by the #806 split flow, which auto-rejects the original
  and writes its own reason. Offering it as a manual reject reason would invite a plain
  reject where a split is the correct action.
- **Illegal content** is folded into "Harmful or abusive". The genuinely-illegal-but-not-
  abusive cases (copyright, promoting illegal activity) are rare and covered by the note.

## Considered options

- **Free text only** (rejected): gives the client nothing consistent to build a log from;
  every moderator phrases the same reason differently.
- **Structured reason code column / enum, exposing A to G** (rejected): the column is
  already free-text prose (the split flow writes sentences into it), and a structured code
  would fork that. It also bakes one client's internal bookkeeping (the letters) into a
  shared tool. Readable labels stay generic, read cleanly in the UI and the eventual CSV,
  and the client can still map a label back to their A to G in their own log.
- **Per-conversation configurable reason list** (deferred): the flexible answer, but it
  needs config storage, a migration and an admin UI to manage the list. Not justified by a
  single client whose categories generalise well. Revisit if a second client needs a
  different set.

## Consequences

- **Frontend-led, small backend delta.** Capture and inline display are frontend. The one
  backend change is adding optional `moderation_reason` to the **batch** moderate request
  and threading it through, so bulk reject can apply one shared reason to all selected
  rows (single reject already supports it). Accept-clears-reason is a small tweak in the
  moderate handler.
- **The reason is internal.** It is shown to moderators inline on rejected rows, never to
  participants.
- **Export leans on this.** The follow-up CSV export (#892) reads `moderation_reason`
  directly as a readable string. Keeping it human-readable, rather than a code needing a
  lookup table, is what makes that export useful without extra machinery.
- **The preset list will drift.** It is a constant, so changing it is a code edit. That is
  the accepted cost of not building per-conversation configuration yet.
