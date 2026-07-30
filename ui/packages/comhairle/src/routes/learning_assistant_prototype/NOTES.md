# Learning Assistant - UI prototype

**Question this answers:** How should the Learning Assistant present itself so it (a) looks
better and (b) clearly tells participants what it does, what they can ask, and what happens
to their data? Today it's a two-line intro paragraph (with a couple of typos) plus a bare
"Type a question here" input - it never really explains itself.

**What the assistant actually is** (for grounding the copy): a RAG Q&A helper. You ask a
question, the answer streams in grounded **only** in the documents attached to that
consultation, with inline citations and clickable source excerpts. Q&A history persists per
session (`GET/POST /api/conversation/:id/chat_sessions`). It renders in two places: inline on
the learn page, and as a "Find out more" sidebar drawer tab.

**Run it:** dev server, then `/learning_assistant_prototype`. Switch with the floating bar or
`?variant=A|B|C`. `?width=sidebar|page` previews the two real surfaces (drawer vs. inline
learn-page embed). Bar + prototype are `dev`-gated, so they never ship.

## Variants

- **A - Guided intro + data note.** Single column, progressive disclosure. Purpose header →
  3-item capability list → input as hero → collapsible "What happens to your questions" →
  Q&A history. Calm, low-commitment.
- **B - About panel + chat thread.** A pinned "About this assistant" accordion (What it does /
  What you can ask / Your data & privacy) over a real top-down chat thread with the input
  pinned to the bottom (messenger style). Inverts today's newest-first card list.
- **C - Example prompts + trust bar.** Leads with a grid of tappable starter questions
  (show-don't-tell of capability), a bold purpose line, and an always-visible slim trust
  strip about data. Reflows 1→2 columns from sidebar to page width.

## VERDICT - TBD

_Awaiting Daniela's pick. Likely outcome: one wins, or a mix (e.g. C's example-prompt grid +
A's collapsible data note, or B's explicit "Your data & privacy" section)._

## ⚠️ Before folding the winner in

- **All data/privacy copy here is PLACEHOLDER.** Confirm the real wording with the team:
  exact retention, whether admins/facilitators can see participant questions, whether anything
  feeds model training. Don't ship the current claims verbatim. The sidebar already has a
  Privacy Policy tab - consider linking to it rather than restating.
- Fix the two typos in the real component's intro copy while you're there
  (`LearningAssistant.svelte`: "Use this this space", "avaliable").
- Prototype was written under prototype constraints (stubbed chat, no streaming/errors/a11y
  polish). Rewrite properly into `LearningAssistant.svelte` - don't lift the files as-is.
- Delete this whole `learning_assistant_prototype/` folder once a direction is chosen.
