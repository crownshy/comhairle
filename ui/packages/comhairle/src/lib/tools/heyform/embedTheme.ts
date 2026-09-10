/**
 * Comhairle's colours, in the shape the HeyForm fork accepts.
 *
 * A HeyForm form stores one fixed set of colours, chosen when the form is built. The form renders in
 * a cross-origin iframe, so we cannot restyle it from out here, and it cannot see the page around
 * it: not the light / dark mode the viewer picked, nor the palette this deployment runs. Left alone
 * it is a white form on whatever background we happen to give it.
 *
 * Our fork takes these five colours per request, as query params on the form URL and in a SET_THEME
 * message, and rebuilds its whole `--heyform-*` variable set from them (descriptions, labels, the
 * hover and disabled shades, the group header background are all derived downstream). Five values is
 * the entire surface.
 */
const EMBED_THEME_TOKENS = {
	backgroundColor: '--card',
	questionTextColor: '--foreground',
	answerTextColor: '--foreground',
	buttonBackground: '--primary',
	buttonTextColor: '--primary-foreground'
} as const;

export type EmbedTheme = Partial<Record<keyof typeof EMBED_THEME_TOKENS, string>>;

/**
 * Solid hex only. The fork also accepts the 4 and 8 digit forms, but it derives its shades by
 * pulling the channels apart and reassembling them at a fixed opacity, so a token that already
 * carries alpha comes out somewhere we did not ask for. Every token mapped above is solid hex in
 * every theme and mode; a token that is not (`--border` and `--input` are `#ffffff19` in dark) is
 * dropped here rather than sent and misread.
 */
const SOLID_HEX = /^#(?:[0-9a-f]{3}|[0-9a-f]{6})$/i;

/**
 * Tokens are read at runtime rather than imported because the CSS is where they are defined.
 * Mirroring the values into TypeScript would leave a second copy of every palette for a theme change
 * to silently strand.
 *
 * @param resolve looks up one custom property, in practice off `getComputedStyle(<html>)`, which is
 * where both `dark` and `data-theme` live.
 */
export function readEmbedTheme(resolve: (token: string) => string): EmbedTheme {
	const theme: EmbedTheme = {};

	for (const [field, token] of Object.entries(EMBED_THEME_TOKENS)) {
		const value = resolve(token).trim();

		if (SOLID_HEX.test(value)) {
			theme[field as keyof EmbedTheme] = value;
		}
	}

	return theme;
}
