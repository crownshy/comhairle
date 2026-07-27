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

	function extractAxisDomain(question: Question): [number, number] | null {
		if (question.type.continuous) {
			return [question.type.continuous.min_value, question.type.continuous.max_value];
		}

		if (question.type.likert_scale) {
			const values = question.type.likert_scale.categories.map(
				(category) => category.value
			) as number[];

			return [Math.min(...values), Math.max(...values)];
		}

		return null;
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
		<div class="flex justify-end">
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
			xAxisLabel={xQuestion.text}
			yAxisLabel={yQuestion.text}
			{xDomain}
			{yDomain}
			points={scatterPoints}
		/>
	{:else}
		<div class="flex flex-col items-center justify-center gap-2">
			<span>Somethign went wrong gathering data.</span>
			<span>Unable to render scatter plot.</span>
		</div>
	{/if}
</ContentCard>
