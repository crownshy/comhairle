<script lang="ts">
	import { cn } from '$lib/utils';

	let {
		index,
		count,
		orientation = 'horizontal'
	}: {
		index: number;
		count: number;
		/** Vertical when the dots sit beside the content rather than under it. */
		orientation?: 'horizontal' | 'vertical';
	} = $props();

	let dots = $derived(Array.from({ length: count }, (_, i) => i));
</script>

<!-- Position in a slide deck. A single slide has no position to report, so it draws nothing.
	Decorative: the count also reaches assistive tech through the progress bar's label. -->
{#if count > 1}
	<div
		class={cn('flex items-center gap-2', orientation === 'vertical' && 'flex-col')}
		aria-hidden="true"
	>
		{#each dots as dot (dot)}
			<!-- The current dot stretches into a pill and the width tweens, so the marker slides
				along the row instead of blinking from one dot to the next. -->
			<span
				class={cn(
					'h-2 rounded-full transition-[width,background-color] duration-300 motion-reduce:transition-none',
					dot === index ? 'bg-primary w-5' : 'bg-accent w-2'
				)}
			></span>
		{/each}
	</div>
{/if}
