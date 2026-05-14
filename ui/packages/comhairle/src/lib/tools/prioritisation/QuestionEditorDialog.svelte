<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Select from '$lib/components/ui/select';
	import { Plus, Trash2 } from 'lucide-svelte';
	import type { PrioritisationStore } from './store.svelte';
	import {
		QUESTION_TYPE_LABELS,
		letterFor,
		type QuestionType,
		type MultipleChoiceQuestion,
		type RatingScaleQuestion
	} from './types';

	let {
		store,
		proposalId,
		questionId,
		onClose
	}: {
		store: PrioritisationStore;
		proposalId: string;
		questionId: string;
		onClose: () => void;
	} = $props();

	let proposal = $derived(store.poll.proposals.find((p) => p.id === proposalId));
	let q = $derived(proposal?.questions.find((x) => x.id === questionId));
	let open = $state(true);

	function handleClose(o: boolean) {
		if (!o) {
			open = false;
			onClose();
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={handleClose}>
	<Dialog.Content class="max-h-[90vh] max-w-xl overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Edit question {q?.order}</Dialog.Title>
			<Dialog.Description>
				Question for proposal: {proposal?.title || `#${proposal?.order}`}
			</Dialog.Description>
		</Dialog.Header>

		{#if q}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1">
					<Label>Type</Label>
					<Select.Root
						type="single"
						value={q.type}
						onValueChange={(v: string) => {
							store.updateQuestion(proposalId, q.id, { type: v as QuestionType });
						}}
					>
						<Select.Trigger class="w-[260px]"
							>{QUESTION_TYPE_LABELS[q.type]}</Select.Trigger
						>
						<Select.Content>
							{#each Object.entries(QUESTION_TYPE_LABELS) as [t, label] (t)}
								<Select.Item value={t}>{label}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>

				<div class="flex flex-col gap-1">
					<Label for="q-prompt">Prompt</Label>
					<Input
						id="q-prompt"
						placeholder="Type a question"
						value={q.prompt}
						oninput={(e) =>
							store.updateQuestion(proposalId, q.id, {
								prompt: (e.target as HTMLInputElement).value
							})}
					/>
				</div>

				<div class="flex flex-col gap-1">
					<Label for="q-desc">Description (optional)</Label>
					<Textarea
						id="q-desc"
						placeholder="Add a description"
						value={q.description ?? ''}
						oninput={(e) =>
							store.updateQuestion(proposalId, q.id, {
								description: (e.target as HTMLTextAreaElement).value
							})}
					/>
				</div>

				<div class="flex items-center gap-2">
					<Checkbox
						id="q-optional"
						checked={q.optional}
						onCheckedChange={(v) =>
							store.updateQuestion(proposalId, q.id, { optional: v === true })}
					/>
					<Label for="q-optional">Optional</Label>
				</div>

				{#if q.type === 'multiple_choice'}
					{@const mc = q as MultipleChoiceQuestion}
					<div class="flex flex-col gap-2">
						<Label>Choices</Label>
						{#each mc.choices as c, i (c.id)}
							<div class="flex items-center gap-2">
								<span
									class="bg-muted flex size-7 items-center justify-center rounded text-xs font-semibold"
									>{letterFor(i)}</span
								>
								<Input
									placeholder={`Choice ${i + 1}`}
									value={c.label}
									oninput={(e) =>
										store.updateChoice(
											proposalId,
											q.id,
											c.id,
											(e.target as HTMLInputElement).value
										)}
								/>
								<Button
									variant="ghost"
									size="icon"
									onclick={() => store.removeChoice(proposalId, q.id, c.id)}
									disabled={mc.choices.length <= 2}
								>
									<Trash2 class="size-4" />
								</Button>
							</div>
						{/each}
						<Button
							variant="outline"
							size="sm"
							onclick={() => store.addChoice(proposalId, q.id)}
						>
							<Plus class="mr-1 size-3.5" /> Add choice
						</Button>
					</div>
				{/if}

				{#if q.type === 'rating_scale'}
					{@const rs = q as RatingScaleQuestion}
					<div class="grid grid-cols-2 gap-3">
						<div class="flex flex-col gap-1">
							<Label>Min</Label>
							<Input
								type="number"
								value={rs.min}
								oninput={(e) =>
									store.updateQuestion(proposalId, q.id, {
										min: Number((e.target as HTMLInputElement).value)
									})}
							/>
						</div>
						<div class="flex flex-col gap-1">
							<Label>Max</Label>
							<Input
								type="number"
								value={rs.max}
								oninput={(e) =>
									store.updateQuestion(proposalId, q.id, {
										max: Number((e.target as HTMLInputElement).value)
									})}
							/>
						</div>
						<div class="flex flex-col gap-1">
							<Label>Min label</Label>
							<Input
								value={rs.minLabel}
								oninput={(e) =>
									store.updateQuestion(proposalId, q.id, {
										minLabel: (e.target as HTMLInputElement).value
									})}
							/>
						</div>
						<div class="flex flex-col gap-1">
							<Label>Max label</Label>
							<Input
								value={rs.maxLabel}
								oninput={(e) =>
									store.updateQuestion(proposalId, q.id, {
										maxLabel: (e.target as HTMLInputElement).value
									})}
							/>
						</div>
					</div>
				{/if}
			</div>
		{/if}

		<Dialog.Footer>
			<Button
				variant="outline"
				onclick={() => store.duplicateQuestion(proposalId, questionId)}
			>
				Duplicate
			</Button>
			<Button onclick={() => handleClose(false)}>Done</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
