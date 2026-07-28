# ADR-0008: Report pieces embed in TipTap as frozen snapshots

**Status:** accepted (explicitly revisitable)
**Date:** 2026-07-23

## Context

The End-of-engagement [report view](../../CONTEXT.md) is a human-authored TipTap document that
pulls report pieces in — an editor curates auto-generated insights and drops in component blocks.
Report pieces are dumb Svelte components (data in, markup out).

The question is what a report block *stores* in the document. Two models:

- **Live** — the node stores a query/reference and re-renders against current data on every view.
  Always fresh, interactive. But it only works in JS contexts: it breaks in the email and
  `@tiptap/static-renderer` paths (no DOM, no JS), and needs a Svelte node-view bridge we don't
  have.
- **Frozen** — at publish/freeze time the dumb component is rendered to HTML and that HTML is
  baked into the node. Every downstream path (participant web view, email, print) shows the same
  output because it is just HTML.

## Decision

**Embed report pieces as frozen HTML snapshots**, rendered from the same dumb components at
freeze/publish time. This matches the glossary definition of the End-of-engagement report as a
"final frozen snapshot", and it renders in every path including email because it carries no
runtime dependency.

A live in-app preview *while the author is composing* is a separate, optional nicety layered on
top; it does not change what gets stored or shipped.

## Consequences

- The stored artifact is portable HTML, not a live binding — a future reader may expect embedded
  reports to update after publish. They do not, by design; re-freeze to refresh.
- This is deliberately **revisitable**: if we later want post-publish-live embeds, it means adding
  a node-view bridge and a static fallback, and revising this ADR. We chose frozen first because
  it is simpler and the end-of-engagement report is a snapshot by definition.
