<script lang="ts">
	import type { ProposalResponseDto, Question, RankedProposal } from '@crownshy/api-client/api';
	import ContentCard from '../ContentCard.svelte';
	import ScatterPlot, { type ScatterPoint } from '../ScatterPlot.svelte';
	import * as Select from '$lib/components/ui/select';

	type Props = {
		proposals: RankedProposal[];
		toolConfig: { alignment_question_id: string; questions: Question[] };
	};

	let { proposals, toolConfig }: Props = $props();

	let selectedProposal = $state(proposals[0]);

	const xQuestion = $derived(
		toolConfig.questions.find((question) => question.id === toolConfig.alignment_question_id)
	);
	const yQuestion = $derived(
		toolConfig.questions.find((question) => question.id !== toolConfig.alignment_question_id)
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
		extractScatterPoints(selectedProposal.responses, xQuestion.id, yQuestion.id)
	);
</script>

<ContentCard>
	<div class="mb-10">
		<h2 class="text-lg font-bold">Alignent and importance</h2>
		<p class="text-muted-foreground text-sm">
			See the distribution chart showing the level of consensus or disagreement for each
			participant or proposal.
		</p>
	</div>

	<div class="flex justify-end">
		<Select.Root
			type="single"
			onValueChange={(v) => (selectedProposal = proposals.find((p) => p.id === v))}
		>
			<Select.Trigger>{selectedProposal.title}</Select.Trigger>
			<Select.Content>
				{#each proposals as proposal (proposal.id)}
					<Select.Item value={proposal.id}>{proposal.title}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>

	<ScatterPlot xAxisLabel={xQuestion.text} yAxisLabel={yQuestion.text} points={scatterPoints} />
</ContentCard>
