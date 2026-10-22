<!--
	@component A small colored tag pill for a theme (or other short label), colored
	deterministically by `themeTagColor` so the same label reads as the same color
	everywhere it appears (statement cards, the theme explorer, etc). Picks its
	light/dark hex in CSS off the `.dark` class the app already toggles on <html>,
	rather than reading the theme store directly, so this has no runtime dependency
	on it.
-->
<script lang="ts">
	import { themeTagColor } from '$lib/utils/themeTagColor';

	type Props = { label: string };

	let { label }: Props = $props();

	const light = $derived(themeTagColor(label, false));
	const dark = $derived(themeTagColor(label, true));
</script>

<span
	class="theme-tag inline-flex items-center gap-0.5 rounded-lg px-1.5 py-0.5 text-xs font-medium"
	style="--tag-light: {light}; --tag-dark: {dark};"
>
	{label}
</span>

<style>
	.theme-tag {
		background-color: color-mix(in srgb, var(--tag-light) 18%, transparent);
		color: var(--tag-light);
	}

	:global(.dark) .theme-tag {
		background-color: color-mix(in srgb, var(--tag-dark) 20%, transparent);
		color: var(--tag-dark);
	}
</style>
