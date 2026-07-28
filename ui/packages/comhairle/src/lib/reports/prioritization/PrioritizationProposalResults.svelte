<script lang="ts">
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Dialog from '$lib/components/ui/dialog';
	import { RankedProposal, type Question, type ToolConfig } from '@crownshy/api-client/api';
	import ContentCard from '../ContentCard.svelte';
	import { cn } from '$lib/utils';
	import BarChart from '../BarChart.svelte';
	import KdePlot from '../KdePlot.svelte';
	import Button, { buttonVariants } from '$lib/components/ui/button/button.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';

	type Props = {
		proposals: RankedProposal[];
		toolConfig: ToolConfig;
	};

	let { proposals, toolConfig }: Props = $props();

	function extractBarChartData(question: Question, proposal: RankedProposal) {
		if (!question.type.likert_scale) return;

		const data = question.type.likert_scale.categories.map((category) => ({
			category: category.label,
			count: extractQuestionResponses(proposal, question.id).filter(
				(response) => response.value === category.value
			).length
		}));

		return data;
	}

	const chartConfig = {
		categories: { label: 'Categories', color: 'var(--primary)' }
	} satisfies Chart.ChartConfig;

	function extractQuestionResponses(
		proposal: RankedProposal,
		questionId: string
	): { value: string | number; question_id: string; section_id?: string | null }[] {
		const data = proposal.responses
			.map((response) => response.response.filter((r) => r.question_id === questionId))
			.flat();

		return data;
	}

	let selectedProposal = $state<RankedProposal | null>(null);
	let openDialog = $state(false);
</script>

<ContentCard>
	<div class="mb-10">
		<h2 class="text-lg font-bold">Prioritisation results</h2>
		<p class="text-muted-foreground text-sm">
			See the result for each question of each proposal.
		</p>
	</div>

	<div class="p-4">
		{#each proposals as proposal, index (proposal.id)}
			<article
				class={cn(
					'flex flex-col gap-10 px-6 py-10',
					index !== proposals.length - 1 && 'border-b'
				)}
			>
				<div class="mb-5 flex items-center justify-between">
					<h3 class="text-lg font-bold">
						Proposal {index + 1}: {proposal.title}
					</h3>
					<Button
						variant="outline"
						onclick={() => {
							selectedProposal = proposal;
							openDialog = true;
						}}>View Proposal</Button
					>
				</div>
				{#each toolConfig.questions as question (question.id)}
					<div class="flex flex-col gap-4">
						<div>
							<h4 class="mb-2 font-bold">{question.text}</h4>
							<p class="text-muted-foreground text-xs">
								{proposal.responses.reduce(
									(acc, res) =>
										res.response.filter((r) => r.question_id === question.id)
											.length + acc,
									0
								)} responses
							</p>
						</div>
						{#if question.type.likert_scale}
							<BarChart
								data={extractBarChartData(question, proposal)}
								xKey="category"
								yKey="count"
								{chartConfig}
							/>
						{/if}
						{#if question.type.continuous}
							<KdePlot
								minLabel={question.type.continuous.min_label}
								maxLabel={question.type.continuous.max_label}
								category={question.text}
								rawData={{
									[question.text]: extractQuestionResponses(
										proposal,
										question.id
									).map((entry) => entry.value)
								}}
								maxX={question.type.continuous.max_value}
							/>
						{/if}
						{#if question.type === 'text'}
							{@const textResponses = extractQuestionResponses(proposal, question.id)}
							<div>
								{#each textResponses as res, index (index)}
									<p
										class={cn(
											'py-3',
											index !== textResponses.length - 1 && 'border-b'
										)}
									>
										{res.value}
									</p>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</article>
		{/each}
	</div>
</ContentCard>

<Dialog.Root bind:open={openDialog}>
	<Dialog.Content
		class="flex h-auto max-h-[80vh] min-h-[30vh] w-full max-w-6xl flex-col gap-8 sm:w-full sm:max-w-6xl"
	>
		<Dialog.Header>
			<span class="bg-primary text-primary-foreground w-fit rounded-full px-2 py-0.5 text-xs"
				>Proposal</span
			>
			<Dialog.Title class="text-lg">{selectedProposal?.title}</Dialog.Title>
		</Dialog.Header>
		<ContentCard class="overflow-y-auto">
			{#each selectedProposal?.sections as section (section.id)}
				<ContentRenderer content={section.body} />
			{/each}
		</ContentCard>
		<Dialog.Close
			class={cn(buttonVariants({ variant: 'default', size: 'default' }), 'mt-auto self-end')}
			onclick={() => {
				selectedProposal = null;
			}}>Close</Dialog.Close
		>
	</Dialog.Content>
</Dialog.Root>
