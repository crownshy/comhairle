export type ThemeName = 'comhairle' | 'bloom' | 'scotgov';
export type ThemeMode = 'light' | 'dark';

export interface Theme {
	name: ThemeName;
	mode: ThemeMode;
}
