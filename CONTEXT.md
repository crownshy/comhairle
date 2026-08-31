# Comhairle

Comhairle is a platform for consultation and deliberation at scale. Admins (policy makers) create Conversations that route participants (citizens, stakeholders) through a sequence of engagement tools.

## Language

**Conversation**:
A single consultation/deliberation instance that participants take part in and admins configure. Created in a draft state (`is_live: false`) and later launched.
_Avoid_: Topic (used loosely in the UI/design), consultation

**Workflow**:
The ordered sequence of Steps a Conversation routes participants through. Every Conversation has exactly one active Workflow.

**Step**:
A single stage in a Workflow, backed by one engagement tool (learn/onboarding, survey, Pol.is poll, prioritisation, etc.).

**Design board**:
The primary editing surface of a Conversation's Workflow, on the `…/design` route. A horizontal, left-to-right sequence of Step cards with a persistent left **Tool palette**. Historically labelled "Manage"; the tab is now labelled **Design**.
_Avoid_: Manage (old label), canvas.

**Tool palette**:
The persistent left rail on the Design board listing the addable engagement tools. Hovering a tool reveals an **Add** action that eagerly appends a Step; hovering a tool's name shows a tooltip (title, one-line description, "Learn more" → Tools Guide in a new tab). Replaces the old modal (`ToolSelectionModal`). Each tool has an internal key (used to build `tool_setup`) distinct from its **display name** shown to admins (e.g. key `Polis` → "Wiki Poll (Pol.is)", `Learn` → "Rich content page", `Prioritization` → "Prioritisation tool", `Thinking Space` → "Thinking space").
_Avoid_: Tool selection modal (being removed).

**Estimated time**:
A per-Step duration in minutes shown as a card pill when the board's "Estimated time" toggle is on. Distinct from the typical-duration ranges described narratively in the Tools Guide.
_Status_: Not backed yet — displayed from a hardcoded per-tool default for now; a real editable `estimated_minutes` (nullable) column is deferred pending the same team decision as [[data-protocol]] (flagged on the PR).

**Tools Guide**:
The admin reference pages at `/admin/info/tools/<key>`, one long-form editorial page per tool (sections: What you need to know, How it works, Mostly used in, Data collection and analysis, A typical participant experience, How to set this up, The open source tool we use). Opened in a new tab from a tool's "Learn more" link.

**Workflow template**:
A named, pre-defined set of Steps used to seed a new Conversation's Workflow at creation time. Applied once at creation; the resulting Workflow is then freely editable. A template has *display* content (name, description, badges, preview, step list shown on its card) that is distinct from its *creation* steps (the real `tool_setup` configs instantiated on "Get started"). Display content may reference steps whose backing tool does not yet exist (e.g. "Online video conference"); such steps are shown but not instantiated. Template content is provisional pending product decisions.
_Avoid_: Preset
_Note_: Templates are no longer create-time-only. The design board's "Template" dropdown re-applies a template to an existing Workflow: it **destructively replaces** the whole Workflow (deletes every Step + its data, then creates the template's Steps) behind an "are you sure" confirmation. This reverses ADR-0002's create-time-only stance (see the amendment there). The Workflow still does not persist which template was applied, so the chip label is session-local (resets to "Blank" on reload).

**Start from blank**:
Creating a Conversation immediately (with an auto-generated title and an empty Workflow), then editing it on the configure tab. Create-then-edit, not fill-then-create.
_Avoid_: Empty workflow (that's the underlying template key)

**Data protocol**:
A per-Step declaration of who may see the data participants produce in that Step. Canonical four-level ladder, least-to-most open: **Confidential** (no one) → **Restricted** (organiser only) → **Collaborative** (organiser + other participants) → **Open** (everyone).
_Avoid_: Private/Limited (stale Learn-guide wording for Confidential/Restricted), "data sharing", "data policy".
_Status_: Only Confidential and Restricted are backed today — they map onto the existing `request_user_share_permission` boolean (`false`→Confidential, `true`→Restricted). Collaborative and Open appear in the UI for design fidelity but are disabled pending a team decision on introducing a real `data_protocol` enum column (flagged as an open question on the PR).

**Role assignment**:
An explicit grant that links an actor (user or organisation) to a named role on a resource (`resource_type` + `resource_id`). Role assignments are durable records and form the source of truth for authorization.

**Permission action**:
A single allowed operation (for example list, grant, revoke, read, update) that routes enforce. Roles grant sets of permission actions; authorization succeeds when any assigned role on the target resource grants the required action.

**Authorization precedence**:
Permission checks resolve in this order: resource ownership allows, then system admin grant allows globally, then role-action mapping on the target resource is evaluated. There are currently no explicit deny rules.

### Participant journey

**Finished** (a participant is finished):
The point at which a participant has a `done` progress row for **every** Step in a Workflow. Reaching it is what lands them on the Thank You page. Distinct from a Conversation being **complete** (`is_complete`), which is an admin closing the whole Conversation, and from a single Step's `progressStatus: 'done'`. Matches what participants see: the last step's button is labelled "Finish".
_Avoid_: Submission (already means one HeyForm survey response - see `tools/heyform.rs`), completion (collides with `Conversation.is_complete`).
_Note_: Skipping an optional Step still writes `done`, so a participant who skips everything has still finished.

**Sealed**:
A participant who has [[#finished-a-participant-is-finished]] in a Conversation with [[#revisit-after-finishing]] turned off. A sealed participant can reach no Step and comhairle rejects their writes. Sealed is **derived**, never stored: it is recomputed from the Workflow's Steps and that participant's progress rows. One consequence of deriving it: adding a Step to a live Workflow un-seals everyone who had already finished.
_Avoid_: Locked, closed (closed reads as the Conversation being closed).

**Revisit after finishing**:
The per-Conversation setting (`allow_revisit_after_finishing`, default `true`) governing whether a participant may return to Steps once they are [[#finished-a-participant-is-finished]]. Off means sealed. Orthogonal to the per-Step [[#revisitable-step]] flag, which governs navigation *before* they finish; the Conversation setting overrides it afterwards.

**Revisitable step**:
The per-Step `can_revisit` flag (default `false`), controlling whether a participant may navigate back to that Step once they have completed it. Governs mid-flow navigation only. Once a participant is [[#finished-a-participant-is-finished]] it is subordinate to [[#revisit-after-finishing]].

### Participant step chrome

The participant-facing frame around a Step, introduced by the mobile exploration and shared
by both breakpoints. Replaces the old `StepHeader` plus `StepSelector` pair. See
[ADR-0017](documentation/adr/0017-step-brief-slides-split-at-horizontal-rules.md) and
[ADR-0018](documentation/adr/0018-one-pager-innermost-first-navigation.md).

**Step brief**:
A Step's description *as presented to participants*: a sequence of [[#slide]]s rather than
one block of prose. The description is the source, the brief is the presentation. The brief
has exactly two surfaces, the [[#cover]] and the [[#hint]], and both show the same content.
_Avoid_: Intro, blurb.

**Slide**:
One screen of a [[#step-brief]]: the run of top-level rich-text nodes between two horizontal
rules in the Step description. Admins author a break by typing `---`. A description with no
horizontal rule is a single Slide. Slides are split per locale, so counts may differ between
languages.
_Avoid_: Page (the Learn tool's `tool_config.pages` already owns that word, and the pager
traverses both).

**Cover**:
The first [[#slide]] of a [[#step-brief]], shown as a full screen on entering a Step, before
the tool body mounts. Carries an illustration (the Slide's first image, else the tool's
icon) and a derived meta line (duration, and a per-tool count such as opinions or follow-up
questions). Its last Slide's forward control reads "Start".
_Avoid_: Intro screen (the prototype's name), splash.

**Hint**:
The pill in the centre of the [[#pager]] and the modal it opens, which reopens the whole
[[#step-brief]] mid-Step. Same content as the [[#cover]].
_Status_: The label is provisional and held in one constant. It says "Hint" because that is
what the Figma says, but what it opens is the full Step description rather than a nudge;
"About" and "Help" are the live alternatives, open for the team.

**Pager**:
The persistent bar at the bottom of the participant viewport: back on the left, [[#hint]] in
the middle, forward or Skip on the right. Its arrows traverse the innermost open sequence
first ([[#slide]]s, then a tool-internal sequence, then the Step boundary). The forward slot
states one thing at a time: Next, Skip, Start, or a disabled chevron.
_Avoid_: Footer (that is the site-wide `Footer.svelte`, which the workflow routes no longer
render), toolbar.

**Step dropdown**:
The chevron beside the Step label in the participant header, listing the Workflow's Steps
with their status. Replaces `StepSelector` at both breakpoints, including the desktop
horizontal stepper. Navigation permissions are unchanged: only completed, revisitable Steps
are links, and a [[#sealed]] participant's dropdown is read-only.

### Organizations and access

**Organization Administrator**:
A user explicitly assigned elevated permissions on one Organization, including organization update, organization delete, and organization member add/remove.
_Avoid_: Org owner, org contact, organization user.

**Organization Member**:
A user associated with an Organization for membership purposes, without implied administrative permissions.
_Avoid_: Organization admin (unless they also hold Organization Administrator assignment).

**Primary host organization**:
The single Organization linked directly on a Conversation as its primary institutional host. This remains distinct from both the Conversation owner (a user) and any co-hosting organizations.
_Avoid_: Co-host, conversation owner.

**Co-hosting organization**:
An Organization explicitly associated with a Conversation as an additional host beside the primary host organization. Co-hosting organizations are inferred by ownership of the Conversation co-host role.
_Avoid_: Primary host organization, member organization.

**Conversation co-host role**:
A conversation-scoped role intended for organization actors, granting read-only access (`ConversationRead`) by default.
_Avoid_: Content editor (that role implies update access).

**Organization contact email**:
A communication address for the Organization entity itself. It is not a permission grant and is distinct from both member emails and Organization Administrator emails.
_Avoid_: Admin email, owner email.

**Initial Organization Administrators**:
The set of users selected during Organization creation who receive Organization Administrator permissions for that new Organization.
_Avoid_: Default members, contact email recipients.

**Silent account bootstrap**:
When an Initial Organization Administrator email has no matching user account, the system auto-creates a full email-password user account without a blocking confirmation step, then assigns permissions.
_Avoid_: OTP-only bootstrap, manual pre-provisioning.

**Initial password reset email**:
The first-access email sent to newly created Organization Administrators at creation time, containing a one-time password reset link with 24-hour validity so they set their own password before first login.
_Avoid_: Magic-link sign-in email, plaintext password email.

### Polis admin (Discuss step)

The custom admin UI for a Pol.is Step, replacing the old Setup **iframe**. Its glossary and analytics are **adopted from the civic_os admin** (`bloom/civic_os/packages/admin`), taken as the source of truth because the equivalent surfaces were built out there first. Terms below carry civic_os's meaning unless noted. Canonical Polis-step subtabs: **`Configure · Setup · Moderation · Insights`** (civic_os's "Participants" tab is dropped — see Insights/Report and the Participants note).

**Moderation**:
The Polis statement-review subtab — sync statements from Polis, add/seed statements, CSV import, and accept/reject each with status filter chips (`all · seeded · accepted · pending · rejected`). Backed by the `polis_statement_aux` table. Supersedes the design's "Statements" label and the two dead entries in today's step editor (the `Statements` "coming soon" placeholder and the never-rendered `Moderate` subtab).
_Avoid_: Statements (design label), Moderate (old dead tab).

**Insights**:
A **per-Step report** — the read-only analytics surface for a single Step, shown as a subtab in that Step's editor. For Polis: Themes, Areas of Consensus, Areas of Difference computed over the Polis `report_data` export (ported from civic_os's `report.ts` pure functions). Insights is a general per-Step-report pattern: each Step type that supports reporting (Polis, Thinking Space, HeyForm, …; not Learn) gets its own Insights subtab.
_Avoid_: Report (that's the conversation-level tab — a different, global concept).

**Report**:
The **conversation-level, global** reporting tab in the top nav (`Configure · Workflow · Knowledge base · Events · Recruit · Monitor · Notify · Report`). Aggregates across the whole Conversation. Distinct from a Step's **Insights** subtab.
_Avoid_: Insights (that's per-Step).

**Participants** (Polis admin):
_Out of scope / deferred._ civic_os has a Participants tab (demographics + recruitment goals, tied to its "questions attached" model), but comhairle has no equivalent concept and no top-level Participants tab. Not built as part of the Polis admin; revisited later as a separate effort (possibly HeyForm-adjacent) once its home is decided.
_Avoid_: reusing civic_os's demographics Participants inside a Polis subtab.

**Setup**:
The Polis-step subtab replacing the old admin **iframe**: native controls for the Polis conversation config plus comhairle display flags. Two write paths — (1) **Polis-proxied** config (`is_active` = "conversation is open", `topic`, `description`, `strict_moderation` = "no comments without approval") goes through the `PolisUpdateConfig` route (widened `UpdatePollRequest` + server-side admin `login()` + `update_poll()`); (2) **comhairle `tool_config`** display flags (`required_votes`, `show_remaining_statements`, `label_seeds_as_conversation_starter`) via `UpdateConversationWorkflowStep`. No client-side Polis auth. The Polis-proxied fields are **also mirrored into `tool_config`** (they are fields on `PolisToolConfig`) so the form can pre-fill — Polis has no read path (see ADR-0003 amendment). Saving a config field writes Polis first, then the mirror. Seed authoring lives in the **Moderation** subtab, not Setup.
_Note_: `is_active` (Polis "open/closed") is **plumbed but not surfaced** in the Setup UI — participant access in comhairle is governed by conversation launch + the Workflow, not a per-Polis toggle, so exposing it would be redundant. The field stays on `PolisToolConfig`/`PolisUpdateConfig` (defaulted active) for future use.
_Status_: The design's "Participants can see the **visualization**" checkbox is **deferred** — the custom participant embed (`PolisEmbed.svelte`) renders no opinion-map, so the flag would control nothing. Building a participant PCA/opinion-map component (data via `get_math_pca`/`report_data`) is a separate follow-up, bundled with retiring the `PolisReport.svelte` iframe.

**Preview poll / Live poll**:
A Polis Step is backed by **two** separate Pol.is conversations: a **preview** poll (used while the Conversation is a draft — admin-only, for previewing and staging seeds) and a **live** poll (created at **launch**, where participants actually vote). The step editor targets the right one via `conversation.isLive` (`tool_config` = live, `preview_tool_config` = preview). Real participant voting is **post-launch only**.
_Status_: `launch` creates a fresh live poll and migrates only seed **text** (`post_seed_comment`) — it does **not** carry over aux metadata (`moderation_status`, `themes`), so pre-launch moderation/theming does not survive launch. It also does not yet filter rejected seeds (`// TODO: filter seed statements`). Both are follow-up backend fixes, not part of the tab UI work.

**Seed statement**:
A statement authored by the moderator (not a participant) to spark discussion, `is_seed: true`. Posted server-side to the active Polis poll (preview while draft, live after launch) via a new backend seed route wrapping `post_seed_comment`, then surfaced locally after sync. Distinct from `moderation_status` and from a [[#derived-statement]] (which is `is_seed: false`). Polis auto-approves a seed on post (`mod: 1`); a non-seed host post lands `mod: 0` (pending).
_Avoid_: Seeded status (it's a boolean flag, not a moderation status).

**Derived statement**:
A statement an admin authors while moderating, as a split or reword of an existing participant statement (see [[#split]]). Posted to Polis as `is_seed: false` (a real, votable, non-seed statement, not a host seed) and carries `original_statement_id` pointing at the aux row it was derived from. The discriminator: `is_seed: false` **and** `original_statement_id` set = derived; `is_seed: true` = seed; neither = raw participant statement. Never rendered as a host seed.
_Avoid_: calling it a seed; "edited statement" (Polis has no in-place edit — a derived statement is always a *new* statement).

**Split**:
The moderation act of replacing one participant statement with one or more clean, separately-votable [[#derived-statement]]s. A single-replacement split is a **reword** (disambiguating one statement); a multi-replacement split breaks a composite statement into parts. One operation either way: post each replacement `is_seed: false`, auto-accept it (`mod: 1`), reject the original (`mod: -1`), and record lineage. No automatic text-splitting: the admin types every replacement by hand.

**Polis statement aux**:
Comhairle's `polis_statement_aux` sidecar table, one row per Polis statement, holding admin-only metadata Polis doesn't store: `moderation_status`, human-authored `themes`, `is_seed`, `moderation_reason`, `visible_statement_when_submitted`, and `original_statement_id` (self-referential lineage for [[#derived-statement]]s). Populated by sync; comhairle-only fields (including `original_statement_id`) are preserved across re-sync, not overwritten from Polis.

**Moderation status**:
The three-value review state of a statement: `accepted · pending · rejected` (enum `ModerationStatus`). Not the same as `is_seed`.

**Theme**:
A human-authored topic tag string in `polis_statement_aux.themes: string[]`, added via the admin ThemePicker. Polis has no theme concept; sync never imports one. (Future: T3C may write machine themes into the same store.)

**Opinion group**:
A cluster of participants who voted similarly, discovered by Polis's math (PCA + k-means), labelled A/B/C… Backed by `GroupReportData` (`group_id`, `members`, `total_members`, `representative_comments`). A single Polis poll yields a **variable** number of opinion groups (typically 1–5, not a fixed two), so all report components render N groups, not a hardcoded A/B pair. **Not** a demographic segment, recruitment cohort, or any admin-defined set — it is derived purely from vote patterns.
_Avoid_: Group (bare — collides with invitee groupings), cluster, faction.

**Representative comment**:
A statement that most distinguishes an Opinion group from the others (Polis's "representative comments", per `GroupReportData.representative_comments`). Used in the report's Groups section to characterise what each group believes. Distinct from an [[#area-of-consensus]] statement, which is one *every* group agrees on.

**Area of Consensus** (shown as "Area of consensus" in Insights):
A statement all opinion groups agree on, ranked by Polis's `group_informed_consensus` (the product of each group's smoothed agree%, `(agrees+1)/(total+2)`, so it scores high only when *every* group agrees). The section lists all statements by this score, highest first, read straight off `report_data` (not recomputed). It is agree-oriented: it surfaces "all groups agree", not "all groups disagree".

**Area of Difference** (shown as "Area of disagreement" in Insights):
A statement the opinion groups split hardest on, ranked by Polis's `divisiveness`, highest first, read straight off `report_data` (not recomputed).

### Reporting

**Report component**:
The primitive of the reporting system: a self-contained, configurable widget fed by (usually) one tool's insight data, e.g. an "Areas of Agreement" list, an engagement stat card, a Prioritisation ranking, a beeswarm chart. The unit that gets built once per tool and reused. **Report views are compositions of report components** filtered by audience + timing; the components are the thing you design, the views are arrangements.
_Avoid_: widget, block, card (too generic).
_Status_: Partially skeletoned. Each tool folder already exports an (unused) `ReportUI` slot; only Polis has real components — the Insights set in `lib/reports/polis/**` (`VoteBar`, `StatementVoteBlock`, `AreaOfConsensus`) backed by `tools/polis/report.ts`. A separate set (`components/report/**` + `utils/report.ts`) feeds only the `/waves` mock.

**Embeddable section block**:
The subset of [[#report-component]]s a facilitator can pull into the End-of-engagement report from the editor. Section-level and self-contained (Polis: *Key stats*, *Areas of consensus*, *Areas of disagreement*, *Consensus continuum*, *Opinion groups*) — **not** the sub-primitives they compose from (`VoteBar`, `OpinionGroupCard`), and **not** the whole-page `PolisInsights` composition. Maintained as an explicit allow-list.
_Avoid_: report piece (use "section block" for the embeddable unit).

**Report component embed** (a.k.a. the snapshot node):
A TipTap node in the report's `summary` document that carries an embedded [[#embeddable-section-block]]. It stores both a **reference** (`toolStepId`, `componentType`, `config`) and the **frozen HTML** rendered from that component; the frozen HTML is what renders everywhere, the reference is the recipe for a future re-freeze / refresh (see [ADR-0012](documentation/adr/0012-report-component-embeds-store-reference-plus-frozen-html.md), building on [ADR-0008](documentation/adr/0008-report-pieces-embed-in-tiptap-as-frozen-snapshots.md)). Because the HTML is baked in, deleting the source Step does not blank the report — it only disables refresh.

**Report view**:
A composition of report components. There are exactly four, each a different audience × timing × scope arrangement over the shared per-tool components:
1. **Insights** — admin, live, per-tool/Step. A "summary of raw data": current responses + realtime insights; helps spot missing voices. (Already exists for Polis.)
2. **In-progress feedback** — participant, live, per-tool/Step; appears **as a Step in the participant journey**. The only view that shows **individual** data (the participant's own response) alongside the aggregate from others.
3. **Presentation mode** — public room screen, live, per-tool; a looping, simplified, low-interaction "highlights" display for a live audience. Either cycles all tools or shows one picked tool.
4. **End of engagement report** — participant + public, final (frozen snapshot), conversation-level cross-tool; **human-authored**: auto-generated insights that an editor curates in a rich-text (TipTap) document, pulling component blocks in.
_Avoid_: report type, report page, Monitor (the ops/funnel tab is a separate concern, not one of the four).
_Note_: Views 1–3 are system-defined compositions over one per-tool live insight producer; view 4 freezes that output and wraps it in author-edited prose.
