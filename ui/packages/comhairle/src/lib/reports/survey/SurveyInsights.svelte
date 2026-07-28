<script lang="ts">
	import VBarChart from '$lib/components/Charts/VBarChart.svelte';
	import type { Insight } from './insights-loader';

	interface Props {
		data: Insight[];
	}

	let { data }: Props = $props();
</script>

{#each data as section (section.title)}
	<div class="py-10">
		<h2>{section.title}</h2>
		{#if section.chart.type === 'Barplot'}
			<p>Label!</p>
			<VBarChart data={section.chart.data} />
		{/if}
		{#if section.chart.type === 'Doughnut'}
			<VBarChart data={section.chart.data} />
		{/if}
		{#if section.chart.type === 'Line'}
			<VBarChart data={section.chart.data} />
		{/if}
	</div>
{/each}
