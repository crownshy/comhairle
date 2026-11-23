# ADR-0053: Special category answers need consent per question version

**Status:** proposed
**Date:** 2026-10-09

## Context

Some demographic questions collect sensitive data, such as ethnicity, pregnancy status or
caring responsibilities. The brief requires informed consent before this data is collected
and stored.

A question's wording can change. A consent given to one wording may not cover another.

## Decision

**1. Special category questions show the admin's consent text before they are asked.** The
admin authors that text on the question. Consent is recorded per participant and per
Question version.

**2. Answers are stored only after consent.** A participant who declines is not asked the
question, and no answer is stored. This holds even when the question is required, so the
question is skipped rather than blocking the participant.

**3. A new Question version asks for consent again.** Responses are keyed by Question
version (see ADR-0051), so consent for one version does not cover another.

## Consequences

- Consent records are a separate table from responses, so consent can be audited on its own.
- Declining a required question still lets the participant continue. Admins who need the
  answer should not mark sensitive questions as required in a way that implies it is
  mandatory.
- Reports must not use answers from participants who did not consent.
