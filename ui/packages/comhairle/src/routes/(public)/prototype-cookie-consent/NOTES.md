# Cookie consent prototype

**Question:** what should the cookie consent moment look like so it does not read as
"ugh, cookies again" to a 12 to 26 year old audience?

Four variants on the existing landing page, switchable via `?variant=`:
`/?variant=A` through `/?variant=D`. Arrow keys cycle, the replay button re-shows a
dismissed variant. Switcher is dev-only.

## What is actually being consented to

From `CookiesSettingsContent.svelte`, this app sets exactly two cookies, both strictly
necessary: `auth_token` and `paraglide_lang`. Umami analytics is also loaded on every
page from the root layout, unconditionally, and sets no cookie.

That matters for the design. Under PECR, strictly necessary cookies do not need consent,
so the current banner is asking a question that has no real answer: "Reject all" cannot
do anything except break the site. That is a large part of why it feels like noise.

## The variants

- **A - Nothing to decide.** No overlay, no page block. A slim strip that states the two
  cookies in one sentence and gets out of the way. No reject button, with an inline
  explanation of why there is not one. This is the honest reading of the current cookie
  set, and it only holds while nothing optional exists.
- **B - Swipe deck.** Full screen, one thing per page, progress rail, a cue at the end of
  every page. Same idiom as the participant step brief (ADR-0023, ADR-0024), so consent
  reads as part of the product rather than a legal interruption.
- **C - Ask me anything.** A short thread. The site opens with two lines, you reply with
  chips ("What are the two?", "Who sees this?"). Reading more is a reply, not a link out
  to a policy page.
- **D - The jar.** Direct manipulation. Each stored thing is a disc you can tap to
  inspect; the two essentials are locked, the optional one tips out into a tray. The
  playful one, and the one with the most room to go wrong tonally.

B, C and D treat Umami as a genuine opt-out so there is something real to decide.
That is a design proposal, not current behaviour: today the analytics script loads
regardless. If we keep the "your call" framing in the winning variant, the root layout
has to actually honour it.

## Verdict

TODO - fill in which variant won and why, then delete this folder and the `?variant=`
mount in `../+page.svelte`.
