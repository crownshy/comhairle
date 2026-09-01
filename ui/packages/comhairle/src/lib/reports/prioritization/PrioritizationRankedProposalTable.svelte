<script lang="ts">
	import { slide } from 'svelte/transition';
	import * as Table from '$lib/components/ui/table';
	import { ChevronDown } from 'lucide-svelte';
	import type { RankedProposal } from '@crownshy/api-client/api';
	import Crown from '$lib/components/icons/Crown.svelte';
	import DivergingProgress from '../DivergingProgress.svelte';
	import type { ToolConfig } from '$lib/tools/prioritization';

	type Props = {
		proposals: RankedProposal[];
		toolConfig: ToolConfig<string>;
	};
	let { proposals, toolConfig }: Props = $props();

	const ROW_ANIMATION_DURATION_MS = 1000;
	let rowLimit = $state(4);
	let visibleRows = $derived(proposals.slice(0, rowLimit));
	let alignmentQuestion = $derived(
		toolConfig.questions.find((question) => question.id === toolConfig.alignmentQuestionId)
	);

	let progressDomain = $derived.by(() => {
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
	<Table.Root class="w-full table-fixed">
		<Table.Header>
			<Table.Row class="text-xs">
				<Table.Head class="text-muted-foreground w-[10%] px-5"
					><div class="flex justify-center">Ranking</div></Table.Head
				>
				<Table.Head class="text-muted-foreground w-2/5 px-5">Proposal</Table.Head>
				<Table.Head class="text-muted-foreground w-2/5 px-5">Alignment Score</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#each visibleRows as proposal, index (proposal.id)}
				<Table.Row
					class="hover:[&,&>svelte-css-wrapper]:[&>th,td]:bg-accent hover:cursor-pointer"
				>
					<Table.Cell class="text-muted-foreground border-l-6 px-5 py-9">
						<div
							transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}
							class="flex justify-center"
						>
							{#if index === 0}
								<Crown />
							{:else}
								{index + 1}
							{/if}
						</div>
					</Table.Cell>
					<Table.Cell class="text-foreground h-auto px-5 text-wrap!">
						<div transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}>
							{proposal.title}
						</div>
					</Table.Cell>
					<Table.Cell class="text-primary h-auto px-5 text-wrap!">
						<div
							transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}
							class="flex items-center gap-4"
						>
							<DivergingProgress
								value={proposal.alignmentRating}
								min={progressDomain?.[0]}
								max={progressDomain?.[1]}
							/>
						</div>
					</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>

	{#if rowLimit < proposals.length}
		<button
			class="bg-border flex w-full items-center justify-center gap-2 py-3 text-center"
			type="button"
			onclick={() => (rowLimit = proposals.length)}
			>See all {proposals.length} proposals <ChevronDown /></button
		>
	{/if}
{:else}
	<p>No alignment question selected. Please select in Setup page</p>
{/if}
