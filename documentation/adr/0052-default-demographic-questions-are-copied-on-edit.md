# ADR-0052: Default demographic questions are copied on edit

**Status:** proposed
**Date:** 2026-10-09

## Context

Super Administrators define default demographic questions for the whole system. Each
Organization's question bank inherits them. Organizations can also want to adapt a question,
for example to change its wording or options.

If an Organization edited the shared question in place, it would change it for every other
Organization. If every Organization got a private copy up front, a participant's answer
would not carry over between Organizations.

## Decision

**1. Organizations reference a Default demographic question by its shared UUID.** Inherited
questions use the same identity, so a participant's answer to one carries over to every
Organization that inherits it.

**2. Editing an inherited question forks an Organization-owned copy with a new UUID.** The
copy starts from the default's latest published version. The fork is independent from then
on, and the original is never changed by an Organization.

**3. Default questions are changed only by Super Administrators,** and their new versions
are shared with Organizations that have not forked them.

### Admin UI clarification (2026-10-13)

Organizations must explicitly duplicate a default question before editing it in the admin
UI. Default questions show a "Default" marker instead of edit actions, including consent
editing within a form. The duplicate is a new Organization-owned question, with no draft
or published version inherited from the source, and first publishes as v1. Forms using
the default continue to reference it unless the Organization replaces it with the duplicate.
The API's implicit fork behavior remains available for existing callers.

## Consequences

- Existing data for the five migrated defaults (age, ethnicity, gender, zipcode and political
  party) keeps its UUIDs and version 1.
- Forking loses the link to later default versions. The admin UI should say that a question
  is a fork and show the default it came from.
- Forks can share a question's answers only with the Organization that made them.
