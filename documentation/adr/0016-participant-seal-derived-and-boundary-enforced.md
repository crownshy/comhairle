# ADR-0016: The participant seal is derived, and enforced only at comhairle's boundary

**Status:** proposed
**Date:** 2026-08-17
**Issue:** [#860](https://github.com/crownshy/comhairle/issues/860)

## Context

Waves asked for a conversation-level option: once a participant reaches the Thank You
page, they can no longer return to any engagement step. The issue's acceptance criteria
require this to be "enforced at the backend/system level, not only through frontend UI",
and to survive browser Back and direct URL entry.

Two things in the existing model shape the answer.

**There is no participant-side finish state.** `Conversation.is_complete` means an admin
closed the whole conversation, not that a participant finished. Reaching the Thank You
page is currently derived on the fly in the frontend as "every `user_progress` row is
`done`" (`return/+page.ts`). Nothing is recorded when a participant lands there.

**Most participant data never passes through comhairle.** Of the five steps in the Waves
flow, three are HeyForm surveys rendered as a cross-origin iframe posting straight to
`forms.crown-shy.com`, and Polis steps talk to the Polis server from the browser.
Prioritisation is the only tool whose participant writes reach our API. So "block it at
the system level" is not uniformly available: for surveys and polls there is no comhairle
request to block.

See [CONTEXT.md](../../CONTEXT.md) for **Finished**, **Sealed**, **Revisit after
finishing** and **Revisitable step**.

A note on the name: "submission" was the first choice and was rejected, because
`tools/heyform.rs` already uses `Submission` for a single survey response. "Completion"
was rejected too, because `Conversation.is_complete` already means an admin closed the
whole conversation. "Finished" collides with neither, and matches the button participants
press on the last step.

## Decision

**1. Sealed is derived, never stored.** A participant is sealed when
`Conversation.allow_revisit_after_finishing` is false and every step in the workflow has a
`done` progress row for them. One Rust helper, `is_sealed(db, user_id, workflow_id)`, is
the only definition in the system. It is recomputed per check rather than stamped onto
`user_participation` as a `completed_at`.

Trade-off, accepted: the seal is not stable under workflow edits. Adding a step to a live
workflow creates `not_started` rows for existing participants
(`user_progress::create_for_workflow_participants`), which un-seals everyone who had
already finished and re-opens their earlier answers until they finish again. This fails
**open** on the guarantee Waves asked for, and it fails silently. We accepted it because
recording the seal buys stability at the cost of the opposite failure, where an admin adds
a step and several hundred sealed participants can never see it, and because it keeps this
change to zero schema. Revisit if adding steps to live workflows becomes routine.

**2. The backend decides; the frontend obeys.** `is_sealed` backs both a write gate on the
API and a `sealed` flag on `UserParticipationDto`, returned by
`GET /conversation/{id}/workflow/{id}/participation`. The participant-facing redirect is
the UX expression of a backend decision, not the enforcement itself. No sealed-ness is
computed client side. Deriving it in the frontend would have meant a second definition that
could drift from the one the write gates enforce, which is the failure this decision exists
to prevent.

Sealed is a property of a participant's relationship to a *workflow*, so it belongs on a
response that is keyed by both. The participation endpoint already is, already requires a
user, and is already fetched once in the conversation layout. Two homes were considered and
rejected. `LocalizedWorkflowStepWithProgressDto` (the steps list the workflow layout fetches
with `withUserProgress=true`) would repeat one workflow-level fact on every step and leave
the frontend reading it back off an arbitrary element of the list. `WorkflowDto` is also
what the admin create, get, update and delete handlers return, none of which have a
participant, so every one of them would have to serialize a `sealed` that means nothing.

**3. Enforcement stops at comhairle's boundary.** The write gate covers `SetUserProgress`
and `CreateProposalResponse`. HeyForm and Polis writes are out of reach without changing a
forked third-party service, and are explicitly not covered. Thinking space and audio
recordings are ungated for now.

Trade-off, accepted: after sealing, a participant who kept the raw iframe URL could still
submit a survey response. There is no path to that URL from the comhairle UI once the page
gate is in, and a second submission lands as a separate HeyForm response rather than
overwriting the first, so it is detectable at analysis time. Closing it properly means a
signed short-lived token validated by the HeyForm fork, which was judged too large for the
20 Aug milestone.

**4. The gate means "reject if *already* sealed".** The `SetUserProgress` write that marks
the final step `done` is the write that brings the seal into existence, so it is evaluated
against the pre-write state. A duplicate same-status write is an idempotent success, not a
rejection. Getting this backwards makes the flow uncompletable.

**5. The seal binds a participant identity, nothing stronger.** An anonymous participant
who clears cookies returns as a new identity with a clean seal, and their second pass
lands as a separate participant in the data. Hardening that is an identity question, not a
revisit question, and is out of scope.

**6. `allow_revisit_after_finishing` defaults to `true`.** Today the seal does not exist
and a `can_revisit` step stays reachable forever, so defaulting to `true` makes `is_sealed`
always false for existing conversations and reproduces current behaviour exactly. No
backfill.

## Consequences

- Revisit after finishing (conversation-level) and `can_revisit` (per-step) are orthogonal
  and both stay. The per-step flag governs navigation before they finish; the conversation
  setting overrides it after. Free navigation across steps 1-5, which the issue lists as an
  acceptance criterion, is therefore admin configuration rather than new code.
- The seal is recomputed from live state, so an admin toggling the setting takes effect
  immediately for participants who have already finished, in both directions.
- Every read site must consult the same flag. `s/[workflow_step_id]/+page.ts`,
  `thank_you/+page.ts` and `return/+page.ts` each filter on `canRevisit` independently
  today, and `/return` is the magic link emailed to participants, so a disagreement there
  is the visible one.
- The workflow layout reuses the participation row the conversation layout already fetched,
  which is the row for the conversation's first workflow. A conversation that ever runs a
  second workflow costs one extra request there rather than reading a seal for the wrong
  workflow.
- The write gate is scoped to step contribution writes, not to the participant. Thank You
  page writes (feedback, email registration, conversation preferences, account upgrade) and
  the public report route stay open to sealed participants, and the Waves prize-draw
  follow-up depends on that.
- The gate is `!preview` guarded, so an admin previewing a live conversation they have
  participated in is not sealed out of their own preview.
- Three gaps are known and deliberate: survey and Polis writes, ungated thinking space and
  audio recordings, and anonymous re-entry.
