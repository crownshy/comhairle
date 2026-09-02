<script lang="ts">
	import type { ComponentProps, Snippet } from 'svelte';
	import StepChrome from './StepChrome.svelte';
	import { cn } from '$lib/utils';

	let {
		chrome,
		content,
		bar,
		masked = true,
		class: className
	}: {
		/**
		 * The chrome's props, passed as one object rather than eleven. The shell renders
		 * StepChrome itself: a caller that could swap or omit it could drift from what
		 * participants see, which is the thing this component exists to prevent.
		 */
		chrome: ComponentProps<typeof StepChrome>;
		content: Snippet;
		/** The bottom row. Omitted leaves the row collapsed, which is what `auto` gives us. */
		bar?: Snippet;
		/**
		 * Whether the scroll fades out at the bottom. Off while a phone keyboard is up, where
		 * the visible strip is short enough that the fade eats the content.
		 */
		masked?: boolean;
		class?: string;
	} = $props();
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
		'grid grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr_auto] overflow-hidden',
		className
	)}
>
	<StepChrome {...chrome} />

	<!-- The bottom of the scroll dissolves into the background instead of stopping at a hard
	     line, so a cut-off paragraph reads as content continuing under the bar. The padding
	     matches the fade, so the mask sits over empty space once the reader is at the end and
	     never dims the last line. -->
	<main
		data-step-scroll
		class={cn(
			'flex min-h-0 w-full flex-col overflow-y-auto',
			masked && 'mask-b-from-[calc(100%-2.5rem)] pb-10'
		)}
	>
		{@render content()}
	</main>

	{#if bar}
		{@render bar()}
	{/if}
</div>
