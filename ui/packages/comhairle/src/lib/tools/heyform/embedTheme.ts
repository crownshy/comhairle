// Comhairle's colours in the shape the HeyForm fork accepts. The fork takes these five fields per
// request and derives its whole palette from them (NOTES.md, "Theme").
const EMBED_THEME_TOKENS = {
	backgroundColor: '--card',
	questionTextColor: '--foreground',
	answerTextColor: '--foreground',
	buttonBackground: '--primary',
	buttonTextColor: '--primary-foreground'
} as const;

export type EmbedTheme = Partial<Record<keyof typeof EMBED_THEME_TOKENS, string>>;

// Solid hex only. The fork derives shades by splitting channels, so a token carrying alpha would
// come out wrong; such tokens are dropped rather than sent.
const SOLID_HEX = /^#(?:[0-9a-f]{3}|[0-9a-f]{6})$/i;

/**
 * Reads the embed palette from the live CSS custom properties, so the CSS stays the one source.
 *
 * @param resolve looks up one custom property, in practice off `getComputedStyle(<html>)`.
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
