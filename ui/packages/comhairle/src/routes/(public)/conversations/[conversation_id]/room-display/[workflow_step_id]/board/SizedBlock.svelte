<!--
	@component Scales Tailwind's text and spacing variables inside one block by `scale`.
	It reads the `--room-base-*` values set on the display root (+page.svelte), so nested
	blocks do not compound. `display: contents` keeps the wrapper out of the layout.
-->
<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		/** The block's multiplier, from `blockScale`. */
		scale: number;
		children: Snippet;
	};

	let { scale, children }: Props = $props();
</script>

<div class="room-block contents" style="--room-scale: {scale}">
	{@render children()}
</div>

<style>
	.room-block {
		--spacing: calc(var(--room-base-spacing) * var(--room-scale));
		--text-xs: calc(var(--room-base-text-xs) * var(--room-scale));
		--text-sm: calc(var(--room-base-text-sm) * var(--room-scale));
		--text-base: calc(var(--room-base-text-base) * var(--room-scale));
		--text-lg: calc(var(--room-base-text-lg) * var(--room-scale));
		--text-xl: calc(var(--room-base-text-xl) * var(--room-scale));
		--text-2xl: calc(var(--room-base-text-2xl) * var(--room-scale));
		--text-3xl: calc(var(--room-base-text-3xl) * var(--room-scale));
		--text-4xl: calc(var(--room-base-text-4xl) * var(--room-scale));
		--text-5xl: calc(var(--room-base-text-5xl) * var(--room-scale));
		--text-6xl: calc(var(--room-base-text-6xl) * var(--room-scale));
	}
</style>
