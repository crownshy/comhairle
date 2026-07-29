<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import VBarChart from '$lib/components/Charts/VBarChart.svelte';
	import type { Insight } from './insights-loader';

	interface Props {
		data: Insight[];
	}

	let { data }: Props = $props();
</script>

{#each data as section (section.title)}
	<div class="h-68 py-10">
		<h2>{section.title}</h2>
		{#if section.chart.type === 'BarChart'}
			{#if section.chart.variant === 'label'}
				<BarChart data={section.chart.data} />
			{:else}
				<VBarChart data={section.chart.data} />
			{/if}
		{/if}
		{#if section.chart.type === 'Doughnut'}
			<VBarChart data={section.chart.data} />
		{/if}
		{#if section.chart.type === 'Line'}
			<VBarChart data={section.chart.data} />
		{/if}
	</div>
{/each}
