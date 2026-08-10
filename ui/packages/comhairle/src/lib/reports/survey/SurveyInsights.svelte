<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import VBarChart from '$lib/components/Charts/VBarChart.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import Responses from './Responses.svelte';
	import type { InsightQuestion } from '@crownshy/api-client/api';

	type ChartType = 'Bar' | 'Doughnut' | 'KdePlot' | 'Text';

	interface Props {
		data: InsightQuestion[];
	}

	let { data }: Props = $props();

	function getType(kind: InsightQuestion['kind']): ChartType | undefined {
		if (!kind) {
			return undefined;
		}
		switch (kind) {
			case 'yes_no':
				return 'Doughnut';
			case 'opinion_scale':
			case 'multiple_choice':
				return 'Bar';
			case 'number':
			case 'short_text':
			case 'long_text':
				return 'Text';
		}
	}
</script>

{#each data as section (section.id)}
	{@const type = getType(section.kind)}
	<div class="py-10">
		<h2 class="text-md font-bold">{section.title}</h2>
		{#if type === 'Bar'}
			<p>Bar chart</p>
			<!-- <BarChart data={section.data} /> -->
		{/if}
		{#if type === 'Doughnut'}
			<Doughnut data={section.choices ?? []} value="count" />
		{/if}
		{#if type === 'KdePlot'}
			<p>KdePlot chart</p>
			<!-- <KdePlot data={section.chart.data} /> -->
		{/if}
		{#if type === 'Text'}
			<p>Text chart</p>
			<!-- <Responses data={section.chart.data} /> -->
		{/if}
	</div>
{/each}
