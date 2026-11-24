# ADR-0065: A moderator who can't edit sees only Moderate

**Status:** proposed
**Date:** 2026-11-24
**Builds on:** [ADR-0055](0055-seeds-in-polis-setup-moderation-in-engage.md)

## Context

A conversation Moderator (`moderator` role, actions Read and Moderate) gets the same admin as
the policy team who owns the conversation: every Set up section, Launch, Recruit, Monitor,
Close and Report. Most of it fails or does nothing for them, because the API refuses their
writes. The daily moderation email (#1097) sends them in, and they still have to find their
one task in the full sidebar (#1224, #1926).

## Decision

**1. Hide by role on the existing routes.** We don't build a separate stripped route. A user
who can moderate but cannot edit (`conversation_moderate` without `conversation_update`) gets
the moderation surface: the sidebar shows a single Moderate link, with no stage around it,
and nothing else. Moderate stays open for them before launch and before a Polis step exists,
so their sidebar is never empty or locked. Everyone who can edit sees exactly what they saw before.

**2. Other sections redirect to Moderate.** The conversation layout load sends a moderator who
opens any other section by URL to `/moderate`. The check
reads the pathname untracked, so moving between sections doesn't rerun the layout load.
Client-side links to other sections don't exist on this surface, so a typed URL is the only
way in, and that is a full load.

**3. The link points at access, it doesn't grant it.** The email's link and the `/moderate`
URL work only for someone already granted the role on the Team page. Opening the link without
the role gets the same refusal as any other admin URL.

**4. Inside a conversation, the decision is made from actions, not role names.** The
conversation layout already knows the user's actions on it (`conversationActions`), so the
sidebar and the redirect read those. A role that later gains Moderate without Update (or a
group granting it) gets the same surface with no change here. Outside a conversation, the
workspace sidebar, the login landing and the conversation cards only have `GetUserRoles`, so
they read role names (decision 5). They act only when every role is `moderator`, which can't
carry edit rights, so they never send someone who can edit to the moderation surface.

**5. Someone who only moderates gets a moderation-only workspace.** When every role a user
holds (from `GetUserRoles`) is a conversation `moderator` role, the workspace sidebar shows
Conversations and nothing else, and `/admin`, where login lands, sends them to Moderate if
they moderate one conversation, or to the conversations list if they moderate several. Their
conversation cards open Moderate.
One other role anywhere (site, organisation, or another conversation role) keeps the full
workspace and the usual card links, while each conversation still gets the surface that
matches the user's actions on it, through the redirect in decision 2.

## Considered options

- **A separate route for moderators** (e.g. `/moderate/<conversation_id>`). Easier to send,
  but it duplicates the moderation pages and gives owners and moderators two URLs for the
  same queue.
- **Lock sections instead of hiding them**, like Notify's "Admins only". A moderator would
  scroll past nine locked rows to reach one.

## Consequences

- Observer, Translator and DataAccess still see the full sidebar. Translator and step editor
  surfaces are later slices of #1224.
- There is no moderation home across conversations yet. A moderator of several picks one
  from the conversations list, and the card opens Moderate.
- If the conversation layout fails to load, it sends the user to the conversations list, not
  `/admin`. For someone who moderates one conversation `/admin` leads straight back to it, so
  a failing fetch would loop.
- Conversation ownership isn't a role, so it doesn't show in `GetUserRoles`. Someone who owns
  a conversation and moderates another, with no other role, would get the moderation-only
  workspace. Creating a conversation needs a system role, so this only hits older accounts.
- The moderation email now links to `/moderate/step/<step_id>` directly instead of through
  the ADR-0055 redirect.
