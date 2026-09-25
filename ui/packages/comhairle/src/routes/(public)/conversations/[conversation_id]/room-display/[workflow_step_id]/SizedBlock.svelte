<!--
	@component One block of the board, at the size the board says.

	Sets `--room-scale` and, from it, redefines the Tailwind text and spacing variables
	for everything inside, so every `text-*`, `gap-*`, `p-*` and `size-*` utility in the
	block grows together and none of the markup knows about it. `display: contents`
	keeps the wrapper out of the layout: the block's own element stays a direct flex or
	grid child of whatever it was in before.

	The base values are `--room-base-*`, captured once on the display root from the
	theme (+page.svelte). Reading those rather than the live variables means a block
	inside a block scales from the theme, not from its parent, and a variable that
	referred to itself would be a cycle anyway.
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
