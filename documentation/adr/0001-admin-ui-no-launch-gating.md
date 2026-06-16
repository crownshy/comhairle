# ADR-0001: Admin UI drops edit-protection gating but keeps data-availability gating

**Status:** Accepted
**Date:** 2026-06-16

## Context

Until this rework, the admin UI gated each conversation section by an `activeStatus` flag — `PreLaunch`, `Launch`, or `Both` — defined in [conversation-steps.ts](../../ui/packages/comhairle/src/lib/config/conversation-steps.ts). One flag conflated two different reasons to gate a section:

1. **Edit protection.** Sections marked `PreLaunch` (Workflow, Knowledge base) became read-only after launch, to protect in-flight participant journeys from disruptive edits — reordering steps mid-conversation, changing the knowledge base under participants' feet, etc.
2. **Data availability.** Sections marked `Launch` (Recruit, Monitor, Moderate, Notify, Report) were inaccessible before launch, because their content depends on participant activity that doesn't exist yet — there are no participants to recruit, no statements to moderate, no metrics to monitor.

In practice the first kind hurts: admins regularly need to tweak a launched conversation (typo fixes, copy changes, swapping a misbehaving question). A long-standing bug in the Workflow prev/next arrows already lets them bypass it, and the team has been relying on that bypass. A proper role-based gate (super-admin edits anything; regular admin is restricted) is planned but not built.

The second kind doesn't hurt — it accurately reflects that the section has nothing to show pre-launch.

## Decision

For the v1 admin UI rework: **remove edit-protection gating, keep data-availability gating.**

| Section | Behaviour |
|---|---|
| Configure, Events | No gating (unchanged) |
| Workflow, Knowledge base | No gating *(was: read-only after launch — gating removed)* |
| Recruit, Monitor, Moderate, Notify, Report | `requiresLive: true` — disabled in the tab bar until launch |

Concretely:

- The `activeStatus` enum is gone. A single `requiresLive?: boolean` flag on `ConversationSection` expresses data-availability gating only.
- The tab bar renders all nine tabs in the same order at all times. Tabs whose `requiresLive` is true render as disabled (`aria-disabled`, dimmed, no navigation) until `conversation.isLive`.
- The prev/next "bypass bug" for Workflow editing stops being a bug — letting admins edit a launched workflow is now intended.

## Consequences

**Accepted risks:**

- An admin can reorder, rename, or delete workflow steps in a running conversation. This may invalidate participant progress or break in-flight responses. The team accepts this risk because the cost of *not* being able to edit has been higher in practice than the cost of the occasional bad edit.

**Reversal path:** When the super-admin role lands, edit protection returns — but as a *role* check rather than a *state* check. Regular admins will be re-gated to the pre-2026 behaviour or a stricter subset; super-admins remain ungated. This ADR should be superseded at that point, not deleted. The `requiresLive` flag is orthogonal to that change and stays.

**Out of scope here:**

- Backend authorisation. This ADR is about UI affordance only. The API has always permitted these edits; nothing changes server-side.
- Route-level guards. `requiresLive` is enforced in the tab bar only; deep-linking to e.g. `/monitor` pre-launch still loads the page (it will just render empty state). If that becomes a problem, add `+page.ts` redirects.
- A "warn before editing a launched conversation" confirmation. May be added later as a softer guard; not part of this ADR.
