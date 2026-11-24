# Moderation lab

A design sandbox for the Polis Moderate page with the moderation rules banner and panel.
Open `/moderation-lab` on the dev server. No API, login or Polis needed.

- Built on staging from just before PR #1906, which removed the rules banner and panel.
- `mockApi.ts` swaps the shared API client's transport for an in-browser fake, so accept,
  reject, split and undo all work. Data resets when you reload the page.
- The real page is `src/lib/tools/polis/PolisModeration.svelte`; the rules dialog is
  `polis-moderation/ModerationRules.svelte`. Edit those and the lab updates live.
- The lab hardcodes the default reject reasons. On the real page, "Reasons for rejecting"
  shows the conversation's moderation policy, while the practice round always uses the
  platform defaults so its feedback stays right whatever an organiser changes.
- Not shipped: delete this folder before merging anything from this branch.
