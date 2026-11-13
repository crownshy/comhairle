export interface ThemeConfig {
	/** 32x32 PNG tab icon, used by browsers that don't support SVG favicons. */
	favicon: string;
	/** SVG tab icon, preferred by modern browsers. Can switch colour for dark mode. */
	faviconSvg?: string;
	/** 180x180 PNG on a solid background, used for iOS home screen and Safari favourites. */
	appleTouchIcon?: string;
	/** The 1200x630 card shown when a link is shared and the page has no image of its own. */
	socialImage: string;
}

export const DEFAULT_THEME = 'comhairle' as const;

const COMHAIRLE_ICONS = {
	favicon: 'favicon-32.png',
	faviconSvg: 'favicon.svg',
	appleTouchIcon: 'apple-touch-icon.png'
} as const;

export const THEMES = {
	comhairle: { ...COMHAIRLE_ICONS, socialImage: 'comhairle-social.png' },
	'scot-gov': { ...COMHAIRLE_ICONS, socialImage: 'scot-gov-social.png' },
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
