# Prioritisation Tool — Deferred from the prototype

The current prototype is a frontend-only click-through with the thinnest
possible backend stub. The backend variant accepts an opaque JSON blob and
nothing more; the whole poll lives in browser `localStorage` keyed by the
workflow-step id.

This is on purpose — it lets product/design iterate on the participant and
facilitator flows without paying the cost of schema design, migrations,
WS plumbing, and translation wiring up front.

Below is the list of things a production implementation will need.

## 1. Persistence

- Postgres tables: `prioritisation_polls`, `_proposals`, `_questions`,
  `_choices`, `_answers`, `_submissions`, `_report_pages`. Foreign-keyed
  through `workflow_step_id`. Add migrations under `api/migrations/`.
- Replace the `serde_json::Value` blob in
  `api/src/tools/prioritisation.rs` with a strongly-typed
  `PrioritisationToolConfig` that references the rows above (or stores
  the canonical id).
- `clone_tool` should deep-copy the design (proposals, questions,
  settings, report pages, *not* submissions) when a conversation goes
  live. Right now it just clones the blob.

## 2. HTTP API

All endpoints listed in the original spec, restated here for traceability:

- `POST /workflow_step/{id}/prioritisation` — create / replace.
- `GET /workflow_step/{id}/prioritisation` — read.
- `PATCH .../proposals/{id}`, `PATCH .../questions/{id}`, etc.
- `POST .../publish`, `.../pause`, `.../resume`, `.../end`.
- `POST .../submissions` — participant submit.
- `GET .../results` — facilitator aggregates (cache-friendly).
- `GET /poll/{join_code}` — public join endpoint at the
  `poll.comhairle.scot` domain.

The frontend store (`store.svelte.ts`) is structured so swapping
localStorage for `fetch()` is mostly a matter of replacing
`saveJSON`/`loadJSON` with API calls; the component contract doesn't
change.

## 3. WebSocket events

- `prioritisation_results_updated` (throttle 1 Hz).
- `prioritisation_poll_state_changed` (immediate).
- `prioritisation_submission_received` (count-only, for the manage
  panel's "X completed" counter).

Reuse the existing WS subscriber pattern used by Polis.

## 4. Public join domain

- DNS / TLS / SvelteKit route at `poll.comhairle.scot/{joinCode}` that
  redirects to the workflow step participant view, bypassing the normal
  conversation-flow login.
- Server-side join-code generation with collision retries (the prototype
  uses a 5-digit client-side random number — `00012` is plausible there).
- QR codes generated server-side from the canonical URL (the prototype
  uses a decorative SVG; replace with the existing `qrcode` library).

## 5. Translation

The prototype stores plain strings. For production:

- Wrap poll title, instruction, proposal title/content, question prompts,
  question descriptions, choice labels, and report-page content in
  `TranslatableField` instances.
- Pipe new strings through the existing `new_translation` flow.
- Add a per-poll primary locale (today, inferred from the conversation).

## 6. Real-time multi-browser sync

The prototype is single-machine: a second tab on the same machine sees
the same localStorage, but a different machine sees its own empty poll.
Production needs:

- Server-of-truth (the new tables) + WS broadcast (see §3).
- Optimistic local writes with rollback on server reject.

## 7. Drag-and-drop direct ranking

The spec sketch shows a drag-rank UI as well as the importance/agreement
flow. Treat this as a separate poll mode:

- A `mode: 'rate' | 'rank'` field on the poll.
- A different participant component for `rank` (sortable list of
  proposals, no per-question form).
- A different aggregation file: rank aggregation is its own problem
  (Borda count, Kemeny-Young, Schulze). Don't try to express it as
  importance × agreement.

## 8. Async / shareback narrative

> "We hear from teachers… we'd like to share this back with parents."

A standalone view of the *report* pages (the existing
`Shareback.svelte`) accessible without participating. Likely lives on
the conversation overview rather than the workflow step itself, plus a
publish-toggle for the report.

## 9. Shared question core (currently inverted)

The prototype takes the **opposite** position from most prioritisation
tools: every proposal owns its own question list, and there is no
shared core. This was a deliberate product choice during prototyping
(see commit history), but it means cross-proposal ranking and the
combined-metrics math in `documentation/prioritisation-aggregation.md`
are intentionally not computed.

Production needs the inverse data model:

- A poll-level "shared core" of questions answered for every proposal
  (typically importance + agreement).
- Optional per-proposal extras layered on top.
- A many-to-many join between `proposal_id` and `question_id` so the
  shared core questions are reused, while extras stay attached to a
  single proposal.

Once that lands, re-enable the combined metrics by computing them only
over the shared-core axes. The existing `aggregation.ts` per-question
aggregates carry over unchanged for the per-proposal extras.

## 10. Reporting / export

- CSV export of submissions (per row: participant, proposal, question,
  answer) for offline analysis.
- PDF export of the report / shareback.
- Demographic breakdowns (if conversation has demographics enabled).

## 11. Stakeholder / delegate flow

The spec mentions "stakeholders" and "delegates" as distinct roles. No
code difference is anticipated — it's a policy question of *who is
allowed to participate*. Surface this in the conversation-level access
control rather than the tool.

## 12. Tests

- Unit tests for `aggregation.ts` (worked example from
  `documentation/prioritisation-aggregation.md` becomes a Vitest
  snapshot).
- Integration test of the participant flow with Playwright.
- API tests once the routes exist.

## 13. API client regeneration

Adding the `prioritisation` variant to `ToolConfig` / `ToolSetup` on the
backend means the generated TypeScript client in
`ui/packages/api-client/` will need regenerating before the frontend's
zod parsing will accept the new shape. This is a normal step in the
existing publish-api-client workflow; the prototype only sends
`{ type: 'prioritisation', data: {} }` through `tool_setup`, which is
already a passthrough union member-shaped object, but a typed shape
will land with the production rewrite.
