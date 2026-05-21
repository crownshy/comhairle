<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import type { QuestionConfig } from './types';

	type Props = {
		open: boolean;
		question?: QuestionConfig | null;
		onOpenChange: (open: boolean) => void;
		onSave: (question: QuestionConfig) => void;
	};

	let { open, question = null, onOpenChange, onSave }: Props = $props();

	let draftText = $state('');
	let errorMessage = $state<string | null>(null);

	const isEditing = $derived(!!question);

	$effect(() => {
		if (open) {
			draftText = question?.text ?? '';
			errorMessage = null;
		}
	});

	function save() {
		const text = draftText.trim();
		if (!text) {
			errorMessage = 'Question text is required.';
			return;
		}
		onSave({ id: question?.id ?? crypto.randomUUID(), text });
		onOpenChange(false);
	}
</script>

<Dialog.Root {open} onOpenChange={(o) => onOpenChange(o)}>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>{isEditing ? 'Edit question' : 'New question'}</Dialog.Title>
			<Dialog.Description>
				Participants answer this as one of the main reflection prompts.
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-2 py-2">
			<Label for="ts-question-text">Question</Label>
			<Textarea
				id="ts-question-text"
				bind:value={draftText}
				placeholder="Write your question…"
				rows={3}
			/>
			{#if errorMessage}
				<p class="text-destructive text-sm">{errorMessage}</p>
			{/if}
		</div>

		<Dialog.Footer>
			<Button variant="outline" onclick={() => onOpenChange(false)}>Cancel</Button>
			<Button onclick={save}>{isEditing ? 'Save question' : 'Create question'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
