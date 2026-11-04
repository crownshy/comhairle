export interface ThemeConfig {
	favicon: string;
	/** The 1200x630 card shown when a link is shared and the page has no image of its own. */
	socialImage: string;
}

export const DEFAULT_THEME = 'comhairle' as const;

export const THEMES = {
	comhairle: { favicon: 'favicon.png', socialImage: 'comhairle-social.png' },
	'scot-gov': { favicon: 'favicon.png', socialImage: 'scot-gov-social.png' },
	waves: { favicon: 'waves-favicon.png', socialImage: 'waves-social.png' }
} as const satisfies Record<string, ThemeConfig>;

export type ThemeName = keyof typeof THEMES;

export function resolveThemeName(raw: string | undefined): ThemeName {
	if (raw && raw in THEMES) return raw as ThemeName;
	return DEFAULT_THEME;
}

export type ThemeMode = 'light' | 'dark';

export interface Theme {
	name: ThemeName;
	mode: ThemeMode;
}
