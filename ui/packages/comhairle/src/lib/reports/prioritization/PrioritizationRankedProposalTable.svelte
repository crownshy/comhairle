<script lang="ts">
	import type { RankedProposal } from '@crownshy/api-client/api';
	import type { ToolConfig } from '$lib/tools/prioritization';
	import PrioritizationRankingTable from './PrioritizationRankingTable.svelte';

	type Props = {
		proposals: RankedProposal[];
		toolConfig: ToolConfig<string>;
	};
	let { proposals, toolConfig }: Props = $props();

	let alignmentQuestion = $derived(
		toolConfig.questions.find((question) => question.id === toolConfig.alignmentQuestionId)
	);

	let progressDomain = $derived.by((): [number, number] => {
		if (!alignmentQuestion) return [0, 0];

		switch (alignmentQuestion.type.kind) {
			case 'likert': {
				const values = alignmentQuestion.type.categories.map((cat) => cat.value);
				return [
					Math.min(...values) * proposals.length,
					Math.max(...values) * proposals.length
				];
			}
			case 'continuous': {
				return [
					alignmentQuestion.type.minValue * proposals.length,
					alignmentQuestion.type.maxValue * proposals.length
				];
			}
			default: {
				return [0, 0];
			}
		}
	});
</script>

{#if alignmentQuestion}
	<PrioritizationRankingTable
		rows={proposals.map((proposal) => ({
			key: proposal.id,
			title: proposal.title,
			alignment: proposal.alignmentRating
		}))}
		domain={progressDomain}
		scoreHeading="Alignment Score"
	/>
{:else}
	<p>No alignment question selected. Please select in Setup page</p>
{/if}
