# ADR-0009: The Learning Assistant is gated on parsed knowledge base documents

**Status:** accepted
**Date:** 2026-07-23

## Context

The participant-facing **Learning Assistant** (a Q&A helper that answers from a conversation's
knowledge base via server-side RAG) was shown purely on the strength of two conversation flags:
`chatBotId` and `enableQaChatBot`. Nothing checked whether the knowledge base actually had any
usable documents.

When an admin enabled the assistant but had uploaded no documents (or the documents were still
parsing), the backend RAG call failed and streamed its failure back as a normal answer payload. The
participant saw a raw `**ERROR**: 'id'` bubble where an answer should be. This was the bug in
[#373](https://github.com/crownshy/comhairle/issues/373).

Two participant surfaces render the assistant, and each independently client-fetched
`ListDocuments`:

- the workflow **step page**
  ([s/[workflow_step_id]/+page.svelte](../../ui/packages/comhairle/src/routes/(public)/conversations/[conversation_id]/[[preview]]/workflow/[workflow_id]/s/[workflow_step_id]/+page.svelte)),
  which renders `LearnUI` (itself also fetching the same list), and
- the **support sidebar**
  ([ConversationSupportSidebar.svelte](../../ui/packages/comhairle/src/lib/components/ConversationSupportSidebar.svelte)),
  in the shared workflow layout.

## Decision

**Show the Learning Assistant only when the knowledge base has at least one fully parsed
(`parse_status === 'DONE'`) document, and enforce that from a single source.**

- The document fetch is hoisted into the nearest shared ancestor,
  [workflow/[workflow_id]/+layout.ts](../../ui/packages/comhairle/src/routes/(public)/conversations/[conversation_id]/[[preview]]/workflow/[workflow_id]/+layout.ts).
  It returns `availableDocuments` (the DONE-filtered list) and `hasKnowledgeBaseDocs`
  (`availableDocuments.length > 0`). A failed fetch falls back to "no documents", which safely
  hides the assistant instead of surfacing a backend error.
- Both surfaces read that one value: the step page passes it down to `LearnUI`; the sidebar takes
  it as a prop from the layout. The two per-component `ListDocuments` `$effect` fetches are removed.
- "Nothing uploaded" and "uploaded but still parsing" both resolve to hidden — a still-parsing
  document cannot answer questions yet, so showing the assistant would reproduce the bug.

This is a "gate only" fix: it does not add defensive handling for an `**ERROR**:` payload arriving
*with* documents present. "No usable documents" is treated as the sole trigger for that payload.

On the admin side, the enable toggle moves onto the **Knowledge base** tab next to the documents
that make it work, and shows an inline hint when it is on but no document has parsed yet. See
ADR-0006, which listed `enableQaChatBot` among the Configure **Access** toggles; that toggle no
longer lives on Configure. (Renaming the tab to "Learning Assistant" is a separate branding change
kept out of this PR to avoid bloating it.)

## Considered options

- **Gate + defensive error-payload handling** (rejected for this ticket): also route any
  `**ERROR**:`-prefixed answer into the existing error box. Belt-and-suspenders, but only warranted
  if no-docs is *not* the sole cause; a separate ticket if such a payload is ever seen with docs
  present.
- **Duplicate the DONE-doc check in each surface** (rejected): smaller diff, but two places that
  drift. The hoisted single source matches the repo's SvelteKit convention (hoist a shared fetch to
  the nearest common layout `load`, read via `await parent()`).
- **Hard-disable the admin toggle until a document parses** (rejected): stronger guarantee but
  fights the natural "enable, then add documents" flow and reads as an unexplained dead control. An
  inline hint gives the same feedback without blocking.

## Consequences

- **Enabling the assistant with an empty knowledge base is now a silent no-op for participants** (by
  design). The admin-side inline hint is what tells the admin why, so that feedback must stay
  accurate — it uses the same `parse_status === 'DONE'` definition as the participant gate.
- **The gate depends on `ListDocuments` succeeding during the layout `load`.** A transient failure
  hides the assistant for that navigation rather than erroring; acceptable, and self-heals on the
  next load (the fetch is keyed by `depends('app:documents')`).
- **The internal flag is still named `enableQaChatBot`.** Renaming it to `enable_learning_assistant`
  is a coordinated full-stack change (DB migration + Rust + frontend) deferred to
  [#736](https://github.com/crownshy/comhairle/issues/736).
