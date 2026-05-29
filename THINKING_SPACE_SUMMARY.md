# Thinking Space — Summary Step

Closing step for the Thinking Space participant flow. After the Q&A review screen, an AI agent draws together the user's answers into a coherent 2nd-person paragraph ("You believe…"). The participant can edit it and submits it as their final position statement.

UX flow:
1. Question round (have)
2. Review/edit answers (have)
3. **NEW** Summary review/edit + submit (this doc)

---

## What's done (frontend, mocked)

- New `summary` phase added to `ThinkingSpacePhase`.
- `Summary.svelte` component: loading state → editable textarea pre-filled with a mocked 2nd-person paragraph → Submit button. Accepts `initialSummary` prop — when set, skips the AI fetch entirely.
- `summary.ts` — stubs for `fetchSummary()`, `saveSummary()`, `hydrateSummary()`, all marked `TODO` for real endpoints.
- Wired into `ThinkingSpaceEmbed`: overview Continue → summary → `onDone`.
- Revisit behaviour: if Q&A complete on load, land directly on summary phase. `hydrateSummary()` is called — when it returns a string, no AI re-call; when null, the mock fetch runs.
- **Demo-only persistence**: `saveSummary` / `hydrateSummary` round-trip via `localStorage` so the revisit-skips-AI flow works end-to-end without a backend. Both calls are marked `TODO` for the real endpoints — swap the localStorage bodies when the API lands.

## What's NOT done yet (deliberately, pending decisions below)

- Real backend call to generate the summary.
- Storing the submitted summary.
- Storing `other_questions` for **root** questions (only follow-ups currently send `other_questions`; root questions have no alternatives by construction — confirm whether this matters).

---

## Backend asks / open questions

### Summary generation endpoint
- [ ] **New endpoint** — `POST /api/tools/thinking_space/summary` (or similar)?
  - Input: `workflow_step_id`, `user_id` (server-side), full Q&A history (or fetched server-side from saved answers?).
  - Output: one coherent paragraph, 2nd person, streamed or single response?
- [ ] **RAGFlow agent** — new agent or reuse existing? Need a prompt spec — agreed format: "You believe… You feel… You would welcome…" 2nd person, ~3 short paragraphs.
- [ ] **Trigger timing** — generated on "submit answers" (before showing summary screen), or on-demand when user lands on summary screen?
- [ ] **Streaming vs single response** — current converse endpoint streams SSE. Match that or simpler one-shot JSON?

### Summary storage
- [ ] **Where does the submitted (possibly edited) summary live?**
  - New table `thinking_space_summaries`? Or new field on workflow step participant record?
  - Fields: `workflow_step_id`, `user_id`, `summary_text`, `created_at`, `updated_at`, `is_edited` (bool, useful for analysis?).
- [ ] **CRUD endpoints**:
  - `GET /api/tools/thinking_space/summary?workflow_step_id=…&user_id=…` — hydrate on revisit.
  - `POST /api/tools/thinking_space/summary` — save edited summary.
  - `PATCH` — re-edits after submit? Or one-shot only?
- [ ] **Do we keep the original AI-generated version alongside the user's edit** (for research — diff between AI draft and human submission)?

### Other questions storage
- [ ] Already stored for follow-ups via `other_questions` field on `CreateAnswerRequest` ✅ (confirmed in `api.ts:453`).
- [ ] Root questions have no alternatives (they're fixed config). Confirm this is fine.

### Revisit behaviour
- [ ] Agreed: revisit shows summary screen only, editable, **no AI re-call**.
- [ ] Needs: hydrate saved summary on load → if exists, skip straight to `summary` phase with stored text.

---

## Checklist

### Backend
- [ ] Agree on summary generation flow (streaming vs single, where Q&A history comes from).
- [ ] Stand up RAGFlow agent for summary generation + prompt spec.
- [ ] Schema for storing submitted summary.
- [ ] Endpoints: generate, get, save/update.

### Frontend
- [x] Add `summary` phase to ThinkingSpacePhase.
- [x] Mock `Summary.svelte` w/ loading + editable textarea + submit.
- [x] Stub `fetchSummary()` / `saveSummary()` / `hydrateSummary()` w/ TODOs.
- [x] Revisit goes straight to summary phase; `initialSummary` prop skips fetch.
- [ ] Swap `fetchSummary` mock for real endpoint when ready.
- [ ] Wire `saveSummary()` to backend.
- [ ] Wire `hydrateSummary()` to backend GET.
- [ ] Loading + error states for real network calls.
