# Seed statements live in Polis Setup; moderation lives in Engage

A Polis step used to have a Moderation subtab under Process design. It held both the seed statements and the review of participant statements. Admins write seeds while they design the step, before launch, but they moderate while the conversation runs. Reaching moderation through Process design meant going back into the step editor mid-conversation, and "Moderation" was a confusing place to look for seeds (#1352, #1224).

Seeds now sit in the Polis step's Setup subtab. Moderation is its own page at `/admin/conversations/<id>/moderate`, under Engage > Moderate in the sidebar. It lists the conversation's Polis steps, and `/moderate/step/<step_id>` shows the moderation UI for one step. The step editor has no Moderation subtab. The old `design/step/<step_id>/moderation` URL returns a 308 to the new page, and `?subtab=moderation` returns a 307, so old links still work (#1533).

Polis has no way to delete a statement. Removing a seed in Setup rejects it with the reason "Removed by admin", and Restore accepts it again. Removed seeds show in a collapsed list, and adding the same text again is flagged and skipped, because Polis refuses duplicate text.

## Considered options

- **Keep one Moderation subtab for both.** The step editor needs no new route, but admins still go back into Process design to moderate a live conversation.
- **Moderation as a subtab under each step, plus a seed section in Setup.** Seeds move correctly, but moderation stays inside the design stage.
- **Hard-delete seeds through the API.** Polis would still keep the statement, so comhairle and Polis would disagree about what exists.

## Consequences

- Moderate follows the Engage rules in ADR-0052: it shows only when the conversation has a Polis step, and it stays locked until launch.
- A removed seed is a rejected statement, so it appears in the moderation log like any other rejection.
- Launch still copies every seed's text into the live poll, including removed ones. #1535 tracks filtering them out.
