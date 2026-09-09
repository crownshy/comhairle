# ADR-0036: Feedback about the flow is asked on the thank-you page, not as a step

**Status:** proposed
**Date:** 2026-09-09
**Branch:** participant chrome demo branch

## Context

We want to ask participants how the experience went: whether the information helped, whether
they felt heard, whether they enjoyed it, what to change. Ten or so questions, most of them
multiple choice, a couple free text. The questions are partner-specific (one asks about trust
in the commissioning government), so they have to be written per conversation, not shipped in
the app.

The thank-you page already had a "Give feedback" button under its "Want to contribute more?"
disclosure. It opened one free-text box and wrote to a `feedback` table that nothing in the
admin reads. So those replies went nowhere, and there was no way to ask a structured question.

The obvious shape is a survey step at the end of the workflow. HeyForm steps already do
everything needed: the admin writes questions in the builder, the participant answers in the
embed, and Insights and CSV export report per question.

## Decision

**1. It is a survey attached to the conversation, not a step in the workflow.** A step is
counted in "Step N of M", gates the "You completed it" moment, and shows up in the report
next to the topic. Feedback about the platform is a different kind of thing from a
contribution to the conversation. It is asked after the finish, it is optional, and its
answers are kept apart from the conversation's data.

`conversation_feedback_survey` holds one HeyForm tool config per conversation. The config is
the same type a HeyForm step stores, so the builder (`HeyFormManage`), the embed
(`HeyFormEmbed`) and the labelled insights all reuse without change. The tool-agnostic
`ToolConfig` is stored rather than a HeyForm-specific shape so a different survey tool can
take the slot later without a migration.

**2. It is the one thing on the thank-you page that does not fold away.** ADR-0027 put
everything after the numbers behind disclosures. The ask breaks that rule on purpose: the
moment to ask how it went is while finishing is still the moment, and a disclosure reads as
"only if you want". The card sits directly under the stats and does nothing until its button
is pressed. The survey then takes the whole screen, the way the step brief does (ADR-0023),
with one close button: a form squeezed into a card on a page with other things on it read as
an afterthought, and the participant is either answering it or not. Closing without finishing
puts the card back as it was, so the offer stands.

**3. It supersedes the free-text box.** A conversation with a survey hides the old button
even when its toggle is on. One ask, not two.

**4. It is asked once per participant, not once per browser.** Finishing writes a row in
`conversation_feedback_survey_completion` for that user and conversation, and the survey read
carries `completed` for the requesting user, so the card opens on its thank-you state on any
device. The marker is written by the page when the embedded form reports it is done, so it is
client-asserted; the answers themselves are on HeyForm, tagged with the same user id, and the
marker only decides whether to ask again. Rejected: `localStorage`, which is per browser and
gone with cleared site data, so a guest returning on their phone would be asked twice.

**5. Admins get it under Configure, as a Feedback tab.** Create, build the questions, read
the responses, remove. Removal deletes the form and every response on the HeyForm side, so it
is behind a confirmation.

Rejected: seeding the ten questions from the app. The form field schema is HeyForm's, and a
seed that drifts from it produces a broken form with no error. The admin pastes the questions
into the builder; it takes a few minutes.

Rejected: a preview and a live form, the way a step has. The survey is not part of launch.
There is one form, and an admin's test submission from the thank-you page preview lands in
the real responses. That is documented in the Feedback tab rather than engineered around.

## Consequences

- The `feedback` table and its endpoints remain for conversations without a survey. Nothing
  reads them from the admin, which was already true.
- Participants can only reach the card by finishing the flow, so the survey's response
  count is bounded by completions. That is the population we want to hear from.
- The HeyForm form is created when the admin presses Create, not when the conversation is
  created, so conversations that never ask cost nothing on the HeyForm server.
- A conversation's thank-you page fetches the survey on its own; it is not hoisted to the
  conversation layout, because no other page needs it.
