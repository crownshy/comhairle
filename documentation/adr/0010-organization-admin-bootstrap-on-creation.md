# ADR-0010: Assign Organization Administrators at organization creation with silent account bootstrap

**Status:** Proposal - to be accepted
**Date:** 2026-08-05

## Context

When creating an Organization in the admin dashboard, we need to assign administrative authority to specific users at creation time. This assignment must be distinct from the Organization contact email and from general member association. Some entered admin emails may not yet have user accounts.

The prior magic-link-only onboarding approach reused verification behavior and produced the wrong account shape for this use case. Missing admins were being created via an OTP-style path rather than as full email-password users with a stable identity model.

We considered whether to block creation and ask for confirmation before creating missing accounts, and whether to issue random passwords or magic-link sign-in. We chose non-blocking full account bootstrap plus first-login password reset to keep the creation flow fast while preserving proper account semantics.

## Proposal

1. Add an explicit creation input for Initial Organization Administrators (email list) that is separate from contact email and member emails.
2. Grant each resolved user Organization Administrator permissions on the new Organization: update organization, delete organization, add members, remove members.
3. For emails with no existing user account, auto-create full email-password accounts silently, with username generated from email local-part plus unique suffix and a generated temporary password.
4. Grant the same Organization Administrator permissions to those new users.
5. Send a first-login password reset email to newly created Organization Administrators only, using the password reset mechanism.
6. First-login password reset links expire after 24 hours, and admins can resend the initial password reset email from the Organization Users surface.
7. Organization creation is not rolled back for downstream assignment/email failures; instead return a partial-failure summary.

## Consequences

- We introduce a clear semantic split between Organization contact details, membership, and administrative authority.
- Silent account bootstrap improves flow speed but requires strong audit logging and clear post-create feedback for partial failures.
- Full account bootstrap preserves a consistent user identity model and login path, but requires secure temporary password generation and first-login reset delivery.
