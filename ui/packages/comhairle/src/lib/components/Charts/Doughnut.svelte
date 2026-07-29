<script lang="ts">
	import { PieChart, Text } from 'layerchart';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import type { ComponentProps } from 'svelte';
	import type { ChartData } from './types';

	let {
		data,
		key = 'label',
		value = 'value',
		height = 300,
		...props
	}: Omit<ComponentProps<typeof PieChart>, 'data'> & { data: ChartData[] } = $props();

	const chartConfig = {
		desktop: { label: 'Desktop', color: 'var(--chart-2)' },
		mobile: { label: 'Mobile', color: 'var(--chart-3)' }
	} satisfies Chart.ChartConfig;

	let majority = $derived.by(() => {
		let majority = data[0];

		for (let i = 1; i < data.length; i++) {
			if (data[i].value > majority.value) {
				majority = data[i];
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
		padding={{ top: 24, bottom: 24, left: 80, right: 80 }}
		cornerRadius={2}
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
				value={majority.value}
				textAnchor="middle"
				verticalAnchor="middle"
				class="text-4xl! font-bold"
				dy={-15}
			/>
			<Text
				value={majority.label}
				textAnchor="middle"
				verticalAnchor="middle"
				class="text-xl!"
				dy={15}
			/>
		{/snippet}
	</PieChart>
</Chart.Container>
