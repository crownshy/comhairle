<script lang="ts">
	import type { ProposalResponseDto, RankedProposal } from '@crownshy/api-client/api';
	import ContentCard from '../ContentCard.svelte';
	import ScatterPlot, { type ScatterPoint } from '$lib/components/Charts/ScatterPlot.svelte';
	import * as Select from '$lib/components/ui/select';
	import { extractAxisDomain } from './utils';
	import type { ToolConfig } from '$lib/tools/prioritization';

	type Props = {
		proposals: RankedProposal[];
		toolConfig: ToolConfig;
	};

	let { proposals, toolConfig }: Props = $props();

	let selectedProposal = $state(proposals[0]);

	const xQuestion = $derived(
		toolConfig.questions.find((question) => question.id === toolConfig.alignmentQuestionId)
	);
	const yQuestion = $derived(
		toolConfig.questions.find((question) => question.id !== toolConfig.alignmentQuestionId)
	);

	function extractScatterPoints(
		responses: ProposalResponseDto[],
		xQuestionId: string,
		yQuestionId: string
	): ScatterPoint[] {
		const points: ScatterPoint[] = [];

		for (const res of responses) {
			// Filter out section responses for now
			const questionResponses = res.response.filter((r) => !r.section_id);

			const xValue = questionResponses.find((r) => r.question_id === xQuestionId)?.value;
			const yValue = questionResponses.find((r) => r.question_id === yQuestionId)?.value;

			if (typeof xValue === 'number' && typeof yValue === 'number') {
				points.push({ x: xValue, y: yValue, id: res.userId });
			}
		}

		return points;
	}

	let scatterPoints = $derived(
		xQuestion && yQuestion
			? extractScatterPoints(selectedProposal.responses, xQuestion.id, yQuestion.id)
			: []
	);

	let xDomain = $derived.by(() => (xQuestion ? extractAxisDomain(xQuestion) : null));
	let yDomain = $derived.by(() => (yQuestion ? extractAxisDomain(yQuestion) : null));
</script>

<ContentCard>
	<div class="mb-10">
		<h2 class="text-lg font-bold">Alignent and importance</h2>
		<p class="text-muted-foreground text-sm">
			See the distribution chart showing the level of consensus or disagreement for each
			participant or proposal.
		</p>
	</div>

	{#if xQuestion && yQuestion && xDomain && yDomain}
		<div class="mb-4 flex justify-end">
			<Select.Root
				type="single"
				onValueChange={(v) => {
					const found = proposals.find((p) => p.id === v);
					if (found) selectedProposal = found;
				}}
			>
				<Select.Trigger>{selectedProposal.title}</Select.Trigger>
				<Select.Content>
					{#each proposals as proposal (proposal.id)}
						<Select.Item value={proposal.id}>{proposal.title}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<ScatterPlot
			points={scatterPoints}
			xAxisLabel={xQuestion.text}
			yAxisLabel={yQuestion.text}
			{xDomain}
			{yDomain}
		/>
	{:else}
		<div class="flex flex-col items-center justify-center gap-2">
			<span>Somethign went wrong gathering data.</span>
			<span>Unable to render scatter plot.</span>
		</div>
	{/if}
</ContentCard>
