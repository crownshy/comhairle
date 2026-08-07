<script lang="ts">
	import type { PrioritizationInsightsResponse, WorkflowStepDto } from '@crownshy/api-client/api';
	import MetricOverviewCard from '../MetricOverviewCard.svelte';
	import ContentCard from '../ContentCard.svelte';
	import PrioritizationRankedProposalTable from './PrioritizationRankedProposalTable.svelte';
	import PrioritizationScatterPlot from './PrioritizationScatterPlot.svelte';
	import PrioritizationProposalResults from './PrioritizationProposalResults.svelte';
	import { resolveToolConfig } from '$lib/tools/prioritization/prioritizationApi';

	type Props = {
		insights: PrioritizationInsightsResponse | null;
		step: WorkflowStepDto;
		error?: string;
	};

	let { insights, step, error }: Props = $props();
	let toolConfig = $derived(resolveToolConfig(step, !!step.toolConfig));

	// Use average incase some steps weren't completed and a proposal has less
	// responses than others
	const averageNumParticipants = $derived.by(() => {
		if (!insights) return 0;

		const total = insights.rankedProposals.reduce(
			(acc, proposal) => proposal.responses.length + acc,
			0
		);

		return Math.ceil(total / insights.rankedProposals.length);
	});
</script>

{#if error}
	<div class="flex w-full flex-col gap-10">
		<h2>{error}</h2>
	</div>
{/if}
{#if insights}
	<div class="flex w-full flex-col gap-10">
		<div class="flex gap-4">
			<MetricOverviewCard
				superText="Participants"
				metric={averageNumParticipants}
				subText="unique voters"
			/>
		</div>

		<ContentCard>
			<div class="mb-10">
				<h2 class="text-lg font-bold">Proposal Ranking</h2>
				<p class="text-muted-foreground text-sm">See how all proposals rank.</p>
			</div>

			<PrioritizationRankedProposalTable proposals={insights.rankedProposals} />
		</ContentCard>

		<PrioritizationScatterPlot proposals={insights.rankedProposals} {toolConfig} />

		<PrioritizationProposalResults proposals={insights.rankedProposals} {toolConfig} />
	</div>
{/if}
