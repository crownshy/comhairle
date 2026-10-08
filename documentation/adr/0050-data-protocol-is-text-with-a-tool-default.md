# ADR-0050: The Data protocol is text with a tool default

**Status:** proposed
**Date:** 2026-10-07

## Context

Each step had a "Data protocol" dropdown with four levels: Confidential, Restricted,
Collaborative and Open. Only the first two did anything. They set
`request_user_share_permission`, which only Thinking Space reads, to show its consent prompt.
Participants never saw the level, so it told them nothing about how their answers are used.

Issue #666 asks for a field where admins say how a step's data is used, shown to participants,
with a default for each tool. The thinking behind it is in #642.

## Decision

**1. The Data protocol is a translatable rich text field on the step, `workflow_step.data_protocol`.**
It is nullable, like the conversation's privacy policy.

**2. Blank shows the tool's default; the default is not copied in.** Defaults live in paraglide
messages keyed by tool, so fixing the wording fixes every step that never customised it, and
the defaults get translated once. Admins who want to edit it press "Start from the default",
which copies it into the field.

Rejected: **copying the default into every new step.** Old steps would keep stale wording and
every step would carry its own translation rows for text nobody touched.

**3. The four-level ladder is gone.** Thinking Space keeps its consent prompt as a plain switch
shown only on Thinking Space steps.

**4. Participants read it in the Find out more Privacy tab, above the conversation's privacy
policy.** A link from the step bar is the next step; design is sketching options for it.

**5. Admins can edit it while the conversation is live.** Locking it, and the privacy policy,
once live is an open question on #666.

## Consequences

- The default wording was agreed on #666. It is English only until it gets translated.
- `request_user_share_permission` keeps its name, though it now only backs the Thinking Space
  switch.
