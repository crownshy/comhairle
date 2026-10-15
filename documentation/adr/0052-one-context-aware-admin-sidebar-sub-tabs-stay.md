# One context-aware admin sidebar; section sub-tabs stay

The admin sidebar is a single column that changes with where you are. In the workspace it shows the latest conversations and workspace links. Inside a conversation it shows that conversation's sections grouped by Conversation Stage (Set up, Launch, Engage, Results), with Engage and Results locked until launch for the reasons in ADR-0001. The workspace links stay at the bottom in both modes. This replaces the main conversation tab strip, so the sidebar picks the section and the existing sub-tab strips (Configure, step editor, events, invites) pick within it.

## Considered options

- **Two columns** (an icon rail for workspace and conversations, plus a conversation panel). Faster switching, but it costs width and the rail's dots don't say which conversation is which.
- **Every sub-page nested in the sidebar, no tabs at all** (the first version of #1205). One place to look, but Configure alone has six sub-pages and step editors have their own, so the sidebar gets long and scrolls off at high zoom (#851).
- **A conversation switcher dropdown** at the top. Left out for a "Conversations" back link, since the workspace sidebar's latest list already covers switching in two clicks.

## Consequences

- The sidebar no longer lists every conversation, so the admin layout stops loading them on every page. Only the conversations list page loads the full set.
- Moderation has no sidebar entry until it moves out of Process design into its own route (#1224).
