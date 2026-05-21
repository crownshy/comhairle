<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { LoaderCircle, Plus, Trash2 } from 'lucide-svelte';
	import QuestionField from './QuestionField.svelte';
	import type { PrioritizationStore } from '../store.svelte';
	import type { LikertCategory, Question, QuestionType, ToolConfig } from '../types';

	type Props = {
		open: boolean;
		question?: Question | null;
		store: PrioritizationStore;
		toolConfig: ToolConfig;
		onOpenChange: (open: boolean) => void;
	};

	let { open, question = null, store, toolConfig, onOpenChange }: Props = $props();

	const defaultLikertCategories: LikertCategory[] = [
		{ label: 'Strongly disagree', value: 1 },
		{ label: 'Disagree', value: 2 },
		{ label: 'Neutral', value: 3 },
		{ label: 'Agree', value: 4 },
		{ label: 'Strongly agree', value: 5 }
	];

	function newQuestion(): Question {
		return {
			id: crypto.randomUUID(),
			text: '',
			type: { kind: 'likert', categories: defaultLikertCategories.map((c) => ({ ...c })) }
		};
	}

	function cloneType(t: QuestionType): QuestionType {
		if (t.kind === 'likert')
			return { kind: 'likert', categories: t.categories.map((c) => ({ ...c })) };
		if (t.kind === 'continuous')
			return {
				kind: 'continuous',
				subSteps: t.subSteps,
				minValue: t.minValue,
				maxValue: t.maxValue,
				minLabel: t.minLabel,
				maxLabel: t.maxLabel
			};
		return { kind: 'text' };
	}

	function cloneQuestion(q: Question): Question {
		return { id: q.id, text: q.text, type: cloneType(q.type) };
	}

	let draft = $state<Question>(newQuestion());
	let saving = $state(false);
	let errorMessage = $state<string | null>(null);

	const isEditing = $derived(!!question);

	$effect(() => {
		if (open) {
			draft = question ? cloneQuestion(question) : newQuestion();
			errorMessage = null;
		}
	});

	function setKind(kind: QuestionType['kind']) {
		if (kind === draft.type.kind) return;
		if (kind === 'likert') {
			draft.type = {
				kind: 'likert',
				categories: defaultLikertCategories.map((c) => ({ ...c }))
			};
		} else if (kind === 'continuous') {
			draft.type = {
				kind: 'continuous',
				subSteps: 10,
				minValue: 0,
				maxValue: 10,
				minLabel: '',
				maxLabel: ''
			};
		} else {
			draft.type = { kind: 'text' };
		}
	}

	function addCategory() {
		if (draft.type.kind !== 'likert') return;
		const nextValue = (draft.type.categories.at(-1)?.value ?? 0) + 1;
		draft.type.categories = [...draft.type.categories, { label: '', value: nextValue }];
	}

	function removeCategory(cIndex: number) {
		if (draft.type.kind !== 'likert') return;
		draft.type.categories = draft.type.categories.filter((_, i) => i !== cIndex);
	}

	function validate(): string | null {
		if (!draft.text.trim()) return 'Question text is required.';
		if (draft.type.kind === 'likert') {
			if (draft.type.categories.length < 2) return 'Likert needs at least 2 options.';
			if (draft.type.categories.some((c) => !c.label.trim())) {
				return 'Every option needs a label.';
			}
		}
		if (draft.type.kind === 'continuous') {
			if (draft.type.subSteps < 2) return 'Slider needs at least 2 steps.';
			if (draft.type.maxValue <= draft.type.minValue)
				return 'Slider max must be greater than min.';
		}
		return null;
	}

	async function save() {
		const err = validate();
		if (err) {
			errorMessage = err;
			return;
		}
		saving = true;
		errorMessage = null;
		try {
			const existing = toolConfig.questions ?? [];
			const cleaned = cloneQuestion(draft);
			const next = isEditing
				? existing.map((q) => (q.id === cleaned.id ? cleaned : q))
				: [...existing, cleaned];
			await store.saveToolConfig({
				questions: next,
				randomizeOrder: toolConfig.randomizeOrder
			});
			onOpenChange(false);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to save question.';
		} finally {
			saving = false;
		}
	}

	const kindOptions: Array<{ value: QuestionType['kind']; label: string }> = [
		{ value: 'likert', label: 'Likert scale' },
		{ value: 'continuous', label: 'Slider' },
		{ value: 'text', label: 'Free text' }
	];

	function kindLabel(kind: QuestionType['kind']): string {
		return kindOptions.find((k) => k.value === kind)?.label ?? kind;
	}

	/** Drives the live preview at the bottom of the dialog. Reset whenever the user switches answer type so old values don't sit on a slider that no longer accepts them. */
	let previewValue = $state<number | null>(null);
	$effect(() => {
		void draft.type.kind;
		previewValue = null;
	});
</script>

<Dialog.Root {open} onOpenChange={(o) => onOpenChange(o)}>
	<Dialog.Content class="max-h-[90vh] min-w-[70vw] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>{isEditing ? 'Edit question' : 'New question'}</Dialog.Title>
			<Dialog.Description>
				This question will apply to every proposal in this step.
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-4 py-2">
			<div class="space-y-2">
				<Label>Question type</Label>
				<Select.Root
					type="single"
					value={draft.type.kind}
					onValueChange={(v) => v && setKind(v as QuestionType['kind'])}
				>
					<Select.Trigger class="w-full">{kindLabel(draft.type.kind)}</Select.Trigger>
					<Select.Content>
						{#each kindOptions as opt (opt.value)}
							<Select.Item value={opt.value}>{opt.label}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			<div class="space-y-2">
				<Label for="q-text">Question</Label>
				<Input
					id="q-text"
					bind:value={draft.text}
					placeholder="e.g. How strongly do you support this proposal?"
				/>
			</div>

			{#if draft.type.kind === 'likert'}
				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<Label>Options</Label>
						<Button type="button" variant="ghost" size="sm" onclick={addCategory}>
							<Plus class="mr-1 h-3 w-3" /> Add option
						</Button>
					</div>
					{#each draft.type.categories as _, cIndex (cIndex)}
						<div class="flex items-center gap-2">
							<Input
								class="flex-1"
								placeholder="Label"
								bind:value={
									(draft.type as { categories: LikertCategory[] }).categories[
										cIndex
									].label
								}
							/>
							<Input
								type="number"
								class="w-20"
								placeholder="Value"
								bind:value={
									(draft.type as { categories: LikertCategory[] }).categories[
										cIndex
									].value
								}
							/>
							<Button
								type="button"
								variant="ghost"
								size="icon"
								onclick={() => removeCategory(cIndex)}
								disabled={draft.type.kind === 'likert' &&
									draft.type.categories.length <= 2}
								aria-label="Remove option"
							>
								<Trash2 class="h-4 w-4" />
							</Button>
						</div>
					{/each}
				</div>
			{:else if draft.type.kind === 'continuous'}
				{@const ctype = draft.type}
				<div class="space-y-3">
					<div class="flex items-center gap-3">
						<Label class="w-20 shrink-0">Range</Label>
						<Input
							type="number"
							class="w-28"
							bind:value={ctype.minValue}
							aria-label="Minimum value"
						/>
						<span class="text-muted-foreground text-sm">to</span>
						<Input
							type="number"
							class="w-28"
							bind:value={ctype.maxValue}
							aria-label="Maximum value"
						/>
					</div>
					<div class="flex items-center gap-3">
						<Label for="q-min-label" class="w-20 shrink-0">End labels</Label>
						<Input
							id="q-min-label"
							class="flex-1"
							bind:value={ctype.minLabel}
							placeholder="Low end"
						/>
						<Input
							class="flex-1"
							bind:value={ctype.maxLabel}
							placeholder="High end"
							aria-label="High-end label"
						/>
					</div>
					<div class="flex items-center gap-3">
						<Label for="q-steps" class="w-20 shrink-0">Steps</Label>
						<Input
							id="q-steps"
							type="number"
							min="2"
							max="100"
							class="w-28"
							bind:value={ctype.subSteps}
						/>
						<span class="text-muted-foreground text-xs">
							Step size: {ctype.subSteps > 0
								? ((ctype.maxValue - ctype.minValue) / ctype.subSteps).toFixed(2)
								: '—'}
						</span>
					</div>
				</div>
			{:else}
				<p class="text-muted-foreground text-xs">
					Note: text answers are not yet stored by the backend in v1.
				</p>
			{/if}
		</div>

		{#if draft.text.trim() || draft.type.kind !== 'text'}
			<div class="bg-muted/30 mt-4 space-y-2 rounded-lg border p-4">
				<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
					Preview
				</p>
				<QuestionField
					question={{
						id: 'preview',
						text: draft.text || 'Your question will appear here',
						type: draft.type
					}}
					value={previewValue}
					onChange={(v) => (previewValue = v)}
				/>
			</div>
		{/if}

		{#if errorMessage}
			<p class="text-destructive text-sm">{errorMessage}</p>
		{/if}

		<Dialog.Footer>
			<Button variant="outline" onclick={() => onOpenChange(false)} disabled={saving}>
				Cancel
			</Button>
			<Button onclick={save} disabled={saving}>
				{#if saving}<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />{/if}
				{isEditing ? 'Save question' : 'Create question'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
