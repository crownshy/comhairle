export interface ThemeConfig {
	favicon: string;
}

export const DEFAULT_THEME = 'comhairle' as const;

export const THEMES = {
	comhairle: { favicon: 'favicon.png' },
	'scot-gov': { favicon: 'favicon.png' },
	waves: { favicon: 'waves-favicon.png' }
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
