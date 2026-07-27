<script lang="ts">
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { BarChart, type ChartContextValue } from 'layerchart';
	import { cubicInOut } from 'svelte/easing';

	type Props = {
		data: any[];
		series?: any[];
		xKey: string;
		yKey: string;
		chartConfig: any;
	};

	let { data, series, xKey, yKey, chartConfig }: Props = $props();

	let context = $state<ChartContextValue>();
</script>

<Chart.Container config={chartConfig} class="aspect-auto h-62 w-full">
	<BarChart
		bind:context
		{data}
		{series}
		x={xKey}
		axis="x"
		y={yKey}
		props={{
			bars: {
				stroke: 'none',
				radius: 8,
				motion: {
					x: { type: 'tween', duration: 500, easing: cubicInOut },
					width: { type: 'tween', duration: 500, easing: cubicInOut },
					height: { type: 'tween', duration: 500, easing: cubicInOut },
					y: { type: 'tween', duration: 500, easing: cubicInOut }
				}
			},
			highlight: { area: { fill: 'none' } }
		}}
	></BarChart>
</Chart.Container>
