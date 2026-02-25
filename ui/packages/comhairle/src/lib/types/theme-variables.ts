/**
 * Complete theme variable structure matching CSS custom properties
 * This ensures all themes have consistent variable definitions
 */
export interface ThemeVariables {
	colors: {
		background: string;
		foreground: string;
		card: string;
		'card-foreground': string;
		popover: string;
		'popover-foreground': string;
		primary: string;
		'primary-foreground': string;
		secondary: string;
		'secondary-foreground': string;
		muted: string;
		'muted-foreground': string;
		accent: string;
		'accent-foreground': string;
		destructive: string;
		'destructive-foreground': string;
		border: string;
		input: string;
		ring: string;
		'chart-1': string;
		'chart-2': string;
		'chart-3': string;
		'chart-4': string;
		'chart-5': string;
		sidebar: string;
		'sidebar-foreground': string;
		'sidebar-primary': string;
		'sidebar-primary-foreground': string;
		'sidebar-accent': string;
		'sidebar-accent-foreground': string;
		'sidebar-border': string;
		'sidebar-ring': string;
	};
	fonts: {
		sans: string;
		serif: string;
		mono: string;
	};
	radius: string;
	shadows: {
		'2xs': string;
		xs: string;
		sm: string;
		default: string;
		md: string;
		lg: string;
		xl: string;
		'2xl': string;
	};
}

/**
 * Complete theme configuration with light and dark modes
 */
export interface ThemeConfig {
	light: ThemeVariables;
	dark: ThemeVariables;
}

/**
 * Type-safe color variable names
 */
export type ColorVariable =
	| 'background'
	| 'foreground'
	| 'card'
	| 'card-foreground'
	| 'popover'
	| 'popover-foreground'
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
	| 'border'
	| 'input'
	| 'ring'
	| 'chart-1'
	| 'chart-2'
	| 'chart-3'
	| 'chart-4'
	| 'chart-5'
	| 'sidebar'
	| 'sidebar-foreground'
	| 'sidebar-primary'
	| 'sidebar-primary-foreground'
	| 'sidebar-accent'
	| 'sidebar-accent-foreground'
	| 'sidebar-border'
	| 'sidebar-ring';

/**
 * Type-safe shadow variable names
 */
export type ShadowVariable = '2xs' | 'xs' | 'sm' | 'default' | 'md' | 'lg' | 'xl' | '2xl';

/**
 * Type-safe font variable names
 */
export type FontVariable = 'sans' | 'serif' | 'mono';

/**
 * Helper to validate theme has all required variables
 */
export function validateTheme(theme: Partial<ThemeVariables>): theme is ThemeVariables {
	const requiredColors: ColorVariable[] = [
		'background',
		'foreground',
		'card',
		'card-foreground',
		'popover',
		'popover-foreground',
		'primary',
		'primary-foreground',
		'secondary',
		'secondary-foreground',
		'muted',
		'muted-foreground',
		'accent',
		'accent-foreground',
		'destructive',
		'destructive-foreground',
		'border',
		'input',
		'ring',
		'chart-1',
		'chart-2',
		'chart-3',
		'chart-4',
		'chart-5',
		'sidebar',
		'sidebar-foreground',
		'sidebar-primary',
		'sidebar-primary-foreground',
		'sidebar-accent',
		'sidebar-accent-foreground',
		'sidebar-border',
		'sidebar-ring'
	];

	const requiredFonts: FontVariable[] = ['sans', 'serif', 'mono'];

	const requiredShadows: ShadowVariable[] = ['2xs', 'xs', 'sm', 'default', 'md', 'lg', 'xl', '2xl'];

	if (!theme.colors || !theme.fonts || !theme.shadows || !theme.radius) {
		return false;
	}

	const hasAllColors = requiredColors.every((color) => color in theme.colors!);
	const hasAllFonts = requiredFonts.every((font) => font in theme.fonts!);
	const hasAllShadows = requiredShadows.every((shadow) => shadow in theme.shadows!);

	return hasAllColors && hasAllFonts && hasAllShadows;
}

/**
 * Generate CSS custom property string from theme variables
 */
export function generateThemeCSS(variables: ThemeVariables): string {
	const lines: string[] = [];

	// Colors
	Object.entries(variables.colors).forEach(([key, value]) => {
		lines.push(`  --${key}: ${value};`);
	});

	lines.push('');

	// Fonts
	lines.push(`  --font-sans: ${variables.fonts.sans};`);
	lines.push(`  --font-serif: ${variables.fonts.serif};`);
	lines.push(`  --font-mono: ${variables.fonts.mono};`);

	lines.push('');
	lines.push(`  --radius: ${variables.radius};`);
	lines.push('');

	// Shadows
	Object.entries(variables.shadows).forEach(([key, value]) => {
		const varName = key === 'default' ? 'shadow' : `shadow-${key}`;
		lines.push(`  --${varName}: ${value};`);
	});

	return lines.join('\n');
}
