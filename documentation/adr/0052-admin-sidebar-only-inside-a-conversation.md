# Admin sidebar only inside a conversation; section sub-tabs stay

The admin sidebar appears only while you edit a conversation, and it is that conversation's navigation: a "Workspace" back link, the conversation's name and status, and its sections grouped by Conversation Stage (Set up, Launch, Engage, Results). Engage and Results stay locked until launch for the reasons in ADR-0001. The sidebar replaces both the main conversation tab strip and the header bar with Preview and Launch, which gives the editing area more room. The existing sub-tab strips (Configure, step editor, events, invites) stay, so the sidebar picks the section and the tabs pick within it.

Workspace pages (the conversations list, media library, email templates and so on) have no sidebar. `/admin` is a landing page with the workspace links and the latest conversations, and workspace pages get a breadcrumb back to it (#1486).

## Considered options

- **One sidebar that switches between workspace and conversation** (the first version of this ADR). It kept a persistent column on pages that only needed a few links, and loaded conversations on every page to fill it.
- **Two columns** (an icon rail for workspace and conversations, plus a conversation panel). Faster switching, but it costs width and the rail's dots don't say which conversation is which.
- **Every sub-page nested in the sidebar, no tabs at all** (the first version of #1205). One place to look, but Configure alone has six sub-pages and step editors have their own, so the sidebar gets long and scrolls off at high zoom (#851).
- **A conversation switcher dropdown** at the top. Left out for the back link, since the landing page's latest list covers switching in two clicks.

## Consequences

- The admin layout stops loading conversations on every page. The landing page loads five; the conversations list loads its own page.
- Launch, End and Re-open move into the sidebar's status dropdown, so they are always one click away while editing.
- Moderation has no sidebar entry until it moves out of Process design into its own route (#1224).
