<script lang="ts">
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import Responses from './Responses.svelte';
	import SurveyBarChart from './SurveyBarChart.svelte';
	import type { ChoiceQuestion, NonChoiceQuestion, SurveyQuestion } from './insights-loader';
	import {
		isHeyFormChoiceFieldKind,
		isHeyFormNonChoiceFieldKind,
		isHeyFormOtherFieldKind
	} from '$lib/tools/heyform/guards';

	interface Props {
		data: SurveyQuestion[];
	}

	let { data }: Props = $props();

	const isValidQuestion = (section: SurveyQuestion): boolean =>
		section.kind && !isHeyFormOtherFieldKind(section.kind);

	const isChoiceQuestion = (section: SurveyQuestion): section is ChoiceQuestion =>
		isHeyFormChoiceFieldKind(section.kind);

	const isNonChoiceQuestion = (section: SurveyQuestion): section is NonChoiceQuestion =>
		isHeyFormNonChoiceFieldKind(section.kind);

	const isNumericArray = (arr: unknown[]): arr is number[] =>
		!!arr[0] && typeof arr[0] === 'number';

	const isStringArray = (arr: unknown[]): arr is string[] =>
		!!arr[0] && typeof arr[0] === 'string';
</script>

{#each data as section (section.id)}
	{#if isValidQuestion(section)}
		<div class="py-10">
			<h2 class="text-md font-bold">{section.title}</h2>
			<!-- <div class="flex flex-row"> -->
			<!-- 	<h3 class="text-muted-foreground mr-10 text-sm"> -->
			<!-- 		{section.answers.length} -->
			<!-- 		{section.answers.length === 1 ? 'response' : 'responses'} -->
			<!-- 		· -->
			<!-- 		{Math.round(section.answers.length / section.total) * 100}% Completion -->
			<!-- 	</h3> -->
			<!-- </div> -->
			{#if isChoiceQuestion(section)}
				{#if section.answers.length <= 3}
					<Doughnut data={section.answers} key="label" value="count" />
				{:else}
					<SurveyBarChart
						data={section.answers}
						x="label"
						y="count"
						kind={section.kind}
					/>
				{/if}
			{/if}
			{#if isNonChoiceQuestion(section)}
				{#if isNumericArray(section.answers)}
					<KdePlot
						data={{ answers: section.answers }}
						maxX={section.properties?.total ?? 10}
					/>
				{/if}
				{#if isStringArray(section.answers)}
					<Responses data={section.answers} />
				{/if}
			{/if}
		</div>
	{/if}
{/each}
