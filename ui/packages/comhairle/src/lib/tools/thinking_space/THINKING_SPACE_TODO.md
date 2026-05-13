# Thinking Space — Prototype Notes & Backend Gaps

Thinking Space is the next iteration of the **elicitation bot**. The current
implementation is a frontend-only prototype that re-uses the existing
`elicitationbot` tool slug and the existing `topic` field on `ToolConfig`.
Everything else is mocked client-side so we can validate the interaction
pattern without waiting on backend / RAGFlow changes.

## What's real today

- **Topic** is still persisted via `UpdateConversationElicitationBotWorkflowStep`
  (the same endpoint the old elicitation bot used).
- **Tool slug** is unchanged (`elicitationbot`). The two SvelteKit route files
  for admin "design step" and the public participant page have been pointed
  at `lib/tools/thinking_space/` components.
- **Workflow step `description`** is rendered on the Welcome screen — that
  field is already editable via the existing `CommonStepConfig` dialog, so we
  treat topic + description as "given" and only let the admin configure
  questions + follow-up count here.

## What's currently mocked (in the browser)

| Concern                                        | Where it lives now                                                                           | What's faked                                                                          |
| ---------------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| List of main questions                         | `localStorage` keyed by `workflowStepId` (`config.ts`)                                       | Admin can add / remove / reorder; never leaves the browser                            |
| Global follow-up count (0–5)                   | Same `localStorage` blob                                                                     | Same                                                                                  |
| Follow-up question options shown in the picker | `mockFollowups.ts`                                                                           | Random pick of 5 from a generic bank — does **not** consider the user's actual answer |
| Claim extraction                               | `mockFollowups.ts` `extractMockClaim()`                                                      | Truncates the user's answer to ~180 chars and treats that as the "claim"              |
| Participant progress (answers, claims, phase)  | `localStorage` keyed by `(workflowStepId, conversationId, userId)` (`participantStorage.ts`) | Survives refresh on the same browser, but never reaches the server                    |
| Submit action                                  | `ThinkingSpaceEmbed.handleSubmit()`                                                          | Marks everything `approved` locally and shows the thank-you screen. **No API call.**  |

## To get to a "real" Thinking Space

### 1. Rename the tool slug (optional, mostly cosmetic)

The slug is currently `elicitationbot`. If we want the URL paths, type names
and API endpoints to read "thinkingspace" instead, that's a coordinated
rename across:

- `data_model/schema.json` and the generated `ToolConfig` enum in Rust
- `api/src/tools/elicitation_bot.rs` route handler + module name
- Database migrations referencing the existing enum variant
- All references to `'elicitationbot'` in the frontend (`thinking_space/index.ts` + the two route files)

For the prototype I've kept it as `elicitationbot` so no backend work was
needed.

### 2. Extend `ElicitationBotToolConfig` to persist questions + follow-up count

Today the config is just `{ topic: string }`. We need something like:

```rust
pub struct ThinkingSpaceToolConfig {
    pub topic: String,
    pub questions: Vec<ThinkingSpaceQuestion>,
    pub follow_up_count: u8, // 0..=5
}

pub struct ThinkingSpaceQuestion {
    pub id: Uuid,
    pub text: String,
    pub order: i32,
}
```

Then:

- Update `api/src/tools/elicitation_bot.rs` PUT handler to accept the new
  fields.
- Add the regenerated API client types in `ui/packages/api-client`.
- Replace `loadConfig` / `saveConfig` (`config.ts`) with calls to the API
  client. Remove the localStorage shim.
- Update `ThinkingSpaceManage.svelte` to bind to the server payload directly
  via `superForm` (matches the pattern in `ElicitationBotManage.svelte`).

Optionally also store **translations** for each question, following the same
`textContent` / `textTranslations` shape used elsewhere (see
`frontend/src/lib/components/Translation/TranslatableField.svelte`).

### 3. Generate real follow-up question options

RAGFlow currently returns **one** follow-up per turn. The picker UI needs an
**array** of 4–5. Two viable paths:

1. **Extend the RAGFlow agent template** (`api/src/agent_templates/ragflow-elicitation-bot.json`) so its final message is a structured JSON blob, e.g.

    ```json
    {
    	"claim": "…",
    	"follow_up_options": ["…", "…", "…", "…", "…"]
    }
    ```

    then update the frontend SSE parser (`agentClient.svelte.ts`) to extract
    `follow_up_options` alongside the existing `opinion:` marker. Falls back to
    `[returned_question]` when parsing fails so we degrade gracefully.

2. **Add a dedicated "question generator" endpoint** that takes the user's
   most recent answer + the main question and returns N options. Slower to
   build but keeps the existing agent template stable.

Either way, replace `generateFollowUpOptions()` in `mockFollowups.ts` with a
call that goes through `AgentClient` (see `lib/api/agentClient.svelte.ts`).

### 4. Real claim extraction

The existing RAGFlow agent already emits claims via the `<br>\n\nopinion:`
marker (see trace 3 / `parseSSELine`). We can keep that — Thinking Space just
needs to:

- Send each main answer + follow-up answer through `AgentClient.send()`.
- Hide the agent's chat reply from the UI (we don't surface it any more).
- Collect emitted claims silently and only show them on the review page.

`participantStorage.ts` becomes the in-progress cache; the **server-side**
claim persistence should reuse the existing logic in
`claimStorage.ts` / the elicitation bot models. The review-page approve /
edit / remove actions need to flow through `apiClient` instead of
localStorage.

### 5. Submit action

Right now `handleSubmit()` in `ThinkingSpaceEmbed.svelte` is a no-op. It
should:

- Call `apiClient.SetUserProgress('done', …)` (same as other tools do via
  `stepComplete()` in the public route).
- Possibly POST a finalised list of approved claims to a new endpoint so
  that downstream reports / Pol.is statements can consume them.

## Open product questions (parking lot)

- Auto-advance to next question — fade-only or short countdown? Needs user testing.
- Allow back-navigation to previous questions? Currently forward-only.
- Show 4 vs 5 follow-up options? Both fit fine; pick one after testing.
- Should the participant be able to **add their own claim** at the review
  step (the old elicitation bot allowed this)?
- Multi-language support — questions and the topic should support the
  existing `textContent` / `textTranslations` model.

## Files

```
ui/packages/comhairle/src/lib/tools/thinking_space/
├── ThinkingSpaceEmbed.svelte    # participant orchestrator (welcome → review → submitted)
├── ThinkingSpaceManage.svelte   # admin config (topic + questions + follow-up count)
├── Welcome.svelte
├── QuestionFlow.svelte          # main question + follow-up picker + stacking answers
├── ReviewPage.svelte            # approve / edit / remove claims
├── Submitted.svelte
├── config.ts                    # localStorage-backed admin config (TEMPORARY)
├── participantStorage.ts        # localStorage-backed per-user progress (TEMPORARY)
├── mockFollowups.ts             # mock follow-up bank + mock claim extraction (TEMPORARY)
├── types.ts
├── index.ts                     # exports UserUI / ManageUI; TOOL_NAME still 'elicitationbot'
└── THINKING_SPACE_TODO.md       # this file
```

The old elicitation-bot files in `lib/tools/elicitation_bot/` are no longer
wired into either route and can be deleted once you're confident the new
flow is what you want.
