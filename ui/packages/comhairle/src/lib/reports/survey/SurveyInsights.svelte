<script lang="ts">
	import { PersistedState } from 'runed';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import SurveyBarChart, { defaultView, type View } from './SurveyBarChart.svelte';
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
	import MetricOverviewCard from '../MetricOverviewCard.svelte';

	interface Props {
		data: SurveyQuestion[];
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
	}

	let { data, conversationId, workflowId, workflowStepId }: Props = $props();
	const isValidQuestion = (section: SurveyQuestion): boolean =>
		section.kind && !isHeyFormOtherFieldKind(section.kind);
	const isStringArray = (arr: unknown[]): arr is string[] =>
		!!arr[0] && typeof arr[0] === 'string';
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

	// Each admin's chart choices, kept in their browser per step.
	const views = new PersistedState<Record<string, View>>(
		`survey-insights-views:${workflowStepId}`,
		{}
	);

	const viewOf = (question: ChoiceQuestion): View =>
		views.current[question.id] ?? defaultView(question.kind, question.answers.length);

	// Only columns are compact enough to pair up; bars, donuts, scales, groups and free text take the full row.
	const isHalfWidth = (block: Block): boolean =>
		block.kind === 'question' &&
		isChoiceQuestion(block.question) &&
		viewOf(block.question) === 'vertical';

	const validQuestions = $derived(data.filter(isValidQuestion));
	// Every question shares the same submission count, so the largest total is the number of responses.
	const responseCount = $derived(Math.max(0, ...validQuestions.map((q) => q.total)));
	const completion = $derived.by(() => {
		const asked = validQuestions.reduce((sum, q) => sum + q.total, 0);
		const answered = validQuestions.reduce((sum, q) => sum + q.answered, 0);
		return asked === 0 ? 0 : Math.round((answered / asked) * 100);
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
</script>

{#snippet questionBody(section: SurveyQuestion, level: 'h2' | 'h3' = 'h2')}
	{#snippet header()}
		<div class="flex flex-col gap-1">
			<svelte:element this={level} class="text-lg font-semibold"
				>{section.title}</svelte:element
			>
		</div>
	{/snippet}
	{#if isChoiceQuestion(section)}
		<SurveyBarChart
			data={section.answers}
			x="label"
			y="count"
			kind={section.kind}
			bind:view={views.current[section.id]}
			title={header}
		/>
	{:else}
		{@render header()}
	{/if}
	{#if isNonChoiceQuestion(section)}
		{#if section.answers.length <= 0}
			<div class="text-muted-foreground">No responses yet</div>
		{/if}
		{#if isNumericArray(section.answers)}
			<KdePlot
				data={{ answers: section.answers }}
				height={96}
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
		<section aria-label="Summary" class="flex flex-wrap items-end justify-between gap-4">
			<div class="flex flex-wrap gap-4">
				<MetricOverviewCard
					superText="Responses"
					metric={responseCount}
					subText="submitted"
				/>
				<MetricOverviewCard
					superText="Completion"
					metric={completion}
					subText="% of questions answered"
				/>
			</div>
			<Button href={csvExportUrl} download disabled={!canExport}>
				<Download class="size-4" />
				Download CSV
			</Button>
		</section>
		<div class="grid gap-6 lg:grid-cols-2">
			{#each blocks as block (block.kind === 'group' ? block.id : block.question.id)}
				{#if block.kind === 'question'}
					<section
						class={[
							'bg-card flex flex-col gap-4 rounded-sm border p-6',
							!isHalfWidth(block) && 'lg:col-span-2'
						]}
					>
						{@render questionBody(block.question)}
					</section>
				{:else}
					<section
						class="bg-card flex flex-col gap-6 rounded-sm border p-6 lg:col-span-2"
					>
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
	</div>
{/if}
