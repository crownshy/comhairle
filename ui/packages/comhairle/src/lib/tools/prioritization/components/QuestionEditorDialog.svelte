<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { LoaderCircle, Plus, Trash2 } from 'lucide-svelte';
	import QuestionField from './QuestionField.svelte';
	import type { PrioritizationStore } from '../store.svelte';
	import type {
		DraftTranslatableJsonField,
		DraftQuestion,
		ToolConfig,
		DraftFields,
		DraftQuestionType,
		DraftLikertCategory
	} from '../types';
	import type { TranslationDto } from '@crownshy/api-client/api';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import {
		isDraftTranslatableField,
		localizeTranslatableJson,
		traverseTranslatableJsonAndCreateTranslations
	} from '$lib/components/Translation/translationUtils';
	import type { Locale } from '$lib/paraglide/runtime';

	type Props = {
		open: boolean;
		question?: DraftQuestion | null;
		store: PrioritizationStore;
		toolConfig: ToolConfig<DraftTranslatableJsonField>;
		/** Which question set this dialog edits. */
		target?: 'proposal' | 'section';
		onOpenChange: (open: boolean) => void;
		primaryLocale: Locale;
		supportedLocales: Locale[];
	};

	let {
		open,
		question = null,
		store,
		toolConfig,
		target = 'proposal',
		onOpenChange,
		primaryLocale,
		supportedLocales
	}: Props = $props();

	const defaultLikertCategories: DraftLikertCategory[] = [
		{ label: newTranslatableField('Strongly disagree'), value: -2 },
		{ label: newTranslatableField('Disagree'), value: -1 },
		{ label: newTranslatableField('Neutral'), value: 0 },
		{ label: newTranslatableField('Agree'), value: 1 },
		{ label: newTranslatableField('Strongly agree'), value: 2 }
	];

	function newTranslatableField(seed = ''): DraftTranslatableJsonField {
		return { localized: seed };
	}

	function cloneTranslatableText(source: {
		localized: string;
		translations?: TranslationDto;
	}): DraftTranslatableJsonField {
		return {
			localized: source.localized,
			translations: source.translations
		};
	}

	function emptyDraft(): DraftFields {
		return {
			text: newTranslatableField(),
			type: { kind: 'likert', categories: defaultLikertCategories.map((c) => ({ ...c })) }
		};
	}

	function cloneType(t: DraftQuestionType): DraftQuestionType {
		if (t.kind === 'likert')
			return {
				kind: 'likert',
				categories: t.categories.map((c) => ({
					value: c.value,
					label: cloneTranslatableText(c.label)
				}))
			};
		if (t.kind === 'continuous')
			return {
				kind: 'continuous',
				subSteps: t.subSteps,
				minValue: t.minValue,
				maxValue: t.maxValue,
				minLabel: cloneTranslatableText(t.minLabel),
				maxLabel: cloneTranslatableText(t.maxLabel)
			};
		return { kind: 'text' };
	}

	let draft = $state<DraftFields>(emptyDraft());
	let editingId = $state<string | undefined>(undefined);
	let saving = $state(false);
	let errorMessage = $state<string | null>(null);

	const isEditing = $derived(editingId !== undefined);

	const textTransSource = $derived.by(() => {
		// Force re-evaluation on toggle open as component isn't unmounted
		// when new questions are created
		void open;
		return createTextContentSource({
			getTranslation: () => draft.text.translations,
			getPrimaryLocale: () => primaryLocale,
			getSupportedLanguages: () => supportedLocales,
			getPrimaryFallback: () => draft.text.localized,
			onEdit: async (content) => {
				draft.text.localized = content;
			}
		});
	});

	const likertCategoryTransSources = $derived.by(() => {
		// Force re-evaluation on toggle open as component isn't unmounted
		// when new questions are created
		void open;
		if ('kind' in draft.type && draft.type.kind !== 'likert') return [];

		return draft.type.categories.map((category, index) => {
			return createTextContentSource({
				getTranslation: () => category.label.translations ?? undefined,
				getPrimaryLocale: () => primaryLocale,
				getSupportedLanguages: () => supportedLocales,
				getPrimaryFallback: () => category.label.localized ?? '',
				onEdit: async (content) => {
					draft.type.categories[index].label.localized = content;
				}
			});
		});
	});

	const continuousTransSources = $derived.by(() => {
		// Force re-evaluation on toggle open as component isn't unmounted
		// when new questions are created
		void open;
		if ('kind' in draft.type && draft.type.kind !== 'continuous') return {};

		return {
			minLabel: createTextContentSource({
				getTranslation: () => draft.type.minLabel.translations ?? undefined,
				getPrimaryLocale: () => primaryLocale,
				getSupportedLanguages: () => supportedLocales,
				getPrimaryFallback: () => draft.type.minLabel.localized ?? '',
				onEdit: async (content) => {
					draft.type.minLabel.localized = content;
				}
			}),
			maxLabel: createTextContentSource({
				getTranslation: () => draft.type.maxLabel.translations ?? undefined,
				getPrimaryLocale: () => primaryLocale,
				getSupportedLanguages: () => supportedLocales,
				getPrimaryFallback: () => draft.type.maxLabel.localized ?? '',
				onEdit: async (content) => {
					draft.type.maxLabel.localized = content;
				}
			})
		};
	});

	$effect(() => {
		if (open) {
			if (question) {
				editingId = question.id;
				draft = { text: question.text, type: cloneType(question.type) };
			} else {
				editingId = undefined;
				draft = emptyDraft();
			}
			errorMessage = null;
		}
	});

	function setKind(kind: DraftQuestionType['kind']) {
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
				minLabel: newTranslatableField(''),
				maxLabel: newTranslatableField('')
			};
		} else {
			draft.type = { kind: 'text' };
		}
	}

	function addCategory() {
		if (draft.type.kind !== 'likert') return;
		const nextValue = (draft.type.categories.at(-1)?.value ?? 0) + 1;
		draft.type.categories = [
			...draft.type.categories,
			{ label: newTranslatableField(''), value: nextValue }
		];
	}

	function removeCategory(cIndex: number) {
		if (draft.type.kind !== 'likert') return;
		draft.type.categories = draft.type.categories.filter((_, i) => i !== cIndex);
	}

	function validate(): string | null {
		if (!draft.text.localized.trim()) return 'Question text is required.';
		if (draft.type.kind === 'likert') {
			if (draft.type.categories.length < 2) return 'Likert needs at least 2 options.';
			if (draft.type.categories.some((c: DraftLikertCategory) => !c.label.localized.trim())) {
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
			// Create translations for any new translatable fields in draft
			const questionWithNewlyCreatedTranslations =
				await traverseTranslatableJsonAndCreateTranslations(draft, primaryLocale);

			const existing =
				(target === 'section' ? toolConfig.sectionQuestions : toolConfig.questions) ?? [];
			const id = editingId ?? crypto.randomUUID();
			const next: DraftQuestion = {
				id,
				...structuredClone(questionWithNewlyCreatedTranslations)
			};
			const updated =
				editingId !== undefined
					? existing.map((q) => (q.id === editingId ? next : q))
					: [...existing, next];

			await store.saveToolConfig({
				...toolConfig,
				questions: target === 'section' ? toolConfig.questions : updated,
				sectionQuestions: target === 'section' ? updated : toolConfig.sectionQuestions
			});
			onOpenChange(false);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to save question.';
		} finally {
			saving = false;
		}
	}

	const kindOptions: Array<{ value: DraftQuestionType['kind']; label: string }> = [
		{ value: 'likert', label: 'Likert scale' },
		{ value: 'continuous', label: 'Slider' },
		{ value: 'text', label: 'Free text' }
	];

	function kindLabel(kind: DraftQuestionType['kind']): string {
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
				{target === 'section'
					? 'This question will be asked about every section of every proposal in this step.'
					: 'This question will apply to every proposal in this step.'}
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-4 py-2">
			<div class="space-y-2">
				<Label for="q-text">Question</Label>
				{#if isDraftTranslatableField(draft.text)}
					<Input
						id="q-text"
						bind:value={draft.text.localized}
						placeholder="e.g. How strongly do you support this proposal?"
					/>
				{:else}
					<TranslatableField
						source={textTransSource}
						{primaryLocale}
						supportedLanguages={supportedLocales}
					/>
				{/if}
			</div>

			<div class="space-y-2">
				<Label>Question type</Label>
				<Select.Root
					type="single"
					value={draft.type.kind}
					onValueChange={(v) => v && setKind(v as DraftQuestionType['kind'])}
				>
					<Select.Trigger class="w-full">{kindLabel(draft.type.kind)}</Select.Trigger>
					<Select.Content>
						{#each kindOptions as opt (opt.value)}
							<Select.Item value={opt.value}>{opt.label}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			{#if draft.type.kind === 'likert'}
				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<Label>Options</Label>
						<Button type="button" variant="ghost" size="sm" onclick={addCategory}>
							<Plus class="mr-1 h-3 w-3" /> Add option
						</Button>
					</div>
					<div class="flex flex-col gap-4">
						{#each draft.type.categories as category, cIndex (cIndex)}
							<div class="flex items-start gap-2">
								<div class="flex-1">
									{#if isDraftTranslatableField(category.label)}
										<Input
											class="flex-1"
											placeholder="Label"
											bind:value={
												(
													draft.type as {
														categories: DraftLikertCategory[];
													}
												).categories[cIndex].label.localized
											}
										/>
									{:else}
										<TranslatableField
											source={likertCategoryTransSources[cIndex]}
											{primaryLocale}
											supportedLanguages={supportedLocales}
										/>
									{/if}
								</div>
								<Input
									type="number"
									class="w-20"
									placeholder="Value"
									bind:value={
										(draft.type as { categories: DraftLikertCategory[] })
											.categories[cIndex].value
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
						<div class="flex items-start gap-3">
							{#if isDraftTranslatableField(ctype.minLabel)}
								<Input
									id="q-min-label"
									class="flex-1"
									bind:value={ctype.minLabel.localized}
									placeholder="Low end"
								/>
							{:else if continuousTransSources.minLabel}
								<TranslatableField
									source={continuousTransSources.minLabel}
									{primaryLocale}
									supportedLanguages={supportedLocales}
								/>
							{/if}
							{#if isDraftTranslatableField(ctype.minLabel)}
								<Input
									class="flex-1"
									bind:value={ctype.maxLabel.localized}
									placeholder="High end"
									aria-label="High-end label"
								/>
							{:else if continuousTransSources.maxLabel}
								<TranslatableField
									source={continuousTransSources.maxLabel}
									{primaryLocale}
									supportedLanguages={supportedLocales}
								/>
							{/if}
						</div>
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
			{/if}
		</div>

		{#if draft.text.localized.trim() || draft.type.kind !== 'text'}
			<div class="bg-muted/30 mt-4 space-y-2 rounded-lg border p-4">
				<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
					Preview
				</p>
				<QuestionField
					question={localizeTranslatableJson(draft)}
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
