# ADR-0056: The preview password is checked by the API, not the frontend

**Status:** proposed
**Date:** 2026-10-19

## Context

Issue #1081: an admin wants someone without a Comhairle account to look at a conversation
before it launches. The planning day settled on one password per conversation, set by the
admin, shared with the preview link.

Before this, a draft conversation could only be read by its team. `get_conversation` checked
Read permission and the frontend redirected everyone else to `/`.

## Decision

**1. A draft is readable by its team, or by a browser holding a preview grant.** Being logged
in is not enough on its own. Preview visitors join with an anonymous account, so every one of
them is logged in, and "logged in or has the password" would let any guest in.

**2. The grant is a cookie the API sets and the API checks.** `POST
/conversation/{id_or_slug}/preview_access` verifies the password against an argon2 hash and
sets an HttpOnly cookie, `preview-access-{conversation_id}`, holding a JWT signed with the
session secret. Rejected: a cookie only the frontend checks. The API would still refuse the
anonymous visitor, and anyone could skip a frontend-only gate by calling the API directly.

**3. A grant lasts 7 days and is tied to the password it was issued for.** The JWT carries
the password row's `updated_at`. Changing the password moves it and clearing the password
deletes the row, so both lock out everyone at once. There is one password, so there is no way
to revoke a single person.

**4. The hash lives in its own table, `conversation_preview_password`.** Keeping it off
`conversation` means it can never be serialized into a conversation DTO by accident.

**5. Server-side rendering forwards grant cookies to the API.** The server API client only
sends the cookies it is given, so the root layout passes `preview-access-*` cookies along with
`auth-token`. Without this, a reload sends the visitor back to the password form.

**6. The password only matters before launch.** A live conversation is already readable by
anyone with the link.

## Consequences

- Reviewers can walk the whole flow, including submitting. Before launch, tools write to their
  preview config, so this does not touch live data. It does leave anonymous participation rows
  on the draft. #876 (preview writing real data after launch) is unchanged.
- The unlock route uses the strict per-IP auth rate limit.
- Only `get_conversation` and the documents check look at grants. Other reads of a draft, such
  as workflow steps and tool data, do not check `is_live` at all today. That gap is older
  than this change and not closed by it.
