/**
 * Theme TypeScript Interface
 * 
 * This defines the shape of theme objects that can be:
 * 1. Stored in the database
 * 2. Injected at runtime as CSS custom properties
 * 3. Used with the existing design-tokens.css system
 */

/**
 * Core semantic color tokens - these map to shadcn/ui expected variables
 */
export interface ThemeColors {
  background: string;
  foreground: string;
  primary: string;
  'primary-foreground': string;
  secondary: string;
  'secondary-foreground': string;
  muted: string;
  'muted-foreground': string;
  accent?: string;
  'accent-foreground'?: string;
  destructive?: string;
  'destructive-foreground'?: string; 
  card?: string;
  'card-foreground'?: string;
  popover?: string;
  'popover-foreground'?: string;
  border?: string;
  input?: string;
  ring?: string;
}

export interface ThemeSidebar {
  sidebar: string;
  'sidebar-foreground': string;
  'sidebar-primary'?: string;
  'sidebar-primary-foreground'?: string;
  'sidebar-accent'?: string;
  'sidebar-accent-foreground'?: string;
  'sidebar-border'?: string;
  'sidebar-ring'?: string;
  'sidebar-radius'?: string;
}

export interface ThemeNavigation {
  'nav-background'?: string;
  'nav-text'?: string;
}

export interface ThemeTypography {
  'font-serif'?: string;
  'font-sans'?: string;
  'font-mono'?: string;
}

export interface ThemeLayout {
  radius?: string;
  'admin-background'?: string;
}

export interface Theme extends 
  ThemeColors, 
  Partial<ThemeSidebar>, 
  Partial<ThemeNavigation>, 
  Partial<ThemeTypography>, 
  Partial<ThemeLayout> {
  // Allow additional custom properties
  [key: string]: string | undefined;
}

export interface ThemeMetadata {
  id: string;
  name: string;
  description?: string;
  primaryColor?: string;
}

export type BuiltInTheme = 
  | 'figma'
  | 'scot-gov'
  | 'comhairle';

/**
 * Convert a Theme object to CSS custom properties string
 * 
 * @example
 * const css = themeToCss(theme);
 * // Returns: "--background: #fff; --foreground: #000; ..."
 */
export function themeToCss(theme: Theme): string {
  return Object.entries(theme)
    .filter(([_, value]) => value !== undefined)
    .map(([key, value]) => `--${key}: ${value}`)
    .join('; ');
}

/**
 * Convert a Theme object to a CSS style object for Svelte
 * 
 * @example
 * <div style={themeToStyleObject(theme)}>
 */
export function themeToStyleObject(theme: Theme): Record<string, string> {
  const styles: Record<string, string> = {};
  for (const [key, value] of Object.entries(theme)) {
    if (value !== undefined) {
      styles[`--${key}`] = value;
    }
  }
  return styles;
}

/**
 * Merge a partial theme with defaults
 */
export function mergeTheme(partial: Partial<Theme>, defaults: Theme): Theme {
  return { ...defaults, ...partial };
}

/**
 * Default theme values (Crown Shy Light)
 * Use this as a fallback when DB theme is incomplete
 */
export const defaultTheme: Theme = {
  background: 'hsl(0 0% 100%)',
  foreground: 'hsl(222.2 84% 4.9%)',
  primary: 'hsl(221.2 83.2% 53.3%)',
  'primary-foreground': 'hsl(210 40% 98%)',
  secondary: 'hsl(210 40% 96.1%)',
  'secondary-foreground': 'hsl(222.2 47.4% 11.2%)',
  muted: 'hsl(210 40% 96.1%)',
  'muted-foreground': 'hsl(215.4 16.3% 46.9%)',
  accent: 'hsl(210 40% 96.1%)',
  'accent-foreground': 'hsl(222.2 47.4% 11.2%)',
  destructive: 'hsl(0 84.2% 60.2%)',
  'destructive-foreground': 'hsl(210 40% 98%)',
  border: 'hsl(214.3 31.8% 91.4%)',
  input: 'hsl(214.3 31.8% 91.4%)',
  ring: 'hsl(221.2 83.2% 53.3%)',
  card: 'hsl(0 0% 100%)',
  'card-foreground': 'hsl(222.2 84% 4.9%)',
  popover: 'hsl(0 0% 100%)',
  'popover-foreground': 'hsl(222.2 84% 4.9%)',
  sidebar: 'hsl(0 0% 100%)',
  'sidebar-foreground': 'hsl(222.2 47.4% 11.2%)',
  radius: '0.5rem',
};

/**
 * Validate that a theme object has all required properties
 */
export function isValidTheme(obj: unknown): obj is Theme {
  if (typeof obj !== 'object' || obj === null) return false;
  
  const required: (keyof ThemeColors)[] = [
    'background',
    'foreground', 
    'primary',
    'primary-foreground',
    'secondary',
    'secondary-foreground',
    'muted',
    'muted-foreground',
  ];
  
  return required.every(key => key in obj && typeof (obj as Record<string, unknown>)[key] === 'string');
}

// =============================================================================
// DESIGN TOKEN TYPE HELPERS
// =============================================================================

/**
 * Color scale values (50-950)
 */
export type ColorScale = 50 | 100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 | 950;

/**
 * Border radius tokens
 */
export type RadiusToken = 'none' | 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl' | 'full';

/**
 * Spacing tokens
 */
export type SpacingToken = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 16;

/**
 * Semantic color token names (for type-safe access)
 */
export type SemanticColorToken =
  | 'background'
  | 'foreground'
  | 'primary'
  | 'primary-foreground'
  | 'secondary'
  | 'secondary-foreground'
  | 'muted'
  | 'muted-foreground'
  | 'accent'
  | 'accent-foreground'
  | 'destructive'
  | 'destructive-foreground'
  | 'card'
  | 'card-foreground'
  | 'popover'
  | 'popover-foreground'
  | 'border'
  | 'input'
  | 'ring'
  | 'sidebar'
  | 'sidebar-foreground'
  | 'sidebar-primary'
  | 'sidebar-primary-foreground'
  | 'sidebar-accent'
  | 'sidebar-accent-foreground'
  | 'sidebar-border'
  | 'sidebar-ring';

/**
 * Type-safe design token accessor
 * Provides autocomplete for all design tokens
 * 
 * @example
 * tokens.color.semantic('primary')     // => 'var(--primary)'
 * tokens.color.csBlue(500)             // => 'var(--cs-blue-500)'
 * tokens.spacing(4)                    // => 'var(--spacing-4)'
 * tokens.radius('md')                  // => 'var(--radius-md)'
 */
export const tokens = {
  color: {
    semantic: (token: SemanticColorToken) => `var(--${token})`,
    csBlue: (scale: ColorScale) => `var(--cs-blue-${scale})`,
    csGrey: (scale: ColorScale) => `var(--cs-grey-${scale})`,
    red: (scale: ColorScale) => `var(--red-${scale})`,
    green: (scale: ColorScale) => `var(--green-${scale})`,
  },
  spacing: (value: SpacingToken) => `var(--spacing-${value})`,
  radius: (value: RadiusToken) => `var(--radius-${value})`,
} as const;
