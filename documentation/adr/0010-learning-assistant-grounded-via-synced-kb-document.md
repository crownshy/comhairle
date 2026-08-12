# ADR-0010: The Learning Assistant is grounded in learn-step content via a synced knowledge-base document

**Status:** accepted
**Date:** 2026-08-05

## Context

[#780](https://github.com/crownshy/comhairle/issues/780) wants the participant-facing
**Learning Assistant** (both the inline one at the bottom of a learn page and the "Find out
more" sidebar) to answer against the **actual text on the learn step**, so prompts like
"explain this page in simpler words" work against what the reader is looking at rather than
knowledge-base retrieval alone. Its parent, [#591](https://github.com/crownshy/comhairle/issues/591),
frames the broader goal: the assistant should "know the material" the participant is reading,
and proposes adding learn-step content as an input source to the knowledge base.

The mechanics that constrain us:

- The assistant is a **RAGFlow** chat. Each turn the client sends only the new `{ question }`
  ([chatClient.svelte.ts](../../ui/packages/comhairle/src/lib/api/chatClient.svelte.ts));
  RAGFlow owns the session, the full history, and retrieval.
- The chat session is **per-conversation, per-user** and spans **all steps**, not per-step.
- The system prompt is "answer using ONLY the information in the knowledge base below", with a
  `{knowledge}` variable filled by RAG retrieval
  ([bot_service.rs](../../api/src/bot_service.rs)).
- A `Conversation` already owns a `knowledge_base_id` (dataset) and a `chat_bot_id`. Documents
  upload into that KB, a background worker
  ([process_documents.rs](../../api/src/worker_service/process_documents.rs)) polls parse
  status, and once a document is parsed it connects the chat bot to the KB (see ADR-0007).

A proposal spec (now removed, its job done) explored three mechanisms: (1) inject live page
content into the RAGFlow system prompt per request, (2) seed the content into session history
once per step, (3) sync the content into the knowledge base as a document. The team discussed
and converged on (3), which is also what the parent issue #591 proposed.

## Decision

**Ground the assistant by syncing a conversation's learn-step content into that conversation's
knowledge base as a single reserved-name document, retrieved via the existing RAG `{knowledge}`
path.** No per-request page injection.

- **One document per conversation.** All of the conversation's learn steps are concatenated
  into a single document with clear per-step headings, uploaded under a **reserved name** so a
  re-sync can find and replace it deterministically.
- **Current-step retrieval hint.** Keep sending the current step title (the existing
  `[Reading "<title>"]` prefix
  ([LearningAssistant.svelte](../../ui/packages/comhairle/src/lib/components/LearningAssistant/LearningAssistant.svelte)))
  from the inline assistant, and **start sending it from the sidebar too**, to bias retrieval
  toward the step the reader is on. No page-level markers or custom indexing.
- **Vague-query handling.** The system prompt instructs the model to ask a clarifying question
  when a request does not make clear what "this" refers to, instead of guessing.
- **Sync lifecycle.** A manual admin **"sync" button** triggers delete plus re-upload plus
  re-parse through the existing `DocumentJob` worker (which also connects the chat bot to the KB
  once the document parses, per ADR-0007). Re-publishing out of **Draft mode** is the intended
  future auto-trigger once that feature exists. No periodic re-sync worker for now.
- **Scope.** Grounded Q&A against the learn content is the **must**. Precise "explain this page"
  behaviour is **good-to-have**.

## Considered options

- **Per-request system-prompt variable** (rejected): extend the chat body to carry live page
  content and bind it to a per-turn prompt variable, the way the *agent* path already uses
  `inputs` ([ragflow_bot.rs](../../api/src/bot_service/ragflow_bot.rs)). Best token behaviour
  and the truest "this exact page" grounding, but chat currently sends `inputs: None` so it
  needs an unproven RAGFlow capability, it fights the "answer ONLY from the knowledge base"
  prompt, and it stands up a second grounding mechanism alongside the KB work #591 already
  wants. Rejected in favour of reusing the KB infra, which satisfies both #591 and #780's must.
- **Seed content once per step into history** (rejected): no backend change, but every seeded
  step accumulates a copy in the session history and it still leans on the KB-only prompt while
  stuffing content into the question.
- **Page-level markers / custom indexing** (rejected for now): would let "this page" resolve
  precisely, but RAGFlow's KB config exposes no custom-marker support and the need is
  deprioritized. Revisit only if the data shows it matters.
- **Periodic re-sync worker** (rejected for now): a safety net against an admin forgetting to
  sync, but it re-parses unchanged content on a timer. Revisit if staleness shows up in real
  usage.

## Consequences

- **The assistant answers from parsed chunks, not the whole page.** "Explain *this* page"
  retrieves imperfectly and may prompt a clarifying question. This is the accepted good-to-have,
  matching the must / good-to-have split agreed with the team.
- **Staleness is admin-owned until Draft-mode auto-sync lands.** Edits to a learn step are
  invisible to the assistant until someone clicks sync, so the button copy must say so plainly.
- **Re-syncing breaks the citations in answers that predate it.** A sync deletes the reserved
  document and re-uploads it under a new id, so source links in older assistant answers point at
  a document that no longer exists; opening one fails (the API returns 404, and RAGFlow's
  "document not found" path can surface as 500). We do not rewrite or invalidate prior answers.
  For now this is handled with messaging rather than prevention: the download endpoint surfaces a
  clean "no longer available" instead of the raw RAGFlow error, and the PDF viewer shows a
  plain-language note telling the reader the materials were updated and to ask again for
  up-to-date sources. A fuller fix (stable ids across syncs, or invalidating stale answers) is
  deferred until real usage shows it matters.
- **The synced document is subject to ADR-0007 gating.** Until it parses (`parse_status` is
  `DONE`) the assistant can be hidden or unable to answer from it, and the first sync carries
  parse latency.
- **Grounding is single-language.** The sync builds one shared document from the conversation's
  `primary_locale` (`learn_content` resolves content in that locale, not the requester's
  `LocaleExtractor` cookie), because a single shared knowledge-base artifact needs a stable,
  deterministic source language rather than one that varies with whichever admin synced last. A
  participant reading a translated step is therefore grounded in the primary-locale text. True
  multi-language grounding would need a synced document per locale plus retrieval keyed to the
  participant's language; deferred as a follow-up until the need is shown.
- **No live step content is plumbed to the layout-level sidebar** (a simplification versus
  #780's original framing). Both surfaces just query the shared KB; only the lightweight title
  hint is passed to the sidebar.
- **The existing "answer using ONLY the knowledge base" prompt now works in our favour**, since
  the learn content lives in the KB. The only prompt change is the clarification instruction.
- **Follow-up (post-MVP validation).** Reviewing collected question/answer pairs to categorise
  failures and prioritise good-to-haves (like "explain this page") is a validation loop after
  this ships, not a gate on it.
