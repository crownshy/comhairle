<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { Plus, Trash2 } from 'lucide-svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import type { PrioritisationStore } from './store.svelte';
	import { QUESTION_TYPE_LABELS, type QuestionType } from './types';
	import QuestionEditorDialog from './QuestionEditorDialog.svelte';

	let {
		store,
		proposalId,
		onClose
	}: {
		store: PrioritisationStore;
		proposalId: string;
		onClose: () => void;
	} = $props();

	let proposal = $derived(store.poll.proposals.find((p) => p.id === proposalId));
	let open = $state(true);
	let editingQuestionId = $state<string | null>(null);

	function handleClose(o: boolean) {
		if (!o) {
			open = false;
			onClose();
		}
	}

	function onImage(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;
		const reader = new FileReader();
		reader.onload = () => {
			store.updateProposal(proposalId, { imageDataUrl: String(reader.result) });
		};
		reader.readAsDataURL(file);
	}

	function addQuestion(type: QuestionType) {
		const q = store.addQuestion(proposalId, type);
		if (q) editingQuestionId = q.id;
	}
</script>

<Dialog.Root bind:open onOpenChange={handleClose}>
	<Dialog.Content class="max-h-[90vh] max-w-3xl overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Edit proposal {proposal?.order}</Dialog.Title>
			<Dialog.Description>
				Set the proposal's content and the questions participants will be asked about it.
			</Dialog.Description>
		</Dialog.Header>

		{#if proposal}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1">
					<Label for="prop-title">Proposal title</Label>
					<Input
						id="prop-title"
						placeholder="Enter proposal title or headline"
						value={proposal.title}
						oninput={(e) => {
							const target = e.target as HTMLInputElement;
							store.updateProposal(proposalId, { title: target.value });
						}}
					/>
				</div>

				<div class="flex flex-col gap-1">
					<Label for="prop-image">Header image (optional)</Label>
					<input
						id="prop-image"
						type="file"
						accept="image/*"
						onchange={onImage}
						class="text-sm"
					/>
					{#if proposal.imageDataUrl}
						<img
							src={proposal.imageDataUrl}
							alt="Proposal header"
							class="mt-2 max-h-40 rounded-md object-cover"
						/>
						<Button
							variant="ghost"
							size="sm"
							onclick={() =>
								store.updateProposal(proposalId, { imageDataUrl: undefined })}
						>
							Remove image
						</Button>
					{/if}
				</div>

				<div class="flex flex-col gap-1">
					<Label>Proposal content</Label>
					<RichTextEditor
						value={proposal.content || null}
						placeholder="Enter proposal content"
						minHeight="200px"
						onChange={(json) => store.updateProposal(proposalId, { content: json })}
					/>
				</div>

				<!-- Per-proposal questions -->
				<div class="flex flex-col gap-2 border-t pt-4">
					<Label>Questions for this proposal</Label>
					<p class="text-muted-foreground text-xs">
						Each proposal has its own questions. Cross-proposal ranking is intentionally
						not supported in the prototype.
					</p>
					{#each proposal.questions as q (q.id)}
						<div class="flex items-center gap-2 rounded-md border p-2">
							<span class="text-muted-foreground w-12 text-xs"
								>Q{q.order}{q.optional ? '*' : ''}</span
							>
							<button
								class="flex-1 truncate text-left text-sm hover:underline"
								onclick={() => (editingQuestionId = q.id)}
							>
								{q.prompt || 'Untitled question'}
							</button>
							<span class="text-muted-foreground text-xs"
								>{QUESTION_TYPE_LABELS[q.type]}</span
							>
							<Button
								variant="ghost"
								size="icon"
								onclick={() => store.removeQuestion(proposalId, q.id)}
								aria-label="Remove question"
							>
								<Trash2 class="size-4" />
							</Button>
						</div>
					{/each}
					{#if proposal.questions.length === 0}
						<p class="text-muted-foreground text-xs">
							Add at least one question. Pick a type to get started.
						</p>
					{/if}
					<div class="flex flex-wrap gap-2">
						{#each Object.entries(QUESTION_TYPE_LABELS) as [type, label] (type)}
							<Button
								variant="outline"
								size="sm"
								onclick={() => addQuestion(type as QuestionType)}
							>
								<Plus class="mr-1 size-3.5" />
								{label}
							</Button>
						{/each}
					</div>
				</div>
			</div>
		{/if}

		<Dialog.Footer>
			<Button onclick={() => handleClose(false)}>Done</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

{#if editingQuestionId}
	<QuestionEditorDialog
		{store}
		{proposalId}
		questionId={editingQuestionId}
		onClose={() => (editingQuestionId = null)}
	/>
{/if}
