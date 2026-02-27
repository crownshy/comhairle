import type { Theme, ThemeName, ThemeMode } from '$lib/types/theme';

const MODE_STORAGE_KEY = 'comhairle-theme-mode';

function getInitialMode(): ThemeMode {
	if (typeof window === 'undefined') return 'light';
	const storedMode = localStorage.getItem(MODE_STORAGE_KEY) as ThemeMode | null;
	if (storedMode) return storedMode;
	return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

class ThemeStore {
	private _theme = $state<Theme>({ name: 'comhairle', mode: getInitialMode() });

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

	/** Called once from +layout.svelte with the server-provided theme name */
	initFromServer(name: ThemeName) {
		this._theme.name = name;
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
}

export const themeStore = new ThemeStore();
