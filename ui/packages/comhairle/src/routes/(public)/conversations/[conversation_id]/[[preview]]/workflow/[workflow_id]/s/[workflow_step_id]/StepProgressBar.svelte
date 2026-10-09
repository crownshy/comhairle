<script lang="ts">
	import { prefersReducedMotion, Tween } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { cn } from '$lib/utils';
	import { m } from '$lib/paraglide/messages';
	import type { StepItem } from './stepItems';

	type Props = {
		steps: StepItem[];
		currentIndex: number;
		/** Fill of the current step's track, 0 to 1. */
		fill: number;
		/** The tool's position inside the step, such as "Page 3 of 8". */
		position?: string;
	};

	let { steps, currentIndex, fill, position }: Props = $props();

	// Browsers can't transition the fill of a native <progress>, so the value is tweened instead.
	const tweenedFill = Tween.of(() => Math.min(1, Math.max(0, fill)), {
		duration: () => (prefersReducedMotion.current ? 0 : 300),
		easing: cubicOut
	});

	function isDone(step: StepItem, index: number) {
		return (
			index < currentIndex ||
			step.status === 'completed' ||
			step.status === 'completed-locked'
		);
	}
</script>

<!-- Only the current step's track is read out; the other stubs are empty and stay silent. -->
<div class="flex items-center gap-1.5">
	{#each steps as step, index (step.id)}
		{#if index === currentIndex}
			<progress
				class="bg-accent [&::-webkit-progress-bar]:bg-accent [&::-webkit-progress-value]:bg-primary [&::-moz-progress-bar]:bg-primary h-2 min-w-0 flex-1 appearance-none overflow-hidden rounded-full border-0 [&::-moz-progress-bar]:rounded-full [&::-webkit-progress-value]:rounded-full"
				value={tweenedFill.current}
				max="1"
				aria-label={m.step_x_of_y({ current: currentIndex + 1, total: steps.length })}
				aria-valuetext={position}
			></progress>
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
<!-- Announces page turns inside a tool, since the pager swaps content without moving focus. -->
<p class="sr-only" aria-live="polite">{position ?? ''}</p>
