<script lang="ts">
	import { cn } from '$lib/utils';
	import { isUnderRepresented, type RepresentationDimension } from './sampleMonitorData';

	type Props = {
		dimensions: RepresentationDimension[];
		/** Name of the selected dimension; bound so the card's headline number can follow it. */
		selected: string;
	};

	let { dimensions, selected = $bindable() }: Props = $props();

	let current = $derived(
		dimensions.find((dimension) => dimension.name === selected) ?? dimensions[0]
	);
	let underGroups = $derived(current.rows.filter(isUnderRepresented).map((row) => row.group));
</script>

<!-- At a glance Reach card: demographic tabs and the groups that need attention, no chart.
	 The full comparison chart lives in the Recruitment tab (RepresentationBars). -->
<div class="flex flex-col gap-3">
	<div
		role="tablist"
		aria-label="Demographic"
		class="bg-muted flex flex-wrap gap-0.5 self-start rounded-lg p-0.5"
	>
		{#each dimensions as dimension (dimension.name)}
			<button
				type="button"
				role="tab"
				aria-selected={selected === dimension.name}
				onclick={() => (selected = dimension.name)}
				class={cn(
					'min-h-8 rounded-md px-2.5 text-sm',
					selected === dimension.name
						? 'bg-card font-semibold shadow-sm'
						: 'text-muted-foreground'
				)}
			>
				{dimension.name}
			</button>
		{/each}
	</div>

	{#if underGroups.length > 0}
		<p class="text-sm">
			<span class="text-muted-foreground">Under-represented:</span>
			<span class="font-semibold">{underGroups.join(', ')}</span>
		</p>
	{:else}
		<p class="text-muted-foreground text-sm">
			Every {current.name.toLowerCase()} group is well covered.
		</p>
	{/if}
</div>
