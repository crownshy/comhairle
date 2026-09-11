<script lang="ts">
	import { BarChart } from 'layerchart';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { cubicInOut } from 'svelte/easing';
	import { scaleBand } from 'd3-scale';
	import type { TwoAxisChartValues } from '$lib/components/Charts/types';

	let {
		config,
		height = 300,
		props: BarProps,
		context = $bindable(),
		...props
	}: TwoAxisChartValues = $props();

	let chartConfig = {
		key1: { label: 'Label', color: 'var(--chart-1)' },
		key2: { label: 'Value', color: 'var(--chart-2)' }
	} satisfies Chart.ChartConfig;
</script>

<Chart.Container config={chartConfig}>
	<BarChart
		bind:context
		data={config.data}
		x={config.x}
		y={config.y}
		series={config.series}
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
			...BarProps
		}}
		xScale={scaleBand().paddingInner(0.7)}
		{height}
		{...props}
	>
		{#snippet tooltip()}
			{#if tooltip !== undefined}
				<Chart.Tooltip />
			{/if}
		{/snippet}
	</BarChart>
</Chart.Container>
