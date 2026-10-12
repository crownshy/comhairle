<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';
	import { Frown, Smile } from 'lucide-svelte';
	import JourneyDetail from './JourneyDetail.svelte';
	import { segmentClass } from './journeyStyles';
	import type { ParticipantJourney, StepProgress } from './sampleMonitorData';

	type Props = {
		steps: StepProgress[];
		journeys: ParticipantJourney[];
	};

	let { steps, journeys }: Props = $props();

	/** Latest journeys first; earlier ones are further down the scrolling list. */
	let ordered = $derived(
		[...journeys].sort((a, b) =>
			(b.visits[0]?.startedAt ?? '').localeCompare(a.visits[0]?.startedAt ?? '')
		)
	);
	let selectedCode = $state(
		[...journeys].sort((a, b) =>
			(b.visits[0]?.startedAt ?? '').localeCompare(a.visits[0]?.startedAt ?? '')
		)[0]?.code ?? ''
	);
	let selected = $derived(
		journeys.find((journey) => journey.code === selectedCode) ?? journeys[0]
	);
	let selectedIndex = $derived(
		Math.max(
			0,
			ordered.findIndex((journey) => journey.code === selectedCode)
		)
	);

	// Previous / next: select the person and scroll the list so their row sits at the top.
	// Clicking a row directly doesn't scroll, since that row is already in view.
	let listElement: HTMLUListElement | undefined = $state();
	let scrollToSelected = false;

	function stepTo(index: number) {
		selectedCode = ordered[index].code;
		scrollToSelected = true;
	}

	$effect(() => {
		const code = selectedCode;
		if (!scrollToSelected || !listElement) return;
		scrollToSelected = false;
		const row = listElement.querySelector<HTMLElement>(`[data-code="${code}"]`);
		if (!row) return;
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		listElement.scrollTo({ top: row.offsetTop, behavior: reduceMotion ? 'auto' : 'smooth' });
	});

	function totalMinutes(journey: ParticipantJourney): number {
		return journey.visits.reduce((sum, visit) => sum + visit.minutes, 0);
	}

	/**
	 * Every row shares one fixed time scale (0 to 20 minutes, stretched if someone goes
	 * over), so a longer bar means more time spent today.
	 */
	const SCALE_MINUTES = 20;
	let longest = $derived(Math.max(SCALE_MINUTES, ...journeys.map(totalMinutes)));

	function outcome(journey: ParticipantJourney): {
		label: string;
		tone: string;
		face: 'happy' | 'sad' | null;
	} {
		const visits = journey.visits;
		if (visits.some((visit) => visit.status === 'left')) {
			const leftAt = visits.findIndex((visit) => visit.status === 'left');
			return {
				label: `Left at ${steps[leftAt].shortName}`,
				tone: 'text-amber-600 font-semibold',
				face: 'sad'
			};
		}
		if (visits.some((visit) => visit.status === 'in_progress')) {
			return { label: 'Still going', tone: 'text-muted-foreground', face: null };
		}
		return { label: 'Completed', tone: 'text-foreground', face: 'happy' };
	}
</script>

<!--
	Today's journeys. Each row is one anonymous participant, each segment a step as wide as
	the time spent on it. Selecting a row shows that person's day in JourneyDetail above.
-->
<Card.Root id="today-journeys" tabindex={-1} class="scroll-mt-6 focus:outline-none">
	<Card.Header>
		<Card.Title>Live pulse</Card.Title>
		<Card.Description class="text-base">
			Who took part today and how far they got. Each bar is one person, split by step: longer
			segments mean more time spent.
		</Card.Description>
	</Card.Header>
	<Card.Content class="flex flex-col gap-6">
		<!-- Selected person's journey, at the top so it reads as the card's focus. -->
		{#if selected}
			<div class="bg-muted/50 rounded-2xl p-5">
				<JourneyDetail
					journey={selected}
					{steps}
					scaleMinutes={longest}
					position={selectedIndex + 1}
					count={ordered.length}
					onPrevious={selectedIndex > 0 ? () => stepTo(selectedIndex - 1) : undefined}
					onNext={selectedIndex < ordered.length - 1
						? () => stepTo(selectedIndex + 1)
						: undefined}
				/>
			</div>
		{/if}
		<div class="flex flex-wrap items-center justify-between gap-3">
			<ul class="flex flex-wrap gap-4 text-sm" aria-label="Legend">
				<li class="inline-flex items-center gap-2">
					<span class="bg-muted-foreground/45 size-3.5 rounded-sm"></span>Step done
				</li>
				<li class="inline-flex items-center gap-2">
					<span
						class="bg-primary size-3.5 animate-pulse rounded-sm motion-reduce:animate-none"
					></span>Still going
				</li>
				<li class="inline-flex items-center gap-2">
					<span class="size-3.5 rounded-sm bg-amber-500"></span>Left at this step
				</li>
				<li class="inline-flex items-center gap-2">
					<span class="size-3.5 rounded-sm bg-green-500"></span>Completed
				</li>
			</ul>
			<span class="text-primary text-sm font-semibold">{journeys.length} people today</span>
		</div>

		<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
		<ul
			bind:this={listElement}
			class="relative -mx-1 flex max-h-[17.5rem] flex-col gap-1 overflow-y-auto px-1"
			aria-label="Participants today, latest first"
			tabindex="0"
		>
			{#each ordered as journey (journey.code)}
				{@const result = outcome(journey)}
				{@const isSelected = journey.code === selectedCode}
				<li data-code={journey.code}>
					<button
						type="button"
						aria-pressed={isSelected}
						onclick={() => (selectedCode = journey.code)}
						class={cn(
							'grid w-full grid-cols-[4.5rem_minmax(0,1fr)_12rem] items-center gap-3 rounded-xl px-3 py-2 text-left transition-colors duration-300 motion-reduce:transition-none',
							isSelected ? 'bg-accent' : 'hover:bg-muted/60'
						)}
					>
						<span class="text-sm font-semibold tabular-nums">{journey.code}</span>
						<span class="flex min-w-0 items-center gap-2">
							<span
								class="flex h-4 items-stretch"
								style:width="{(totalMinutes(journey) / longest) * 100}%"
							>
								{#each journey.visits as visit, index (index)}
									{#if visit.minutes > 0}
										<span
											class={cn(
												'border-card min-w-1 border-r-2 first:rounded-l-md last:rounded-r-md last:border-r-0',
												segmentClass(journey, visit, index),
												// Live: the step this person is on right now.
												visit.status === 'in_progress' &&
													'animate-pulse motion-reduce:animate-none'
											)}
											style:flex-grow={visit.minutes}
											title="{steps[index].name}: {visit.minutes} min"
										></span>
									{/if}
								{/each}
							</span>
						</span>
						<!-- Fixed sub-columns so the minutes, the outcome and the marker line up across rows. -->
						<span
							class="grid grid-cols-[3.25rem_minmax(0,1fr)_1.25rem] items-center gap-2 text-sm"
						>
							<span class="text-right font-semibold tabular-nums"
								>{totalMinutes(journey)} min</span
							>
							<span class={cn('leading-tight', result.tone)}>{result.label}</span>
							{#if result.face === 'happy'}
								<Smile class="size-5 shrink-0 text-green-600" aria-hidden="true" />
							{:else if result.face === 'sad'}
								<Frown class="size-5 shrink-0 text-red-800" aria-hidden="true" />
							{:else}
								<!-- Live marker: this person is active right now. Same slot as the faces. -->
								<span
									class="relative flex size-5 shrink-0 items-center justify-center"
									aria-hidden="true"
								>
									<span
										class="bg-primary absolute inline-flex size-3 animate-ping rounded-full opacity-60 motion-reduce:animate-none"
									></span>
									<span
										class="bg-primary relative inline-flex size-3 rounded-full"
									></span>
								</span>
							{/if}
						</span>
					</button>
				</li>
			{/each}
		</ul>
	</Card.Content>
	<Card.Footer class="text-muted-foreground text-sm">
		People are shown by an anonymous code. Time counts only while the step is open.
	</Card.Footer>
</Card.Root>
