# End-to-end tests

Playwright tests that drive the real app in a browser. Today they cover the admin side
(`admin.*.test.ts`). Participant tests belong here too.

## Running them

You need the API on `:3000` with a seeded admin (`just seed`), and a `.playwright.env` in
`ui/packages/comhairle`:

```
PLAYWRIGHT_ADMIN_USERNAME=admin@crown-shy.com
PLAYWRIGHT_ADMIN_PASSWORD=adminPassword123!
```

Then, from `ui/packages/comhairle`:

```bash
npx playwright test --project=chromium
```

Playwright builds the app and serves it with `vite preview` on `:4173`. If a preview is already
running there it reuses it, which is faster when you're iterating, but it serves the last build.
After changing app code (not test code), rebuild with `pnpm build` or you'll be testing the old
app.

Don't run `pnpm test:unit` at the same time. Both regenerate `src/lib/paraglide` and break each
other's build.

To debug one test: `npx playwright test e2e/admin.conversation.design.test.ts --debug`. When a
test fails, `test-results/<test>/error-context.md` has the page's accessibility tree at the
moment it failed. Read that first: it shows the real labels and roles on the page.

CI runs the Chromium project on pull requests to `staging`
(`.github/workflows/playwright.yml`).

## Layout

- `admin.<section>.<tab>.test.ts`: one file per admin page.
- `utils/testing.ts`: the `test` to import. It opens the app before each test and provides
  `cleanup`. Import `test` from here, not from `@playwright/test`, or the page starts blank.
- `utils/auth.ts`: `login(page)`. Every test gets a fresh browser, so every test logs in.
- `utils/navigation/`: getting around. `Conversation.open` finds or creates the shared
  "Playwright testing" conversation. `Conversation.openLaunched` does the same for a second,
  launched one (Recruit, Notify, Monitor and Report only open after launch, and launching locks
  set-up). `Conversation.openTab` clicks a sidebar section and waits for it to load.
- `utils/components/`: helpers for inputs that need more than a click (switches, rich text,
  the step board). Make one object per field, for example
  `await Switch('Show conversation publicly', { page, cleanup })` or
  `await Textbox('Title', { page, cleanup })`. Each one records what it changed so its
  `expect()` can check it after a reload, and passing `cleanup` undoes the change after the
  test.

## Writing a test

**Find things by role and label, scoped to a region.**
`page.getByRole('main').getByRole('button', { name: 'Add step' })` is good.
`page.getByText('Configure')` is not: the page header has guide links like "Configure a step",
and a loose text match will hit those. Use `exact: true` when one label contains another
("Reason" and "What counts under this reason"). If an element has no accessible name, give it
one in the app (`aria-label`). That fixes it for screen readers too.

**Wait for the page, not the clock.** Most failures we fixed were tests reading a page before it
had loaded. The page tells you when it's ready:

- sidebar links get `aria-current="page"` once their section has loaded
- Configure tabs get `aria-selected="true"`
- the Recruit and step sub-tab links get `aria-current="page"`

Assert on those (`openTab` already does), then use `expect(locator)` assertions, which retry.
`locator.isVisible()` and `count()` answer straight away without waiting, so only use them to
branch on state you already know has loaded. Don't add `sleep()`.

**Expect clicks before hydration.** Pages are server-rendered, so a button can be visible
before the app has attached its handlers, and an early click does nothing. Locally this rarely
shows, but it broke every test on the slower CI runner. When a click should open a menu or
dialog, retry it until that appears (`expect(...).toPass()`, see `Conversation.open`), and wait
for `networkidle` before typing into a page you just navigated to.

**Know how saving works.** Autosave sends the data without reloading the page, so anything
rendered from server data (the sidebar title, the workflow step strip) only updates after a
reload or a later navigation. Check server-rendered state after `testWithRefresh`, or wait for
something that changes once the save lands. The step board helper waits for the workflow step
strip, for example.

**Expect leave guards.** Some editors (the Learn step pages) cancel in-app navigation with a
`confirm()` while a save is in flight. Playwright dismisses the dialog, so the click does
nothing. Retry the click inside `expect(...).toPass()` (see `Design.subtab`).

**Clean up after yourself.** The tests share one database and one conversation. Register undo
steps with `cleanup(fn)`. They run in reverse order after the test, so later changes come off
first. Make the start of a test tolerate leftovers from a failed run: the step board helper
deletes existing steps first, and the Team test removes the role if it's already there.

**Run it twice.** A test that passes once can still race. Before you're done, run it with
`--repeat-each=3`.

## Participant tests

Nothing exists yet, but the same rules apply. Things to work out first:

- **Which conversation.** Participants need a launched conversation with steps in it. Build on
  `Conversation.openLaunched` and set up the steps the test needs as the admin, before switching
  to a participant.
- **Which user.** Use a second browser context (`browser.newContext()`) for the participant so
  the admin session doesn't leak in. Anonymous participation depends on the conversation's
  Access settings ("Automatically log in with an anonymous account").
- **External tools.** Polis and HeyForm steps load from other services. Decide whether a test
  covers the embed or stops at the step shell, and say which in the test name.
- **Preview.** `/conversations/<id>/preview` walks the steps without launching. It's a cheaper
  place to cover step navigation.
