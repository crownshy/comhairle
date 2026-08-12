# ADR-0013: Learn-content sync uses draft content and flattens rich text

**Status:** accepted
**Date:** 2026-08-11
**Amends:** [ADR-0010](0010-learning-assistant-grounded-via-synced-kb-document.md)

## Context

ADR-0010 introduced grounding the Learning Assistant in learn-step content by syncing it
into the conversation's knowledge base as a reserved document (`learn-step-content.md`).
The first cut read each step's **published** `tool_config` and dumped each page's stored
text straight into the document.

Two problems surfaced in testing:

1. **You cannot publish without launching.** Promoting a step's draft
   (`preview_tool_config`) to published (`tool_config`) requires launching the conversation.
   So an admin building a conversation in preview had no published learn content, and a sync
   produced nothing - the assistant was untestable until launch. That defeats staged
   authoring.

2. **The synced document contained raw JSON.** Modern page text is stored as Rich (TipTap /
   ProseMirror) content, i.e. a JSON document. Passing it through untouched meant the KB
   document, its retrieval, and the source previews all showed `{"type":"doc",...}` instead
   of readable prose.

## Decision

**1. Sync reads the draft (`preview_tool_config`), not the published config.** Learn content
becomes available to the assistant the same way an uploaded knowledge-base document is:
immediately, before launch, with no publish step. This reverses ADR-0010's "published only"
choice.

Trade-off, accepted: in a live conversation the assistant can be grounded in edits a
participant cannot see on the page yet. Sync is a manual admin action, and matching the
immediacy of uploaded documents was judged more valuable than strict published-alignment.
The earlier "unpublished changes" warning is therefore removed - there is nothing to warn
about.

**2. Rich text is flattened to markdown before syncing.** `resolve_page_content` runs
`rich_text_to_markdown`, which walks the ProseMirror JSON (headings, paragraphs, lists,
hard breaks) into plain markdown and passes non-document content (plain / already-markdown)
through unchanged. The KB document is now readable text.

**3. Non-PDF *uploads* render in a text viewer.** The shared document viewer gained a `text`
kind (`TextViewer`) that fetches and renders `.md` / `.txt` as escaped, wrapped text (no
`{@html}`, so an uploaded file cannot inject markup). This is the fallback for genuine text
uploads; learn content itself uses (4).

**4. Learn-content sources render the real pages, not the flattened document.** The flattened
markdown exists only for RAGFlow retrieval and is never shown to a user. When an assistant
source is the learn-content document (matched by the reserved name), the viewer
(`LearnContentViewer`) instead fetches `GET /documents/learn_content` - the learn pages as
raw TipTap - and renders them through the normal `ContentRenderer`, exactly as a participant
sees them. The cited passage is highlighted by text-matching the retrieved chunk against the
rendered text (RAGFlow only returns highlight coordinates for PDFs, so a coordinate highlight
is not available here). This keeps markdown/PDF out of the user-facing surface while the app's
own rich-text renderer does the display.

## Consequences

- Admins can author and test the assistant entirely in preview.
- Retrieval quality improves (no JSON noise) and source previews are legible.
- The assistant may reflect unpublished draft content in a live conversation; revisit if
  participant-alignment becomes a requirement (e.g. a per-step "sync published only" toggle).
- Rich-text flattening is intentionally simple (no tables, marks, or images). Extend
  `rich_text_to_markdown` if richer structure needs to reach retrieval.
