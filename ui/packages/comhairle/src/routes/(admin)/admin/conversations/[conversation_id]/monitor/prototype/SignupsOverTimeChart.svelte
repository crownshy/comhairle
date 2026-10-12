<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { LineChart } from 'layerchart';
	import { scaleUtc } from 'd3-scale';
	import { cn } from '$lib/utils';
	import { ACTION_KINDS } from './actionKinds';
	import ActionLogItem from './ActionLogItem.svelte';
	import type { ActivityLogEntry, SignupPoint } from './sampleMonitorData';

	type Props = {
		signups: SignupPoint[];
		/** Running total of people who finished every step. */
		completions: SignupPoint[];
		targetPace: SignupPoint[];
		projection: SignupPoint[];
		/** The Action calendar's log: every logged action is marked on the chart. */
		actions: ActivityLogEntry[];
		target: number;
	};

	let { signups, completions, targetPace, projection, actions, target }: Props = $props();

	const chartConfig = {
		joined: { label: 'Joined', color: 'var(--primary)' },
		completed: { label: 'Completed', color: 'var(--color-green-600)' },
		projection: { label: 'If nothing changes', color: 'var(--primary)' },
		target: { label: 'Even pace to target', color: 'var(--muted-foreground)' }
	} satisfies Chart.ChartConfig;

	let allPoints = $derived([...targetPace, ...signups, ...completions, ...projection]);

	let sortedActions = $derived([...actions].sort((a, b) => a.date.getTime() - b.date.getTime()));

	// One marker per logged action, in that action's colour. Labels step down in three
	// rows so actions on neighbouring days don't print on top of each other.
	let annotations = $derived(
		sortedActions.map((action, index) => {
			const kind = ACTION_KINDS[action.kind];
			return {
				type: 'line' as const,
				x: action.date,
				label: action.scheduled ? `${kind.label} (planned)` : kind.label,
				// The last marker sits at the right edge, so its label reads leftwards from the
				// line to stay inside the card. Every label keeps a small gap from its line.
				labelPlacement:
					index === sortedActions.length - 1
						? ('top-left' as const)
						: ('top-right' as const),
				labelXOffset: 6,
				labelYOffset: 4 + (index % 3) * 16,
				props: {
					line: {
						style: `stroke: ${kind.lineColor}; stroke-width: 1.5px;  opacity: ${action.scheduled ? 0.1 : 0.3}`
					},
					label: {
						class: cn(
							kind.fill,
							'text-sm font-medium',
							action.scheduled && 'opacity-70'
						)
					}
				}
			};
		})
	);

	/** Which key item is hovered or focused: an action kind, or 'planned'. */
	let hoveredKey = $state<string | null>(null);
	/** Which key item was clicked: its actions stay listed inside the card. */
	let selectedKey = $state<string | null>(null);
	/** Hovering previews a key item in the same panel; otherwise the clicked one shows. */
	let shownKey = $derived(hoveredKey ?? selectedKey);

	/** The actions a key item stands for, shown in a card under the key. */
	function actionsForKey(key: string): ActivityLogEntry[] {
		return key === 'planned'
			? sortedActions.filter((action) => action.scheduled)
			: sortedActions.filter((action) => action.kind === key);
	}

	/** Kinds that appear on the chart, for the key underneath. */
	let kindsShown = $derived([...new Set(sortedActions.map((action) => action.kind))]);

	/** The tooltip can pass a series label instead of a date, so accept anything. */
	function formatDay(day: unknown): string {
		if (day instanceof Date)
			return day.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' });
		return String(day ?? '');
	}
</script>

<!--
	Running totals of joined and completed against an even pace to the target, with each
	action from the Action calendar marked. The key under the chart lists those actions:
	hover previews them in the panel below it, click keeps them open.
-->
<Card.Root>
	<Card.Header>
		<Card.Title>Signups and completions over time</Card.Title>
		<Card.Description
			>Running totals of people who joined and who completed, with the actions from your
			Action calendar marked</Card.Description
		>
	</Card.Header>
	<Card.Content>
		<div class="relative">
			<Chart.Container config={chartConfig} class="aspect-auto h-[300px] w-full">
				<LineChart
					data={allPoints}
					x="day"
					xScale={scaleUtc()}
					y="total"
					yDomain={[
						0,
						Math.max(target, ...[...signups, ...projection].map((point) => point.total))
					]}
					legend
					{annotations}
					series={[
						{
							key: 'target',
							label: chartConfig.target.label,
							data: targetPace,
							value: 'total',
							color: chartConfig.target.color,
							props: { class: '[stroke-dasharray:2_4]' }
						},
						{
							key: 'projection',
							label: chartConfig.projection.label,
							data: projection,
							value: 'total',
							color: chartConfig.projection.color,
							props: { class: '[stroke-dasharray:6_5]' }
						},
						{
							key: 'completed',
							label: chartConfig.completed.label,
							data: completions,
							value: 'total',
							color: chartConfig.completed.color,
							props: { strokeWidth: 3 }
						},
						{
							key: 'joined',
							label: chartConfig.joined.label,
							data: signups,
							value: 'total',
							color: chartConfig.joined.color,
							props: { strokeWidth: 3 }
						}
					]}
					props={{
						xAxis: { format: (day: unknown) => formatDay(day) }
					}}
				>
					{#snippet tooltip()}
						<Chart.Tooltip labelFormatter={(day: unknown) => formatDay(day)} />
					{/snippet}
				</LineChart>
			</Chart.Container>
		</div>
		<ul
			class="text-muted-foreground mt-3 flex flex-wrap gap-x-1 gap-y-1 text-sm"
			aria-label="Actions marked on the chart. Select one to see its details"
		>
			{#each [...kindsShown, 'planned'] as key (key)}
				{@const isPlanned = key === 'planned'}
				<li>
					<button
						type="button"
						class={cn(
							'hover:bg-muted focus-visible:ring-ring inline-flex cursor-pointer items-center gap-1.5 rounded-md px-2 py-1 focus-visible:ring-2 focus-visible:outline-none',
							shownKey === key && 'bg-muted text-foreground',
							selectedKey === key && 'font-semibold'
						)}
						aria-pressed={selectedKey === key}
						aria-controls="key-details"
						onclick={() => (selectedKey = selectedKey === key ? null : key)}
						onmouseenter={() => (hoveredKey = key)}
						onmouseleave={() => (hoveredKey = null)}
						onfocus={() => (hoveredKey = key)}
						onblur={() => (hoveredKey = null)}
					>
						<span
							class={cn(
								'h-3 w-0.5',
								isPlanned
									? 'bg-muted-foreground opacity-50'
									: ACTION_KINDS[key as keyof typeof ACTION_KINDS].dot
							)}
						></span>{isPlanned
							? 'Planned (faded)'
							: ACTION_KINDS[key as keyof typeof ACTION_KINDS].label}
					</button>
				</li>
			{/each}
		</ul>

		<!-- Actions for the hovered or clicked key item. -->
		<div id="key-details" aria-live="polite">
			{#if shownKey}
				{@const keyActions = actionsForKey(shownKey)}
				<div class="bg-muted/50 mt-3 flex flex-wrap gap-3 rounded-xl p-4">
					{#if keyActions.length}
						<div class="grid gap-3 sm:grid-cols-2">
							{#each keyActions as action (action.date.toISOString() + action.title)}
								<ActionLogItem entry={action} />
							{/each}
						</div>
					{:else}
						<span class="text-muted-foreground text-sm">Nothing logged yet.</span>
					{/if}
				</div>
			{/if}
		</div>
	</Card.Content>
</Card.Root>
