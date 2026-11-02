# ADR-0067: Invite-only events are joined by accepting an invite

**Status:** proposed
**Date:** 2026-11-02

## Context

An event's `signup_mode` is open or invite. The invite setting only stopped anonymous
callers: any logged-in user could POST to the attendances endpoint, join an invite-mode
event, and name their own role, including `facilitator`. The event invite page also made two
calls, accepting the invite and then self-registering, so registration lived partly in the
browser.

## Decision

**1. Accepting an invite is what registers someone.** `AcceptInvite` creates the attendance,
resolves the breakout seat, and sends the reminders and confirmation email. The invite page
calls nothing else. Anonymous email invites take the same route through
`AutoRegisterEventAttendance`.

**2. On an invite-mode event, only the conversation owner or an admin can self-register.**
Everyone else gets a 403 (`EventInvalidSignupMode`). Open events still take anyone. The
public event page shows an invite-only message rather than Register to Attend, so owners and
admins have no button even though the API accepts them.

**3. The requested role only counts for the owner or an admin.** For anyone else the
attendance is created as `participant`, whatever the request says. Invites only ever grant
`participant`. Facilitators are still added through `/facilitator`.

**4. The event and the invite must belong to the conversation in the path.** Anyone can
create a conversation and own it, so without this check, owning any conversation would grant
access to every event. A mismatch returns 404.

**5. A used single-use invite is expired.** It returns 410 (`InviteExpired`) and registers
nobody, including the person who first used it. Any other invite can be accepted again:
nothing changes and the response is a 409, which the page treats as success.

## Consequences

- Logged-in users without an invite can no longer join invite-mode events. They need an
  invite, or the event needs to be switched to open.
- Attendances created before this decision keep their role, so any unexpected ones need
  checking separately.
- Accepting is not one transaction. The attendance, the invite response and the emails are
  written in turn, so a mail failure leaves someone registered without their confirmation or
  reminders, and retrying does not resend them. Two people using one single-use link at the
  same moment can both register.
- Someone who rejected an open invite and later accepts it is registered, but gets the 409
  and no confirmation email.
