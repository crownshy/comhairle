<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Plus, GripVertical, Trash2, Copy } from 'lucide-svelte';
	import { Sortable } from '$lib/components/Sortable';
	import type { PrioritizationStore } from './store.svelte';
	import ProposalEditorDialog from './ProposalEditorDialog.svelte';
	import QuestionEditorDialog from './QuestionEditorDialog.svelte';
	import { QUESTION_TYPE_LABELS, type QuestionType } from './types';

	let { store }: { store: PrioritizationStore } = $props();

	let editingProposalId = $state<string | null>(null);
	let editingQuestionId = $state<string | null>(null);

	let proposals = $derived(store.poll.proposals);
	let questions = $derived(store.poll.toolConfig.questions);
	let randomize = $derived(store.poll.toolConfig.randomizeOrder);

	function addQuestion(type: QuestionType) {
		const q = store.addQuestion(type);
		editingQuestionId = q.id;
	}
</script>

<div class="flex flex-col gap-6">
	<!-- Title / description -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Poll details</Card.Title>
			<Card.Description>
				What is this poll about? Participants see the title and description before
				answering.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<Label for="poll-title">Title</Label>
				<Input
					id="poll-title"
					placeholder="Enter poll title"
					value={store.poll.title}
					oninput={(e) => store.setTitle((e.target as HTMLInputElement).value)}
				/>
			</div>

			<div class="flex flex-col gap-1">
				<Label for="poll-description">Description</Label>
				<Textarea
					id="poll-description"
					placeholder="Add a short description for participants"
					value={store.poll.description}
					oninput={(e) => store.setDescription((e.target as HTMLTextAreaElement).value)}
				/>
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Questions -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Questions</Card.Title>
			<Card.Description>
				These questions are asked once per proposal. Define them here, then add the
				proposals below.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			{#if questions.length === 0}
				<p class="text-muted-foreground text-sm">
					No questions yet. Pick a type below to add one.
				</p>
			{/if}

			<Sortable
				items={questions}
				onReorder={(ids) => store.reorderQuestions(ids)}
				class="flex flex-col gap-2"
			>
				{#snippet item({ item: q })}
					<div
						class="bg-card hover:bg-muted/50 flex items-center gap-2 rounded-md border p-3"
					>
						<span
							class="text-muted-foreground cursor-grab"
							aria-label="Drag to reorder"
						>
							<GripVertical class="size-4" />
						</span>
						<span class="text-muted-foreground w-12 text-xs">
							Q{q.order}{q.optional ? '*' : ''}
						</span>
						<button
							class="flex-1 truncate text-left text-sm hover:underline"
							onclick={() => (editingQuestionId = q.id)}
						>
							{q.prompt || 'Untitled question'}
						</button>
						<span class="text-muted-foreground text-xs">
							{QUESTION_TYPE_LABELS[q.type]}
						</span>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => store.duplicateQuestion(q.id)}
							aria-label="Duplicate question"
						>
							<Copy class="size-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => store.removeQuestion(q.id)}
							aria-label="Remove question"
						>
							<Trash2 class="size-4" />
						</Button>
					</div>
				{/snippet}
			</Sortable>

			<div class="flex flex-wrap gap-2 pt-2">
				{#each Object.entries(QUESTION_TYPE_LABELS) as [type, label] (type)}
					<Button
						variant="outline"
						size="sm"
						onclick={() => addQuestion(type as QuestionType)}
					>
						+ {label}
					</Button>
				{/each}
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Proposals -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Proposals</Card.Title>
			<Card.Description>
				Add the ideas participants will rate. Each proposal will be shown with the questions
				defined above.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			<div class="flex items-center justify-between rounded-md border p-3">
				<div>
					<Label for="randomize" class="font-medium">Random order</Label>
					<p class="text-muted-foreground text-xs">
						Show proposals to each participant in a different random order.
					</p>
				</div>
				<Switch
					id="randomize"
					checked={randomize}
					onCheckedChange={(v) => store.setRandomizeOrder(v === true)}
				/>
			</div>

			{#if proposals.length === 0}
				<p class="text-muted-foreground text-sm">
					No proposals yet. Add at least two proposals.
				</p>
			{/if}

			<Sortable
				items={proposals}
				onReorder={(ids) => store.reorderProposals(ids)}
				class="flex flex-col gap-2"
			>
				{#snippet item({ item: p })}
					<div class="bg-card flex items-center gap-2 rounded-md border p-3">
						<span
							class="text-muted-foreground cursor-grab"
							aria-label="Drag to reorder"
						>
							<GripVertical class="size-4" />
						</span>
						<span class="text-muted-foreground w-20 text-xs">Proposal {p.order}</span>
						<button
							class="flex-1 truncate text-left text-sm hover:underline"
							onclick={() => (editingProposalId = p.id)}
						>
							{p.title || 'Untitled proposal'}
						</button>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => store.duplicateProposal(p.id)}
							aria-label="Duplicate proposal"
						>
							<Copy class="size-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => store.removeProposal(p.id)}
							aria-label="Remove proposal"
						>
							<Trash2 class="size-4" />
						</Button>
					</div>
				{/snippet}
			</Sortable>

			<Button
				variant="outline"
				onclick={() => {
					const p = store.addProposal();
					editingProposalId = p.id;
				}}
			>
				<Plus class="mr-1 size-4" /> Add proposal
			</Button>
		</Card.Content>
	</Card.Root>
</div>

{#if editingProposalId}
	<ProposalEditorDialog
		{store}
		proposalId={editingProposalId}
		onClose={() => (editingProposalId = null)}
	/>
{/if}

{#if editingQuestionId}
	<QuestionEditorDialog
		{store}
		questionId={editingQuestionId}
		onClose={() => (editingQuestionId = null)}
	/>
{/if}
