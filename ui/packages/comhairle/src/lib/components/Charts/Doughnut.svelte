<script lang="ts">
	import { PieChart, Text } from 'layerchart';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import type { OneAxisChartValues } from './types';

	let {
		data,
		key = 'label',
		value = 'value',
		height = 300,
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

	let majority = $derived.by(() => {
		let majority = data?.[0];

		for (let i = 1; i < (data?.length ?? 1); i++) {
			if (Number(data?.[i][value]) > Number(majority?.[value])) {
				majority = data?.[i];
			}
		}

		return majority;
	});
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
		{#snippet aboveMarks()}
			<Text
				value={String(majority?.[value] ?? '')}
				textAnchor="middle"
				verticalAnchor="middle"
				class="text-4xl! font-bold"
				dy={-15}
			/>
			<Text
				value={String(majority?.[key] ?? '')}
				textAnchor="middle"
				verticalAnchor="middle"
				class="text-xl!"
				dy={15}
			/>
		{/snippet}
	</PieChart>
</Chart.Container>
