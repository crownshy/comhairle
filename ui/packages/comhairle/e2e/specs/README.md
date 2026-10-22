Gherkin scenarios to turn into Playwright tests. There is no Cucumber runner; each `.feature` file is a checklist for writing the matching `*.test.ts` in `e2e/` (for example `participant.polis.feature` becomes `e2e/participant.polis.test.ts`).

Tags:

- `@bug-<issue>`: describes the correct behaviour, which fails today because of that open bug.
- `@likely-bug`: describes the correct behaviour, which reading the code suggests fails today. A comment above the scenario says what the code suggests. Once someone confirms it and files an issue, swap the tag for `@bug-<issue>`.
- `@decision-needed`: open product question. The scenario encodes the assumed answer.
- `@smoke`: part of the short pass to run before every release.

The `participant.*` files cover the participant journey, one file per area. Each scenario name starts with its case id from the participant journey checklist (J- getting in, S- step shell, L- Learn, A- Learning Assistant, P- Polis, H- survey, R- prioritization, T- Thinking Space, X- Lived Experience, E- elicitation bot, F- finishing up, V- events, RP- report, C- everywhere). Unless a scenario says otherwise it runs against the launched conversation; most tools read a different config in preview, so the matching test can loop over preview and live.

Some scenarios need browser fakes rather than real hardware: `context.setOffline` for the offline cases, network throttling for the slow ones, and Chromium's fake media stream flags for camera and microphone.
