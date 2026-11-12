<script lang="ts">
	import type { PrioritizationPublicQuestion } from '@crownshy/api-client/api';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import PrioritizationQuestionChart, {
		type QuestionChart
	} from './PrioritizationQuestionChart.svelte';
	import PrioritizationRankingTable from './PrioritizationRankingTable.svelte';
	import PrioritizationThemes from './PrioritizationThemes.svelte';
	import { bucketAnswers, rankingDomain, type PrioritizationEmbedData } from './embedData';

	/**
	 * Renders one embeddable Prioritization section block from its public report data, with
	 * the same charts the step's Insights page uses.
	 */
	let { data }: { data: PrioritizationEmbedData } = $props();

	function questionChart(question: PrioritizationPublicQuestion): QuestionChart {
		if (question.scale) {
			return {
				kind: 'continuous',
				label: question.question,
				values: bucketAnswers(question.scale),
				min: question.scale.min,
				max: question.scale.max,
				minLabel: question.scale.minLabel,
				maxLabel: question.scale.maxLabel
			};
		}
		return { kind: 'likert', choices: question.choices };
	}
</script>

{#snippet questionResults(question: PrioritizationPublicQuestion)}
	<div class="flex flex-col gap-2">
		<div>
			<h4 class="text-base font-bold">{question.question}</h4>
			<p class="text-muted-foreground text-sm">{question.answered} responses</p>
		</div>
		{#if question.tooFewAnswers}
			<p class="text-muted-foreground text-base">
				Too few answers to show without pointing at someone.
			</p>
		{:else}
			<PrioritizationQuestionChart chart={questionChart(question)} />
		{/if}
	</div>
{/snippet}

{#if data.componentType === 'prioritization-key-stats'}
	<div class="flex flex-wrap gap-4">
		<MetricOverviewCard
			superText="Participants"
			metric={data.stats.participants}
			subText="unique reviewers"
		/>
		<MetricOverviewCard
			superText="Proposals"
			metric={data.stats.proposals}
			subText="reviewed"
		/>
		<MetricOverviewCard superText="Reviews" metric={data.stats.reviews} subText="handed in" />
	</div>
{:else if data.componentType === 'prioritization-ranking'}
	{@const ranking = data.ranking}
	{#if ranking.alignmentQuestion}
		<div class="bg-card overflow-hidden rounded-xl border">
			<PrioritizationRankingTable
				rows={ranking.proposals.map((proposal, index) => ({
					key: String(index),
					title: proposal.title,
					alignment: proposal.alignment ?? null
				}))}
				domain={rankingDomain(ranking.alignmentQuestion, ranking.proposals)}
				scoreHeading={ranking.alignmentQuestion.text}
			/>
		</div>
	{:else}
		<p
			class="border-border bg-card text-muted-foreground rounded-xl border border-dashed p-8 text-center text-base"
		>
			This step has no alignment question, so its proposals can't be ranked.
		</p>
	{/if}
{:else if data.componentType === 'prioritization-proposal'}
	{@const proposal = data.proposal}
	<div class="bg-card flex flex-col gap-6 rounded-xl border p-4">
		<div>
			<h3 class="text-lg font-bold">{proposal.title}</h3>
			<p class="text-muted-foreground text-sm">{proposal.reviews} reviews</p>
		</div>
		{#each proposal.questions as question (question.question)}
			{@render questionResults(question)}
		{/each}
		{#each proposal.sections as section, index (index)}
			{#if section.questions.length > 0}
				<section class="flex flex-col gap-4 rounded-xl border p-4">
					<ContentRenderer content={section.body} />
					{#each section.questions as question (question.question)}
						{@render questionResults(question)}
					{/each}
				</section>
			{/if}
		{/each}
		{#if proposal.questions.length === 0 && proposal.sections.every((s) => s.questions.length === 0)}
			<p class="text-muted-foreground text-base">
				This proposal has no rated questions to show. Written answers stay with the
				facilitators.
			</p>
		{/if}
	</div>
{:else if data.componentType === 'prioritization-themes'}
	<PrioritizationThemes themes={data.themes} />
{/if}
