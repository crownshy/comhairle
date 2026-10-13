# HeyForm embed: notes

The participant-facing form (`HeyFormEmbed.svelte`) is our HeyForm fork rendered in a
cross-origin iframe. We cannot reach into its document, so everything below goes over
`postMessage` or the form URL.

## Message contract

Every message the fork posts is tagged `source: 'HEYFORM'` (see its `sendMessageToParent`):

| Message            | Payload            | Meaning                                  |
| ------------------ | ------------------ | ---------------------------------------- |
| `FORM_RESIZE`      | `{ height: <px> }` | height the frame needs for this question |
| `FORM_STEP_CHANGE` | `{}`               | a new question became active             |
| `HIDE_EMBED_MODAL` | `{}`               | the form finished                        |

Messages we send are tagged `source: 'COMHAIRLE'`:

| Message          | Payload                               | Meaning                                |
| ---------------- | ------------------------------------- | -------------------------------------- |
| `REQUEST_RESIZE` | `{}`                                  | asks the fork to re-emit `FORM_RESIZE` |
| `SET_THEME`      | `{ theme: { <colour>: <hex>, ... } }` | repaints the form in our palette       |

We post with target origin `'*'` rather than the survey base URL. The fork gates on
`source: 'COMHAIRLE'`, and a survey origin that redirects would silently drop a message
addressed to the original URL.

## Height

The fork measures its active question and posts `FORM_RESIZE`; we size the iframe to it.
Long or grouped questions get the room they need and short ones do not leave an empty
card. The URL carries `hostScroll=true`, which tells the fork that the page scrolls and
the frame never does. Without it a swipe on a phone goes to the form's inner scroller
first, so the participant scrolls twice.

The fork's first `FORM_RESIZE` is a one-shot emit. On a hard refresh a cached iframe can
boot and emit before our listener is attached during hydration, so the height is missed.
`requestResizeUntilAnswered` pings `REQUEST_RESIZE` on `load` and keeps pinging until a
height arrives or a timeout passes.

Until the first height arrives the markup falls back to the skeleton's fixed height. A
fork build without the emit renders every form at that height, so the emit must be
deployed alongside this component.

## Keeping the question in view

Because the page scrolls and not the frame, answering a tall question near the bottom
can leave the next one above the fold, or the page shrinks and strands the viewer at the
footer. `alignFrameTop` pulls the frame's top edge back to the top of the viewport.
Scrolling the window to 0 would be wrong: a step description can run to many paragraphs,
so top-of-page routinely leaves the question off screen.

Two rules keep this from fighting the user:

- `FORM_STEP_CHANGE` is the only trigger. `FORM_RESIZE` also fires on mount and on every
  reflow (a textarea growing, validation, fonts settling), so acting on it would hijack
  someone reading or typing further down the page.
- We only scroll up, and only when the frame's top is already above the viewport.

The new question's height lands in a `FORM_RESIZE` just after the step change. Aligning
before it applies would scroll against the outgoing question's box, so the align waits for
that message, bounded by `ALIGN_AFTER_STEP_CHANGE_MS`.

## Theme

A HeyForm form stores one fixed set of colours chosen when it was built, and the iframe
cannot see the viewer's light / dark mode or this deployment's palette. Our fork accepts
five colours per request and derives its whole `--heyform-*` variable set from them.
`embedTheme.ts` maps our tokens onto those five fields.

The palette goes on the form URL for the first paint (`bootPalette`, read once at setup).
It is deliberately not reactive: a palette that changed the URL would reload the frame
and throw away everything the participant has typed. Reading it at setup is safe because
`app.html` sets the `dark` class in a pre-paint script and the server stamps `data-theme`
on the document, so the palette is settled before any component runs.

A change after boot, the viewer toggling light / dark mid-form, goes over `SET_THEME` and
the fork repaints in place. Two pieces of state make that reliable:

- `framePalette` is what the frame is currently showing, keyed by theme name and mode.
  A toggle back and forth, or a rerun of the effect for another reason, posts nothing.
  It resets on `load` because a fresh document shows whatever its URL asked for.
- `frameListening` covers a race. Our `load` fires when the document arrives, but the
  renderer attaches its message listener when it mounts, some way after that, so a theme
  posted at `load` can be dropped. The frame's own messages are the proof it is up, and
  the `REQUEST_RESIZE` ping guarantees one arrives even if the mount emit was missed. So
  the palette is held until we hear from the frame.

The effect that watches the theme reads the colours a frame later rather than
synchronously. `ThemeProvider` writes `dark` and `data-theme` in an effect of its own,
and the delay means we never sample the outgoing palette if that effect runs after ours.

Only solid hex tokens are sent. The fork derives its hover and disabled shades by pulling
the channels apart and reassembling them at a fixed opacity, so a token that already
carries alpha (`--border` and `--input` are `#ffffff19` in dark) would come out wrong.
Such tokens are dropped rather than sent.
