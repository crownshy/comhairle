<script lang="ts">
	import { prefersReducedMotion, Tween } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { cn } from '$lib/utils';
	import type { StepItem } from './stepItems';

	type Props = {
		steps: StepItem[];
		currentIndex: number;
		/** Fill of the current step's track, 0 to 1. */
		fill: number;
	};

	let { steps, currentIndex, fill }: Props = $props();

	// Browsers can't transition the fill of a native <progress>, so the value is tweened instead.
	const tweenedFill = Tween.of(() => Math.min(1, Math.max(0, fill)), {
		duration: () => (prefersReducedMotion.current ? 0 : 300),
		easing: cubicOut
	});

	const CURRENT_TRACK_CLASS = [
		'h-2 min-w-0 flex-1 appearance-none overflow-hidden rounded-full border-0 bg-accent',
		'[&::-webkit-progress-bar]:bg-accent',
		'[&::-webkit-progress-value]:rounded-full [&::-webkit-progress-value]:bg-primary',
		'[&::-moz-progress-bar]:rounded-full [&::-moz-progress-bar]:bg-primary'
	].join(' ');

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
			<progress class={CURRENT_TRACK_CLASS} value={tweenedFill.current} max="1"></progress>
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
