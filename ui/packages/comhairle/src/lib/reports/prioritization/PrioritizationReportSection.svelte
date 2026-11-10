<script lang="ts">
	import type {
		PrioritizationPublicQuestion,
		PrioritizationPublicRanking
	} from '@crownshy/api-client/api';
	import { ChevronDown } from 'lucide-svelte';
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import Crown from '$lib/components/icons/Crown.svelte';
	import * as Table from '$lib/components/ui/table';
	import DivergingProgress from '$lib/reports/DivergingProgress.svelte';
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';
	import PrioritizationThemes from './PrioritizationThemes.svelte';
	import { bucketAnswers, type PrioritizationEmbedData } from './embedData';

	/**
	 * Renders one embeddable Prioritization section block from its public report data, with
	 * the same charts the step's Insights page uses.
	 */
	let { data }: { data: PrioritizationEmbedData } = $props();

	const RANKING_ROWS_SHOWN = 4;
	let rankingRowLimit = $state(RANKING_ROWS_SHOWN);

	/**
	 * The ranking bars share one scale: an answer's range times the most reviews any proposal
	 * got, so the proposal with the most support fills its bar.
	 */
	function rankingDomain(ranking: PrioritizationPublicRanking): [number, number] {
		const mostReviews = Math.max(1, ...ranking.proposals.map((proposal) => proposal.reviews));
		return [ranking.min * mostReviews, ranking.max * mostReviews];
	}
</script>

{#snippet questionResults(question: PrioritizationPublicQuestion)}
	<div class="flex flex-col gap-2">
		<div>
			<h4 class="text-base font-bold">{question.question}</h4>
			<p class="text-muted-foreground text-sm">{question.answered} responses</p>
		</div>
		{#if question.scale}
			<KdePlot
				minLabel={question.scale.minLabel}
				maxLabel={question.scale.maxLabel}
				data={{ [question.question]: bucketAnswers(question.scale) }}
				minX={question.scale.min}
				maxX={question.scale.max}
			/>
		{:else}
			<BarChart
				data={question.choices.map((choice) => ({
					category: choice.label,
					count: choice.count
				}))}
				x="category"
				y="count"
				axis="x"
			/>
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
	{@const [min, max] = rankingDomain(ranking)}
	<div class="bg-card overflow-hidden rounded-xl border">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head class="text-muted-foreground w-[10%] px-5 text-center"
						>Ranking</Table.Head
					>
					<Table.Head class="text-muted-foreground w-2/5 px-5">Proposal</Table.Head>
					<Table.Head class="text-muted-foreground w-2/5 px-5"
						>{ranking.question}</Table.Head
					>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each ranking.proposals.slice(0, rankingRowLimit) as proposal, index (proposal.title)}
					<Table.Row>
						<Table.Cell class="text-muted-foreground px-5 py-6">
							<div class="flex justify-center">
								{#if index === 0}
									<Crown />
								{:else}
									{index + 1}
								{/if}
							</div>
						</Table.Cell>
						<Table.Cell class="h-auto px-5 text-base text-wrap!"
							>{proposal.title}</Table.Cell
						>
						<Table.Cell class="text-primary h-auto px-5">
							<DivergingProgress value={proposal.alignment} {min} {max} />
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
		{#if rankingRowLimit < ranking.proposals.length}
			<button
				class="bg-border flex w-full items-center justify-center gap-2 py-3 text-center text-base"
				type="button"
				onclick={() => (rankingRowLimit = ranking.proposals.length)}
			>
				See all {ranking.proposals.length} proposals <ChevronDown />
			</button>
		{/if}
	</div>
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
		{#each proposal.sections as section (section.position)}
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
