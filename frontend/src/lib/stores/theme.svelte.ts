import type { Theme, ThemeName, ThemeMode } from '$lib/types/theme';

const THEME_STORAGE_KEY = 'comhairle-theme';
const MODE_STORAGE_KEY = 'comhairle-theme-mode';

function getStoredTheme(): Theme {
	if (typeof window === 'undefined') {
		return { name: 'comhairle', mode: 'light' };
	}

	const storedName = localStorage.getItem(THEME_STORAGE_KEY) as ThemeName | null;
	const storedMode = localStorage.getItem(MODE_STORAGE_KEY) as ThemeMode | null;

	const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

	return {
		name: storedName || 'comhairle',
		mode: storedMode || (prefersDark ? 'dark' : 'light')
	};
}

class ThemeStore {
	private _theme = $state<Theme>(getStoredTheme());

	get theme(): Theme {
		return this._theme;
	}

	get name(): ThemeName {
		return this._theme.name;
	}

	get mode(): ThemeMode {
		return this._theme.mode;
	}

	get isDark(): boolean {
		return this._theme.mode === 'dark';
	}

	setTheme(name: ThemeName) {
		this._theme.name = name;
		if (typeof window !== 'undefined') {
			localStorage.setItem(THEME_STORAGE_KEY, name);
		}
	}

	setMode(mode: ThemeMode) {
		this._theme.mode = mode;
		if (typeof window !== 'undefined') {
			localStorage.setItem(MODE_STORAGE_KEY, mode);
		}
	}

	toggleMode() {
		this.setMode(this.isDark ? 'light' : 'dark');
	}

	setThemeAndMode(name: ThemeName, mode: ThemeMode) {
		this._theme = { name, mode };
		if (typeof window !== 'undefined') {
			localStorage.setItem(THEME_STORAGE_KEY, name);
			localStorage.setItem(MODE_STORAGE_KEY, mode);
		}
	}
}

export const themeStore = new ThemeStore();
