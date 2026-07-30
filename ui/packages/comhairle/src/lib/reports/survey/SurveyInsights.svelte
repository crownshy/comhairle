<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import VBarChart from '$lib/components/Charts/VBarChart.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import type { Insight } from './insights-loader';
	import Responses from './Responses.svelte';

	interface Props {
		data: Insight[];
	}

	let { data }: Props = $props();
</script>

{#each data as section (section.title)}
	<div class="py-10">
		<h2 class="text-md font-bold">{section.title}</h2>
		{#if section.chart.type === 'BarChart'}
			{#if section.chart.variant === 'label'}
				<BarChart data={section.chart.data} axis="x" />
			{:else}
				<VBarChart data={section.chart.data} />
			{/if}
		{/if}
		{#if section.chart.type === 'Doughnut'}
			<Doughnut data={section.chart.data} />
		{/if}
		{#if section.chart.type === 'Line'}
			<KdePlot data={section.chart.data} />
		{/if}
		{#if section.chart.type === 'Text'}
			<Responses data={section.chart.data} />
		{/if}
	</div>
{/each}
