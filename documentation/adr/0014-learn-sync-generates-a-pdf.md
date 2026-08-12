# ADR-0014: Learn-content sync generates a text-bearing PDF

**Status:** accepted
**Date:** 2026-08-11
**Supersedes:** parts 2 and 4 of [ADR-0013](0013-learn-sync-uses-draft-content-and-flattens-rich-text.md)
**Amends:** [ADR-0010](0010-learning-assistant-grounded-via-synced-kb-document.md)

## Context

ADR-0013 grounded the Learning Assistant by flattening each learn step's rich text to
markdown, syncing that markdown into the conversation's RAGFlow knowledge base, and rendering
the cited source through a bespoke `LearnContentViewer` (which re-fetched the raw TipTap and
re-rendered the pages, text-matching the retrieved chunk to place a highlight).

That worked but carried two costs:

- **Weak structure.** Flattening to markdown never rendered tables well, and the source view
  was plain text rather than what a participant reads.
- **A bespoke surface to maintain.** A custom viewer, a flattener, and a `/learn_content`
  render path existed only for learn content, plus a text-matching highlighter because RAGFlow
  returns highlight coordinates (`positions`) only for PDFs.

## Decision

**Sync learn content as a text-bearing PDF.** At sync time the frontend fetches the raw learn
content (`GET /documents/learn_content`, still reading the draft `preview_tool_config`),
converts the ProseMirror JSON to a `pdfmake` document definition, and uploads the generated
PDF under the reserved name `Learning material.pdf`. The backend keeps owning the RAGFlow dance
(delete the prior reserved doc, upload, enqueue the parse job); `sync_learning_content` is now
a multipart receiver rather than building the document itself.

A PDF is the one format where RAGFlow parses clean text natively (good retrieval), returns
per-chunk highlight coordinates, and renders tables properly - so the **existing** PDF viewer
displays it with native passage highlighting, no special-casing.

**Hard constraint:** the PDF must carry a real, selectable text layer. `pdfmake` emits real
text runs and real table cells; canvas-to-image generators (`html2canvas`, `jsPDF.html()`)
must not be used - a rasterised PDF has no text and retrieval silently returns nothing.

## Consequences

- The bespoke path is deleted: no `LearnContentViewer`, no server-side `rich_text_to_markdown`
  / `build_learn_content_document`, no `isLearnSource` branch. Learn content is "just a PDF."
- Tables and marks now reach both retrieval and the source view.
- `pdfmake` (~1MB, browser-only) is a new client dependency, dynamically imported at sync time
  so it stays out of the SSR/initial bundle. The TipTap -> pdfmake mapping lives in
  `lib/learn/tiptapToPdf.ts` (pure, unit-tested).
- The reserved name changed `.md` -> `.pdf`. Existing `Learning material.md` documents are
  orphaned; a conversation is corrected on its next sync. Kept from ADR-0013: draft sync
  (part 1) and the `TextViewer` fallback for genuine text *uploads* (part 3).
- Retained the RAGFlow `?name=` filter workaround (list unfiltered, match the reserved name in
  Rust) - the shared instance returns `102 "You don't own the document"` for any name filter.
