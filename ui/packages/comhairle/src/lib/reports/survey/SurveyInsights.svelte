<script lang="ts">
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import SurveyBarChart from './SurveyBarChart.svelte';
	import SurveyFreeTextQuestion from './SurveyFreeTextQuestion.svelte';
	import type { ChoiceQuestion, NonChoiceQuestion, SurveyQuestion } from './insights-loader';
	import {
		isHeyFormChoiceFieldKind,
		isHeyFormNonChoiceFieldKind,
		isHeyFormOtherFieldKind
	} from '$lib/tools/heyform/guards';
	import { Button } from '$lib/components/ui/button';
	import { Download } from '@lucide/svelte';
	import { permissions } from '$lib/permissions.svelte';

	interface Props {
		data: SurveyQuestion[];
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
	}

	let { data, conversationId, workflowId, workflowStepId }: Props = $props();
	const isValidQuestion = (section: SurveyQuestion): boolean =>
		section.kind && !isHeyFormOtherFieldKind(section.kind);

	type Block =
		| { kind: 'question'; question: SurveyQuestion }
		| { kind: 'group'; id: string; title: string; questions: SurveyQuestion[] };

	// Consecutive questions with the same HeyForm parent group render together.
	const blocks = $derived.by(() => {
		const result: Block[] = [];
		for (const question of data.filter(isValidQuestion)) {
			const parent = question.properties?.parent;
			const last = result.at(-1);
			if (!parent) {
				result.push({ kind: 'question', question });
			} else if (last?.kind === 'group' && last.id === parent.id) {
				last.questions.push(question);
			} else {
				result.push({
					kind: 'group',
					id: parent.id,
					title: parent.title,
					questions: [question]
				});
			}
		}
		return result;
	});

	const canExport = $derived(
		permissions.can('conversation', 'conversation_export', conversationId)
	);

	// Browser-navigated download: cookie auth covers it, and the backend sets
	// Content-Disposition, so no fetch/Blob plumbing is needed here.
	const csvExportUrl = $derived(
		`/api/tools/survey_tool/conversation/${conversationId}/workflow_step/${workflowStepId}/submissions/csv`
	);

	const isChoiceQuestion = (section: SurveyQuestion): section is ChoiceQuestion =>
		isHeyFormChoiceFieldKind(section.kind);

	const isNonChoiceQuestion = (section: SurveyQuestion): section is NonChoiceQuestion =>
		isHeyFormNonChoiceFieldKind(section.kind);

	const isNumericArray = (arr: unknown[]): arr is number[] =>
		!!arr[0] && typeof arr[0] === 'number';

	const isStringArray = (arr: unknown[]): arr is string[] =>
		!!arr[0] && typeof arr[0] === 'string';
</script>

{#snippet questionBody(section: SurveyQuestion, level: 'h2' | 'h3' = 'h2')}
	<svelte:element this={level} class="text-lg font-semibold">{section.title}</svelte:element>
	<p class="text-muted-foreground text-base">
		{section.answered}
		{section.answered === 1 ? 'response' : 'responses'}
		·
		{Math.round((section.answered / section.total) * 100)}% Completion
	</p>
	{#if isChoiceQuestion(section)}
		{#if section.answers.length <= 3}
			<Doughnut data={section.answers} key="label" value="count" />
		{:else}
			<SurveyBarChart data={section.answers} x="label" y="count" kind={section.kind} />
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
			<SurveyFreeTextQuestion
				{conversationId}
				{workflowId}
				{workflowStepId}
				questionId={section.id}
				questionTitle={section.title}
				answers={section.answers}
			/>
		{/if}
	{/if}
{/snippet}
{#if data.length <= 0}
	<span class="text-muted-foreground">No responses yet</span>
{:else}
	<div class="flex max-w-3xl flex-col gap-6">
		<div class="flex justify-end">
			<Button href={csvExportUrl} download size="sm" disabled={!canExport}>
				<Download class="size-4" />
				Download CSV
			</Button>
		</div>
		{#each blocks as block (block.kind === 'group' ? block.id : block.question.id)}
			{#if block.kind === 'question'}
				<section class="bg-card flex flex-col gap-4 rounded-xl border p-6">
					{@render questionBody(block.question)}
				</section>
			{:else}
				<section class="bg-card flex flex-col gap-8 rounded-xl border p-6">
					<h2 class="text-xl font-semibold">{block.title}</h2>
					{#each block.questions as question (question.id)}
						<div class="flex flex-col gap-4 border-t pt-6">
							{@render questionBody(question, 'h3')}
						</div>
					{/each}
				</section>
			{/if}
		{/each}
	</div>
{/if}
