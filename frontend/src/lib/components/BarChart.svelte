<script lang="ts">
	import { scaleBand } from 'd3-scale';
	import { BarChart, type ChartContextValue } from 'layerchart';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { cubicInOut } from 'svelte/easing';

	const chartData = [
		{ month: 'January', accepts: 186, rejects: 80 },
		{ month: 'February', accepts: 305, rejects: 200 },
		{ month: 'March', accepts: 237, rejects: 120 },
		{ month: 'April', accepts: 73, rejects: 190 },
		{ month: 'May', accepts: 209, rejects: 130 },
		{ month: 'June', accepts: 214, rejects: 140 }
	];

	const chartConfig = {
		desktop: { label: 'Desktop', color: 'var(--secondary)' },
		mobile: { label: 'Mobile', color: 'var(--primary)' }
	} satisfies Chart.ChartConfig;

	let context = $state<ChartContextValue>();
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Responses</Card.Title>
	</Card.Header>
	<Card.Content>
		<Chart.Container config={chartConfig}>
			<BarChart
				bind:context
				data={chartData}
				xScale={scaleBand().padding(0.25)}
				x="month"
				axis="x"
				series={[
					{ key: 'accepts', label: 'Accepted', color: chartConfig.desktop.color },
					{ key: 'rejects', label: 'Rejected', color: chartConfig.mobile.color }
				]}
				legend
				x1Scale={scaleBand().paddingInner(0.2)}
				seriesLayout="stack"
				rule={false}
				props={{
					bars: {
						stroke: 'none',
						strokeWidth: 0,
						rounded: 'all',
						// use the height of the chart to animate the bars
						initialY: context?.height,
						initialHeight: 0,
						motion: {
							y: { type: 'tween', duration: 500, easing: cubicInOut },
							height: { type: 'tween', duration: 500, easing: cubicInOut }
						}
					},
					highlight: { area: { fill: 'none' } },
					xAxis: { format: (d) => d.slice(0, 3) }
				}}
			>
				{#snippet tooltip()}
					<Chart.Tooltip />
				{/snippet}
			</BarChart>
		</Chart.Container>
	</Card.Content>
</Card.Root>
