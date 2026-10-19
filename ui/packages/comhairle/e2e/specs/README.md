Gherkin scenarios to turn into Playwright tests. There is no Cucumber runner; each `.feature` file is a checklist for writing the matching `*.test.ts` in `e2e/`.

Tags:

- `@bug-<issue>`: describes the correct behaviour, which fails today because of that open bug.
- `@decision-needed`: open product question. The scenario encodes the assumed answer.
