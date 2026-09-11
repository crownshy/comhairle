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
	import { handleNested } from './attachments';

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

{#if data.length <= 0}
	<span class="text-muted-foreground">No responses yet</span>
{:else}
	{#each data as section (section.id)}
		{#if isValidQuestion(section)}
			<div class="py-10" {@attach handleNested(section.properties?.parent)}>
				<h2 class="text-md font-bold">{section.title}</h2>
				<div class="flex flex-row">
					<h3 class="text-muted-foreground mr-10 text-sm">
						{section.answered}
						{section.answered === 1 ? 'response' : 'responses'}
						·
						{Math.round((section.answered / section.total) * 100)}% Completion
					</h3>
				</div>
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
					{#if section.answers.length <= 0}
						<div class="text-muted-foreground">No responses yet</div>
					{/if}
					{#if isNumericArray(section.answers)}
						<KdePlot
							data={{ answers: section.answers }}
							maxX={section.properties?.total ?? Math.max(10, ...section.answers)}
							minLabel={section.properties?.leftLabel}
							centerLabel={section.properties?.centerLabel}
							maxLabel={section.properties?.rightLabel}
						/>
					{/if}
					{#if isStringArray(section.answers)}
						<Responses data={section.answers} />
					{/if}
				{/if}
			</div>
		{/if}
	{/each}
{/if}
