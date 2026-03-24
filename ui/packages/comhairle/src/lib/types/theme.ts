export type ThemeName = 'comhairle' | 'scot-gov' | 'waves';
export type ThemeMode = 'light' | 'dark';

export interface Theme {
	name: ThemeName;
	mode: ThemeMode;
}
