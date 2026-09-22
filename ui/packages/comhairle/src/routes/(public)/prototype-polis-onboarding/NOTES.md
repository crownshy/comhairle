# Polis onboarding prototype (#1203)

**Question:** what should the first minute of a Polis step look like for someone who has
never used Polis, so they get the point and actually read it?

Four variants on a throwaway route, switchable from the dev-only bar at the bottom:
`/prototype-polis-onboarding?variant=A` through `D`, plus `&audience=young` or `policy`.
Arrow keys cycle variants, the replay button restarts one. After onboarding, each variant
hands off to a stand-in for the real voting screen, so the hand-off can be judged too.

It sits on its own route rather than inside `PolisEmbed.svelte` so it can go in front of
testers without a login, a conversation, or a running Polis.

## What first-timers get stuck on

From the ticket: why am I voting on other people's sentences, why can't I reply, what
happens to my votes, and why would I write my own.

## Habits the variants borrow

- People close intro carousels without reading them. Every variant starts with
  something to tap within a couple of seconds.
- Instagram story polls show the result straight after you vote. People expect that
  reward, and it is also the Polis payoff (how you compare), so A, C and D use it.
- Swiping and tapping on one card at a time is familiar from dating and shopping apps, so
  the practice card looks the same as the real one.
- A quiz makes people guess before they're told, and a guess makes them read the answer.

## ADHD-friendly rules applied to all four

- The first screen is a tap, not a paragraph.
- One idea per screen, headlines under about ten words.
- Visible progress ("Warm-up 2 of 3") and a time cost up front ("20 sec").
- Skip is always on screen.
- "Doesn't count" on every practice statement, so nobody worries about a wrong vote.
- Feedback straight after each tap (percentages, moving dots, a check mark).

## The variants

- **A - Poll reveal.** Three practice votes. After each tap, bars show how the crowd
  voted. Then a screen showing the two groups you just made appear and the statement both
  groups agreed on, then four one-line rules. The most complete, and the longest.
- **B - Coach marks.** No intro. The real voting screen with a spotlight on one control
  at a time: the statement, the thumbs (you vote on a practice statement), "Add your own",
  then "no replies". The lightest touch, for people who skip every tour. It teaches the
  controls but only hints at why Polis is interesting.
- **C - Living map.** A crowd of dots above a practice statement. Each vote moves the
  whole crowd and drops your dot in. After three votes, two groups are shaded and the
  common-ground statement shows. The most "why is this cool" per second, and the hardest
  to build for real.
- **D - Myth or fact.** Three true-or-false calls on the things people get wrong
  (replies, who sees your votes, adding your own), with a practice vote inside the first
  one. Ends on a score. Aimed squarely at the confusion list, weaker on the payoff.

Practice statements are deliberately low stakes (pineapple on pizza, meeting length) so
they don't prime anyone on the real topic. The two split statements are correlated on
purpose so a pattern shows up after only three votes.

## Needs checking before any of this ships

- "Nobody sees how you voted" (A and D). Confirm what admins and reports can see before
  we promise it.
- The crowd numbers are fake. The real version either keeps a fixed fake crowd or says so
  on screen.
- Seen-once behaviour isn't built. The real version would follow `polisGuidance.ts`.
- Still open from the ticket: whether this lives inside the Polis step or as
  step-description slides. Slides already exist (ADR-0017), so a plain slides version can
  be tested against these without building anything.

## How to test

Give 3 to 5 people who have never used Polis one variant each on their phone, with no
explanation. Then ask them:

1. What are you being asked to do here?
2. What happens to your votes?
3. Can you reply to someone's opinion?
4. Why would you add your own?

Also note whether they hit Skip, and how long they took.

## Verdict

TODO - fill in which variant won and why, then delete this folder.
