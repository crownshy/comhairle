# ADR-0051: Demographic forms pin question versions

**Status:** proposed
**Date:** 2026-10-09

## Context

The prototype UI says "Changes apply to every form that uses this question". Editing a
question changes every form and every conversation that uses it, including live ones.

The brief asks for forms to be versioned, with v2 created on re-publish. A form's
participants should not see a question change partway through a conversation.

## Decision

**1. A Form version pins the Question versions it contained when it was published.** Editing
a Demographic question creates a Question draft. Nothing changes for forms or conversations
until the question is published, and a form only picks up the new version when it is
re-published.

**2. Publishing a form uses each question's latest published version.** Unpublished Question
drafts are ignored. The form does not wait for them and does not publish them.

**3. Form steps can choose the latest Form version or a pinned one.** A pinned step keeps its
Form version even when newer ones exist.

## Consequences

- Each Form version stores its own question-version list, so the schema grows a join table
  per form version.
- Admins may be surprised that a question edit does not show in a form until it is
  re-published. The admin UI should say which forms are behind.
- Responses are keyed to Question versions, so answers from one version are not reused for
  another (see ADR-0053).
