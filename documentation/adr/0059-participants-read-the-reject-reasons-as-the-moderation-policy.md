# ADR-0059: Participants read the reject reasons as the moderation policy

**Status:** Proposal - to be discussed with the team
**Date:** 2026-10-19
**Amends:** ADR-0038 (participants can now read a step's reasons)

## Context

#1046: we moderate participant statements, and nothing tells participants that. There is no
page saying what the rules are, who applies them, or what happens to a statement that gets
rejected. The ticket left three questions open: who writes the policy, whether it is one
platform-wide policy or one per conversation, and where participants read it.

ADR-0038 already gives every Polis step a list of reject reasons, each a label and an
optional description, owned by the conversation, with five defaults when a conversation has
no policy.

## Decision

- **The published policy is the reason list moderators pick from.** Participants see the
  same labels and descriptions, resolved the same way as the admin reason picker
  (`rejectReasonsForStep`): the step's own policy, otherwise the conversation's first policy,
  otherwise the defaults. Publishing a separate text would let the rules participants read
  drift from the rules moderators apply.
- **So it is per conversation, and the admin writes it.** That already follows from
  ADR-0038. Editing the policy in Configure changes what participants read.
- **Fixed copy around the list.** One sentence before it says who checks statements, and one
  after says what happens to a rejected statement. These are translated messages, the same
  for every conversation. The wording avoids saying statements are checked *before* anyone
  sees them, because that only holds when Polis strict moderation is on.
- **It lives in the composer.** Under the writing tips, a line says every statement is
  checked against the moderation policy, with a link that opens the policy in a sheet. That
  is the moment the rules matter. The Your Rights sheet (ADR-0019) can link to the same
  sheet later.
- **A participant route.** `GET /tools/polis/moderation_policy?workflow_step_id=&preview=`
  needs a signed-in user (participants always are) but no conversation permission, and
  returns only `label` and `description`. The admin policy routes stay behind
  `ConversationUpdate`. `preview` reads the draft config so the participant preview shows
  the policy the step will launch with.

## Consequences

- Reason descriptions now face participants as well as moderators, so admins should write
  them in plain language. The defaults already read that way.
- A reason without a description shows as a bare label, which can be too terse. The admin
  editor could nudge for a description later.
- Reason labels and descriptions are not translated. The fixed copy around them is.
- The fixed copy needs sign-off from the team running moderation before this ships.
