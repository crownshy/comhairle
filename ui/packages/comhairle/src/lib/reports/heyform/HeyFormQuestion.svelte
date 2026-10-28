<script lang="ts">
	import type { HeyFormPublicQuestion } from '@crownshy/api-client/api';
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import SurveyBarChart from '$lib/reports/survey/SurveyBarChart.svelte';
	import { isHeyFormChoiceFieldKind } from '$lib/tools/heyform/guards';
	import { scaleAnswers } from './embedData';

	/** One question's results, drawn with the same charts as the admin Insights tab. */
	let { question }: { question: HeyFormPublicQuestion } = $props();
</script>

<section class="flex flex-col gap-2">
	<h3 class="text-lg font-semibold">{question.title}</h3>
	<p class="text-muted-foreground text-base">
		{question.answered}
		{question.answered === 1 ? 'response' : 'responses'}
	</p>
	{#if question.answered === 0}
		<p class="text-muted-foreground text-base">No responses yet</p>
	{:else if question.scale}
		<KdePlot
			data={{ answers: scaleAnswers(question.scale) }}
			maxX={question.scale.max}
			minLabel={question.scale.leftLabel ?? undefined}
			centerLabel={question.scale.centerLabel ?? undefined}
			maxLabel={question.scale.rightLabel ?? undefined}
		/>
	{:else if question.choices.length <= 3}
		<Doughnut data={question.choices} key="label" value="count" />
	{:else if isHeyFormChoiceFieldKind(question.kind)}
		<SurveyBarChart data={question.choices} x="label" y="count" kind={question.kind} />
	{/if}
</section>
