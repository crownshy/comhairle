<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';
	import { STEP_SCROLL_ATTRIBUTE } from '$lib/utils/stepScroll';

	type Props = {
		header: Snippet;
		content: Snippet;
		/** Floated over the bottom of the scroll. */
		bar?: Snippet;
		class?: string;
	};

	let { header, content, bar, class: className }: Props = $props();

	// Measured so the scroll can reserve room for the bar, plus a gap above it.
	let barHeight = $state(80);
	const BAR_GAP_PX = 24;
	let scrollPadding = $derived(bar ? barHeight + BAR_GAP_PX : 0);
</script>

<!-- minmax(0,1fr) stops a wide header row pushing the grid past the viewport. -->
<div
	class={cn(
		'relative grid grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr] overflow-hidden',
		className
	)}
>
	{@render header()}

	<!-- The reserve sits on an inner wrapper because WebKit leaves a scroller's own bottom
		padding out of its scroll height. -->
	<div {...{ [STEP_SCROLL_ATTRIBUTE]: '' }} class="flex min-h-0 w-full flex-col overflow-y-auto">
		<div
			class="flex min-h-full w-full shrink-0 flex-col"
			style:padding-bottom="{scrollPadding}px"
		>
			{@render content()}
		</div>
	</div>

	{#if bar}
		<div
			class="bg-background/70 border-border/40 absolute inset-x-0 bottom-0 z-10 border-t backdrop-blur-lg"
			bind:clientHeight={barHeight}
		>
			{@render bar()}
		</div>
	{/if}
</div>
