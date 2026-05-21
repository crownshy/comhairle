# Vendored: svelte-pdf-viewer

Source: https://github.com/karbasia/svelte-pdf-viewer (`src/lib`, `main` branch)

These files are copied verbatim. The upstream repo publishes under the npm name
`svelte-pdf`, which is already taken on npm by an unrelated package, and a
`github:` install does not work because the repo ships no built `dist/`. Vendoring
the source is therefore the only viable way to consume it.

Project-side glue lives in the parent `PdfViewer/` directory.

## Local modifications

All edits are marked inline with `// Local patch (comhairle)`.

1. **Resize reactivity** — fit-to-width/page scale did not recompute on container
   resize (the `calculatedScale` derived read `clientWidth` untracked). A reactive
   `resizeTick` signal is bumped by the `ResizeObserver` and tracked by
   `calculatedScale`.

2. **Zoom under a fitMode** — the `+`/`-` buttons only mutated `scale`, but with
   `fitMode="width"`/`"page"` the rendered scale is derived from the container
   and ignores `scale`, so zoom did nothing. A `userZoom` multiplier is now
   applied on top of the fit scale; `zoomIn`/`zoomOut`/`resetZoom` adjust
   `userZoom` (and call `triggerRender()`) when a fitMode is active.

3. **Toolbar styling** — the navigation toolbar now uses the app's shadcn
   `Button` component instead of raw `<button>` elements, all colours come from
   the theme CSS variables (no hardcoded hex), and the two button groups wrap
   onto separate rows on narrow screens. `PDFViewer.svelte` imports
   `$lib/components/ui/button`; the toolbar markup and the `.pdf-navigation` /
   `.pdf-nav-group` rules in `PDFViewer.css` carry the change. The document is
   also top-aligned (`.pdf-content`) instead of vertically centred.

4. **Fit scale in continuous mode** — `calculatedScale` only worked when
   `currentPageProxy` was set, which never happens in `displayMode="continuous"`,
   so continuous mode ignored `fitMode` and rendered at scale 1.0. It now falls
   back to `allPageProxies[0]` for the page, and measures the scroll container
   (whose `clientWidth` excludes the vertical scrollbar) so pages fit exactly.
   `.scroll-container` also lost its horizontal padding for the same reason.

5. **Mobile page navigation** — the Previous/Next buttons carry a
   `pdf-page-button` class and are hidden below 640px (`PDFViewer.css`), since
   continuous scroll replaces page stepping on small screens.

When re-pulling from upstream, re-apply these patches (or drop them if upstream
fixes the resize reactivity / fitMode zoom).
