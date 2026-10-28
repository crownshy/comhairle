# Issue tracker: GitHub

Issues and specs live in GitHub Issues on `crownshy/comhairle`. Use the `gh` CLI for everything.

## Writing an issue

Keep issues short enough to skim. The first line says what changes and for whom, so a reader knows what the ticket is from that sentence alone. Background and technical notes go in a collapsed `<details>` block at the end.

```markdown
**In short:** one sentence: what changes, for whom.

## Done when

- [ ] Observable outcome
- [ ] Observable outcome

## Blocked by

- #123

## Open questions

- Only questions that are still open.

<details><summary>Details</summary>

Background, current state, technical notes, links.

</details>
```

Leave out any section that would be empty. Write "Done when" as behaviour someone can check, not a list of layers to change. Keep file paths and code in Details, since they go stale.

Branch for an issue with `gh issue develop <number>` so the PR links itself. Don't put "Closes #<number>" in PR bodies.

## Conventions

- **Create an issue**: `gh issue create --title "..." --body "..."`. Use a heredoc for multi-line bodies.
- **Read an issue**: `gh issue view <number> --json number,title,body,labels,comments`.
- **List issues**: `gh issue list --state open --json number,title,body,labels,comments --jq '[.[] | {number, title, body, labels: [.labels[].name], comments: [.comments[].body]}]'` with `--label` and `--state` filters.
- **Sub-issue of a parent**: `gh issue create --parent <parent> ...`, or `gh issue edit <parent> --add-sub-issue <child>` afterwards.
- **Blocking**: use GitHub's native issue dependencies: `gh issue create --blocked-by <n>,<n> ...` or `gh issue edit <child> --add-blocked-by <blocker>`. Also list blockers under "Blocked by" in the body so they show when skimming. A ticket is unblocked when every blocker is closed.
- **Count open blockers**: `gh api repos/crownshy/comhairle/issues/<n> --jq .issue_dependencies_summary.blocked_by` (0 means it can start).
- **Comment**: `gh issue comment <number> --body "..."`
- **Labels**: `gh issue edit <number> --add-label "..."` / `--remove-label "..."`
- **Close**: comment with the reason first (`gh issue comment`), then `gh issue close <number>`.

File issues only into `crownshy/comhairle`. Keep partner and colleague names out of issue titles and bodies.

## Pull requests as a triage surface

**PRs as a request surface: no.**

## When a skill says "publish to the issue tracker"

Create a GitHub issue in the format above.

## When a skill says "fetch the relevant ticket"

Read it as in **Read an issue** above.
