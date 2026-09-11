<script lang="ts">
	import type { PrioritizationInsightsResponse, WorkflowStepDto } from '@crownshy/api-client/api';
	import MetricOverviewCard from '../MetricOverviewCard.svelte';
	import ContentCard from '../ContentCard.svelte';
	import PrioritizationRankedProposalTable from './PrioritizationRankedProposalTable.svelte';
	import PrioritizationScatterPlot from './PrioritizationScatterPlot.svelte';
	import PrioritizationProposalResults from './PrioritizationProposalResults.svelte';
	import { resolveToolConfig } from '$lib/tools/prioritization/prioritizationApi';
	import { localizeTranslatableJson } from '$lib/components/Translation/translationUtils';

	type Props = {
		insights: PrioritizationInsightsResponse | null;
		step: WorkflowStepDto;
		error: string | null;
	};

	let { insights, step, error }: Props = $props();
	let toolConfig = $derived(localizeTranslatableJson(resolveToolConfig(step, !!step.toolConfig)));

	const numDistinctParticipants = $derived.by(() => {
		if (!insights) return 0;

		const voterIds = new Set();

		for (const proposal of insights.rankedProposals) {
			for (const response of proposal.responses) {
				voterIds.add(response.userId);
			}
		}

		return voterIds.size;
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
				metric={numDistinctParticipants}
				subText="unique voters"
			/>
		</div>

		<ContentCard>
			<div class="mb-10">
				<h2 class="text-lg font-bold">Proposal Ranking</h2>
				<p class="text-muted-foreground text-sm">See how all proposals rank.</p>
			</div>

			<PrioritizationRankedProposalTable proposals={insights.rankedProposals} {toolConfig} />
		</ContentCard>

		<PrioritizationScatterPlot proposals={insights.rankedProposals} {toolConfig} />

		<PrioritizationProposalResults proposals={insights.rankedProposals} {toolConfig} />
	</div>
{/if}
