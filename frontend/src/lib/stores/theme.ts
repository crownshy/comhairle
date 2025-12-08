/**
 * Theme Store
 *
 * Manages theme state with localStorage persistence.
 * 
 * Current: localStorage-based
 * Future: Can be refactored to load from DB via API
 *
 * @example
 * import { themeStore, setTheme, availableThemes } from '$lib/stores/theme';
 *
 * // Subscribe to theme changes
 * $themeStore // => 'scot-gov'
 *
 * // Change theme
 * setTheme('comhairle');
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { 
  type Theme, 
  type ThemeMetadata, 
  type BuiltInTheme,
  defaultTheme, 
  mergeTheme, 
  themeToCss 
} from '$lib/types/theme';

// Re-export BuiltInTheme for convenience
export type { BuiltInTheme } from '$lib/types/theme';

// =============================================================================
// BUILT-IN THEME DEFINITIONS
// =============================================================================

/**
 * Built-in theme configurations
 * These define the CSS variable overrides for each theme
 */
export const builtInThemes: Record<BuiltInTheme, Partial<Theme>> = {
  'figma': {
    background: '#ffffff',
    foreground: '#0a0a0a',
    primary: '#001c3b',
    'primary-foreground': '#ffffff',
    secondary: '#0066bd',
    'secondary-foreground': '#ffffff',
    accent: '#ebf1ff',
    'accent-foreground': '#0a0a0a',
    muted: '#f3f4f6',
    'muted-foreground': '#52534e',
    destructive: '#d32205',
    'destructive-foreground': '#ffffff',
    border: '#e5e7eb',
    input: '#e5e7eb',
    ring: '#0066bd',
    card: '#ffffff',
    'card-foreground': '#0a0a0a',
    popover: '#ffffff',
    'popover-foreground': '#0a0a0a',
    sidebar: '#030712',
    'sidebar-foreground': '#f9fafb',
    'sidebar-primary': '#ffffff',
    'sidebar-primary-foreground': '#111827',
    'sidebar-accent': '#1f2937',
    'sidebar-accent-foreground': '#f9fafb',
    radius: '0.5rem',
  },
  'scot-gov': {
    primary: '#0065bd',
    'primary-foreground': '#ffffff',
    secondary: '#333e48',
    'secondary-foreground': '#ffffff',
    muted: '#f8f8f8',
    'muted-foreground': '#333e48',
    accent: '#0065bd',
    'accent-foreground': '#ffffff',
    border: '#b3b3b3',
    ring: '#0065bd',
  },
  'comhairle': {
    background: 'hsla(0, 0%, 100%, 1)',
    foreground: 'hsl(30 10% 3.9%)',
    primary: 'hsl(88, 33%, 9%)',
    'primary-foreground': 'white',
    secondary: 'hsl(102, 34%, 30%)',
    'secondary-foreground': 'white',
    muted: 'hsla(100, 29%, 10%, 1)',
    'muted-foreground': '#272f1d',
    radius: '6.25rem',
    'nav-background': 'hsla(78, 60%, 89%, 0.08)',
    'nav-text': 'hsla(79, 100%, 96%, 1)',
    sidebar: 'hsl(88, 33%, 9%)',
    'sidebar-radius': '10px',
    'sidebar-foreground': 'hsla(79, 100%, 96%, 1)',
    'admin-background': 'hsla(60, 67%, 98%, 1)',
  },
};

/**
 * Theme metadata for UI display
 */
export const availableThemes: ThemeMetadata[] = [
  {
    id: 'figma',
    name: 'Figma',
    description: 'Dark sidebar with blue accents',
    primaryColor: '#001c3b',
  },
  {
    id: 'scot-gov',
    name: 'Scottish Government',
    description: 'Official Scottish Government branding',
    primaryColor: '#0065bd',
  },
  {
    id: 'comhairle',
    name: 'Comhairle',
    description: 'Comhairle organization theme',
    primaryColor: 'hsl(88, 33%, 9%)',
  },
];

// =============================================================================
// STORE IMPLEMENTATION
// =============================================================================

const THEME_STORAGE_KEY = 'crown-shy-theme';
const CUSTOM_THEME_STORAGE_KEY = 'crown-shy-custom-theme';
const DEFAULT_THEME_ID: BuiltInTheme = 'crown-shy-light';

/**
 * Get initial theme ID from localStorage
 */
function getInitialThemeId(): BuiltInTheme | 'custom' {
  if (!browser) return DEFAULT_THEME_ID;

  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY);
    if (stored && (isBuiltInTheme(stored) || stored === 'custom')) {
      return stored as BuiltInTheme | 'custom';
    }
  } catch (error) {
    console.warn('Failed to load theme from localStorage:', error);
  }

  return DEFAULT_THEME_ID;
}

/**
 * Get custom theme overrides from localStorage
 */
function getCustomThemeOverrides(): Partial<Theme> | null {
  if (!browser) return null;

  try {
    const stored = localStorage.getItem(CUSTOM_THEME_STORAGE_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch (error) {
    console.warn('Failed to load custom theme from localStorage:', error);
  }

  return null;
}

/**
 * Check if a string is a valid built-in theme
 */
function isBuiltInTheme(value: string): value is BuiltInTheme {
  return value in builtInThemes;
}

/**
 * The current theme ID store
 */
const themeIdStore = writable<BuiltInTheme | 'custom'>(getInitialThemeId());

/**
 * Custom theme overrides store (for DB-loaded themes later)
 */
const customThemeStore = writable<Partial<Theme> | null>(getCustomThemeOverrides());

/**
 * Derived store: the complete resolved theme object
 */
export const resolvedTheme = derived(
  [themeIdStore, customThemeStore],
  ([$themeId, $customTheme]) => {
    if ($themeId === 'custom' && $customTheme) {
      return mergeTheme($customTheme, defaultTheme);
    }
    
    const builtIn = builtInThemes[$themeId as BuiltInTheme] || {};
    return mergeTheme(builtIn, defaultTheme);
  }
);

/**
 * Derived store: CSS custom properties string
 */
export const themeCss = derived(resolvedTheme, ($theme) => themeToCss($theme));

/**
 * The main theme store (exports the theme ID)
 */
export const themeStore = {
  subscribe: themeIdStore.subscribe,
  
  /**
   * Set a built-in theme
   */
  set: (themeId: BuiltInTheme) => {
    if (!isBuiltInTheme(themeId)) {
      console.warn(`Invalid theme: ${themeId}. Using default.`);
      themeId = DEFAULT_THEME_ID;
    }

    if (browser) {
      try {
        localStorage.setItem(THEME_STORAGE_KEY, themeId);
      } catch (error) {
        console.warn('Failed to save theme to localStorage:', error);
      }
    }

    themeIdStore.set(themeId);
  },

  /**
   * Set a custom theme (for DB-loaded themes)
   */
  setCustom: (overrides: Partial<Theme>) => {
    if (browser) {
      try {
        localStorage.setItem(THEME_STORAGE_KEY, 'custom');
        localStorage.setItem(CUSTOM_THEME_STORAGE_KEY, JSON.stringify(overrides));
      } catch (error) {
        console.warn('Failed to save custom theme to localStorage:', error);
      }
    }

    customThemeStore.set(overrides);
    themeIdStore.set('custom');
  },

  /**
   * Reset to default theme
   */
  reset: () => {
    if (browser) {
      try {
        localStorage.removeItem(THEME_STORAGE_KEY);
        localStorage.removeItem(CUSTOM_THEME_STORAGE_KEY);
      } catch (error) {
        console.warn('Failed to remove theme from localStorage:', error);
      }
    }
    
    customThemeStore.set(null);
    themeIdStore.set(DEFAULT_THEME_ID);
  },
};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Set a built-in theme
 */
export function setTheme(themeId: BuiltInTheme) {
  themeStore.set(themeId);
}

/**
 * Set a custom theme (for future DB integration)
 * 
 * @example
 * // When you fetch theme from DB:
 * const dbTheme = await api.getOrganizationTheme();
 * setCustomTheme(dbTheme);
 */
export function setCustomTheme(overrides: Partial<Theme>) {
  themeStore.setCustom(overrides);
}

/**
 * Get theme metadata by ID
 */
export function getThemeMetadata(themeId: string): ThemeMetadata | undefined {
  return availableThemes.find((t) => t.id === themeId);
}

/**
 * Get the current resolved theme object
 */
export function getCurrentTheme(): Theme {
  return get(resolvedTheme);
}

/**
 * Get the current theme CSS string
 */
export function getCurrentThemeCss(): string {
  return get(themeCss);
}
