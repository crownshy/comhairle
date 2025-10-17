<script lang="ts">
	import { scaleBand } from 'd3-scale';
	import { BarChart, type ChartContextValue } from 'layerchart';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { cubicInOut } from 'svelte/easing';

	const chartData = [
		{ gender: 'male', population: 40, conversation: 45 },
		{ gender: 'female', population: 55, conversation: 45 },
		{ gender: 'other', population: 5, conversation: 10 }
	];

	const chartConfig = {
		conversation: { label: 'Conversation', color: 'var(--primary)' },
		population: { label: 'Population', color: 'var(--secondary)' }
	} satisfies Chart.ChartConfig;

	let context = $state<ChartContextValue>();
</script>

<Card.Root class="min-w-[400px]">
	<Card.Header>
		<Card.Title>Gender Comparison</Card.Title>
	</Card.Header>
	<Card.Content>
		<Chart.Container config={chartConfig} class="pl-5">
			<BarChart
				bind:context
				data={chartData}
				yScale={scaleBand().padding(0.25)}
				orientation="horizontal"
				y="gender"
				axis="y"
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
						// use the height of the chart to animate the bars
						initialX: context?.width,
						initialWidth: 0,
						motion: {
							x: { type: 'tween', duration: 500, easing: cubicInOut },
							width: { type: 'tween', duration: 500, easing: cubicInOut },
							height: { type: 'tween', duration: 500, easing: cubicInOut }
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
					Gender comparison
				</div>
			</div>
		</div>
	</Card.Footer>
</Card.Root>
