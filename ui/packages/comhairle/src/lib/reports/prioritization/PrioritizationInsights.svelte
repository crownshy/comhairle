<script lang="ts">
	import type { PrioritizationInsightsResponse, WorkflowStepDto } from '@crownshy/api-client/api';
	import MetricOverviewCard from '../MetricOverviewCard.svelte';
	import ContentCard from '../ContentCard.svelte';
	import PrioritizationRankedProposalTable from './PrioritizationRankedProposalTable.svelte';
	import PrioritizationScatterPlot from './PrioritizationScatterPlot.svelte';

	type Props = {
		insights: PrioritizationInsightsResponse;
		step: WorkflowStepDto;
	};

	let { insights, step }: Props = $props();

	// Use average incase some steps weren't completed and a proposal has less
	// responses than others
	const averageNumParticipants = $derived.by(() => {
		const total = insights.rankedProposals.reduce(
			(acc, proposal) => proposal.responses.length + acc,
			0
		);

		return Math.ceil(total / insights.rankedProposals.length);
	});
</script>

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

	<PrioritizationScatterPlot
		proposals={insights.rankedProposals}
		toolConfig={step.toolConfig ?? step.previewToolConfig}
	/>
</div>
