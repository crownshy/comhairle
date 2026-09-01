# ADR-0020: The logo asks before it leaves a step, and hands over the way back

**Status:** proposed
**Date:** 2026-09-01
**Branch:** `mobile-exploration/participant-step-chrome`

## Context

The participant step header carries the Comhairle mark on the left, linking to `/`. That is
the ordinary web convention, and in the step it is the wrong one.

The mark sits next to nothing else on a phone, which makes it the most tappable thing on
the screen and reads as the way out. A participant who takes it is dropped on the marketing
homepage: no step chrome, no progress bar, and nothing telling them their answers were
kept. Whether they can find their way back depends on browser history.

It is worse than the same tap elsewhere on the site, for the same reason ADR-0019 gave for
Your Rights: every tool except Learn is an embedded surface whose in-progress state we do
not persist per keystroke. The link is one tap and no confirmation.

Two separate things are wrong. Leaving is too easy for something we want to be deliberate,
and leaving is unrecoverable in the only sense participants care about, which is knowing
how to come back.

## Decision

**1. Inside a step, the logo opens a dialog rather than navigating.** The dialog says the
progress is saved, offers the link back, and puts the emphasis on staying: "Keep going" is
the filled button, "Leave" is quiet. Outside a step the logo is unchanged.

Rejected: removing the logo from the step header. It is the only mark of whose service this
is, and a participant arriving from a link needs it.

Rejected: `beforeNavigate` interception. It catches the logo and everything else, including
the deliberate step-to-step moves the pager and dropdown make, so every one of them would
need an exemption.

**2. The link offered is `/workflow/<id>/return`, not the current step's address.** That is
the route already behind the "come back to the conversation" links participants are emailed,
so it stays right after they move on, and it handles the finished and sealed cases. A
step-specific address would rot the moment they advanced.

**3. An anonymous participant is also told their anonymous id.** For them the link alone
only works in this browser, because progress is held against a session-backed anonymous
user. The id is what `/auth/anonymous-login` takes, so it is the honest second half of "how
to come back". A signed-in participant does not see that line and does not need it.

**4. Preview gets no dialog.** There is no progress to keep and no participant to reassure,
so the logo stays a plain link home. The dialog is the exception, not the default: the
chrome only asks when there is something to lose.

## Consequences

- `StepChrome` takes `returnUrl` and `anonymousId`. Passing neither is the honest way to
  say "this is not a real participation", which is what preview does.
- The copy claims progress is saved. That is true of everything the API has recorded, which
  is per-step progress and submitted tool data. It is not true of the state inside a tool
  that has not been submitted, and the dialog does not promise otherwise.
- The clipboard copy can be refused (insecure origin, or the browser says no). The address
  is in a readonly field and gets selected on failure, so it is always takeable by hand.
- Nothing else in the step chrome navigates away any more: the pager stays in the workflow,
  Your Rights opens in place (ADR-0019), and now the logo asks first. Leaving a step is a
  choice a participant has to make on purpose.
