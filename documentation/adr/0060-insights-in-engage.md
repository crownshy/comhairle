# Insights live in Engage

Each Step's Insights used to be a subtab of the step editor under Process design, next to Setup and Configure. To see how a running conversation was going, an admin had to go back into the editing area and open steps one by one, the same problem moderation had before ADR-0055 (#1549).

Insights is now its own page at `/admin/conversations/<id>/insights`, under Engage > Insights in the sidebar, between Moderate and Monitor. It lists the Steps whose tool has Insights (Polis, survey, Thinking Space and Prioritisation today; the list is `hasInsights` in `lib/reports/insightsTools.ts`), and `/insights/step/<step_id>` shows one Step's Insights with the same components as before. The step editor has no Insights subtab. The old `design/step/<step_id>/insights` URL returns a 308 to the new page, and `?subtab=insights` returns a 307.

Insights stays per-Step. The conversation-level view is still the Report.

## Considered options

- **Results, next to Report.** The Report is built from Insights, so they belong together there. But admins check Insights while the conversation runs to spot gaps and steer recruitment, which is what Engage is for. Both stages unlock at launch, so the choice does not change when Insights is reachable.
- **Show the entry only when a Step has Insights**, like Moderate and Polis steps. Almost every conversation has a poll or survey, and Monitor shows for every conversation too, so the entry is always listed and the page explains when there is nothing to show. This keeps one less flag in the conversation layout.

## Consequences

- Insights follows the Engage rules in ADR-0052: it stays locked until launch.
- Insights on preview data before launch (#1556) can't use this entry while Engage is locked. That needs its own decision: unlock this one entry before launch, or show preview Insights somewhere in Set up.
