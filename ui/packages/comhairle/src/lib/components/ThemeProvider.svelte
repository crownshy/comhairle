<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import type { Snippet } from 'svelte';

	interface ThemeProviderProps {
		children: Snippet;
	}

	let { children }: ThemeProviderProps = $props();

	$effect(() => {
		if (typeof document === 'undefined') return;

		const html = document.documentElement;
		
		const themeName = themeStore.name;
		const isDark = themeStore.isDark;

		if (themeName === 'comhairle') {
			html.removeAttribute('data-theme');
		} else {
			html.setAttribute('data-theme', themeName);
		}

		if (isDark) {
			html.classList.add('dark');
		} else {
			html.classList.remove('dark');
		}
	});
</script>

{@render children()}
