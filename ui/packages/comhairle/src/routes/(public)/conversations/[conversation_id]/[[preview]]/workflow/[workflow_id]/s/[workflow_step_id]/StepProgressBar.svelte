<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StepItem } from './stepItems';

	type Props = {
		steps: StepItem[];
		currentIndex: number;
		/** Fill of the current step's track, 0 to 1. */
		fill: number;
	};

	let { steps, currentIndex, fill }: Props = $props();

	let fillPercent = $derived(Math.min(100, Math.max(0, fill * 100)));

	function isDone(step: StepItem, index: number) {
		return (
			index < currentIndex ||
			step.status === 'completed' ||
			step.status === 'completed-locked'
		);
	}
</script>

<!-- aria-hidden because the "Step N of M" line in the header carries the position. -->
<div class="flex items-center gap-1.5" aria-hidden="true">
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
					isDone(step, index) ? 'bg-primary' : 'bg-accent'
				)}
			></div>
		{/if}
	{/each}
</div>
