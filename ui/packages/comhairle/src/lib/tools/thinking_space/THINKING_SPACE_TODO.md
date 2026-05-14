# Thinking Space — Prototype Notes & Remaining Work

Thinking Space is a **separate tool from the elicitation bot**. It has its own
`ThinkingSpaceToolConfig` on the backend and its own `thinkingspace` tool slug,
and is listed independently in `AvailableTools` (`avaliable_tools.ts`). The
two tools are not interchangeable and the old elicitation-bot flow is no
longer wired into either Thinking Space route.

## Product flow (target)

1. **Welcome screen** — renders `workflowStep.description`.
2. **Question flow** — single conversation per session:
    - Participant answers the initial main question.
    - AI returns an **array of N follow-up question options**.
    - Participant **picks one** option from the array and answers it.
    - Each turn the AI returns a fresh array of options; the participant
      keeps picking and answering.
    - `follow_up_count` is the **minimum number of follow-ups** the
      participant must answer before the **Continue** button appears
      (configurable, default 2 — clamped 0–5). It is a **MINIMUM, not a
      ceiling.** Once the threshold is met, the Continue button is
      revealed but the participant is free to keep answering more
      follow-ups if they want. **We never force-quit the user out of a
      conversation they are still engaging with.**
3. **Review** — Continue takes participant to the review page.
4. **End-of-session statement extraction** — only **on submit** do we send
   the **full transcript** (initial answer + all follow-up Q&A pairs) to
   the backend in one call. The backend generates the claim(s) /
   statement(s) **once**, not per response. This is a deliberate departure
   from the elicitation bot, which streams a claim after every turn via the
   `<br>\n\nopinion:` marker.
5. **Submitted** — thank-you screen.

Note: the admin can configure multiple main questions in
`ThinkingSpaceManage.svelte`, but the participant flow runs one
conversation at a time. The relationship between "list of main questions"
and "one session" is currently ambiguous in the implementation — see
Open product questions.

## Backend — what's wired today

- `ThinkingSpaceToolConfig` (api/src/tools/thinking_space.rs) persists:
    - `topic: String`
    - `questions: Vec<ThinkingSpaceQuestion>` (id + text)
    - `follow_up_count: u8` (default 2) — **interpreted as a minimum, not a
      ceiling** (see Product flow). Field name retained for now; rename to
      `min_follow_up_count` is parking-lot until we touch agent / question
      generation.
- `UpdateConversationThinkingSpaceWorkflowStep` route saves all three.
- `ThinkingSpaceTool::routes()` is empty — **no participant-facing endpoints
  yet** (no transcript-submit endpoint, no claim store).

## Frontend — what's wired today

- **Admin (`ThinkingSpaceManage.svelte`)** writes topic + questions +
  follow-up count to the backend tool config. A localStorage mirror still
  exists from when only `topic` was server-side; it can be removed once
  the round-trip from server is trusted.
- **Tool slug** `thinkingspace` (`index.ts`) — referenced by both admin
  `design step` route and the public participant route.
- **Welcome description** rendered from `workflowStep.description` —
  editable via the standard `CommonStepConfig` dialog (no Thinking Space–
  specific code required).

## What's still mocked (browser-only)

| Concern                                        | Where                                                                                        | Faked behaviour                                                                                                                 |
| ---------------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Follow-up question options shown in the picker | `mockFollowups.ts` `generateFollowUpOptions()`                                               | Random pick of N from a generic bank — does **not** consider the user's actual answer                                           |
| Claim extraction                               | `mockFollowups.ts` `extractMockClaim()`                                                      | Truncates the user's answer to ~180 chars and treats that as the "claim" — also runs **per response** instead of end-of-session |
| Participant progress (answers, claims, phase)  | `localStorage` keyed by `(workflowStepId, conversationId, userId)` (`participantStorage.ts`) | Survives refresh on the same browser, never reaches the server                                                                  |
| Submit action                                  | `ThinkingSpaceEmbed.handleSubmit()`                                                          | Marks everything `approved` locally and shows thank-you. **No API call.**                                                       |

## Remaining work

### 1. Follow-up minimum (DONE in QuestionFlow.svelte)

`follow_up_count` is now treated as a **minimum** in
`QuestionFlow.svelte`:

- After each follow-up answer the picker is regenerated and the phase
  stays `'picking'` — no auto-advance.
- `minReached` ($derived: `mainSubmitted && followUpsDone >= followUpCount`)
  reveals the **Continue** button (label "Review my views" on the last
  question).
- The picker keeps offering fresh options after the minimum is hit;
  copy switches to "Want to explore another follow-up? Optional — you
  can keep going for as long as you like, or continue below."
- The `'done'` phase has been removed.

Naming nit: rename `follow_up_count` → `min_follow_up_count` for
clarity at some point. One-pass refactor across backend struct,
`test_helpers.rs`, `workflow_templates.ts`, `api.ts` (regen) and all
`followUpCount` references in `QuestionFlow.svelte` / Welcome / Embed.

### 2. Generate real follow-up question options (array, not one)

RAGFlow currently returns **one** follow-up per turn. The picker UI needs
an **array** of `follow_up_count` options. Two viable paths:

1. **Extend the RAGFlow agent template**
   (`api/src/agent_templates/ragflow-elicitation-bot.json`) so its final
   message is a structured JSON blob, e.g.

    ```json
    {
    	"follow_up_options": ["…", "…", "…", "…", "…"]
    }
    ```

    Note: **drop the `claim` field for Thinking Space** — claim extraction
    happens at end-of-session, not per turn (see #3). Update
    `agentClient.svelte.ts` to extract `follow_up_options` and ignore /
    skip the `opinion:` marker for this flow. Fall back to
    `[returned_question]` when parsing fails so we degrade gracefully.

2. **Add a dedicated "question generator" endpoint** that takes the user's
   most recent answer + the main question and returns N options. Slower
   to build but keeps the existing agent template stable.

Either way, replace `generateFollowUpOptions()` in `mockFollowups.ts` with
a real call.

### 3. End-of-session statement extraction (transcript → claims, once)

This is a **deliberate divergence** from the elicitation bot, which emits
a claim per turn via the `<br>\n\nopinion:` marker. Thinking Space must:

- **Not** stream / collect claims during the question flow.
- On **Submit**, POST the full transcript to a new backend endpoint, e.g.
  `POST /conversations/{id}/workflows/{wid}/steps/{sid}/thinking_space/submit`
  with a body like:

    ```json
    {
    	"answers": [
    		{
    			"question_id": "…",
    			"question_text": "…",
    			"main_answer": "…",
    			"follow_ups": [{ "question": "…", "answer": "…" }]
    		}
    	]
    }
    ```

- The backend then runs **one** statement-extraction pass over the whole
  transcript, generating the views/claims in a single batch (e.g. via an
  agent / LLM call) and persisting them.
- The review page should be driven by that batch response, not by
  per-response mock claims.

`participantStorage.ts` can stay as an in-progress cache; the
**server-side** claim persistence is what reaches the report. The
review-page approve / edit / remove actions need to flow through
`apiClient` instead of localStorage.

### 4. Submit action

`handleSubmit()` in `ThinkingSpaceEmbed.svelte` is a no-op. It should:

- POST the transcript (see #3) and receive generated claims.
- Allow the participant to approve / edit / remove on the review page.
- Persist approved claims server-side so downstream reports / Pol.is
  statements can consume them.
- Call `apiClient.SetUserProgress('done', …)` (same as other tools do via
  `stepComplete()` in the public route).

## Open product questions (parking lot)

- Auto-advance to next question — fade-only or short countdown? Needs
  user testing.
- Allow back-navigation to previous questions? Currently forward-only.
- Show 4 vs 5 follow-up options? Both fit fine; pick one after testing.
- Should the participant be able to **add their own claim** at the review
  step (the old elicitation bot allowed this)?
- Multi-language support — questions and topic should support the
  existing `textContent` / `textTranslations` model. Currently questions
  are stored as plain strings on the tool config.
- Multiple main questions on the tool config — what's the relationship
  to a single participant session? Options: (a) the session is one main
  question only and the rest are unused; (b) the participant picks one
  from the list at the welcome screen; (c) participant cycles through
  all of them with the follow-up minimum applying per question. Today
  the code stacks all of them (option c) but the new "minimum
  follow-ups → Continue button" model assumes a single conversation.
  Decide before wiring the real participant flow.

## Files

```
ui/packages/comhairle/src/lib/tools/thinking_space/
├── ThinkingSpaceEmbed.svelte    # participant orchestrator (welcome → review → submitted)
├── ThinkingSpaceManage.svelte   # admin config (topic + questions + minimum follow-up count)
├── Welcome.svelte
├── QuestionFlow.svelte          # main question + follow-up picker + stacking answers
├── ReviewPage.svelte            # approve / edit / remove claims
├── Submitted.svelte
├── config.ts                    # localStorage mirror of admin config (legacy — server is source of truth now)
├── participantStorage.ts        # localStorage-backed per-user progress (TEMPORARY)
├── mockFollowups.ts             # mock follow-up bank + mock claim extraction (TEMPORARY)
├── types.ts
├── index.ts                     # exports UserUI / ManageUI; TOOL_NAME is 'thinkingspace'
└── THINKING_SPACE_TODO.md       # this file
```

The old elicitation-bot files in `lib/tools/elicitation_bot/` are no longer
wired into either route. Keep around for reference until end-of-session
extraction is implemented (some agent-client / SSE plumbing may be reused)
then delete.
