<script lang="ts">
	import type { ComponentProps, Snippet } from 'svelte';
	import StepChrome from './StepChrome.svelte';
	import { cn } from '$lib/utils';

	let {
		chrome,
		content,
		bar,
		class: className
	}: {
		/**
		 * The chrome's props, passed as one object rather than eleven. The shell renders
		 * StepChrome itself: a caller that could swap or omit it could drift from what
		 * participants see, which is the thing this component exists to prevent.
		 */
		chrome: ComponentProps<typeof StepChrome>;
		content: Snippet;
		/** The bottom bar, floated over the end of the scroll. Omitted renders nothing. */
		bar?: Snippet;
		class?: string;
	} = $props();

	// The bar floats over the scroll, so the scroll reserves the bar's height at its end.
	// Measured rather than fixed because the bars differ: the pager is 5rem, the brief bar
	// is taller. A bar that renders nothing (typing on a phone) measures zero and hands the
	// space back.
	let barHeight = $state(80);

	// Plus a gap, so the last line of content stops short of the glass instead of ending
	// against its edge. Without it a list or a paragraph that runs to the bottom reads as
	// cut off rather than scrolled to the end.
	const BAR_GAP = 24;
	let scrollPadding = $derived(bar ? barHeight + BAR_GAP : 0);
</script>

<!-- A step is exactly one screen: chrome on top, bar on the bottom, and the content takes
     whatever is left and scrolls inside it. The two chrome rows are laid out, not stuck:
     nothing can push them off. The column is minmax(0,1fr), not the implicit auto: an auto
     column floors at the widest row's min-content, so a long header (the opinion count next
     to an untruncated step label) would widen the whole grid past the viewport and shift
     every centred row right.

     The shell has no height of its own. The participant route gives it the viewport; an
     admin participant view gives it a device-sized box. -->
<div
	class={cn(
		'relative grid grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr] overflow-hidden',
		className
	)}
>
	<StepChrome {...chrome} />

	<!-- Children never shrink: this is a flex column with a definite height, so a page taller
	     than the screen would otherwise be squeezed down to its min-height and spill its
	     overflow out the bottom, straight through the padding and under the bar. -->
	<main
		data-step-scroll
		class="flex min-h-0 w-full flex-col overflow-y-auto *:shrink-0"
		style:padding-bottom="{scrollPadding}px"
	>
		{@render content()}
	</main>

	<!-- The bar is glass over the end of the scroll: content shows through it blurred, so a
	     cut-off paragraph reads as continuing underneath rather than stopping at a hard line.
	     The scroll reserves the bar's height plus a gap, so the last line clears it. -->
	{#if bar}
		<div
			class="bg-background/70 border-border/40 absolute inset-x-0 bottom-0 z-10 border-t backdrop-blur-lg"
			bind:clientHeight={barHeight}
		>
			{@render bar()}
		</div>
	{/if}
</div>
