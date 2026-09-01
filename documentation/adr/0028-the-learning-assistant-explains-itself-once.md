# 0028. The Learning assistant explains itself once

Date: 2026-09-01

## Status

Accepted.

## Context

The participant-facing Learning assistant opened as a pale box with a placeholder input,
under three paragraphs of standing prose: what the space is for, what makes a good
question, and a privacy warning. That prose sat above the input on every visit, in
`text-sm`. Answers stacked below the input newest-first, each collapsed behind a
disclosure with an `11px` label.

Two problems, both worse for the younger audience the tool is now being aimed at:

1. **The explanation was permanent and unread.** Standing prose above an input is
   scenery. It cost vertical space on every visit and still did not tell a first-timer
   what to actually type.
2. **Nothing modelled a good question.** The copy described what a good question is
   ("questions that help you learn things") instead of showing one.

Type size compounded both: the assistant, and the support drawer around it, sat at
`text-sm` and smaller while the article beside it was `text-base`.

## Decision

**The assistant introduces itself once per conversation, and shows rather than tells.**

- An intro card on first arrival: three short points, the privacy line, and a "Got it"
  that dismisses it for good (`localStorage`, keyed by conversation, same shape as the
  step tour in `stepTour.ts`). After that it collapses to a "What is this?" control that
  brings it back.
- Everything else that used to stand above the input (where answers come from, that it
  can be wrong, that it is not a person, what not to type) is behind **Learn more**, as
  question-and-answer pairs rather than paragraphs.
- Nothing else sits between the ask bar and the answer. Tappable starter questions were
  tried under the bar and cut: five chips in a scrolling row read as clutter above the
  fold, and the placeholder already invites a question.

**One answer is in focus; the rest is history.** The panel is an ask bar plus exactly one
answer at full size, with its sources as full-width rows. Earlier questions collapse into
a one-line list that promotes a question back into focus when tapped. The old design gave
every answer an equal collapsed row, which made the newest answer, the only one anyone is
reading, no more prominent than a question asked ten minutes ago.

**Nothing below `text-base` for content.** `text-sm` is the floor, and only for meta
labels ("You asked", "Where this came from"). The support drawer around it was raised to
match: it now carries a title and a close control in a header, and a segmented tab strip
at `text-base` instead of a dark bar of `text-sm` triggers.

## Consequences

- The session plumbing moved out of the component into
  `LearningAssistant/assistantState.svelte.ts`, so the panel is layout only and the two
  surfaces (the Learn article and the support drawer) share one reactive session.
- A first-timer gets no worked example of a question, only the intro card and the
  placeholder. If prompts come back they should be conversation-specific and live in tool
  config next to the FAQs, not as topic-neutral constants in `messages/en.json`.
- A returning participant who clears site data sees the intro again. That is one tap, and
  cheaper than the alternative of storing it server side.
- `learning_assistant_summary` and `learning_assistant_description` are gone from every
  locale file. The replacement keys are English only for now, and fall back to English in
  other locales until translated.
