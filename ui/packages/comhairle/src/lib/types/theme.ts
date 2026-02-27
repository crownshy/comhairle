export type ThemeName = 'comhairle' | 'scotgov';
export type ThemeMode = 'light' | 'dark';

export interface Theme {
	name: ThemeName;
	mode: ThemeMode;
}
