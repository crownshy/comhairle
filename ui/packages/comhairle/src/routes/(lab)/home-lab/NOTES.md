# home-lab: Home + Conversations redesign experiment

A throwaway design prototype on mock data. Open `/home-lab` and use the floating
"Prototype scenario" panel (bottom left) to switch between scenarios.

## Scenario links

- Logged out, a few conversations: `/home-lab` (Comhairle theme)
- Logged in: `/home-lab?user=1`
- No conversations: `/home-lab?data=empty`
- 15 conversations: `/home-lab?data=many`
- Another organisation: `/home-lab?org=scot-gov` or `/home-lab?org=waves`

## Which idea each part tests

| Idea                                     | Where                                                                                                                                                        |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1. Home and list combined                | `+page.svelte`, a single page from hero to the full list                                                                                                     |
| 2. Mobile first                          | Built at 375px first, then `sm:`/`md:`/`lg:`                                                                                                                 |
| 3. Organisation branding                 | Theme-colour `LabTopBar` and visitor `Pitch` block, `FeaturedHero` gradient in the org colour, the real theme per organisation                               |
| 4. Empty, 100s, featured                 | `EmptyState`, chips (Popular, Latest, status) + `Pager`, `FeaturedHero`                                                                                      |
| 5. Big Take part in the first screen     | Button on each `FeaturedHero` slide, conversation page button + sticky bar                                                                                   |
| 6. Less reading on the conversation page | `c/[id]`: title and short description, an info bar (run by, time needed, closing) with a big Take part button over a full-width image; quotes and FAQs below |
| 7. Logged in vs logged out               | Welcome plus `PersonalStrip` (continue, report ready) at the top; no "Picked for you" until there is a way to recommend                                      |
| 8. Participant opinions as hooks         | `OpinionHook` on the conversation page only, to keep the home list light                                                                                     |

## Reused from staging

The logo files `ComhairleLogo` uses, `Footer`, `Pager`, shadcn `Button`, `Badge`,
`Carousel`, `Accordion`, `Progress` and the theme tokens.

The experiment lives in its own `(lab)` route group so it can draw its own top bar
without changing the staging NavBar. The three organisations are the three real themes
(`comhairle`, `scot-gov`, `waves` in `comhairle-themes.css`). The layout puts the chosen
one on `<html data-theme>` and in the theme store, and puts the app's own theme back when
you leave the prototype.

## Card rules

- Status sits top-left on the image, in the same `StateBadge` style everywhere.
- Then participants (a total, never a live count) and timing, always "Closes in N days".
  Timing wording lives in `timingLabel()` in `mockData.ts`.
- `LabConversationCard` copies the layout of the public `ConversationCard`, because the
  staging card has no place for the meta row.

## Open questions

- The top bar and the visitor pitch share the theme colour so they read as one block. The
  Waves logo is white lettering, so it only works on a coloured bar like this one.
- Should the featured carousel auto-advance? It doesn't now, for accessibility.
- "Featured", topics, minutes, closing dates, participant counts and quotes aren't in
  `ConversationDto` yet. They'd need API work.

## Moving media on featured slides

A featured conversation can set `featuredMedia` in `mockData.ts` to a GIF or a muted
looping video (`{ kind: 'video', src }`). Put the file in a `media/` folder here and import
it like the photos at the top of `mockData.ts`. Video (MP4/WebM) is far lighter than GIF
for the same clip. Without media, the photo slowly pans and zooms, which stops for people
who turn on "reduce motion".

## Featured slide contrast

The slide text sits on frosted glass: the photo blurred under the theme colour darkened by
30% at 80% strength, fading into the sharp photo only past the text. Blur alone doesn't
protect contrast, because big bright areas (sky, white walls) stay bright when blurred; the
darkening does. Even over pure white it stays above 4.5 for all three themes, and on the
real photos, measured at several points of the slow zoom, the lowest was 6.8 Comhairle,
6.0 Scottish Government and 5.2 Waves. Browsers without blur support still get the
darkened colour, so they pass too.

Also tried and set aside: solid theme colour behind the text (7.8 / 5.8 / 4.9, most on
brand) and a gradual blur from a reference card (7.1 / 6.8 / 6.2, but the theme barely
shows and it's heavier for older phones). The earlier see-through overlay failed on bright
photos.

## Organiser on cards

Every card shows who runs the conversation: above the title on list cards (as staging
`ConversationCard`'s `organizationName`), and next to "Featured" on slides. The names in
`mockData.ts` are made-up teams. The API would need an organiser field per conversation.
Featured slides leave out participants and closing date to keep them light.
