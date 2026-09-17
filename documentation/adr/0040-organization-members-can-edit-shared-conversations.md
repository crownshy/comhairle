# ADR-0040: Organization members can edit conversations shared with their organization

**Status:** Proposal - to be discussed with the team
**Date:** 2026-09-16

## Context

Access to a conversation can come from three places: owning it, a `content_editor` grant
to a user (the Team tab), or a `conversation_co_host` grant to an organization. The
co-host role only granted `ConversationRead`.

Adding someone to an organization and sharing a conversation with that organization put
the conversation in their sidebar, but opening it sent them back to `/admin`. The
conversation admin layout loads the moderation policies, those routes checked
`ConversationUpdate`, and one failed load redirects. Even without that redirect, they
could not edit anything. The only way to let them edit was the Team tab, which only the
owner sees.

Organizations now manage their own members and admins, so a second per-conversation list
of editors is extra work.

Separately, `conversation.organization_id` (the primary host organization) was stored at
creation but never checked. Members of the creator's own organization had no access.

## Decision

- **The co-host role grants `ConversationUpdate`.** Every member of a co-hosting
  organization can open and edit the conversation. Organization admins and members get the
  same access for now. `ConversationAdmin` (managing co-hosts, deleting) is not included.
- **The primary host organization counts as a co-host.** `can_perform_resource_action`
  checks `conversation.organization_id` against the user's organization when no granted
  role allows the action. `list_for_permitted_user` includes those conversations too.
  Nothing is written to `resource_permissions`, so existing conversations need no backfill
  and changing a conversation's organization moves access with it.
- **Reading moderation policies needs `ConversationRead`.** Creating, replacing and
  deleting still need `ConversationUpdate`. This amends the route rule in ADR-0038.
- **The Team tab stays for now.** Existing `content_editor` grants keep working. Removing
  it is a later change.

## Consequences

- Making organization members view-only later means removing `ConversationUpdate` from
  `Role::ConversationCoHost` in `api/src/models/permissions.rs`. Splitting admins from
  members would need `can_perform_resource_action` to check `OrganizationAdmin` on the
  user's organization.
- An access check on a conversation the user has no granted role on now makes one extra
  query against `conversation`.
- Step, translation, event, media and workflow routes still use `RequiredAdminUser`, which
  checks the site-wide Admin role and ignores the conversation. An organization member who
  is not a site Admin can open the conversation and edit its details, but not its steps.
  Moving those routes to per-conversation checks is a follow-up.
