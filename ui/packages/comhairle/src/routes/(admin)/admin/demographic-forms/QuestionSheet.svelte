<script lang="ts">
	import { untrack } from 'svelte';
	import type { DatePrecision } from '@crownshy/api-client/api';
	import { GripVertical, Plus, Trash2, TriangleAlert, X } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import TagBadge from './TagBadge.svelte';
	import ConsentCheck from './ConsentCheck.svelte';
	import { emptyAnswer } from './demographicAnswers';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Switch } from '$lib/components/ui/switch';
	import { Textarea } from '$lib/components/ui/textarea';
	import PhonePreview from './PhonePreview.svelte';
	import QuestionAnswers from './QuestionAnswers.svelte';
	import {
		DATE_PRECISION_LABELS,
		isChoiceKind,
		nextVersionNumber,
		QUESTION_KIND_LABELS,
		SPECIAL_CATEGORY_TAG,
		selectionHint,
		shortId,
		type DemographicQuestion,
		type QuestionKind,
		type Tag
	} from './demographicModel';

	type SaveOptions = { publish: boolean; asNew: boolean };

	type Props = {
		question: DemographicQuestion;
		isNew: boolean;
		addsToForm: boolean;
		/** Scroll to and highlight the consent wording field on open */
		focusConsent?: boolean;
		tags: Tag[];
		/** Forms that already include this question, used to warn before publishing changes */
		usedInFormList: { id: string; name: string }[];
		onCreateTag: (name: string) => Promise<Tag | null>;
		onSave: (question: DemographicQuestion, options: SaveOptions) => void;
		onDelete: () => void;
		onClose: () => void;
	};

	let {
		question,
		isNew,
		addsToForm,
		focusConsent = false,
		tags,
		usedInFormList,
		onCreateTag,
		onSave,
		onDelete,
		onClose
	}: Props = $props();

	let draft = $state(untrack(() => $state.snapshot(question)));
	let previewAnswer = $state(emptyAnswer());
	let previewConsent = $state(false);
	let newTagName = $state('');
	let creatingTag = $state(false);
	let confirmOpen = $state(false);
	let highlightConsent = $state(untrack(() => focusConsent));

	$effect(() => {
		if (!focusConsent) return;
		const timer = setTimeout(() => {
			const field = document.getElementById('consent-text');
			field?.scrollIntoView({ block: 'center' });
			field?.focus();
		}, 150);
		return () => clearTimeout(timer);
	});

	const KIND_OPTIONS: QuestionKind[] = [
		'single_choice',
		'multiple_choice',
		'number',
		'date',
		'open_text',
		'postcode',
		'lat_long',
		'country'
	];
	const DATE_PRECISIONS = Object.keys(DATE_PRECISION_LABELS) as DatePrecision[];

	const kinds: QuestionKind[] = $derived(
		draft.kind === 'custom' ? KIND_OPTIONS.concat('custom') : KIND_OPTIONS
	);
	const unusedTags = $derived(tags.filter((tag) => !draft.tags.some((t) => t.id === tag.id)));
	const title = $derived(isNew ? 'Create a new question' : 'Edit question');
	const publishLabel = $derived(
		addsToForm ? 'Publish and add to form' : `Publish v${nextVersionNumber(draft)}`
	);

	const isChoice = $derived(isChoiceKind(draft.kind));
	const isSupported = $derived(draft.kind !== 'custom');
	const hasText = $derived(draft.text.trim() !== '');
	const hasChoices = $derived(!isChoice || draft.options.some((o) => o.trim() !== ''));
	const hasConsent = $derived(!draft.specialCategory || draft.consentText.trim() !== '');
	const isValid = $derived(isSupported && hasText && hasChoices && hasConsent);
	const canPublish = $derived(isValid && draft.tags.length > 0);
	const usedInForms = $derived(usedInFormList.length);

	// Publishing a question that forms already use changes those forms, so confirm first.
	function handlePublish() {
		if (!isNew && usedInForms > 0) {
			confirmOpen = true;
			return;
		}
		onSave($state.snapshot(draft), { publish: true, asNew: false });
	}

	function setMaxSelections(event: Event & { currentTarget: HTMLInputElement }) {
		const value = Number.parseInt(event.currentTarget.value, 10);
		draft.maxSelections = Number.isNaN(value) ? null : Math.max(1, value);
	}

	function setKind(kind: QuestionKind) {
		draft.kind = kind;
		previewAnswer = emptyAnswer();
		if (isChoiceKind(kind) && draft.options.length === 0) draft.options = ['', '', ''];
	}

	async function addNewTag() {
		const name = newTagName.trim();
		if (name === '') return;
		const existing = tags.find((tag) => tag.name.toLowerCase() === name.toLowerCase());
		if (existing) {
			draft.tags = [...draft.tags, existing];
			newTagName = '';
			return;
		}
		creatingTag = true;
		const tag = await onCreateTag(name);
		creatingTag = false;
		if (tag) {
			draft.tags = [...draft.tags, tag];
			newTagName = '';
		}
	}
</script>

<Sheet.Root open onOpenChange={(open) => !open && onClose()}>
	<Sheet.Content side="right" class="flex w-full flex-col gap-0 p-0 sm:max-w-none lg:w-[64rem]">
		<div class="grid min-h-0 flex-1 gap-6 p-6 lg:grid-cols-[20rem_1fr]">
			<div class="-mx-2 flex min-h-0 flex-col gap-6 overflow-y-auto px-2">
				<Sheet.Header class="border-border border-b p-0 pb-4">
					<span class="text-muted-foreground text-sm">{shortId('Q', draft.id)}</span>
					<Sheet.Title class="text-2xl font-semibold">{title}</Sheet.Title>
					<Sheet.Description>
						{isNew
							? 'Add a new question to your question bank.'
							: 'Publishing creates a new version. Forms keep the version they were published with.'}
					</Sheet.Description>
				</Sheet.Header>

				<div class="flex flex-col gap-2">
					<Label for="question-text" class="text-base">Question wording</Label>
					<Input
						id="question-text"
						bind:value={draft.text}
						placeholder="Type a question"
					/>
				</div>

				<div class="flex flex-col gap-2">
					<Label for="question-kind" class="text-base">Question type</Label>
					<Select.Root
						type="single"
						value={draft.kind}
						onValueChange={(value) => setKind(value as QuestionKind)}
					>
						<Select.Trigger id="question-kind" class="w-full">
							{QUESTION_KIND_LABELS[draft.kind]}
						</Select.Trigger>
						<Select.Content>
							{#each kinds as kind (kind)}
								<Select.Item value={kind} label={QUESTION_KIND_LABELS[kind]}>
									{QUESTION_KIND_LABELS[kind]}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
					{#if draft.kind === 'custom'}
						<span class="text-muted-foreground text-sm">
							This question uses a custom JSON schema. It can be viewed here but not
							edited.
						</span>
					{/if}
				</div>

				{#if draft.kind === 'date'}
					<div class="flex flex-col gap-2">
						<Label for="date-precision" class="text-base">Date format</Label>
						<Select.Root
							type="single"
							value={draft.datePrecision}
							onValueChange={(value) =>
								(draft.datePrecision = value as DatePrecision)}
						>
							<Select.Trigger id="date-precision" class="w-full">
								{DATE_PRECISION_LABELS[draft.datePrecision]}
							</Select.Trigger>
							<Select.Content>
								{#each DATE_PRECISIONS as precision (precision)}
									<Select.Item
										value={precision}
										label={DATE_PRECISION_LABELS[precision]}
									>
										{DATE_PRECISION_LABELS[precision]}
									</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
					</div>
				{/if}

				<div class="flex flex-col gap-2">
					<Label class="text-base">Category</Label>
					<div class="flex flex-wrap gap-2">
						{#each draft.tags as tag (tag.id)}
							<TagBadge tag={tag.name} class="gap-1">
								{tag.name}
								<button
									type="button"
									class="hover:text-foreground"
									aria-label={`Remove category ${tag.name}`}
									onclick={() =>
										(draft.tags = draft.tags.filter((t) => t.id !== tag.id))}
								>
									<X class="size-3" />
								</button>
							</TagBadge>
						{/each}
						{#if draft.specialCategory}
							<TagBadge tag={SPECIAL_CATEGORY_TAG} />
						{/if}
					</div>
					<div class="flex items-center gap-2">
						<DropdownMenu.Root>
							<DropdownMenu.Trigger
								class={buttonVariants({ variant: 'outline', size: 'icon' })}
								aria-label="Add category"
								disabled={unusedTags.length === 0}
							>
								<Plus class="size-4" />
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="start">
								{#each unusedTags as tag (tag.id)}
									<DropdownMenu.Item
										onSelect={() => (draft.tags = [...draft.tags, tag])}
									>
										{tag.name}
									</DropdownMenu.Item>
								{/each}
							</DropdownMenu.Content>
						</DropdownMenu.Root>
						<Input
							class="flex-1"
							bind:value={newTagName}
							placeholder="New category"
							aria-label="New category name"
							onkeydown={(event) => event.key === 'Enter' && addNewTag()}
						/>
						<Button
							variant="outline"
							disabled={newTagName.trim() === '' || creatingTag}
							onclick={addNewTag}>Add</Button
						>
					</div>
				</div>

				{#if draft.kind === 'multiple_choice'}
					<div class="flex flex-col gap-2">
						<Label for="max-selections" class="text-base"
							>Choices people can select</Label
						>
						<Input
							id="max-selections"
							type="number"
							min="1"
							max={Math.max(draft.options.length, 1)}
							placeholder="No limit"
							value={draft.maxSelections ?? ''}
							oninput={setMaxSelections}
						/>
						<span class="text-muted-foreground text-sm">
							Leave empty to let people select as many as they like.
						</span>
					</div>
				{/if}

				<div class="border-border mt-auto flex flex-col gap-4 border-t pt-4">
					<h3 class="text-base font-semibold">Settings</h3>
					{#if isChoice}
						<div class="flex items-center justify-between gap-4">
							<Label for="allow-other" class="text-base">Add "Other" option</Label>
							<Switch id="allow-other" bind:checked={draft.allowOther} />
						</div>
						<div class="flex items-center justify-between gap-4">
							<Label for="prefer-not-to-say" class="text-base"
								>Add "Prefer not to say"</Label
							>
							<Switch id="prefer-not-to-say" bind:checked={draft.preferNotToSay} />
						</div>
					{/if}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col">
							<Label for="special-category" class="text-base"
								>Special category data</Label
							>
							<span class="text-muted-foreground text-sm">
								Asks for explicit consent before answering
							</span>
						</div>
						<Switch id="special-category" bind:checked={draft.specialCategory} />
					</div>
					{#if draft.specialCategory}
						<div class="flex flex-col gap-2">
							<Label for="consent-text" class="text-base">Consent wording</Label>
							<Textarea
								id="consent-text"
								rows={3}
								bind:value={draft.consentText}
								placeholder="Explain why this data is collected and how it will be used"
								class={highlightConsent ? 'ring-primary ring-2' : ''}
								onblur={() => (highlightConsent = false)}
							/>
							<span class="text-muted-foreground text-sm">
								Shown as a checkbox at the bottom of this question. Participants
								tick it before answering, unless they choose Prefer not to say.
							</span>
						</div>
					{/if}
				</div>
				{#if !isNew}
					<AlertDialog.Root>
						<AlertDialog.Trigger
							class={buttonVariants({ variant: 'destructiveOutline' })}
						>
							<Trash2 class="size-4" />Delete this question
						</AlertDialog.Trigger>
						<AlertDialog.Content>
							<AlertDialog.Header>
								<AlertDialog.Title>Delete this question?</AlertDialog.Title>
								<AlertDialog.Description>
									{usedInForms > 0
										? `This question is used in ${usedInForms} ${usedInForms === 1 ? 'form' : 'forms'}. It will be removed from ${usedInForms === 1 ? 'it' : 'them'}, and that counts as unpublished changes. Answers already collected are kept.`
										: 'This question will be removed from your question bank. This cannot be undone.'}
								</AlertDialog.Description>
							</AlertDialog.Header>
							<AlertDialog.Footer>
								<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
								<AlertDialog.Action onclick={onDelete}>Delete question</AlertDialog.Action>
							</AlertDialog.Footer>
						</AlertDialog.Content>
					</AlertDialog.Root>
				{/if}
			</div>

			{#snippet consentFooter()}
				<ConsentCheck
					id="preview-consent"
					text={draft.consentText}
					checked={previewConsent}
					onChange={(checked) => (previewConsent = checked)}
				/>
			{/snippet}
			<PhonePreview
				showNext={draft.specialCategory}
				nextDisabled={!previewConsent}
				footer={draft.specialCategory ? consentFooter : undefined}
				step={1}
				steps={2}
			>
				<p class="text-2xl font-semibold">
					{draft.text.trim() === '' ? 'Type a question' : draft.text}
				</p>

				{#if isChoice}
					{#if draft.kind === 'multiple_choice'}
						<p class="text-muted-foreground text-sm">
							{selectionHint(draft.kind, draft.maxSelections)}
						</p>
					{/if}
					<ul class="flex flex-col gap-2">
						{#each draft.options as _, index (index)}
							<li
								class="border-border flex items-center gap-2 rounded-lg border px-3"
							>
								<input
									class="placeholder:text-primary/50 w-full bg-transparent py-3 text-base outline-none"
									placeholder="Choice"
									aria-label={`Choice ${index + 1}`}
									bind:value={draft.options[index]}
								/>
								<GripVertical class="text-muted-foreground size-4 shrink-0" />
								<button
									type="button"
									class="text-muted-foreground hover:text-foreground"
									aria-label={`Remove choice ${index + 1}`}
									onclick={() => draft.options.splice(index, 1)}
								>
									<X class="size-4" />
								</button>
							</li>
						{/each}
						{#if draft.allowOther}
							<li
								class="text-muted-foreground border-border rounded-lg border border-dashed px-3 py-3 text-base"
							>
								Other (please specify)
							</li>
						{/if}
						{#if draft.preferNotToSay}
							<li
								class="text-muted-foreground border-border rounded-lg border border-dashed px-3 py-3 text-base"
							>
								Prefer not to say
							</li>
						{/if}
					</ul>
					<Button
						variant="link"
						class="self-start px-0"
						onclick={() => draft.options.push('')}
					>
						Add choice
					</Button>
				{:else if isSupported}
					<QuestionAnswers
						question={draft}
						answer={previewAnswer}
						onAnswer={(next) => (previewAnswer = next)}
					/>
				{/if}
			</PhonePreview>
		</div>

		<Sheet.Footer
			class="border-border flex-row items-center justify-end gap-2 border-t px-6 py-4"
		>
			{#if !canPublish}
				<span class="text-muted-foreground mr-auto text-sm">
					{draft.tags.length === 0 ? 'Add a category to publish.' : ''}
				</span>
			{/if}
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			{#if !addsToForm}
				<Button
					variant="outline"
					disabled={!isValid}
					onclick={() => onSave($state.snapshot(draft), { publish: false, asNew: false })}
				>
					Save draft
				</Button>
			{/if}
			{#if !isNew}
				<Button
					variant="outline"
					disabled={!isValid}
					onclick={() => onSave($state.snapshot(draft), { publish: false, asNew: true })}
				>
					Save as new question
				</Button>
			{/if}
			<Button disabled={!canPublish} onclick={handlePublish}>
				{publishLabel}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

<AlertDialog.Root bind:open={confirmOpen}>
	<AlertDialog.Content class="max-h-[90vh] overflow-y-auto sm:max-w-xl">
		<AlertDialog.Header>
			<div class="flex flex-col items-center gap-3 text-center">
				<span
					class="bg-destructive/10 text-destructive flex size-11 shrink-0 items-center justify-center rounded-full"
					aria-hidden="true"
				>
					<TriangleAlert class="size-6" />
				</span>
				<AlertDialog.Title>
					This change affects {usedInForms}
					{usedInForms === 1 ? 'form' : 'forms'}
				</AlertDialog.Title>
			</div>
		</AlertDialog.Header>
		<AlertDialog.Description class="bg-muted px-4 py-4">
			<ul class="mb-4 flex flex-wrap gap-2">
				{#each usedInFormList as usedForm (usedForm.id)}
					<li
						class="bg-primary/10 text-primary flex items-center gap-2 rounded-md px-2 py-1 text-base font-semibold"
					>
						<span class="text-sm font-medium opacity-80">{shortId('F', usedForm.id)}</span>
						{usedForm.name}
					</li>
				{/each}
			</ul>
			Published forms will show unpublished changes until republished. To change it for one form
			only, save it as a new question.
		</AlertDialog.Description>
		<AlertDialog.Footer class="sm:flex-wrap sm:items-start">
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<Button
				variant="outline"
				onclick={() => onSave($state.snapshot(draft), { publish: true, asNew: true })}
			>
				Save as new question
			</Button>
			<div class="flex flex-col items-end gap-1">
				<AlertDialog.Action
					onclick={() => onSave($state.snapshot(draft), { publish: true, asNew: false })}
				>
					Confirm, publish this question.
				</AlertDialog.Action>
				<span class="text-destructive text-sm">
					This will update {usedInForms === 1 ? 'the form' : 'all forms'}.
				</span>
			</div>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
