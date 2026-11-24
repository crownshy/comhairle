<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';
	import { isUnderRepresented, type RepresentationDimension } from './sampleMonitorData';

	type Props = {
		dimensions: RepresentationDimension[];
		/** The demographic tab that's open; bindable so other parts of the page can switch it. */
		selected?: string;
	};

	let { dimensions, selected = $bindable(dimensions[0]?.name ?? '') }: Props = $props();

	let current = $derived(
		dimensions.find((dimension) => dimension.name === selected) ?? dimensions[0]
	);
	let scaleMax = $derived(
		Math.max(
			...current.rows.flatMap((row) => [row.participantPercent, row.populationPercent])
		) * 1.1
	);

	function widthOf(percent: number): string {
		return `${(percent / scaleMax) * 100}%`;
	}
</script>

<!--
	Share of participants in each group (bar) against the population share (tick), one
	demographic at a time. id="demographic-analysis" is the scroll target for RepresentationHint.
-->
<Card.Root id="demographic-analysis" tabindex={-1} class="scroll-mt-6 focus:outline-none">
	<Card.Header class="flex flex-row flex-wrap items-center justify-between gap-3">
		<Card.Title>Demographic analysis</Card.Title>
	</Card.Header>

	<Card.Content class="flex flex-col gap-4"
		><div role="tablist" aria-label="Demographic" class="flex flex-wrap gap-1">
			{#each dimensions as dimension (dimension.name)}
				<button
					type="button"
					role="tab"
					aria-selected={selected === dimension.name}
					onclick={() => (selected = dimension.name)}
					class={cn(
						'min-h-10 border-b-2 px-3 text-base',
						selected === dimension.name
							? 'border-primary font-semibold'
							: 'text-muted-foreground border-transparent'
					)}
				>
					{dimension.name}
				</button>
			{/each}
		</div>
		<ul class="flex flex-wrap gap-5 text-sm" aria-label="Legend">
			<li class="inline-flex items-center gap-2">
				<span class="bg-primary h-2.5 w-3.5 rounded-sm"></span>Your participants
			</li>
			<li class="inline-flex items-center gap-2">
				<span class="h-4 w-0.5 bg-gray-500 opacity-30"></span>
				<span class="text-muted-foreground">Baseline</span>
			</li>
		</ul>
		{#each current.rows as row (row.group)}
			{@const under = isUnderRepresented(row)}
			<div
				class="grid grid-cols-[8rem_minmax(0,1fr)] items-center gap-4 md:grid-cols-[8rem_minmax(0,1fr)_14rem]"
			>
				<span class="text-base font-semibold">{row.group}</span>
				<div
					class="bg-muted/60 relative h-5 rounded"
					role="img"
					aria-label="{row.participantPercent}% of participants, {row.populationPercent}% of population"
				>
					<div
						class={cn(
							'absolute inset-y-0.5 left-0 rounded-sm',
							under ? 'bg-amber-500' : 'bg-primary'
						)}
						style:width={widthOf(row.participantPercent)}
					></div>
					<div
						class="absolute -inset-y-1 w-0.5 bg-gray-500 opacity-30"
						style:left={widthOf(row.populationPercent)}
					></div>
				</div>
				<span
					class={cn(
						'col-start-2 text-sm md:col-start-auto',
						under && 'font-semibold text-amber-500'
					)}
				>
					{#if under}▼ Under-represented{/if}
					({row.participantPercent}% vs {row.populationPercent}%)
				</span>
			</div>
		{/each}
	</Card.Content>
</Card.Root>
