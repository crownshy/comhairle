<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';

	export type QuestionChart =
		| { kind: 'likert'; choices: { label: string; count: number }[] }
		| {
				kind: 'continuous';
				/** Names the plotted series. */
				label: string;
				values: number[];
				min: number;
				max: number;
				minLabel: string;
				maxLabel: string;
		  };

	/** One rated question's answers, as the Insights page and the report both draw them. */
	let { chart }: { chart: QuestionChart } = $props();
</script>

{#if chart.kind === 'likert'}
	<BarChart
		data={chart.choices.map((choice) => ({ category: choice.label, count: choice.count }))}
		x="category"
		y="count"
		axis="x"
	/>
{:else}
	<KdePlot
		minLabel={chart.minLabel}
		maxLabel={chart.maxLabel}
		data={{ [chart.label]: chart.values }}
		minX={chart.min}
		maxX={chart.max}
	/>
{/if}
