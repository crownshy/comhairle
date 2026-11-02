# One admin sidebar that swaps its content; section sub-tabs stay

Every admin page has one sidebar, never two stacked. On workspace pages (the conversations list, media library, email templates and so on) it holds the workspace links, grouped: Conversations on its own, then Library (Media library, Email templates) and Administration (Organisations, Regions, System roles), each shown only to the roles that can use it. Open a conversation and the same sidebar switches to that conversation: a "← Conversations" back link, the conversation's name and status, and its sections grouped by Conversation Stage (Set up, Launch, Engage, Close, Results). The back link switches it back, and it hides when the sidebar is collapsed so only the expand button shows. The profile menu sits in the footer in both cases, with Handbook above it on workspace pages.

Engage, Close and Results stay locked until launch for the reasons in ADR-0001. The conversation sidebar replaces both the main conversation tab strip and the header bar with Preview and Launch, which gives the editing area more room. The existing sub-tab strips (Configure, step editor, events, invites) stay, so the sidebar picks the section and the tabs pick within it.

`/admin` redirects to `/admin/conversations`, as it did before #1205.

## Considered options

- **A landing page at `/admin` with no sidebar on workspace pages** (#1486, built and then dropped). Workspace pages got a top bar with a breadcrumb instead. It meant two kinds of chrome for one admin area, and getting from a conversation to the media library took a trip through the landing page.
- **Two columns** (an icon rail for workspace and conversations, plus a conversation panel). Faster switching, but it costs width and the rail's dots don't say which conversation is which.
- **Workspace links and the conversation stacked in one sidebar** (the old AdminNav). Too long, and it loaded every conversation on every page to list them.
- **Every sub-page nested in the sidebar, no tabs at all** (the first version of #1205). One place to look, but Configure alone has six sub-pages and step editors have their own, so the sidebar gets long and scrolls off at high zoom (#851).

## Consequences

- The sidebar only shows links, so the admin layout no longer loads conversations on every page. The conversations list loads its own page.
- Launch, End and Re-open move onto their own Launch and Close pages, reached from the Launch and Close stages.
- Moderate sits in Engage, and only when the conversation has a Polis step. Monitor sits in Engage too, since you watch it while the conversation runs.
- The status under the title is plain text with a coloured dot, not a menu, since Launch, End and Re-open have their own pages. Preview is a pill beside it rather than an entry under Set up, so it is reachable whatever the stage.
- Once launched, Set up shows a lock and starts folded. It stays editable; the lock is there so changes after launch feel deliberate.
