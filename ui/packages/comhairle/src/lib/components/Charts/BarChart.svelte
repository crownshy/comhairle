<script lang="ts">
	import { BarChart } from 'layerchart';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { cubicInOut } from 'svelte/easing';
	import type { ComponentProps } from 'svelte';
	import { scaleBand } from 'd3-scale';

	let {
		data,
		x = 'label',
		y = 'value',
		height = 300,
		props: BarProps,
		context = $bindable(),
		...props
	}: ComponentProps<typeof BarChart> = $props();

	const chartConfig = {
		desktop: { label: 'Desktop', color: 'var(--chart-1)' },
		mobile: { label: 'Mobile', color: 'var(--chart-2)' }
	} satisfies Chart.ChartConfig;
</script>

<Chart.Container config={chartConfig}>
	<BarChart
		bind:context
		{data}
		{x}
		{y}
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
			<Chart.Tooltip />
		{/snippet}
	</BarChart>
</Chart.Container>
