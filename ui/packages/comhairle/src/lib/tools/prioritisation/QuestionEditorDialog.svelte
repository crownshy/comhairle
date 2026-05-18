<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Select from '$lib/components/ui/select';
	import { Slider } from '$lib/components/ui/slider';
	import { Plus, Trash2 } from 'lucide-svelte';
	import type { PrioritisationStore } from './store.svelte';
	import {
		type ContinuousQuestion,
		type LikertScaleQuestion,
		type QuestionType
	} from './types';

	let {
		store,
		questionId,
		onClose
	}: {
		store: PrioritisationStore;
		questionId: string;
		onClose: () => void;
	} = $props();

	let q = $derived(store.poll.toolConfig.questions.find((x) => x.id === questionId));
	let open = $state(true);

	function handleClose(o: boolean) {
		if (!o) {
			open = false;
			onClose();
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={handleClose}>
	<Dialog.Content class="max-h-[90vh] max-w-xl min-w-[80vw] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Edit question {q?.order}</Dialog.Title>
			<Dialog.Description>
				This question is asked of every proposal in the poll.
			</Dialog.Description>
		</Dialog.Header>

		{#if q}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1">
					<Label for="q-prompt">Prompt</Label>
					<Input
						id="q-prompt"
						placeholder="Type a question"
						value={q.prompt}
						oninput={(e) =>
							store.updateQuestion(q.id, {
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
							store.updateQuestion(q.id, {
								description: (e.target as HTMLTextAreaElement).value
							})}
					/>
				</div>

				<div class="flex items-center gap-2">
					<Checkbox
						id="q-optional"
						checked={q.optional}
						onCheckedChange={(v) =>
							store.updateQuestion(q.id, { optional: v === true })}
					/>
					<Label for="q-optional">Optional</Label>
				</div>

				{#if q.type === 'likert_scale'}
					{@const lk = q as LikertScaleQuestion}
					<div class="flex flex-col gap-2">
						<Label>Categories</Label>
						<p class="text-muted-foreground text-xs">
							Each option has a numeric value (used for analysis) and a label (shown
							to participants).
						</p>
						{#each lk.categories as c, i (i)}
							<div class="flex items-center gap-2">
								<Input
									type="number"
									class="w-20"
									value={c.value}
									oninput={(e) =>
										store.updateLikertCategory(q.id, i, {
											value: Number((e.target as HTMLInputElement).value)
										})}
								/>
								<Input
									placeholder={`Option ${i + 1}`}
									value={c.label}
									oninput={(e) =>
										store.updateLikertCategory(q.id, i, {
											label: (e.target as HTMLInputElement).value
										})}
								/>
								<Button
									variant="ghost"
									size="icon"
									onclick={() => store.removeLikertCategory(q.id, i)}
									disabled={lk.categories.length <= 2}
									aria-label="Remove option"
								>
									<Trash2 class="size-4" />
								</Button>
							</div>
						{/each}
						<Button
							variant="outline"
							size="sm"
							onclick={() => store.addLikertCategory(q.id)}
						>
							<Plus class="mr-1 size-3.5" /> Add option
						</Button>
					</div>
				{/if}

				{#if q.type === 'continuous'}
					{@const ct = q as ContinuousQuestion}
					<div class="flex flex-col gap-3">
						<div class="flex items-center gap-2">
							<Label class="w-24">Range</Label>
							<Input
								type="number"
								class="w-24"
								value={ct.minValue}
								oninput={(e) =>
									store.updateQuestion(q.id, {
										minValue: Number((e.target as HTMLInputElement).value) || 0
									})}
							/>
							<span class="text-muted-foreground text-sm">to</span>
							<Input
								type="number"
								class="w-24"
								value={ct.maxValue}
								oninput={(e) =>
									store.updateQuestion(q.id, {
										maxValue: Number((e.target as HTMLInputElement).value) || 0
									})}
							/>
						</div>
						<div class="flex items-center gap-2">
							<Label class="w-24">End labels</Label>
							<Input
								placeholder="e.g. No support"
								value={ct.minLabel}
								oninput={(e) =>
									store.updateQuestion(q.id, {
										minLabel: (e.target as HTMLInputElement).value
									})}
							/>
							<span class="text-muted-foreground text-sm">·</span>
							<Input
								placeholder="e.g. Full support"
								value={ct.maxLabel}
								oninput={(e) =>
									store.updateQuestion(q.id, {
										maxLabel: (e.target as HTMLInputElement).value
									})}
							/>
						</div>
						<div class="flex items-center gap-2">
							<Label class="w-24">Preview</Label>
							<span class="text-muted-foreground text-xs">
								{ct.minLabel || ct.minValue}
							</span>
							<div class="flex-1">
								<Slider
									type="single"
									value={ct.minValue + (ct.maxValue - ct.minValue) / 2}
									min={Math.min(ct.minValue, ct.maxValue)}
									max={Math.max(ct.minValue, ct.maxValue)}
									step={1}
									disabled
								/>
							</div>
							<span class="text-muted-foreground text-xs">
								{ct.maxLabel || ct.maxValue}
							</span>
						</div>
					</div>
				{/if}
			</div>
		{/if}

		<Dialog.Footer>
			<Button variant="outline" onclick={() => store.duplicateQuestion(questionId)}>
				Duplicate
			</Button>
			<Button onclick={() => handleClose(false)}>Done</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
