/**
 * Fetch the text faces an article draws with before it is on screen. A browser only fetches
 * a face once something visible uses it, and Google Fonts' `display=swap` paints fallback
 * glyphs in the meantime. A step cover uses few of the article's weights and no italics, so
 * the article was drawing in the fallback font on Start and redrawing in Inter a moment later.
 *
 * The family comes off the body, so it follows whichever theme is active.
 */
export function warmFonts() {
	if (typeof document === 'undefined' || !document.fonts) return;
	const family = getComputedStyle(document.body).fontFamily;
	// Body, blockquote (italic 500 in Tailwind Typography), strong and the three heading weights.
	for (const face of ['400', 'italic 400', 'italic 500', '600', '700', '800']) {
		document.fonts.load(`${face} 1rem ${family}`).catch(() => {});
	}
}
