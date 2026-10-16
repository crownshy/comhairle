# Monitor tab prototype

A redesign of the conversation Monitor tab, built as a separate route so it can sit next to the live page:
`/admin/conversations/[conversation_id]/monitor/prototype`. The live Monitor page (`../+page.svelte`) is unchanged.

All numbers come from `sampleMonitorData.ts`. Links point at the real sibling tabs (Recruit, Notify, Process design). Search for `TODO` to find the places that still need wiring up.

## Page structure

The page has four tabs. The active tab is kept in `?tab=` using shallow routing (`replaceState`), so a refresh or a shared link opens the same view without reloading data.

| Tab             | Components                                                                                                                                                                                                                                                                                             |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **At a glance** | `FirstVisitHint` and "Updated …" beside the tabs · `ParticipationFlow` (blue headline: recruited → joined → completed) · three `QuestionCard`s: Working (completion rate per step + typical time to complete), Recruitment (`TargetSparkline`), Reach (`ReachMini`) · `FollowUpActions` (data exports) |
| **Live pulse**  | `ActionHint` (biggest drop-off) · `MonitorStatsBar` · `TodayJourneys` + `JourneyDetail` · `ActivityHeatmap`                                                                                                                                                                                            |
| **Journey**     | `ActionHint` (people inactive 3+ days) · `StepFunnel`                                                                                                                                                                                                                                                  |
| **Recruitment** | `RepresentationHint` · `RecruitmentTarget` + `EditTargetDialog` · `RepresentationBars` · `SignupsOverTimeChart` · `RecruitmentChannels`                                                                                                                                                                |

Shared helpers:

- `ActionHint`: the hint banner, with a `warning` (orange) or `info` (blue) tone. `FirstVisitHint` and `RepresentationHint` are built on it.
- `ActionLogItem` and `actionKinds.ts`: one logged action and the colour, icon and label for each kind, used by `SignupsOverTimeChart`.
- `journeyStyles.ts`: segment colours shared by `TodayJourneys` and `JourneyDetail`.

## For backend: data each section needs

The types in `sampleMonitorData.ts` describe the shapes. Comments marked **New** are data we don't have yet.

| Data                                                  | Used by                                              | Status                                                                                   |
| ----------------------------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Recruited (signed up, opened or not)                  | Participation flow                                   | New                                                                                      |
| Joined / completed per step                           | Participation flow, Step funnel, Working card        | Available (`WorkflowStepStats`)                                                          |
| Signups per day                                       | Signups chart, Target sparkline                      | Available (`DailySignupStats`)                                                           |
| Completions per day                                   | Signups chart                                        | New                                                                                      |
| Median minutes per step, time to complete             | Step funnel, Working card                            | New                                                                                      |
| People inactive 3+ days, per step                     | Step funnel, Journey hint, Follow up                 | New                                                                                      |
| Active now / joined today / completed today           | Live pulse stat bar                                  | New                                                                                      |
| Recruitment target and closing date                   | Recruitment target, Signups chart, Target sparkline  | New (stored with the conversation)                                                       |
| Signup source per participant (invite, open link, QR) | Recruitment channels                                 | New                                                                                      |
| Population baseline per demographic group             | Representation bars, Representation hint, Reach card | New (set by the admin)                                                                   |
| Action log: what the team did and when                | Signups chart markers                                | New (most actions already happen in Recruit and Notify; they need recording with a date) |
| Per-participant step start/end times today            | Live pulse journeys                                  | New (anonymous codes only)                                                               |
| Activity by weekday and time slot                     | Activity heatmap                                     | New                                                                                      |

The under-represented rule is in `isUnderRepresented()`: a group is flagged when its share of participants is below 70% of its population share.

## For frontend: before this replaces the live page

- Swap `sampleMonitorData` for real loaders in `+page.ts`. The `sample.*` reads in `+page.svelte` are the only places data comes in.
- `MonitorStatsBar` is a fork of `$lib/components/StatsBar.svelte`. Either fold the explanation popovers back into the shared component or replace it.
- `StepFunnel`, `SignupsOverTimeChart` and `RepresentationBars` are candidates for `$lib/components` if other pages need them.

Wording: people who reached the end of the last step are **Completed** everywhere in the UI. Data fields keep the API naming (`finished`).

Exports: the contacts and demographics downloads use the existing `/api/conversation/{id}/contacts/export` and `/demographics/export` endpoints. The Thinking space summaries download has no endpoint yet and points at the demographics export (see the `TODO(backend)` in `+page.svelte`).
"Updated 2 minutes ago" beside the tabs is static text; it needs the time of the last data refresh (`TODO(backend)` in `+page.svelte`).
