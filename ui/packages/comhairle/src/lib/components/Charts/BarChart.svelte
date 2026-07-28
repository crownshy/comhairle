<script lang="ts">
	import { BarChart, type ChartState } from 'layerchart';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { cubicInOut } from 'svelte/easing';
	import type { ComponentProps } from 'svelte';

	let { data, x, y, props: BarProps, ...props }: ComponentProps<typeof BarChart> = $props();

	let context = $state<ChartState>();

	const chartConfig = {
		desktop: { label: 'Desktop', color: 'var(--chart-1)' },
		mobile: { label: 'Mobile', color: 'var(--chart-2)' }
	} satisfies Chart.ChartConfig;
</script>

<!-- <DiffBarChart /> -->
<Chart.Container config={chartConfig}>
	<BarChart
		bind:context
		{data}
		x={x ?? 'label'}
		y={y ?? 'value'}
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
		{...props}
	>
		{#snippet tooltip()}
			<Chart.Tooltip />
		{/snippet}
	</BarChart>
</Chart.Container>
