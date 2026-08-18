# ADR-0015: The prioritization review gate defaults to all proposals

**Status:** accepted
**Date:** 2026-08-17

## Context

[Issue #856](https://github.com/crown-shy/comhairle/issues/856) asked for a prioritization step
that participants do not have to complete in full: a configurable **minimum number of proposals a
participant must review** before the step's "Next" unlocks, roughly the Polis / Thinking Space
model. The issue proposed "a sensible default of 1", i.e. out of the box a participant would only
need to review one proposal before moving on.

Before this change the participant gate was `allDone`: every proposal had to be submitted before
the step would let anyone continue. There was no "minimum N" concept at all.

Taking the issue's default literally would have **silently loosened every prioritization step that
already exists**. A step configured before this feature landed carries no `required_reviews` value,
so on deploy every one of them would drop from "review all 9 proposals" to "review 1", with no
admin action and no signal in the Manage UI that anything had changed. The only partner asking for
the feature (Waves) wants the value set to 1 on their own steps, which they can do explicitly.

## Decision

**Unset means all proposals. An admin sets a number only to loosen the gate.**

- `PrioritizationToolConfig.required_reviews` is `Option<i32>`, defaulting to `None`
  ([api/src/tools/prioritization.rs](../../api/src/tools/prioritization.rs)). `None` is not "1", it
  is "every proposal".
- The participant gate resolves `None` to `proposals.length`, so behaviour with no config is
  byte-for-byte the old `allDone`
  ([PrioritizationUser.svelte](../../ui/packages/comhairle/src/lib/tools/prioritization/PrioritizationUser.svelte)).
- A set value is floored at 1 and clamped to the proposal count, so an admin cannot configure a
  gate that no participant can satisfy (e.g. "review 20" on a 9-proposal step).
- Non-positive stored values normalise back to `None` in `sanitize()`, so a hand-edited or
  legacy-bad config fails closed (review all) rather than open (review none).
- In the Manage UI the field is a number input with placeholder **All**; blank clears back to
  `undefined`. Blank is a first-class value, not an error state.

This is a deliberate inversion of the default stated in #856. The configurability the issue asked
for is delivered in full; only the value you get when you configure nothing differs.

## Consequences

- Existing prioritization steps are unaffected by the deploy. No conversation changes behaviour
  until an admin types a number.
- Waves gets what they asked for by setting the field to 1 on their steps, which is one input away.
- The tri-state (blank = all, N = minimum, and the floor/clamp) has to be explained in the Manage
  UI copy, since "leave blank for the stricter behaviour" is not guessable. The field's description
  says so directly.
- The field is named `required_reviews`, **not** `required_votes` as the Polis tool names its
  equivalent. Prioritization has no votes: participants review and submit proposals, and the issue
  thread settled on "review" for the participant-facing copy. Reusing the Polis noun would have put
  the same key with two different defaults (Polis falls back to 10) in two tool configs.

## Alternatives considered

- **Default of 1, as written in the issue.** Rejected for the silent-loosening problem above. A
  gate that weakens itself on deploy is the wrong direction to fail in for a consultation tool.
- **Backfill every existing step with `required_reviews = <its proposal count>` in a migration.**
  Equivalent behaviour, but it freezes the count at migration time: adding a tenth proposal later
  would leave the gate at 9. Resolving `None` at gate time tracks the proposal list.
- **Make the step's `required` flag do this job.** `required` is a workflow-level concept that
  shows the "Skip this step" button and bypasses the tool entirely. It is orthogonal: `required`
  controls whether you may skip the step wholesale, `required_reviews` controls how much
  engagement counts as finishing it. Both can be set, and they compose the way they do for Polis.
