<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StepItem } from './stepItems';

	let {
		steps,
		currentIndex,
		fill
	}: {
		steps: StepItem[];
		currentIndex: number;
		/** Fill of the current step's segment, 0 to 1. */
		fill: number;
	} = $props();

	let fillPercent = $derived(Math.min(100, Math.max(0, fill * 100)));
</script>

<!-- Stubs for the other steps, a flexible track for the current one. Completed stubs read
	filled so the bar carries the status the old stepper carried. -->
<div class="flex items-center gap-1.5 px-3 md:px-6" aria-hidden="true">
	{#each steps as step, index (step.id)}
		{#if index === currentIndex}
			<div class="bg-accent relative h-2 min-w-0 flex-1 rounded-full">
				<div
					class="bg-primary absolute inset-y-0 left-0 rounded-full transition-[width] duration-300"
					style="width: {fillPercent}%"
				></div>
			</div>
		{:else}
			<div
				class={cn(
					'h-2 w-[18px] shrink-0 rounded-full md:w-6',
					index < currentIndex || step.status === 'completed' ? 'bg-primary' : 'bg-accent'
				)}
			></div>
		{/if}
	{/each}
</div>
