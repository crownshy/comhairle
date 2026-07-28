<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import BarChart from './BarChart.svelte';
	import { scaleBand } from 'd3-scale';
	import { cubicInOut } from 'svelte/easing';
	import type { ChartContextValue } from 'layerchart';

	let { x, y, props: BarProps, ...props }: ComponentProps<typeof BarChart> = $props();

	let context = $state<ChartContextValue>();
</script>

<BarChart
	{...props}
	props={{
		...BarProps,
		bars: {
			stroke: BarProps?.bars?.stroke ?? 'none',
			strokeWidth: BarProps?.bars?.strokeWidth ?? 0,
			rounded: BarProps?.bars?.rounded ?? 'all',
			// use the width of the chart to animate the bars
			initialX: context?.width,
			initialWidth: 0,
			motion: {
				x: { type: 'tween', duration: 500, easing: cubicInOut },
				width: { type: 'tween', duration: 500, easing: cubicInOut }
			}
		},
		yAxis: {
			tickLabelProps: {
				dx: -15,
				width: 150
			}
		}
	}}
	orientation="horizontal"
	y={x ?? 'label'}
	x={y ?? 'value'}
	labels
	yScale={scaleBand().paddingOuter(0.4).paddingInner(0.7)}
	grid={false}
	axis="y"
	padding={{ left: 170 }}
/>
