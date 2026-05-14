# Prioritisation Tool — Technical Spec

A new pluggable tool for Comhairle that lets a facilitator publish a set of proposals, collect structured per-proposal responses from participants, view results in real-time, and publish a static summary report after the poll ends.

Implemented as a new `ToolConfig` variant alongside `Polis`, `Learn`, `HeyForm`, `Stories`, and `ElicitationBot`. It attaches to a conversation as a workflow step and follows the existing preview-clone-on-launch pattern.

---

## 1. Scope and integration model

### 1.1 Where it lives in the system

The Prioritisation Tool is a workflow step's tool, not a top-level entity. It plugs into the existing model:

- A `Conversation` owns one or more `Workflow`s.
- A `Workflow` owns ordered `WorkflowStep`s.
- A `WorkflowStep` carries a `preview_tool_config: ToolConfig` (always present, edited by the facilitator) and an `Option<ToolConfig> tool_config` (the live, frozen copy created when the conversation is launched).
- A new enum variant `ToolConfig::Prioritisation(PrioritisationToolConfig)` is added.

This mirrors the existing `setup → clone_tool → sync_data → delete` lifecycle. Nothing about the conversation, workflow, workflow_step, user_participation, or user_progress tables changes — the tool stores its own data under its own ID(s) referenced from the `tool_config` JSONB.

### 1.2 What "preview" vs "live" means for this tool

- **Preview** is the editable draft (the facilitator's view in screens iPhone 17 - 18, 19, 21, 20). Proposals and questions are mutable. The poll cannot be answered.
- **Live** is created from preview at `conversation::launch` time via `PrioritisationToolConfig::clone_tool`. The schema (proposals, questions, settings) is frozen. Participants answer the live copy. The facilitator's Poll Management screen (iPhone 17 - 9, 25) operates on the live copy.

A facilitator can still pause/resume the live poll, edit the timer, and end the poll — these are runtime controls, not schema edits.

### 1.3 Naming

Spec uses **Prioritisation Tool** as the canonical internal name (matches the wireframe heading and the project's "interoperable tool on Comhairle" phrasing). The product-facing name (`Prioritisation Tool` vs `ComhairleVote` vs other) is an open question called out in the wireframes; the code identifier should be `prioritisation` regardless.

---

## 2. Data model

### 2.1 Rust types

Add to `tools.rs`:

```rust
pub enum ToolConfig {
    Polis(PolisToolConfig),
    Learn(LearnToolConfig),
    HeyForm(HeyFormToolConfig),
    Stories(StoriesToolConfig),
    ElicitationBot(ElicitationBotToolConfig),
    Prioritisation(PrioritisationToolConfig), // new
}
```

```rust
pub struct PrioritisationToolConfig {
    pub poll_id: Uuid,            // FK into prioritisation_polls
    pub mode: PrioritisationMode, // Preview or Live
}

pub enum PrioritisationMode { Preview, Live }
```

The `tool_config` JSONB on `workflow_step` stores the discriminator + `poll_id` + `mode`. All mutable poll content lives in dedicated tables so it can be indexed, queried for real-time results, and exported.

### 2.2 New tables

#### `prioritisation_polls`

| column                | type                          | notes                                                                 |
| --------------------- | ----------------------------- | --------------------------------------------------------------------- |
| `id`                  | uuid pk                       |                                                                       |
| `workflow_step_id`    | uuid fk → `workflow_steps.id` | one poll per step                                                     |
| `mode`                | text enum (`preview`,`live`)  |                                                                       |
| `title_translation_id`| uuid fk → `translations.id`   | "What is this poll called?" — uses existing `new_translation` helper  |
| `instruction_translation_id` | uuid fk → `translations.id` | "How should the participants answer this poll?"                  |
| `proposal_sort_mode`  | text enum (see §2.5)          | default `by_proposal_id`                                              |
| `timer_seconds`       | int nullable                  | `null` = forever                                                      |
| `timer_started_at`    | timestamptz nullable          | set when poll first becomes answerable; used to compute time left     |
| `paused_at`           | timestamptz nullable          | non-null while paused                                                 |
| `paused_accumulated_seconds` | int default 0          | so pause/resume doesn't lose elapsed time                             |
| `state`               | text enum (see §2.6)          | `draft`, `published`, `paused`, `ended`                               |
| `created_at`          | timestamptz                   |                                                                       |
| `updated_at`          | timestamptz                   |                                                                       |
| `published_at`        | timestamptz nullable          |                                                                       |
| `ended_at`            | timestamptz nullable          |                                                                       |
| `join_code`           | text unique                   | 5-digit code like `04136` shown on the QR-code screen                 |

#### `prioritisation_proposals`

| column            | type                                       | notes                                          |
| ----------------- | ------------------------------------------ | ---------------------------------------------- |
| `id`              | uuid pk                                    |                                                |
| `poll_id`         | uuid fk → `prioritisation_polls.id`        | cascade delete                                 |
| `display_order`   | int                                        | 1-based, contiguous, unique within poll        |
| `title_translation_id` | uuid fk → `translations.id`           | "What is the title of this proposal?"          |
| `content_translation_id` | uuid fk → `translations.id`         | rich-text body (Markdown stored in translation)|
| `image_asset_id`  | uuid fk → assets nullable                  | optional header image (see wireframe Proposal 1 & 2 in iPhone 17 - 10/11) |
| `created_at`      | timestamptz                                |                                                |
| `updated_at`      | timestamptz                                |                                                |

> The wireframe shows an "Upload image" affordance on the proposal editor. Use the existing asset upload pipeline.

#### `prioritisation_questions`

These are the questions attached to *every* proposal (not poll-level). Per the wireframes, the facilitator defines the question set once on the poll and each participant answers them for each proposal in turn.

| column                 | type                                          | notes |
| ---------------------- | --------------------------------------------- | ----- |
| `id`                   | uuid pk                                       |       |
| `poll_id`              | uuid fk → `prioritisation_polls.id`           | cascade delete |
| `display_order`        | int                                           | 1-based |
| `question_type`        | text enum (see §2.4)                          |       |
| `prompt_translation_id`| uuid fk → `translations.id`                   | "Type a question" |
| `description_translation_id` | uuid fk nullable                        | "Add a description (optional)" |
| `is_optional`          | bool default false                            | the wireframe shows "Question 3 (optional)" |
| `config`               | jsonb                                         | type-specific shape, see §2.4 |
| `created_at`           | timestamptz                                   |       |
| `updated_at`           | timestamptz                                   |       |

#### `prioritisation_answers`

One row per (user, proposal, question).

| column           | type                                              | notes |
| ---------------- | ------------------------------------------------- | ----- |
| `id`             | uuid pk                                           |       |
| `poll_id`        | uuid fk → `prioritisation_polls.id`               | denormalised for query speed |
| `proposal_id`    | uuid fk → `prioritisation_proposals.id`           | cascade delete |
| `question_id`    | uuid fk → `prioritisation_questions.id`           | cascade delete |
| `user_id`        | uuid fk → `users.id`                              |       |
| `value`          | jsonb                                             | shape depends on `question_type` — see §2.4 |
| `submitted_at`   | timestamptz nullable                              | null until the participant hits Submit |
| `updated_at`     | timestamptz                                       |       |

Unique constraint: `(proposal_id, question_id, user_id)`.

#### `prioritisation_submissions`

Tracks the final-submit event so we can show "28 completed" on the management screen (iPhone 17 - 9) and gate the static report from showing partial answers.

| column         | type                                | notes |
| -------------- | ----------------------------------- | ----- |
| `id`           | uuid pk                             |       |
| `poll_id`      | uuid fk                             |       |
| `user_id`      | uuid fk                             |       |
| `submitted_at` | timestamptz                         |       |

Unique on `(poll_id, user_id)`.

#### `prioritisation_reports`

Backs the Report Editor (iPhone 17 - 26/27/28). The report is published once the poll has ended and shown to participants as the "Shareback" view (iPhone 17 - 29/30).

| column         | type                                | notes |
| -------------- | ----------------------------------- | ----- |
| `id`           | uuid pk                             |       |
| `poll_id`      | uuid fk unique                      |       |
| `published_at` | timestamptz nullable                |       |
| `created_at`   | timestamptz                         |       |
| `updated_at`   | timestamptz                         |       |

#### `prioritisation_report_pages`

| column                 | type                                | notes |
| ---------------------- | ----------------------------------- | ----- |
| `id`                   | uuid pk                             |       |
| `report_id`            | uuid fk                             |       |
| `display_order`        | int                                 | 1-based |
| `content_translation_id` | uuid fk → translations             | rich-text body |
| `created_at`           | timestamptz                         |       |
| `updated_at`           | timestamptz                         |       |

### 2.3 Indexes

- `prioritisation_proposals (poll_id, display_order)`
- `prioritisation_questions (poll_id, display_order)`
- `prioritisation_answers (poll_id, question_id)` — for real-time aggregation
- `prioritisation_answers (poll_id, user_id)` — for "have I completed all questions yet"
- `prioritisation_polls (join_code)` unique

### 2.4 Question types

The wireframe enumerates five types in the picker (iPhone 17 - 21) plus a "Slider" called out in the (pre)view sketch. To reconcile: the participant view (iPhone 17 - 10/11) uses a slider for "Do you support or oppose this proposal?" — so the rendered control for `rating_scale` with two labelled endpoints is a slider. The catalog is:

| type             | participant control                              | `config` shape                                                                 | `value` shape |
| ---------------- | ------------------------------------------------ | ------------------------------------------------------------------------------ | ------------- |
| `single_line_text` | one-line `<Input>`                             | `{ max_length?: number }`                                                      | `{ text: string }` |
| `long_text`      | `<Textarea>` (used for "Optional comment (why?)")| `{ max_length?: number }`                                                      | `{ text: string }` |
| `multiple_choice`| labelled radio list (A, B, B, B in wireframe)  | `{ choices: [{ id, label_translation_id, letter }], min_selected, max_selected }` | `{ choice_ids: uuid[] }` |
| `five_star_rating` | five-star widget                               | `{}`                                                                           | `{ stars: 1..5 }` |
| `rating_scale`   | slider with two endpoint labels                  | `{ min: number, max: number, step: number, min_label_translation_id, max_label_translation_id }` | `{ value: number }` |

> Wireframe consistency note: the "A / B / B / B" letters on the multiple-choice options in iPhone 17 - 10/11 should be auto-assigned from the choice's position (`letter` is derived, not stored), to avoid an editor having to manage letters by hand. Decision needed: do we want them as visual prefix only, or do they carry semantic weight? Recommend prefix-only.

### 2.5 `proposal_sort_mode`

From the "Things to discuss" wireframe and the iPhone 17 - 25 sort-by control:

- `by_proposal_id` — default; the facilitator's authoring order.
- `by_question:<question_id>` — sort by aggregate result on a specific question. On the facilitator's real-time view (iPhone 17 - 25) this controls the dashboard list. On the participant's overview list (separate wireframe at the end showing "Click on each proposal to start voting") it controls how proposals are arranged for them — this is opt-in via "advanced settings" per the note "By default, we could sort all proposals by id…". For v1, recommend exposing this only on the facilitator's result view; participant ordering stays `by_proposal_id`.

### 2.6 Poll `state` lifecycle

```
draft ──publish──▶ published ──pause──▶ paused
                       │  ▲                │
                       │  └────resume──────┘
                       │
                       └──end──▶ ended
```

- `draft`: editable, no participants can answer. This is the state during workflow-step authoring (mode = `preview`).
- `published`: live and answerable. Timer (if any) runs.
- `paused`: live but not answerable; timer is frozen. Already-submitted answers are kept.
- `ended`: read-only. Facilitator can edit the Report. Participants see the published Report.

State transitions on the live copy only. The preview copy stays in `draft` forever and is the source of truth for any future re-launch.

---

## 3. Backend API

All routes mounted under the existing workflow-step namespace so they inherit auth and conversation-scoping. Concrete paths follow your existing convention; pseudocode below uses RESTful naming.

### 3.1 Facilitator routes (auth: workflow facilitator)

| method | path | body | returns |
| ------ | ---- | ---- | ------- |
| `POST` | `/prioritisation/polls` (called by `setup`) | `{ workflow_step_id }` | `PrioritisationPoll` |
| `GET`  | `/prioritisation/polls/:poll_id` | — | `PrioritisationPoll` with proposals + questions |
| `PATCH`| `/prioritisation/polls/:poll_id` | partial (title, instruction, timer_seconds, proposal_sort_mode) | `PrioritisationPoll` |
| `POST` | `/prioritisation/polls/:poll_id/proposals` | `{ title?, content?, display_order? }` | `Proposal` |
| `PATCH`| `/prioritisation/proposals/:proposal_id` | partial | `Proposal` |
| `PATCH`| `/prioritisation/proposals/:proposal_id/reorder` | `{ display_order }` | reordered list |
| `DELETE`| `/prioritisation/proposals/:proposal_id` | — | `204` |
| `POST` | `/prioritisation/polls/:poll_id/questions` | `{ question_type, prompt, description?, config, is_optional? }` | `Question` |
| `PATCH`| `/prioritisation/questions/:question_id` | partial | `Question` |
| `POST` | `/prioritisation/questions/:question_id/duplicate` | — | `Question` (wireframe shows "Duplicate" action) |
| `DELETE`| `/prioritisation/questions/:question_id` | — | `204` |
| `POST` | `/prioritisation/polls/:poll_id/pause` | — | `PrioritisationPoll` |
| `POST` | `/prioritisation/polls/:poll_id/resume` | — | `PrioritisationPoll` |
| `POST` | `/prioritisation/polls/:poll_id/end` | — | `PrioritisationPoll` |
| `PATCH`| `/prioritisation/polls/:poll_id/timer` | `{ timer_seconds }` | `PrioritisationPoll` |
| `GET`  | `/prioritisation/polls/:poll_id/results` | — | aggregated results, see §3.3 |
| `GET`  | `/prioritisation/polls/:poll_id/report` | — | `Report` with pages |
| `POST` | `/prioritisation/polls/:poll_id/report/pages` | `{ content?, display_order? }` | `ReportPage` |
| `PATCH`| `/prioritisation/report-pages/:page_id` | partial | `ReportPage` |
| `DELETE`| `/prioritisation/report-pages/:page_id` | — | `204` |
| `POST` | `/prioritisation/polls/:poll_id/report/publish` | — | `Report` |

Publish/unpublish on the workflow step itself is the existing mechanism — when the conversation is launched (§4) the poll becomes `published` automatically.

### 3.2 Participant routes (auth: workflow participant via `user_participation`)

| method | path | body | returns |
| ------ | ---- | ---- | ------- |
| `GET`  | `/prioritisation/join/:join_code` | — | minimal poll info: `{ poll_id, title, state }`. Used by the QR-code join flow. |
| `GET`  | `/prioritisation/polls/:poll_id/participant` | — | poll + proposals + questions in the order this participant should answer them; includes any draft answers they've already saved |
| `PUT`  | `/prioritisation/proposals/:proposal_id/answers` | `{ answers: [{ question_id, value }] }` | upserts draft answers for the current user; idempotent |
| `POST` | `/prioritisation/polls/:poll_id/submit` | — | `204` once required answers are present; `409` if some required questions unanswered |
| `GET`  | `/prioritisation/polls/:poll_id/report/published` | — | published report content for the shareback view |

### 3.3 Results aggregation

`GET /prioritisation/polls/:poll_id/results` returns per-proposal aggregates. Shape per question type:

```json
{
  "proposals": [
    {
      "proposal_id": "…",
      "display_order": 1,
      "title": "Convert the space into a public car park",
      "questions": [
        {
          "question_id": "…",
          "question_type": "multiple_choice",
          "results": {
            "choices": [
              { "choice_id": "…", "letter": "A", "label": "Extremely important", "count": 12, "percentage": 0.42 },
              …
            ]
          }
        },
        {
          "question_id": "…",
          "question_type": "rating_scale",
          "results": { "mean": 0.62, "min": 0, "max": 1, "count": 28, "distribution_buckets": [...] }
        }
      ]
    }
  ],
  "total_submissions": 28,
  "total_participants_started": 31
}
```

The facilitator's real-time view (iPhone 17 - 24, 25) renders the horizontal bars from `count` / `percentage` when sorted by a multiple-choice question, or from `mean` when sorted by a rating-scale question.

### 3.4 WebSocket events

Reuse the existing `WorkflowMessageHandler` channel. New message variants:

**Backend → frontend (broadcast to facilitator's session):**

- `prioritisation_results_updated { poll_id, totals: { submissions, started } }` — emitted on any new submission. Frontend then refetches results.
- `prioritisation_poll_state_changed { poll_id, state }` — facilitator UI updates pause/end button state across tabs.

**Frontend → backend (existing user-started/finished workflow step events suffice for marking progress).**

Detailed per-answer streaming is deliberately not pushed over WS to keep payloads small — the facilitator re-pulls `/results` on the throttled event. Throttle backend emission to at most 1/sec.

### 3.5 `ToolConfig` dispatch

Implement the trait methods on `PrioritisationToolConfig`:

- `setup(state)` — creates a row in `prioritisation_polls` with `mode = preview`, `state = draft`, generates a unique `join_code`. Returns `ToolConfig::Prioritisation(PrioritisationToolConfig { poll_id, mode: Preview })`.
- `clone_tool(state)` — called at `conversation::launch`. Deep-copies the preview poll, proposals, questions, and translations into a new poll row with `mode = live`, `state = published`, fresh `join_code` (or reuse — see §6.1), and `published_at = now()`. Sets `timer_started_at = now()` if `timer_seconds` is set. Returns the live `ToolConfig::Prioritisation`.
- `sync_data(state)` — no-op for v1 (no external service to reconcile).
- `delete(state)` — cascade-deletes the poll row; FKs handle the rest.

---

## 4. Lifecycle integration with existing flows

Mapping onto the documented backend traces:

### 4.1 Creation (trace 2 — Creating a Workflow Step with Tool)

When the facilitator adds a "Prioritisation" workflow step:

- `ToolSetup::Prioritisation` is dispatched at step 2d.
- `PrioritisationToolConfig::setup` creates the preview poll row (§3.5) and returns the `ToolConfig` to store on `workflow_steps.preview_tool_config`.

### 4.2 Authoring (no new trace needed)

All proposal/question CRUD happens via the §3.1 routes, scoped to the preview poll. These are independent of the workflow lifecycle.

### 4.3 User registration (trace 3)

No changes. The participant gets a `user_progress` row for this step like any other.

### 4.4 Navigation (trace 4 — User Navigating to Workflow Step)

The page component at the workflow step renders the participant UI (§5.2). On load:

- If `tool_config.mode == Preview` (conversation not launched), show a "not yet started" placeholder.
- Else fetch `/prioritisation/polls/:poll_id/participant`.
- Status flows: `not_started` → `in_progress` on first proposal opened → `done` on successful `POST /submit`.

### 4.5 Launch (trace 5)

`workflow_step::launch` calls `preview_tool_config.clone_tool` (existing step 5e). `PrioritisationToolConfig::clone_tool` performs the preview→live deep copy described in §3.5. The result is written into `workflow_step.tool_config`. The participant routes resolve to the live `poll_id`; the facilitator's authoring UI continues to point at the preview `poll_id`.

> Implication for re-launches: editing the preview poll after a conversation has gone live does *not* affect the running live poll. This matches the rest of the system.

### 4.6 Real-time (trace 6)

The existing `user_started_workflow_step` / `user_finished_workflow_step` events are sufficient for general step tracking. The new `prioritisation_results_updated` event (§3.4) is published by the answer-submit handler and consumed by the facilitator's Poll Management page.

---

## 5. Frontend (SvelteKit + shadcn)

### 5.1 Route map

All under the existing conversation/workflow nesting; concrete URLs adapt to your conventions.

**Facilitator (authoring)**
- `…/steps/:step_id/prioritisation` — landing / "Prioritisation Tool — Create Poll" (iPhone 17 - 2 / 16 / 17)
- `…/steps/:step_id/prioritisation/edit` — Create-a-poll editor (iPhone 17 - 18)
- `…/steps/:step_id/prioritisation/proposals/new` — Add a proposal (iPhone 17 - 19)
- `…/steps/:step_id/prioritisation/proposals/:proposal_id` — Edit proposal
- `…/steps/:step_id/prioritisation/proposals/:proposal_id/questions/new` — Add a question (iPhone 17 - 20, 21)
- `…/steps/:step_id/prioritisation/preview` — Facilitator preview (iPhone 17 - 14, 15)

**Facilitator (live)**
- `…/steps/:step_id/prioritisation/manage` — Poll Management (iPhone 17 - 9, 25)
- `…/steps/:step_id/prioritisation/result` — Full-screen result page (iPhone 17 - 24)
- `…/steps/:step_id/prioritisation/qr` — QR code / join-code display, designed for projection (MacBook Air - 1, iPhone 17 - 8, 22)
- `…/steps/:step_id/prioritisation/report` — Report editor (iPhone 17 - 26, 27, 28)
- `…/steps/:step_id/prioritisation/report/preview` — Report preview (iPhone 17 - 29, 30)

**Participant**
- `…/steps/:step_id/prioritisation` — Welcome / intro card (iPhone 17 - 12)
- `…/steps/:step_id/prioritisation/proposals/:proposal_id` — Answer one proposal (iPhone 17 - 10, 11)
- `…/steps/:step_id/prioritisation/review` — Review your answers
- `…/steps/:step_id/prioritisation/submitted` — Thank-you screen (iPhone 17 - 13)
- `…/steps/:step_id/prioritisation/report` — Shareback (iPhone 17 - 29, 30) — after end

**Public join**
- `poll.comhairle.scot/:join_code` — resolves to the participant route after auth
- `poll.comhairle.scot/:join_code/qr` — QR-display version for projection (no auth)

### 5.2 Component inventory

Built from shadcn primitives. Component names use the project's PascalCase convention.

**Authoring**

- `PrioritisationLanding` — empty-state card with "Create Poll" CTA and "View all polls" link. Reused on the workflow step landing when no proposals exist yet.
- `PollEditor` — wraps the whole "Create a poll" form. Children:
  - `PollTitleField` — `Input`; label "Title", helper "What is this poll called?"
  - `PollInstructionField` — `Textarea`; label "Instruction"
  - `ProposalList` — sortable list of `ProposalListItem`s + "+ Add proposal" button (`Button variant=outline`). Empty-state hint "Add at least two proposals" doubles as the validation message that disables Publish.
  - `QuestionList` — same pattern; lives in the poll editor as the per-proposal question template (every proposal shares this template). "+ Add questions" CTA. Empty state shows the helper "Add the questions you want the participants to answer in each proposal."
  - `PollSettings` — collapsible section. Contains:
    - `TimerField` — `Select` with options `Forever`, `5 min`, `10 min`, `15 min`, `30 min`, `60 min`, `Custom…`
    - `ProposalSortField` — `Select` exposing `proposal_sort_mode` (advanced; defaulted to `by_proposal_id`)
- `ProposalEditor` — full-page form: title, content (rich text), image upload, and a nested `QuestionList` if we want per-proposal overrides (decision: **no**, questions are poll-level only in v1; this keeps the data model clean and matches the wireframe's "Add the questions you want the participants to answer in each proposal" phrasing on the poll editor).
- `QuestionEditor` — modal or stacked screen. Contains:
  - `QuestionTypePicker` — radio list of the 5 types (iPhone 17 - 21). Use shadcn `RadioGroup`.
  - `QuestionPromptField`, `QuestionDescriptionField`
  - Type-specific config sub-components: `MultipleChoiceConfig` (list of choices with auto-letter prefix, add/remove), `RatingScaleConfig` (min, max, step, endpoint labels), `FiveStarRatingConfig` (empty), `LongTextConfig`/`SingleLineTextConfig` (optional max length).
  - `IsOptionalCheckbox`
  - Header actions: `Duplicate`, `Remove`.

**Rich text**

- `RichTextEditor` — used for proposal content and report page content. Use the same rich-text component already in the codebase (whatever Comhairle uses for the existing `Learn` tool's content authoring); spec only that it outputs Markdown stored on the relevant translation row.

**Live management**

- `PollManagementHeader` — title + current state badge (`draft`/`published`/`paused`/`ended`).
- `PollControls` — `Pause the poll` / `Resume the poll` (toggles based on state), `Set timer`, `End the poll` (destructive, requires `AlertDialog` confirmation).
- `PollOverviewCard` — read-only key/value: Poll title, Status, Time left (with inline "Edit time left"), Number of completion ("28 completed").
- `InviteParticipantsCard` — QR-code URL + copy buttons. Uses an existing copy-to-clipboard pattern.
- `LiveResultsPanel` — `Sort by` `Select`, list of `ProposalResultRow`s.
- `ProposalResultRow` — proposal title + horizontal bar(s). One bar per choice for multiple-choice; single bar for slider/rating with a value label.
- `PresentResultCard` — "Present result after poll ends" affordance with the static report link and "Go to report editor".

**Participant**

- `PollIntroCard` — title, instruction, big `Enter` button (iPhone 17 - 12).
- `ProposalAnswerPage` — top progress bar (one segment per proposal), proposal title, image, content (rendered Markdown), list of `QuestionField`s, Previous/Next nav.
- `QuestionField` variants:
  - `MultipleChoiceField` — radio list with letter chips on each option (shadcn `RadioGroup` + custom prefix).
  - `RatingScaleField` — `Slider` with endpoint labels above/below.
  - `FiveStarRatingField` — five interactive stars.
  - `LongTextField`, `SingleLineTextField`.
- `ReviewAnswersPage` — collapsible per-proposal blocks listing the participant's saved answers. Edit jumps back to that proposal.
- `SubmittedPage` — thank-you (iPhone 17 - 13).
- `ShareBackPage` — paginated report viewer (iPhone 17 - 29, 30); uses the same pagination dot indicator as the proposal flow.

**Join**

- `LivePollJoinPage` — used on the projector (`MacBook Air - 1`): poll title, large QR code, fallback URL + 5-digit code in green.
- `LivePollJoinMobile` — used by participants who manually entered the URL: `Live poll` label, title, "Join the poll", `Enter poll number` input (iPhone 17 - 22).

### 5.3 State and data fetching

- Use SvelteKit `load` functions for each route, returning the typed API responses.
- The facilitator's Poll Management page subscribes to the existing WS service; on `prioritisation_results_updated` it invalidates the `/results` query.
- The participant's `ProposalAnswerPage` debounces answer-saves (~500 ms) and writes to `PUT /proposals/:proposal_id/answers`. The Submit button is gated by an in-memory check that every non-optional question on every proposal has a value; on `POST /submit` the server is authoritative.
- Timer rendering: client computes time left from `published_at`, `timer_seconds`, `paused_accumulated_seconds`, and `paused_at` (if currently paused, freeze). The server returns these fields; no per-second WS heartbeat.

### 5.4 Validation rules

| context | rule | UI behaviour |
| ------- | ---- | ------------ |
| Publish poll | ≥ 2 proposals required | "Publish" button disabled, helper text "Add at least two proposals" |
| Publish poll | ≥ 1 question required | similar disabled state |
| Publish poll | poll title non-empty | inline error |
| Save proposal | proposal title non-empty | inline error |
| Save question | prompt non-empty | inline error |
| Save multiple-choice question | ≥ 2 choices | inline error |
| Submit poll (participant) | every non-optional question on every proposal answered | Submit disabled with explanation; Review screen highlights missing items |

### 5.5 Empty / loading / error states

- Poll not yet launched (participant view, mode=preview): card "This poll isn't open yet. Check back when the facilitator starts it."
- Poll paused (participant view): card "The facilitator has paused this poll. Your answers are saved."
- Poll ended (participant view): redirect to Shareback if a published report exists, else "This poll has ended."
- No results yet (facilitator real-time view): "Waiting for participants — share the QR code to begin."

---

## 6. Cross-cutting

### 6.1 Join codes and shareable URLs

- 5-digit numeric, zero-padded (`04136`). Collision-checked at generation.
- Issued on `setup` (the preview poll has a code). On `clone_tool` we can either (a) generate a fresh code for the live poll, or (b) reuse the preview's code by transferring it.
  - **Recommendation: reuse.** Facilitators may share the QR code before launching. The preview's code persists into the live poll; the preview is allocated a new code post-clone so re-launch keeps the same expectation.
- Base URL `poll.comhairle.scot` is shown in wireframes; treat as a configurable env-var (`PUBLIC_PRIORITISATION_JOIN_URL`).
- Three URL variants per the wireframe: `/:code`, `/:code/qr`, `/p/:code/qr` (the wireframe shows both `/04136/qr` and `/p/04136/qr` — pick one; recommend `/:code/qr` for the projector view and `/:code` for participants).

### 6.2 Translations and i18n

Every facilitator-authored text (poll title, instruction, proposal title/content, question prompt/description, multiple-choice choice label, rating-scale endpoint labels, report page content) goes through the existing `new_translation` flow against the conversation's `primary_locale`. This mirrors trace 1b / 2b.

### 6.3 Accessibility

- All slider, radio, and star controls must be keyboard-operable and have visible focus.
- Multiple-choice letter chips (A/B/…) are decorative; the underlying radio's accessible name is the choice label only.
- Star rating uses `role="radiogroup"` with one radio per star and an accessible name "X out of 5".
- QR-code projector view: large type, ≥ AA contrast, no animation.
- Pause state announced via `aria-live="polite"` on the participant page.

### 6.4 Authorisation

- All facilitator routes: gated by existing workflow-facilitator role.
- All participant routes: gated by an active `user_participation` row on the parent workflow.
- The unauthenticated `poll.comhairle.scot/:code` route resolves the code → workflow → conversation, and either redirects to login (workflow requires auth) or proceeds (workflow allows guest participation — same policy as other Comhairle tools).

### 6.5 Deletion and cascade

- Deleting a workflow step → cascades to its preview poll and (if launched) live poll → cascades to proposals, questions, answers, submissions, report, report_pages. Translations are owned by their referencing rows; follow the project's existing translation-cleanup pattern.

### 6.6 Audit / observability

- Log on every state transition (`info!("Poll {} {}", poll_id, transition)`).
- Emit one metric per submission and one per result-update WS broadcast for backpressure monitoring.

---

## 7. Open questions (called out for discussion)

These map directly to the "Things to discuss" and inline notes in the wireframes.

1. **Naming.** `Prioritisation Tool` vs `ComhairleVote` vs other. Affects landing card copy only; identifier remains `prioritisation`.
2. **Facilitator-visible real-time results during the live poll.** Andy's note ("most of the case, we wouldn't want this to be visible to participants") confirms facilitator-only for v1. Spec assumes this. A future toggle on the poll could expose them to participants.
3. **Proposal sorting in participant view.** Spec defaults to `by_proposal_id` for participants and exposes `proposal_sort_mode` only on the facilitator dashboard. Confirm.
4. **Direct ranking interaction.** The final wireframe sketch shows participants directly arranging proposals by priority (drag-and-drop). This is a different interaction model from the per-proposal questionnaire above. Spec recommends keeping v1 to the questionnaire model and treating direct ranking as a sixth question type or a separate poll mode in v2.
5. **Async use case.** Andy's note on async + "we hear from teachers, government officials…" implies the Report editor (§5.2 `ShareBackPage`) is the place where async narrative lands. Spec includes the editor and Shareback view; no separate async data flow needed.
6. **Comment visibility between participants.** Notes raise "should people see others' comments when they are making a comment? Maybe not." Spec assumes comments are private to facilitators in v1. They surface in the result panel and report editor but not the Shareback.
7. **Categorise the summary.** Out of scope for v1; the Report editor is freeform pages.
8. **Stakeholder/delegate evaluation use case.** Architecturally the same as the regular participant flow; the difference is policy (who is on `user_participation`). No spec change needed.
9. **Direct edits like Luma.** Wireframe note "Consider using less labels and inputs (direct edits - like Luma)" — a styling decision for the Poll Editor, not a data-model one. Recommend tackling after the data layer lands.
10. **Image source for proposals.** Wireframes show stock images on the public car park / playground proposals. Spec assumes facilitator-uploaded assets; no AI-image or stock-library feature in v1.

---

## 8. Implementation order (suggested)

1. Migrations for the seven new tables; FKs and indexes.
2. Rust `PrioritisationToolConfig` + `ToolConfig` enum variant + `setup` / `clone_tool` / `delete` implementations.
3. Facilitator authoring API (§3.1) and the basic Poll Editor / Proposal Editor / Question Editor screens.
4. Participant API (§3.2) and answer flow screens.
5. Launch integration into `workflow_step::launch` (already covered by the existing `clone_tool` dispatch — just verify).
6. Real-time results: aggregation endpoint + WS event + facilitator dashboard.
7. Pause / resume / end + timer.
8. Report editor + Shareback.
9. QR/join projector view + public join route.
10. Accessibility pass, validation polish, empty/error states.