# Monitor tab prototype

A redesign of the conversation Monitor tab, built as its own route so it sits next to the live page:

- Prototype: `/admin/conversations/[conversation_id]/monitor/prototype`
- Live page (unchanged): `../+page.svelte`

Everything on the page is sample data from `sampleMonitorData.ts`. Links go to the real sibling tabs (Recruit, Notify, Process design). The data exports call the real API.

## Page structure

Four tabs. The active tab is kept in `?tab=` with shallow routing (`replaceState`), so a refresh or shared link opens the same tab without reloading data. "Updated 2 minutes ago" sits beside the tabs on every tab.

| Tab             | Components, top to bottom                                                                                                                                                                                                                         |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **At a glance** | `FirstVisitHint` · `ParticipationFlow` (recruited → joined → completed) · three `QuestionCard`s: Working (completion rate per step, typical time to complete), Recruitment (`TargetSparkline`), Reach (`ReachMini`) · `FollowUpActions` (exports) |
| **Live pulse**  | `ActionHint` (biggest drop-off) · `MonitorStatsBar` · `TodayJourneys` + `JourneyDetail` · `ActivityHeatmap`                                                                                                                                       |
| **Journey**     | `ActionHint` (people inactive 3+ days) · `StepFunnel`                                                                                                                                                                                             |
| **Recruitment** | `RepresentationHint` · `RecruitmentTarget` + `EditTargetDialog` · `RepresentationBars` · `SignupsOverTimeChart` · `RecruitmentChannels`                                                                                                           |

Shared pieces:

- `ActionHint`: hint banner, `warning` (orange) or `info` (blue). `FirstVisitHint` and `RepresentationHint` use it.
- `actionKinds.ts` + `ActionLogItem`: colour, icon and label per logged action kind, for the signups chart.
- `journeyStyles.ts`: segment colours shared by `TodayJourneys` and `JourneyDetail`.

## Backend: data needed

Shapes are the types in `sampleMonitorData.ts`; comments marked **New** are data that doesn't exist yet.

| Data                                                  | Used by                                             | Status                                 |
| ----------------------------------------------------- | --------------------------------------------------- | -------------------------------------- |
| Joined / completed per step                           | Participation flow, Step funnel, Working card       | Available (`WorkflowStepStats`)        |
| Signups per day                                       | Signups chart, Target sparkline                     | Available (`DailySignupStats`)         |
| Contacts and demographics exports                     | Follow up                                           | Available (existing endpoints)         |
| Recruited (signed up, whether or not they opened it)  | Participation flow                                  | New                                    |
| Completions per day                                   | Signups chart                                       | New                                    |
| Median minutes per step and to complete               | Step funnel, Working card                           | New                                    |
| People inactive 3+ days, per step                     | Step funnel, Journey hint                           | New                                    |
| Active now / joined today / completed today           | Live pulse stat bar                                 | New                                    |
| Recruitment target and closing date                   | Recruitment target, Signups chart, Target sparkline | New (stored with the conversation)     |
| Signup source per participant (invite, open link, QR) | Recruitment channels                                | New                                    |
| Population baseline per demographic group             | Representation bars and hint, Reach card            | New (set by the admin)                 |
| Action log: what the team did and when                | Signups chart markers                               | New                                    |
| Per-participant step start/end times today            | Live pulse journeys                                 | New (anonymous codes only)             |
| Activity by weekday and time slot                     | Activity heatmap                                    | New                                    |
| Thinking space summaries export                       | Follow up                                           | New (currently points at demographics) |
| Time of last data refresh                             | "Updated …" beside the tabs                         | New                                    |

A group is flagged as under-represented when its share of participants is below 70% of its population share (`isUnderRepresented()`).

## Frontend: before this replaces the live page

- Replace `sampleMonitorData` with real loaders in `+page.ts`; the `sample.*` reads in `+page.svelte` are the only data inputs.
- `MonitorStatsBar` is a fork of `$lib/components/StatsBar.svelte`: fold its explanation popovers into the shared component, or replace it.
- `StepFunnel`, `SignupsOverTimeChart` and `RepresentationBars` could move to `$lib/components` if other pages need them.
- Search for `TODO` for the remaining wiring: saving the recruitment target, the refresh time, the summaries export, remembering the first-visit dismissal.

## Conventions

- People who reached the end of the last step are **Completed** in the UI; data fields keep the API name `finished`.
