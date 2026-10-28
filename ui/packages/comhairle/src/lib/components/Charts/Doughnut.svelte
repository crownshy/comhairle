<script lang="ts">
	import { PieChart } from 'layerchart';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import type { OneAxisChartValues } from './types';

	let {
		data,
		key = 'label',
		value = 'value',
		height = 250,
		...props
	}: OneAxisChartValues = $props();

	const colours = [
		'var(--chart-1)',
		'var(--chart-2)',
		'var(--chart-3)',
		'var(--chart-4)',
		'var(--chart-5)'
	];

	const chartConfig = {
		desktop: { label: 'Desktop', color: 'var(--chart-2)' },
		mobile: { label: 'Mobile', color: 'var(--chart-3)' }
	} satisfies Chart.ChartConfig;
</script>

<Chart.Container config={chartConfig}>
	<PieChart
		{data}
		{key}
		{value}
		innerRadius={-40}
		legend
		range={[-180, 180]}
		cRange={colours}
		{height}
		{...props}
	>
		{#snippet tooltip()}
			{#if tooltip !== undefined}
				<Chart.Tooltip />
			{/if}
		{/snippet}
	</PieChart>
</Chart.Container>
