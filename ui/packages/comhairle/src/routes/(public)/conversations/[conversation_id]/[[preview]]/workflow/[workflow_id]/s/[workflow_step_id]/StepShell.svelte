<script lang="ts">
	import type { Snippet } from 'svelte';
	import { PAGER_BAR_SURFACE_CLASS } from '$lib/components/pager/pager';
	import { cn } from '$lib/utils';
	import { STEP_SCROLL_ATTRIBUTE } from '$lib/utils/stepScroll';
	import { MAIN_CONTENT_ID } from '$lib/utils/mainContent';
	import { m } from '$lib/paraglide/messages';

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

<!-- minmax(0,1fr) stops a wide header row pushing the grid past the viewport. overflow-clip,
	not hidden: a hidden box can still be scrolled by focus or scrollIntoView, which slides
	the header and pager off screen. Only the inner scroll should ever move. -->
<div
	class={cn(
		'relative grid grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr] overflow-clip',
		className
	)}
>
	{@render header()}

	<!-- The reserve sits on an inner wrapper because WebKit leaves a scroller's own bottom
		padding out of its scroll height. -->
	<main
		id={MAIN_CONTENT_ID}
		tabindex="-1"
		{...{ [STEP_SCROLL_ATTRIBUTE]: '' }}
		class="flex min-h-0 w-full flex-col overflow-y-auto outline-none"
	>
		<div
			class="flex min-h-full w-full shrink-0 flex-col"
			style:padding-bottom="{scrollPadding}px"
		>
			{@render content()}
		</div>
	</main>

	{#if bar}
		<!-- data-spotlight-keep keeps the pager usable while a tool's spotlight dims the page. -->
		<nav
			data-spotlight-keep
			aria-label={m.pager_label()}
			class={cn(PAGER_BAR_SURFACE_CLASS, 'absolute inset-x-0 bottom-0 z-10')}
			bind:clientHeight={barHeight}
		>
			{@render bar()}
		</nav>
	{/if}
</div>
