# ADR-0062: Polis onboarding practises on real statements, one thing at a time

**Status:** proposed
**Date:** 2026-10-19, revised 2026-10-22

## Context

Most people who reach a Polis step have never used Polis. Issue #1203 asks for a short
onboarding that teaches by doing. A throwaway prototype (`explore/polis-onboarding-prototype`)
tried four variants on invented statements. The direction agreed on the issue afterwards: vote on
real statements from the conversation, a seed first and a participant statement second, show how
everyone voted, show the groups forming, then ask what is missing.

Three builds came before this one:

1. The guidance sat in a text box above the card, and nobody read it.
2. driver.js spotlights over the full voting screen. Too much on screen at once: the frame and
   the first statement arrived together, and the groups card sat above live vote buttons.
3. One thing on screen at a time with no spotlight. Clearer, but the step chrome around it still
   competed for attention, and the instruction line above the card was easy to skip.

## Decision

**1. One thing on screen at a time, under a spotlight.** Each stage is either a screen with one
instruction and a Next button, or the voting card with one instruction card above it. driver.js
dims everything outside that area at 85%, with no popover and no animation. A popover painted in
one place and moved once the card settled, so the instruction lives in the lit area instead.

1. **Intro.** "We want to hear what you think about {topic}", how voting works.
2. **Organisers' statement.** "This was written by the organisers. What do you think?" Vote.
3. **How everyone voted.** That statement, its vote bar with Agree, Neutral and Disagree written
   under it, their own vote, and a line saying the bar changes as more people vote.
4. **Participant statement.** "This one came from someone like you who is taking part." Vote.
5. **How everyone voted** on that one too.
6. **A few more.** Three live votes, with a line saying how many are left before the groups.
7. **Groups.** Always shown. With at least two groups and the person placed in one: every
   group's size as a bar, theirs marked, and what their group mostly agreed and disagreed with.
   Otherwise: as more people vote, people who vote alike will form groups.
8. **Do you want to add something?** Add your own, or Keep voting, with a line saying they can
   always come back and add something. Either ends onboarding.

While a statement saves, the statement skeleton shows, as it does between live votes.

**2. Next sits in the lit card; the pager's forward corner is Skip the intro.** The pager stays
above the overlay and usable, so nobody is trapped. Its forward label comes from a new optional
`nextLabel` on the tool sequence contract (ADR-0047).

Rejected: Next in the pager. Under the overlay it was the one control on screen that was not
lit, and people did not find it.

**3. The practice statements come from Polis's comment list; the counts come from the report.**
`PolisGetReportData` only holds statements a `math/pca2` run has reached, so a participant
statement written a minute ago is missing from it. `PolisApi.fetchStatements()` reads the
participant `/api/v3/comments`, with a second `mod` check on strict steps. Polis only sends vote
counts on its moderator list, so the participant list has none. The counts are taken from the
report by `tid`. A statement the report has not reached shows no bar and says its votes are still
being counted. It never claims they are the first to vote unless the counts say so. Practice
votes are real votes and count towards the step's required votes.

**4. Only seeds means the participant stage is deferred, not dropped.** The first participant
statement Polis serves them later gets the "someone like you" line.

**5. Groups are described by size and statements, never a letter.** Letters are not stable
between `math/pca2` runs. After onboarding, the groups screen comes back every 15 votes, but only
when there are groups to show.

**6. First-timer means no votes on this step.** Onboarding shows when the server and local vote
counts are both zero and it has not been dismissed on this device, keyed per user and step.

**7. The third vote option is "Neutral", not "Pass/Unsure".** It is a position, not a skip.

**8. Adding an opinion is a dialog, not a screen.** A centred dialog on a desktop and a bottom
sheet on a phone, over the voting card, so closing it leaves them on the statement they were on.
The tips sit above the box on a desktop and below it on a phone, so the box is above the fold.
The moderation policy opens in place of the form, not as a second sheet. Submitting swaps to a
thank-you that quotes their opinion and says who can vote on it now, or after the organisers have
checked it on a strict step. The dialog keeps one size across all three views. The spotlight is
off while it is open, because the overlay sits above dialogs.

## Consequences

- `report_data` mapped `group-clusters[].members` straight to participant ids. Those are
  base-cluster ids, so group membership was wrong for every report. Fixed alongside this.
- The vote bar relies on #1242's order (agree, neutral, not voted, disagree) so the words under
  each end match the colours.
- `strict_moderation` and `topic` now reach the voting screen.
- "Skip the intro" in the pager's forward corner bends ADR-0048, which says that slot reads Next
  or Skip.
- The step name scrolling with the tool (ADR-0063) is what stops it showing through the lit area.
- The new copy is English only until it is translated.
- Open on #1203: whether admins pick the practice statements, how many votes `math/pca2` needs
  before groups mean anything, and whether the groups screen should show a map of where the
  person sits.
