<script lang="ts">
	import { scaleBand } from 'd3-scale';
	import { BarChart, type ChartContextValue } from 'layerchart';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { cubicInOut } from 'svelte/easing';

	const chartData = [
		{ age: '<5', population: 2, conversation: 0 },
		{ age: '5-10', population: 10, conversation: 0 },
		{ age: '10-20', population: 20, conversation: 10 },
		{ age: '20-30', population: 25, conversation: 20 },
		{ age: '30-40', population: 30, conversation: 40 },
		{ age: '40-50', population: 15, conversation: 20 },
		{ age: '50-60', population: 12, conversation: 0 },
		{ age: '60-70', population: 5, conversation: 0 },
		{ age: '>70', population: 5, conversation: 0 }
	];

	const chartConfig = {
		conversation: { label: 'Conversation', color: 'var(--primary)' },
		population: { label: 'Population', color: 'var(--secondary)' }
	} satisfies Chart.ChartConfig;

	let context = $state<ChartContextValue>();
</script>

<Card.Root class="min-w-[500px]">
	<Card.Header>
		<Card.Title>Population Comparison</Card.Title>
	</Card.Header>
	<Card.Content>
		<Chart.Container config={chartConfig}>
			<BarChart
				bind:context
				data={chartData}
				xScale={scaleBand().padding(0.25)}
				x="age"
				axis="x"
				series={[
					{
						key: 'population',
						label: chartConfig.population.label,
						color: chartConfig.population.color
					},
					{
						key: 'conversation',
						label: chartConfig.conversation.label,
						color: chartConfig.conversation.color
					}
				]}
				seriesLayout="group"
				props={{
					bars: {
						stroke: 'none',
						rounded: 'none',
						radius: 8,
						// use the height of the chart to animate the bars
						initialY: context?.height,
						initialHeight: 0,
						motion: {
							x: { type: 'tween', duration: 500, easing: cubicInOut },
							width: { type: 'tween', duration: 500, easing: cubicInOut },
							height: { type: 'tween', duration: 500, easing: cubicInOut },
							y: { type: 'tween', duration: 500, easing: cubicInOut }
						}
					},
					highlight: { area: { fill: 'none' } }
				}}
			>
				{#snippet tooltip()}
					<Chart.Tooltip hideLabel />
				{/snippet}
			</BarChart>
		</Chart.Container>
	</Card.Content>
	<Card.Footer>
		<div class="flex w-full items-start gap-2 text-sm">
			<div class="grid gap-2">
				<div class="text-muted-foreground flex items-center gap-2 leading-none">
					Population comparison
				</div>
			</div>
		</div>
	</Card.Footer>
</Card.Root>
