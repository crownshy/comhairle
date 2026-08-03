# Post-Spike Proposal: Ground the Learning Assistant in the current learn-step content

> Status: **proposal, needs a team decision** (do not build the grounding approach
> until we agree). Written for [#780](https://github.com/crownshy/comhairle/issues/780)

## What we want

When a participant is on a **learn step**, the Learning Assistant (both the inline one at
the bottom of the learn page and the "Find out more" sidebar) should be able to answer
against the **actual text on the page**, so prompts like "explain this page in simpler
words" work against what the reader is looking at, not just knowledge-base retrieval.

Scope: **learn steps only.** ?? Other step types have no "page content" to ground on. 

## How it works today (the mechanics that constrain us)

- The assistant is a **RAGFlow** chat. The frontend sends a bare
  `POST /conversation/:id/chat_sessions` with body **`{ question }`** and nothing else
  ([chatClient.svelte.ts:129](../../ui/packages/comhairle/src/lib/api/chatClient.svelte.ts)).
- **RAGFlow owns the session, the full message history, and retrieval.** The client never
  resends history. Each turn the client sends only the new question; RAGFlow replays the
  stored history into the model on its side.
- The chat session is **per-conversation, per-user** and spans **all steps** of the
  journey ([bot_service_user_session.rs](../../api/src/models/bot_service_user_session.rs)),
  not per-step.
- The system prompt is **"answer using ONLY the information in the knowledge base below"**,
  with a `{knowledge}` variable filled by RAG retrieval
  ([bot_service.rs:28](../../api/src/bot_service.rs)).
- There is already a **displayed-vs-LLM split**: `session.send(question, llmQuestion?)`
  shows `question` in the UI bubble but sends `llmQuestion` to the model
  ([chatSession.svelte.ts:110](../../ui/packages/comhairle/src/lib/api/chatSession.svelte.ts)).
  Today the inline assistant uses it to prepend `[Reading "<title>"]`
  ([LearningAssistant.svelte:132](../../ui/packages/comhairle/src/lib/components/LearningAssistant/LearningAssistant.svelte)).
  The **sidebar passes no title and no content at all**.
- The current step's markdown **content** is already derived right next to the inline
  mount ([LearnUI.svelte:49-58](../../ui/packages/comhairle/src/lib/tools/learn/LearnUI.svelte)),
  so it is trivially available inline. The **sidebar sits at the workflow layout** and
  knows nothing about the current step, so live step content has to be plumbed down to it.

## The token question (aka the naive fix might be the wrong one?)

Reusing one session for many questions is the **cheap** part: the client only ever sends
the new question. The **expensive** move is baking the page content into the question
each time, because RAGFlow persists each question as a stored turn and replays every prior
turn back into the model. So the page content would accumulate:

| Approach | Copies of page content the model re-reads by question 10 |
| --- | --- |
| Prepend full content to every question | ~10 (one per stored user turn) |
| Seed content once per step | 1 per step visited |
| Inject into system prompt per turn (not stored as a turn) | 0 stored, 1 live in the current turn |
| Knowledge-base document (retrieved) | 0 stored, only matching chunks when relevant |

The goal, then, is to get the page in front of the model **without persisting it once per
question** into the session history.

## Options

### Option 1: Backend page-content variable (proposed starting point)

Extend the POST body to `{ question, page_content?, page_title? }` and inject the content
into the RAGFlow **system prompt** for that turn (the same mechanism as `{knowledge}`),
**not** into the question string. Rework the prompt so the current page is treated as
authoritative context alongside the knowledge base.

- **Pros:** best on tokens (nothing accumulates in history); content is always the live
  current step, so moving between steps in one session just changes the injected value;
  no parsing/embedding; keeps the clean question in the transcript.
- **Cons / unknown:** needs the RAGFlow chat completion to honor a **per-request prompt
  variable**. The *agent* path already does exactly this via `inputs`
  ([ragflow_bot.rs:1086](../../api/src/bot_service/ragflow_bot.rs)), but chat currently
  sends `inputs: None`, so we **spike this first** (~half a day) before committing. Touches
  Rust (`ChatConversationRequest`, the ragflow adaptor, the chat prompt template) and the
  generated api-client. If the spike fails, fall back to Option 2.

### Option 2: Seed content once per step (client-only fallback)

No backend change. Track which step's content has been seeded into the session; prepend
the full content to `llmQuestion` only on the **first** question per step, send clean
questions after; re-seed when the step changes.

- **Pros:** ships fast; one copy per step in history instead of one per question; reuses
  the existing `send(question, llmQuestion)` split.
- **Cons:** still lands in history (grows as steps are visited); still leans on the
  "answer only from knowledge base" prompt, which fights content-in-the-question; a bit
  implicit.

### Option 3: Step content as knowledge-base documents 

Sync each learn step's content into the conversation's knowledge base as parsed documents,
retrieved via RAG like any other doc.

- **Pros:** zero history cost; fits the existing `{knowledge}` prompt with no prompt
  changes; also answers #591's broader "the assistant should know the material".
- **Cons:** retrieval returns **chunks, not the whole page**, so "explain **this page** in
  simpler words" (the motivating prompt) retrieves poorly, there is no strong lexical
  signal tying the query to the page. Adds parse/embed latency, a staleness + re-sync
  lifecycle on every admin edit, and preview-vs-live handling. It is really the answer to
  **#591** ("knows the material"), not to **#780** ("knows the page you are on").

## For discussion

Start with **Option 1** behind a spike to prove the per-request variable, so we can see it
working with real page grounding and good token behaviour, and keep **Option 2** as the
fast fallback if the spike fails. Treat **Option 3 / KB** as a **separate track for #591**
(general "knows the material"), not the mechanism for "this page". This keeps #780 focused
on live, current-page grounding.

## Open questions for the team

1. **Approach:** Option 1 (system-prompt variable) vs Option 2 (seed-once) vs Option 3
   (KB)? Recommendation above is 1-then-2, KB stays with #591.
2. **RAGFlow capability:** does a chat completion accept a per-request prompt variable we
   can bind page content to (not persisted as a turn)? This is the spike that gates
   Option 1.
3. **Prompt behaviour:** if page content is authoritative context, how do we phrase the
   prompt so it does not answer "not found in the dataset" for page-grounded questions,
   while still refusing genuinely out-of-scope ones?
4. **Content size:** learn pages can be long. Do we cap / truncate the injected content,
   and how does that interact with the model context window?
5. **Preview vs live:** the injected content should follow the same `toolConfig` vs
   `previewToolConfig` selection the page already uses. Confirm.
6. **Relationship to #591:** are #780 (this page) and #591 (the material) two mechanisms
   living side by side, or does one subsume the other long term?

## If we go with Option 1, rough shape of the work

1. **Spike:** confirm per-request prompt-variable injection on a RAGFlow chat completion.
2. **Backend:** add `page_content` / `page_title` to `ChatConversationRequest`, thread
   through `converse_with_chat` and the ragflow adaptor, add the prompt variable and
   reword the prompt.
3. **api-client:** regenerate the typed client for the new body.
4. **Frontend:** pass live step content from `LearnUI` into the inline assistant; plumb
   current-step content down to the sidebar at the workflow layout; send it on the POST
   instead of the `[Readig "<title>"]` string prefix.
5. **Tests:** backend request-shape test; a frontend test that the clean question (not the
   content) is what shows in the transcript.
