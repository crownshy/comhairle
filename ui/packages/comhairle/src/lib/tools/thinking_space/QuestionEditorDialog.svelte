<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { HelpCircle } from 'lucide-svelte';
	import type { QuestionConfig } from './types';

	type Props = {
		open: boolean;
		question?: QuestionConfig | null;
		onOpenChange: (open: boolean) => void;
		onSave: (question: QuestionConfig) => void;
	};

	let { open, question = null, onOpenChange, onSave }: Props = $props();

	let draftText = $state('');
	let draftIntent = $state('');
	let textError = $state<string | null>(null);
	let intentError = $state<string | null>(null);

	const isEditing = $derived(!!question);

	$effect(() => {
		if (open) {
			draftText = question?.text ?? '';
			draftIntent = question?.intent ?? '';
			textError = null;
			intentError = null;
		}
	});

	function save() {
		const text = draftText.trim();
		const intent = draftIntent.trim();
		textError = text ? null : 'Question text is required.';
		intentError = intent ? null : 'Intent is required so the AI can generate good follow-ups.';
		if (textError || intentError) return;
		onSave({ id: question?.id ?? crypto.randomUUID(), text, intent });
		onOpenChange(false);
	}
</script>

<Dialog.Root {open} onOpenChange={(o) => onOpenChange(o)}>
	<Dialog.Content class="min-w-[95vw] md:min-w-[70vw] xl:min-w-[1000px]">
		<Dialog.Header>
			<Dialog.Title>{isEditing ? 'Edit question' : 'New question'}</Dialog.Title>
			<Dialog.Description>
				Participants answer this as one of the main reflection prompts.
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-5 py-2">
			<div class="space-y-2">
				<Label for="ts-question-text">Question</Label>
				<Textarea
					id="ts-question-text"
					bind:value={draftText}
					placeholder="e.g. How financially supported do you feel as a farmer in Scotland today?"
					rows={3}
				/>
				{#if textError}
					<p class="text-destructive text-sm">{textError}</p>
				{/if}
			</div>

			<div class="space-y-2">
				<div class="flex items-center gap-1.5">
					<Label for="ts-question-intent">Why are you asking this? (for the AI)</Label>
					<Tooltip.Provider delayDuration={150}>
						<Tooltip.Root>
							<Tooltip.Trigger
								class="text-muted-foreground hover:text-foreground"
								aria-label="What is this for?"
							>
								<HelpCircle class="size-4" />
							</Tooltip.Trigger>
							<Tooltip.Content class="max-w-xs text-sm">
								This is private. Participants never see it. It guides the AI's
								follow-up questions, so the more specific you are about what you
								want to learn, the sharper the follow-ups will be.
							</Tooltip.Content>
						</Tooltip.Root>
					</Tooltip.Provider>
				</div>
				<Textarea
					id="ts-question-intent"
					bind:value={draftIntent}
					placeholder={'e.g. I want to understand whether they feel financially supported, not just whether they’re farming successfully. I’m especially interested in subsidies, market access, and whether the next generation can afford to take over.'}
					rows={4}
				/>
				{#if intentError}
					<p class="text-destructive text-sm">{intentError}</p>
				{/if}
			</div>
		</div>

		<Dialog.Footer>
			<Button variant="outline" onclick={() => onOpenChange(false)}>Cancel</Button>
			<Button onclick={save}>{isEditing ? 'Save question' : 'Create question'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
