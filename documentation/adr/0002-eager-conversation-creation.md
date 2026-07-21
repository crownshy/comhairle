# ADR-0002: Conversations are created eagerly, not after a form

**Status:** accepted
**Date:** 2026-07-06

## Context

The old "New conversation" flow was *fill-then-create*: a full-page form ([/admin/conversations/new](../../ui/packages/comhairle/src/routes/(admin)/admin/conversations/new/+page.svelte)) collected a title, short description, and workflow-template choice, and only persisted anything when the admin clicked "Create". The UX rework aims for *faster topic setup — users shouldn't have to decide their title/description beforehand*.

## Decision

Invert the flow to **create-then-edit**. The "New conversation" button becomes a dropdown with two options, both of which persist a Conversation immediately and then drop the admin on the conversation's **configure** tab:

- **Start from blank** — creates a Conversation with an auto-generated title (`Untitled YYYY-MM-DD h:mm:ssAM/PM`, local browser time), empty descriptions, and an empty Workflow. Seconds are part of the title because the backend slugifies the title under a unique constraint, so two same-minute creates would otherwise collide (`DuplicateSlug`).
- **Choose from templates** — opens a modal; "Get started" creates the Conversation the same way but seeds its Workflow from the selected template's *creation* steps.

Both entry points (sidebar + dashboard) share one component and one `createConversation` helper extracted from the deleted `/new` route.

## Considered options

- **Keep fill-then-create** (rejected): preserves clean data but keeps the upfront-naming friction the rework exists to remove.
- **Eager create** (chosen): delivers instant entry; the auto-title clears the existing `title min(1)` rule so the record is valid on creation.

## Consequences

**Accepted risk — abandoned drafts.** Every click of "Start from blank" (and every "Get started") mints a real Conversation, even if the admin immediately navigates away. These accumulate as `is_live: false` "Untitled …" records. We accept this for now; a later mitigation could filter or reap empty drafts.

**Descriptions stay required at save, not at creation.** Create-time sends empty `short_description`/`description` (the API applies no length validation), but the configure schema keeps `min(1)` on both — so descriptions are *deferred*, not abolished. A Conversation cannot be saved/launched without them.

**Bug fixed in passing.** The old create path hardcoded a sample-copy string into the real `description` field; eager creation stores genuinely empty descriptions and surfaces that guidance as field placeholders instead.

**Online video conference has no workflow tool.** The Stakeholder engagement template can't instantiate it as a Step, so it creates its **two backed steps (learn + prioritisation) plus one empty placeholder Event** (an online video meeting the admin fills in). Its card still previews the intended learn → video → prioritisation steps; the video step lands as the Event rather than a Step.

**Out of scope:** AI-generated / guided description drafting (noted as a future follow-up), and a real video-conference *workflow tool* (the placeholder Event is the interim stand-in).

## Amendment (design-board template re-application)

The "applied once at creation" stance above has since been **relaxed for the design board only**. The board's "Template" dropdown lets an admin re-apply a template to an existing Workflow, which **destructively replaces** it: every existing Step (and its configuration/data) is deleted, then the template's Steps are created, behind an explicit "this cannot be undone" confirmation dialog. The eager-creation flow is unchanged — this is a separate, deliberate, guarded action on an already-created Conversation. The Workflow still stores no reference to its source template, so the chip label is session-local.
