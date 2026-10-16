<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ChevronLeft, ChevronRight, Frown, Smile } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { addMinutes, isFinished, segmentClass } from './journeyStyles';
	import type { ParticipantJourney, StepProgress, JourneyStepVisit } from './sampleMonitorData';

	type Props = {
		journey: ParticipantJourney;
		steps: StepProgress[];
		/** Minutes that fill the full width, shared with the bars in the list. */
		scaleMinutes: number;
		/** 1-based place of this person in the list, and the list length. */
		position: number;
		count: number;
		/** Undefined at either end of the list, which disables the button. */
		onPrevious?: () => void;
		onNext?: () => void;
	};

	let { journey, steps, scaleMinutes, position, count, onPrevious, onNext }: Props = $props();

	let total = $derived(journey.visits.reduce((sum, visit) => sum + visit.minutes, 0));
	let doneCount = $derived(journey.visits.filter((visit) => visit.status === 'done').length);
	let finished = $derived(isFinished(journey));
	let leftIndex = $derived(journey.visits.findIndex((visit) => visit.status === 'left'));
	let live = $derived(journey.visits.some((visit) => visit.status === 'in_progress'));

	let timeWindow = $derived.by(() => {
		const visited = journey.visits.filter((visit) => visit.startedAt);
		if (visited.length === 0) return '';
		const first = visited[0].startedAt as string;
		const last = visited[visited.length - 1];
		const end = live ? 'now' : addMinutes(last.startedAt as string, last.minutes);
		return `${first} – ${end}`;
	});

	function timeRange(visit: JourneyStepVisit): string {
		if (!visit.startedAt) return '';
		if (visit.status === 'in_progress') return `${visit.startedAt}–now`;
		return `${visit.startedAt}–${addMinutes(visit.startedAt, visit.minutes)}`;
	}

	let reached = $derived(
		journey.visits
			.map((visit, index) => ({ visit, index, step: steps[index] }))
			.filter((entry) => entry.visit.minutes > 0)
	);
</script>

<!-- One person's day: their bar from the list, enlarged, with start time and duration per step. -->
<section aria-label="{journey.code}'s journey" class="flex flex-col gap-4">
	<header class="flex flex-wrap items-start justify-between gap-3">
		<div class="flex flex-col gap-1">
			<div class="flex flex-wrap items-center gap-3">
				<h3 class="text-lg font-semibold">{journey.code}'s journey</h3>
				{#if finished}
					<span
						class="inline-flex items-center gap-1.5 rounded-full bg-green-50 px-3 py-1 text-sm font-semibold text-green-800"
					>
						<Smile class="size-4" aria-hidden="true" />Completed
					</span>
				{:else if leftIndex >= 0}
					<span
						class="inline-flex items-center gap-1.5 rounded-full bg-red-50 px-3 py-1 text-sm font-semibold text-red-800"
					>
						<Frown class="size-4" aria-hidden="true" />Left at {steps[leftIndex]
							.shortName}
					</span>
				{:else if live}
					<span
						class="bg-accent text-primary inline-flex items-center gap-2 rounded-full px-3 py-1 text-sm font-semibold"
					>
						<span class="relative flex size-2.5" aria-hidden="true">
							<span
								class="bg-primary absolute inline-flex size-full animate-ping rounded-full opacity-60 motion-reduce:animate-none"
							></span>
							<span class="bg-primary relative inline-flex size-2.5 rounded-full"
							></span>
						</span>
						Taking part now
					</span>
				{/if}
			</div>
			<p class="text-muted-foreground text-sm">
				{timeWindow} ({total} min in total) {doneCount}
				of {steps.length} steps done
			</p>
		</div>

		<nav aria-label="Browse journeys" class="flex items-center gap-2">
			<Button
				variant="outline"
				size="icon"
				class="rounded-full"
				aria-label="Previous person (more recent)"
				disabled={!onPrevious}
				onclick={onPrevious}
			>
				<ChevronLeft class="size-4" aria-hidden="true" />
			</Button>
			<span class="text-muted-foreground min-w-14 text-center text-sm tabular-nums">
				{position} of {count}
			</span>
			<Button
				variant="outline"
				size="icon"
				class="rounded-full"
				aria-label="Next person (earlier)"
				disabled={!onNext}
				onclick={onNext}
			>
				<ChevronRight class="size-4" aria-hidden="true" />
			</Button>
		</nav>
	</header>

	<div class="relative">
		<div class="bg-card absolute inset-x-0 top-0 h-8 rounded-lg" aria-hidden="true"></div>
		<ol
			class="relative flex min-w-fit gap-0.5"
			style:width="{Math.min(100, (total / scaleMinutes) * 100)}%"
		>
			{#each reached as { visit, index, step } (index)}
				<li
					class="flex min-w-[5.5rem] flex-col gap-2"
					style:flex-grow={visit.minutes}
					style:flex-basis="0"
				>
					<span
						class={cn(
							'h-8 w-full',
							segmentClass(journey, visit, index),
							index === reached[0].index && 'rounded-l-lg',
							index === reached[reached.length - 1].index && 'rounded-r-lg',
							visit.status === 'in_progress' &&
								'animate-pulse motion-reduce:animate-none'
						)}
						aria-hidden="true"
					></span>
					<div class="flex flex-col px-0.5">
						<span class="truncate text-sm font-semibold" title={step.name}
							>{step.shortName}</span
						>
						<span class="text-sm leading-tight font-bold tabular-nums"
							>{visit.minutes} min</span
						>
						<span class="text-muted-foreground text-sm whitespace-nowrap tabular-nums"
							>{timeRange(visit)}</span
						>
					</div>
				</li>
			{/each}
		</ol>
	</div>
</section>
