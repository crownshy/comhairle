# ADR-0039: A Learn page loads without reflowing

**Status:** proposed
**Date:** 2026-09-14
**Branch:** demo branch

## Context

The first Learn step of a session went through six visibly different frames: the article
skeleton, the page in a narrow column, the same page in a wider column, a small video player
in a black box, the full-size video, and a buffering spinner over it.

Three separate causes:

- The article column was `.prose`'s `max-width: 65ch`. A `ch` is the width of the font's "0",
  so the column measured 651px while the fallback font showed and 738px once Inter loaded.
  Inter loaded late because a browser only fetches a face when something on screen uses it,
  and a step cover uses few of the article's weights.
- Uploaded videos were embedded as an `<iframe>` pointing at the .mp4. The browser opens a
  file like that in its own media page, which draws the player at the file's intrinsic size
  until the metadata arrives, then resizes it and shows a buffering spinner.
- LearnUI's code loads on demand (the 502 fix in `StepToolBody`), and nothing fetched it
  until Start was pressed.

## Decision

**1. The Learn column is 46rem.** That is Inter's 65ch at the 18px body, and it holds
whichever font is up. The skeleton, the Listen offer and the assistant use the same width.

**2. A video file renders as `<video>`.** The stored document keeps its `iframe` node. When
the src is a video file (by extension, which media library uploads keep), `renderHTML` emits
`<video controls playsinline preload="metadata">` in the same 16:9 wrapper, with `#t=0.001`
so iOS Safari paints the first frame. YouTube and Vimeo stay iframes.

Rejected: **a separate video node.** Every page already holding an iframe with an .mp4 src
would need migrating, and the stored shape was never what broke.

**3. Embeds hold a placeholder until they can paint.** `ContentRenderer` marks an iframe or
video `data-loaded`, as it already did for images, and until then the wrapper shows a pulsing
block behind the embed.

**4. The step page fetches the tool's code and the article's font faces while the cover is
up** (`preloadTool`, `warmFonts`). On the usual path Start finds both ready and no skeleton
shows.

**5. The skeleton follows the page's outline** when the page is stored as ProseMirror JSON:
headings, text, quotes, lists, and video or image boxes, with line counts estimated from the
text length. Markdown pages keep the generic skeleton.

## Consequences

- The editor renders video files as `<video>` too, so authors see the player participants
  get. Pasting a copied video parses back to the same iframe node.
- `isVideoFileUrl` goes by extension. A video served from a URL without one still opens in an
  iframe.
- Skeleton line counts are tuned to the desktop column, so on a phone the outline runs a
  little shorter than the page.
- Other `.prose` pages (about, privacy) still use 65ch and still shift when the font loads.
