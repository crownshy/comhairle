# ADR-0062: Polis onboarding practises on real statements, one thing at a time

**Status:** proposed
**Date:** 2026-10-19, revised 2026-10-21

## Context

Most people who reach a Polis step have never used Polis. Issue #1203 asks for a short
onboarding that teaches by doing. A throwaway prototype (`explore/polis-onboarding-prototype`)
tried four variants on invented statements. The direction agreed on the issue afterwards: vote on
real statements from the conversation, a seed first and a participant statement second, show how
everyone voted, ask what is missing, then show the groups forming.

The first build put the guidance in a text box above the card, and nobody read it. The second put
it in driver.js spotlights over the full voting screen. Feedback on that one: still too much on
screen at once. The frame and the first statement arrived together, and the groups card sat above
a statement with the vote buttons still live.

## Decision

**1. One thing on screen at a time, alternating instruction and action.** Each stage is either a
screen with one instruction and a Next button, or the voting card with one instruction line above
it. The counter, Add your own, and Continue are hidden until onboarding ends.

1. **Intro.** "We want to hear what you think about {topic}", how voting works, Next.
2. **Organisers' statement.** "This was written by the organisers. What do you think?" Vote.
3. **How everyone voted.** That statement, its vote bar with Agree, Neutral and Disagree written
   under the bar, their own vote, and a line saying the bar changes as more people vote and they
   can come back to vote more. Next.
4. **Participant statement.** "This one came from someone like you who is taking part." Vote.
5. **A few more.** Three live votes, with a line saying how many are left before the groups.
6. **Is anything missing?** Add your own, or Keep voting.
7. **Groups.** Its own screen with the vote buttons hidden. With at least two groups and the
   person placed in one: every group's size as a bar, theirs marked, and what their group mostly
   agreed and disagreed with. Otherwise: no groups yet, what will happen as more people vote, and
   how many people and votes are in so far. Keep voting ends onboarding.

A Skip link sits on every stage.

Rejected: **driver.js spotlights** (the previous version of this ADR). The overlay still showed
the whole voting screen behind it, and a spotlight has nothing to point at for an instruction with
no control. Dropping it also drops the `driver.js` dependency.

**2. The practice statements come from Polis's comment list, not the report.** `PolisGetReportData`
only holds statements that a `math/pca2` run has reached, and fails before the first run, so a
participant statement written a minute ago was invisible and the participant stage was skipped.
`PolisApi.fetchStatements()` reads `/api/v3/comments` with voting patterns, the same list Polis
filters for participants, with a second `mod` check on strict steps. The most-voted seed goes
first, then the most-voted participant statement. Practice votes are real votes and count towards
the step's required votes.

**3. The result screen shows real numbers, with the participant's own vote added in.** Polis's
counts can lag the vote just cast, so the card adds it locally. A statement with no other votes
says "You are the first to vote on this one".

**4. Only seeds means the participant stage is deferred, not dropped.** The first participant
statement Polis serves them later gets the "someone like you" line.

**5. Groups are described by size and statements, never a letter.** Letters are not stable between
`math/pca2` runs. After onboarding, the groups screen comes back every 15 votes, but only when
there are groups to show, so it never interrupts voting to say "nothing yet".

**6. First-timer means no votes on this step.** Onboarding shows when the server and local vote
counts are both zero and it has not been dismissed on this device, keyed per user and step.

**7. The third vote option is "Neutral", not "Pass/Unsure".** It is a position, not a skip. The key
is now `polis_neutral` in every locale and the button lost its "next" chevron.

## Consequences

- `report_data` mapped `group-clusters[].members` straight to participant ids. Those are
  base-cluster ids, so group membership was wrong for every report. Fixed alongside this.
- The onboarding vote bar relies on #1242's order (agree, neutral, not voted, disagree) so the
  words under each end match the colours.
- `strict_moderation` and `topic` now reach the voting screen.
- The new copy is English only until it is translated.
- Open on #1203: whether admins pick the practice statements, how many votes `math/pca2` needs
  before groups mean anything, and whether the groups screen should show a map of where the
  person sits.
