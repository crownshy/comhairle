# Context

Glossary for terms that are easy to confuse when discussing the admin UI and conversation lifecycle. Implementation lives in code; this file is only the language.

## Conversation

A single engagement run by an organisation. Has a lifecycle: pre-launch → launched → complete. Identified by `conversation_id` in routes.

## Workflow

The ordered set of [Steps](#step) that a participant moves through inside a [Conversation](#conversation). A conversation has one workflow.

> Previously surfaced in the UI as **Design**. The route path remains `/admin/conversations/{id}/design`; only the user-visible label changes to *Workflow*.

## Step

One stage of a [Workflow](#workflow). A step is bound to a single *tool* (Polis, HeyForm, Learn, Lived Experience, Elicitation Bot, Thinking Space, Prioritization). Identified by `workflow_step_id` and ordered by `stepOrder`.

A step is the unit a participant interacts with at a single point in time, and the unit reordered in the admin step strip.

## Section / Tab

The top-level admin views inside a [Conversation](#conversation): Configure, Workflow, Knowledge base, Events, Recruit, Monitor, Moderate, Notify, Report.

Sections are *not* [Steps](#step). A section is an admin concern; a step is a participant-facing stage.

> The admin UI now renders sections as a horizontal tab bar. The legacy term "conversation steps" sometimes referred to *sections* (see [conversation-steps.ts](ui/packages/comhairle/src/lib/config/conversation-steps.ts)); that overload is being retired.

## Recruit

The admin section for getting participants into a conversation (email invites, open links, physical materials).

> Route path remains `/admin/conversations/{id}/invites`; only the user-visible label changes to *Recruit*.

## Launched

A conversation that has been published and is accepting participants. Distinct from *complete* (closed to participation).

See [ADR-0001](documentation/adr/0001-admin-ui-no-launch-gating.md) for current policy on editing a launched conversation.
