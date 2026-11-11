# home-lab: Home + Conversations redesign experiment

A throwaway design prototype on mock data. Open `/home-lab` and use the floating
"Prototype scenario" panel (bottom left) to switch between scenarios.

## Scenario links

- Logged out, a few conversations: `/home-lab`
- Logged in: `/home-lab?user=1&org=riverside`
- No conversations: `/home-lab?data=empty&org=riverside`
- 150+ conversations: `/home-lab?data=many&org=riverside`
- A different organisation's brand: `/home-lab?org=health`

## Which idea each part tests

| Idea                                     | Where                                                                                                                   |
| ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| 1. Home and list combined                | `+page.svelte`, a single page from hero to the full list                                                                |
| 2. Mobile first                          | Built at 375px first, then `sm:`/`md:`/`lg:`                                                                            |
| 3. Organisation branding                 | White `LabTopBar` with the org logo, `FeaturedHero` gradient in the org colour, `brandStyle()` re-colouring `--primary` |
| 4. Empty, 100s, featured                 | `EmptyState`, chips (Popular, Latest, status) + `Pager`, `FeaturedHero`                                                 |
| 5. Big Take part in the first screen     | Button on each `FeaturedHero` slide, conversation page button + sticky bar                                              |
| 6. Less reading on the conversation page | `c/[id]`: summary, meta row, button; the rest goes in an accordion                                                      |
| 7. Logged in vs logged out               | `PersonalStrip` (continue, report ready), "Picked for you"                                                              |
| 8. Participant opinions as hooks         | `OpinionHook` on the conversation page only, to keep the home list light                                                |

## Reused from staging

`ComhairleLogo` (light-surface version for the white bar), `Footer`, `Pager`, shadcn
`Button`, `Badge`, `Carousel`, `Accordion`, `Progress` and the theme tokens.

The experiment lives in its own `(lab)` route group so it can draw a white top bar
without changing the staging NavBar. The "Current theme" organisation is the running
theme with nothing overridden; the others override `--primary` with a made-up brand.

## Card rules

- Status sits top-left on the image, in the same `StateBadge` style everywhere.
- Then participants (a total, never a live count) and timing, always "Closes in N days".
  Timing wording lives in `timingLabel()` in `mockData.ts`.
- `LabConversationCard` copies the layout of the public `ConversationCard`, because the
  staging card has no place for the meta row.

## Open questions

- Adopting the white top bar for real means a light variant of the staging NavBar.
- Should the featured carousel auto-advance? It doesn't now, for accessibility.
- "Featured", topics, minutes, closing dates, participant counts and quotes aren't in
  `ConversationDto` yet. They'd need API work.
- The staging list layout of `ConversationCard` is admin-only, so `ConversationRow` is new.

## Moving media on featured slides

A featured conversation can set `featuredMedia` in `mockData.ts` to a GIF or a muted
looping video (`{ kind: 'video', src }`). Put the file in a `media/` folder here and import
it like the photos at the top of `mockData.ts`. Video (MP4/WebM) is far lighter than GIF
for the same clip. Without media, the photo slowly pans and zooms, which stops for people
who turn on "reduce motion".
