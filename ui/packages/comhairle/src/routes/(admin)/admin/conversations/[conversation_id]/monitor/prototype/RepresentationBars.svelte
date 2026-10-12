<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';
	import { isUnderRepresented, type RepresentationDimension } from './sampleMonitorData';

	type Props = {
		dimensions: RepresentationDimension[];
		participantCount: number;
	};

	let { dimensions, participantCount }: Props = $props();

	let selected = $state(dimensions[0]?.name ?? '');
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
	New data viz component: share of participants in each group (bar) against the share
	of the population you want to hear from (black tick), one demographic at a time.
-->
<Card.Root>
	<Card.Header class="flex flex-row flex-wrap items-center justify-between gap-3">
		<div role="tablist" aria-label="Demographic" class="flex flex-wrap gap-1">
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
				<span class="bg-foreground h-4 w-0.5"></span>Local population
			</li>
		</ul>
	</Card.Header>
	<Card.Content class="flex flex-col gap-4">
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
						class="bg-foreground absolute -inset-y-1 w-0.5"
						style:left={widthOf(row.populationPercent)}
					></div>
				</div>
				<span
					class={cn(
						'col-start-2 text-sm md:col-start-auto',
						under && 'font-semibold text-amber-900'
					)}
				>
					{#if under}▼ Under-represented ·{/if}
					{row.participantPercent}% vs {row.populationPercent}%
				</span>
			</div>
		{/each}
	</Card.Content>
	<Card.Footer class="text-muted-foreground border-t pt-4 text-sm">
		Based on {current.shared} of {participantCount} participants who answered the {current.name.toLowerCase()}
		question. Population baseline would be set in Configure.
	</Card.Footer>
</Card.Root>
