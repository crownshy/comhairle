/**
 * Comhairle's colours in the query-param shape our HeyForm fork reads (`applyEmbedTheme`, in the
 * fork's `pages/form/views/FormComponents/theme.ts`). The form renders in a cross-origin iframe, so
 * its URL is the only channel we have for handing it a theme.
 *
 * Read off the resolved cascade rather than the theme token modules: what the form should match is
 * whatever this viewer actually ended up with, which is the theme name stamped on `<html>` crossed
 * with the light/dark class. Both are settled before any component runs (see `app.html`).
 */

/** HeyForm theme field <- comhairle token. */
const TOKEN_BY_HEYFORM_FIELD = {
	backgroundColor: '--card',
	questionTextColor: '--card-foreground',
	/* HeyForm's "answer" colour is its interactive one: option text, typed input, field borders. */
	answerTextColor: '--primary',
	buttonBackground: '--primary',
	buttonTextColor: '--primary-foreground'
} as const;

/** Mirrors the fork's own hex check, so we never send a value it would silently drop. */
const HEX_COLOR = /^#?(?:[0-9a-f]{3}|[0-9a-f]{4}|[0-9a-f]{6}|[0-9a-f]{8})$/i;

/**
 * Any token that is not plain hex is skipped, leaving that slot at HeyForm's own default: the fork
 * parses these with `hexToRgb`, which has no idea what `oklch()` or a `var()` reference is. The
 * leading `#` is stripped because the fork accepts bare hex and it keeps the URL readable.
 */
export function heyFormThemeParams(): Record<string, string> {
	if (typeof window === 'undefined') return {};

	const styles = getComputedStyle(document.documentElement);
	const params: Record<string, string> = {};

	for (const [field, token] of Object.entries(TOKEN_BY_HEYFORM_FIELD)) {
		const value = styles.getPropertyValue(token).trim();

		if (HEX_COLOR.test(value)) {
			params[field] = value.replace(/^#/, '');
		}
	}

	// The fork allowlists this against the webfonts it will load, so an unknown family is dropped.
	const fontFamily = styles
		.getPropertyValue('--font-sans')
		.split(',')[0]
		.trim()
		.replace(/["']/g, '');

	if (fontFamily) {
		params.fontFamily = fontFamily;
	}

	return params;
}
