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

	function extractBarChartData(question: Question, proposal: RankedProposal, section?: unknown) {
		if (!question.type.likert_scale) return;

		const data = question.type.likert_scale.categories.map((category) => ({
			category: category.label,
			count: extractQuestionResponses(proposal, question.id, section?.id).filter(
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
		questionId: string,
		sectionId?: string
	): { value: string | number; question_id: string; section_id?: string | null }[] {
		const data = proposal.responses
			.map((response) =>
				response.response.filter((r) => {
					if (sectionId) {
						return r.section_id === sectionId && r.question_id === questionId;
					}
					return r.question_id === questionId;
				})
			)
			.flat();

		return data;
	}

	function countResponses(
		question: Question,
		proposal: RankedProposal,
		section?: unknown // TODO:
	) {
		const count = proposal.responses.reduce(
			(acc, res) =>
				res.response.filter((r) => {
					if (section) {
						return r.section_id === section.id && r.question_id === question.id;
					} else {
						return r.question_id === question.id;
					}
				}).length + acc,
			0
		);

		return count;
	}

	let selectedProposalOrSection = $state<{
		proposal: RankedProposal | null;
		section: any | null;
	}>({ proposal: null, section: null });
	let openDialog = $state(false);

	$effect(() => {
		if (!openDialog) {
			selectedProposalOrSection = { proposal: null, section: null };
		}
	});

	let showProposalSections = $state<{ [proposalId: string]: boolean }>(
		Object.fromEntries(proposals.map((proposal) => [proposal.id, false])) // TODO: reset to false
	);
</script>

{#snippet sectionToggleButton(label: string, value: boolean, proposalId: string)}
	<button
		type="button"
		onclick={() => (showProposalSections[proposalId] = value)}
		class={cn(
			'rounded-lg px-4 py-1',
			showProposalSections[proposalId] === value && 'bg-background shadow-xl'
		)}>{label}</button
	>
{/snippet}

{#snippet questionType(question: Question, proposal: RankedProposal, section?: unknown)}
	{#if question.type.likert_scale}
		<BarChart
			data={extractBarChartData(question, proposal, section)}
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
				[question.text]: extractQuestionResponses(proposal, question.id, section?.id).map(
					(entry) => entry.value
				)
			}}
			maxX={question.type.continuous.max_value}
		/>
	{/if}
	{#if question.type === 'text'}
		{@const textResponses = extractQuestionResponses(proposal, question.id, section?.id)}
		<div>
			{#each textResponses as res, index (index)}
				<p class={cn('py-3', index !== textResponses.length - 1 && 'border-b')}>
					{res.value}
				</p>
			{/each}
		</div>
	{/if}
{/snippet}

{#snippet questionHeader(question: Question, proposal: RankedProposal, section?: unknown)}
	<div>
		<h4 class="mb-2 font-bold">{question.text}</h4>
		<p class="text-muted-foreground text-xs">
			{countResponses(question, proposal, section)} responses
		</p>
	</div>
{/snippet}

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
							selectedProposalOrSection.proposal = proposal;
							openDialog = true;
						}}>View Proposal</Button
					>
				</div>
				{#if proposal.sections.length > 1 && toolConfig.section_questions.length > 0}
					<div class="bg-muted mb-4 flex w-fit flex-row gap-2 rounded-xl px-2 py-1">
						{@render sectionToggleButton('General', false, proposal.id)}
						{@render sectionToggleButton('Per-section', true, proposal.id)}
					</div>
				{/if}
				{#if showProposalSections[proposal.id]}
					{#each proposal.sections as section, index (section.id)}
						<ContentCard class="flex flex-col gap-6 border-l-6">
							<div class="flex items-center justify-between">
								<h5 class="font-bold">Section {index + 1}</h5>
								<Button
									variant="outline"
									onclick={() => {
										selectedProposalOrSection.section = section;
										openDialog = true;
									}}>View Section</Button
								>
							</div>
							<ContentRenderer class="font-bold" content={section.body} />

							{#each toolConfig.section_questions as sectionQuestion (sectionQuestion.id)}
								{@render questionHeader(sectionQuestion, proposal, section)}
								{@render questionType(sectionQuestion, proposal, section)}
							{/each}
						</ContentCard>
					{/each}
				{:else}
					{#each toolConfig.questions as question (question.id)}
						<div class="flex flex-col gap-4">
							{@render questionHeader(question, proposal)}
							{@render questionType(question, proposal)}
						</div>
					{/each}
				{/if}
			</article>
		{/each}
	</div>
</ContentCard>

<Dialog.Root bind:open={openDialog}>
	<Dialog.Content
		class="flex h-auto max-h-[80vh] w-full max-w-6xl flex-col gap-8 sm:w-full sm:max-w-6xl"
	>
		<Dialog.Header>
			<span class="bg-primary text-primary-foreground w-fit rounded-full px-2 py-0.5 text-xs"
				>{#if selectedProposalOrSection.proposal}Proposal{:else}Section{/if}</span
			>
			<Dialog.Title class="text-lg">{selectedProposalOrSection?.proposal?.title}</Dialog.Title
			>
		</Dialog.Header>
		<ContentCard class="overflow-y-auto">
			{#if selectedProposalOrSection.proposal}
				{#each selectedProposalOrSection.proposal.sections as section (section.id)}
					<ContentRenderer content={section.body} />
				{/each}
			{/if}
			{#if selectedProposalOrSection.section}
				<ContentRenderer content={selectedProposalOrSection.section.body} />
			{/if}
		</ContentCard>
		<Dialog.Close
			class={cn(buttonVariants({ variant: 'default', size: 'default' }), 'mt-auto self-end')}
			onclick={() => {
				selectedProposalOrSection = { proposal: null, section: null };
			}}>Close</Dialog.Close
		>
	</Dialog.Content>
</Dialog.Root>
