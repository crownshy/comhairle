# home-lab: Home + Conversations redesign experiment

A throwaway design prototype on mock data. Open `/home-lab` and use the floating
"Prototype scenario" panel (bottom left) to switch between scenarios.

## Scenario links

- Logged out, a few conversations: `/home-lab` (Comhairle theme)
- Logged in: `/home-lab?user=1`
- No conversations: `/home-lab?data=empty`
- 15 conversations: `/home-lab?data=many`
- Any conversation page with a test banner image: add `&img=cutout` (or `wide-portrait`, `landscape`, `group`, `tall`), or use the panel's Conversation page image row
- Another organisation: `/home-lab?org=scot-gov` or `/home-lab?org=waves`

## Which idea each part tests

| Idea                                     | Where                                                                                                                                                                     |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1. Home and list combined                | `+page.svelte`, a single page from hero to the full list                                                                                                                  |
| 2. Mobile first                          | Built at 375px first, then `sm:`/`md:`/`lg:`                                                                                                                              |
| 3. Organisation branding                 | Theme-colour `LabTopBar` and visitor `Pitch` block, `FeaturedHero` gradient in the org colour, the real theme per organisation                                            |
| 4. Empty, 100s, featured                 | `EmptyState`, chips (Popular, Latest, status) + `Pager`, `FeaturedHero`                                                                                                   |
| 5. Big Take part in the first screen     | Button on each `FeaturedHero` slide, conversation page button + sticky bar                                                                                                |
| 6. Less reading on the conversation page | `c/[id]`: status with people joined and closing, title, a three-line description and Take part, then a rounded image across the white to grey join; quotes and FAQs below |
| 7. Logged in vs logged out               | Welcome plus `PersonalStrip` (continue, report ready) at the top; no "Picked for you" until there is a way to recommend                                                   |
| 8. Participant opinions as hooks         | `OpinionHook` on the conversation page only, to keep the home list light                                                                                                  |

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

- The top bar and the visitor pitch share the theme colour so they read as one block.
- Should the featured carousel auto-advance? It doesn't now, for accessibility.
- "Featured", topics, minutes, closing dates, participant counts and quotes aren't in
  `ConversationDto` yet. They'd need API work.

## Moving media on featured slides

A featured conversation can set `featuredMedia` in `mockData.ts` to a GIF or a muted
looping video (`{ kind: 'video', src }`). Put the file in a `media/` folder here and import
it like the photos at the top of `mockData.ts`. Video (MP4/WebM) is far lighter than GIF
for the same clip.

## Featured slide contrast

The featured card puts its text on a white panel under the photo, so contrast never
depends on the photo, GIF or video: dark text on white, and the organiser and button in
the theme colour (Comhairle 7.8, Scottish Government 5.8, Waves 4.9, all WCAG AA). The
panel has a notch at its top-left where the Featured badge sits on the photo. A soft band
of theme colour along the bottom of the photo is decoration only, with no text on it.

Also tried and set aside, all with text over the photo: a see-through colour overlay
(failed on bright photos), solid theme colour behind the text, frosted glass (blurred
photo under darkened theme colour) and a gradual blur from a reference card. The last
three passed, but only because of how much they darkened or covered the photo.

## Organiser on cards

Every card shows who runs the conversation: above the title on list cards (as staging
`ConversationCard`'s `organizationName`), and next to "Featured" on slides. The names in
`mockData.ts` are made-up teams. The API would need an organiser field per conversation.
Featured slides leave out participants and closing date to keep them light.

## Visitor hero

The logged-out hero is a light gradient made from each theme's accent colour
(`--sidebar-primary`: lime for Comhairle, blue for Scottish Government, lilac for Waves),
with two see-through white shapes for texture. It runs up behind the top bar, which stays
see-through with dark text and logo until the page scrolls, then turns to the theme
colour. Text on the hero is dark: at the darkest point of the gradient it's about 10.8
Comhairle, 7.6 Scottish Government and 5.6 Waves. The Waves logo file is white lettering only, so the light bar uses a prototype-only dark copy (`media/waves-logo-dark.png`, white parts turned #444749); an official dark version should replace it.
