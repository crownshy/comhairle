# ADR-0017: The step brief is the step description, split into slides at horizontal rules

**Status:** proposed
**Date:** 2026-08-31
**Branch:** `mobile-exploration/participant-step-chrome`

## Context

The mobile exploration replaces the participant step chrome. Two of its screens, the
**cover** shown on entering a step and the **Hint** modal reopened from the pager, present
the same content as a sequence of full-bleed slides rather than as one block of prose. See
[CONTEXT.md](../../CONTEXT.md) for **Step brief**, **Slide**, **Cover** and **Hint**.

Today `workflowStep.description` renders inline under the step title in `StepHeader`, as
one undivided run of rich text. Nothing in the model says where one slide would end and the
next begin.

Three things constrain the answer.

**Descriptions are already structured.** `description` is stored as ProseMirror JSON (or
Markdown, or plain text, resolved by `detectContentType`) and rendered by
`renderRichTextToHtml`. It is a node tree, not a string, so a delimiter can be a node
rather than a character sequence someone has to parse out of prose.

**The editor already has a delimiter.** `getBaseExtensions` configures `StarterKit`, which
includes `horizontalRule`. Typing `---` in the step description field today already
produces a `horizontalRule` node. Admins have an authoring gesture for this before we write
any code, and it is the gesture they would guess.

**`description` has no draft/published split.** Tool configs have one
(`previewToolConfig` against `toolConfig`), but the step DTOs carry a single `description`
string. A saved description edit is immediately live to participants.

## Decision

**1. A slide is the run of top-level nodes between two `horizontalRule` nodes.** Splitting
walks `doc.content`, cuts at each `horizontalRule`, and rewraps each run as its own `doc`
for the existing `ContentRenderer`. The rules themselves are consumed and never rendered.

Rejected: a dedicated `slide` node type in the schema, and a real `slides` array field on
the step. Both are better models. Both also cost a TipTap extension plus admin UI, or an
API and data model change with a migration and a Rust round trip, in exchange for
expressiveness this exploration does not yet know it needs. Either can replace the splitter
later without the participant components changing, because they consume an array of
renderable slides and do not care how it was produced.

**2. A description with no horizontal rule is one slide.** This is what makes the change
safe to apply to the real routes with no gate: every existing step in every existing
conversation keeps working, and its cover shows its whole description.

Trade-off, accepted: any existing description that already uses `---` as a visual divider
silently becomes multi-slide. We could not audit production from the branch. The failure is
cosmetic and self-announcing, an unexpected page break rather than lost content.

**3. Splitting happens per locale, independently.** `description` is translatable, so
English may yield three slides where Gaelic yields two. Slide counts are a property of a
locale's content, not of the step. Forcing them to agree would mean either constraining
translators or dropping content.

**4. The description is the source; the brief is the presentation.** One field feeds both
the cover and the Hint modal. There is no second field, and the description no longer
renders inline anywhere. A participant who never opens the Hint has still seen slide one,
because it is the cover they passed through to reach the step.

**5. Slide furniture is derived, not authored.** The cover's illustration is the first
image node in the slide if there is one, falling back to `TOOL_META.icon`. The meta line is
derived from real per-step config (`required_votes`, `follow_up_rounds_count`) plus
`TOOL_META.estimatedMinutes`, which is the same hardcoded map the design board's "Estimated
time" pill already reads. Admins get one source of truth per fact rather than retyping
counts into prose that then drifts from the config.

## Consequences

- Admins author slides with an invisible gesture. Typing `---` changes the participant UI
  and nothing in the editor says so. The admin preview panel on the step's `configure`
  subtab exists to close exactly this gap, which is why it is on the same branch and not a
  later one.
- Every step now opens on a cover. Steps whose description is one short sentence will have
  a thin cover. That is real information about the content, surfaced rather than hidden.
- `StepHeader`'s inline description rendering becomes dead and is removed.
- When `estimated_minutes` lands as a real column, the meta line and the design board pill
  switch over together, because they read the same map.
- The splitter is pure and covered by unit tests. It is the one piece here with a
  meaningful edge case surface: leading rules, trailing rules, consecutive rules, empty
  runs, and non-JSON descriptions that arrive as Markdown or plain text.
