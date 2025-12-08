<script lang="ts">
	/**
	 * ThemeProvider Component
	 *
	 * Wraps your application to apply the selected theme.
	 * Injects CSS custom properties and manages theme class on document.
	 *
	 * @example
	 * <ThemeProvider>
	 *   <YourApp />
	 * </ThemeProvider>
	 *
	 * @example With default theme
	 * <ThemeProvider defaultTheme="scot-gov">
	 *   <YourApp />
	 * </ThemeProvider>
	 */

	import { onMount } from 'svelte';
	import {
		themeStore,
		themeCss,
		setTheme,
		type BuiltInTheme
	} from '$lib/stores/theme';
	import type { Snippet } from 'svelte';

	interface Props {
		/** Override the default theme on mount */
		defaultTheme?: BuiltInTheme;
		/** Children content */
		children: Snippet;
	}

	let { defaultTheme, children }: Props = $props();

	// Set default theme if provided
	if (defaultTheme) {
		setTheme(defaultTheme);
	}

	// Apply theme class to document root
	function applyThemeClass(themeId: string) {
		if (typeof document === 'undefined') return;

		const themeClasses = [
			'crown-shy-light',
			'crown-shy-dark',
			'figma',
			'scot-gov',
			'comhairle',
			'material-light',
			'material-dark',
			'dark',
			'custom'
		];

		themeClasses.forEach((cls) => {
			document.documentElement.classList.remove(cls);
		});

		document.documentElement.classList.add(themeId);

		// Add 'dark' class for dark themes (Tailwind compatibility)
		if (themeId.includes('dark')) {
			document.documentElement.classList.add('dark');
		}
	}

	onMount(() => {
		// Subscribe to theme changes and apply class
		const unsubscribe = themeStore.subscribe((themeId) => {
			applyThemeClass(themeId);
		});

		return unsubscribe;
	});
</script>

<div style={$themeCss} class="contents">
	{@render children()}
</div>
