<script lang="ts">
	import { untrack } from 'svelte';
	import { GripVertical, Plus, Trash2, X } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import TagBadge from './TagBadge.svelte';
	import { emptyAnswer } from './demographicAnswers';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import * as Sheet from '$lib/components/ui/sheet';
	import { Switch } from '$lib/components/ui/switch';
	import PhonePreview from './PhonePreview.svelte';
	import ConsentCheck from './ConsentCheck.svelte';
	import { Textarea } from '$lib/components/ui/textarea';
	import QuestionAnswers from './QuestionAnswers.svelte';
	import {
		defaultPlaceholder,
		isChoiceKind,
		selectionHint,
		shortId,
		QUESTION_KIND_LABELS,
		TAG_OPTIONS,
		type DemographicQuestion,
		type QuestionKind
	} from './demographicPrototypeData';

	type Props = {
		question: DemographicQuestion;
		isNew: boolean;
		addsToForm: boolean;
		/** Scroll to and highlight the consent wording field on open */
		focusConsent?: boolean;
		onSave: (question: DemographicQuestion) => void;
		onSaveAsNew: (question: DemographicQuestion) => void;
		onDelete: () => void;
		/** How many forms currently include this question */
		usedInForms: number;
		/** Names of the forms that include this question */
		usedInFormNames: string[];
		onClose: () => void;
	};

	let {
		question,
		isNew,
		addsToForm,
		focusConsent = false,
		onSave,
		onSaveAsNew,
		onDelete,
		usedInForms,
		usedInFormNames,
		onClose
	}: Props = $props();

	let draft = $state(untrack(() => $state.snapshot(question)));
	const original = untrack(() => JSON.stringify($state.snapshot(question)));
	const hasChanges = $derived(JSON.stringify($state.snapshot(draft)) !== original);

	let previewAnswer = $state(emptyAnswer());
	let previewConsent = $state(false);
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

	const kinds = Object.keys(QUESTION_KIND_LABELS) as QuestionKind[];
	const unusedTags = $derived(TAG_OPTIONS.filter((tag) => !draft.tags.includes(tag)));
	const saveLabel = $derived(addsToForm ? 'Save and add to form' : 'Save');

	// Editing a question that other forms already use affects all of them, so ask first.
	function handleSave() {
		if (!isNew && hasChanges && usedInFormNames.length > 0) {
			confirmOpen = true;
			return;
		}
		onSave($state.snapshot(draft));
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
</script>

<Sheet.Root open onOpenChange={(open) => !open && onClose()}>
	<Sheet.Content
		side="right"
		class="flex w-full flex-col gap-0 p-0 sm:max-w-none lg:w-[64rem] [&>button.absolute]:hidden"
	>
		<div class="grid min-h-0 flex-1 gap-6 p-6 lg:grid-cols-[20rem_1fr]">
			<div class="-mx-2 flex min-h-0 flex-col gap-6 overflow-y-auto px-2">
				<Sheet.Header class="border-border gap-1 border-b p-0 pb-4">
					<span class="text-muted-foreground text-sm">{shortId('Q', draft.id)}</span>
					<Sheet.Title class="line-clamp-3 text-2xl font-semibold">
						{draft.text.trim() || 'New question'}
					</Sheet.Title>
					{#if draft.description.trim()}
						<p class="text-muted-foreground text-base">{draft.description}</p>
					{/if}
					<Sheet.Description class="sr-only">
						Edit the question settings. Changes apply to every form that uses it.
					</Sheet.Description>
				</Sheet.Header>

				<div class="flex flex-col gap-2">
					<Label for="question-kind" class="text-base">Question type</Label>
					<Select.Root
						type="single"
						value={draft.kind}
						onValueChange={(value) => setKind(value as QuestionKind)}
					>
						<Select.Trigger id="question-kind" class="w-full">
							<span class="truncate">{QUESTION_KIND_LABELS[draft.kind]}</span>
						</Select.Trigger>
						<Select.Content>
							{#each kinds as kind (kind)}
								<Select.Item value={kind} label={QUESTION_KIND_LABELS[kind]}>
									<span class="truncate">{QUESTION_KIND_LABELS[kind]}</span>
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>

				{#if !isChoiceKind(draft.kind) && draft.kind !== 'date_split'}
					<div class="flex flex-col gap-2">
						<Label for="question-placeholder" class="text-base">Placeholder text</Label>
						<Input
							id="question-placeholder"
							class="text-foreground"
							bind:value={draft.placeholder}
							placeholder={defaultPlaceholder(draft.kind)}
						/>
					</div>
				{/if}

				<div class="flex flex-col gap-2">
					<Label class="text-base">Tag</Label>
					<div class="flex items-center justify-between gap-2">
						<div class="flex flex-wrap gap-2">
							{#each draft.tags as tag (tag)}
								<TagBadge {tag} class="gap-1">
									{tag}
									<button
										type="button"
										class="hover:text-foreground"
										aria-label={`Remove tag ${tag}`}
										onclick={() =>
											(draft.tags = draft.tags.filter((t) => t !== tag))}
									>
										<X class="size-3" />
									</button>
								</TagBadge>
							{/each}
							{#if draft.specialCategory}
								<TagBadge tag="Special category" />
							{/if}
						</div>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger
								class={buttonVariants({ variant: 'default', size: 'icon' })}
								aria-label="Add tag"
								disabled={unusedTags.length === 0}
							>
								<Plus class="size-4" />
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="end">
								{#each unusedTags as tag (tag)}
									<DropdownMenu.Item
										onSelect={() => (draft.tags = [...draft.tags, tag])}
									>
										{tag}
									</DropdownMenu.Item>
								{/each}
							</DropdownMenu.Content>
						</DropdownMenu.Root>
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
					{#if isChoiceKind(draft.kind)}
						<div class="flex items-center justify-between gap-4">
							<Label for="allow-other" class="text-base">Add "Other" option</Label>
							<Switch id="allow-other" bind:checked={draft.allowOther} />
						</div>
					{/if}
					<div class="flex items-center justify-between gap-4">
						<Label for="prefer-not-to-say" class="text-base"
							>Add "Prefer not to say"</Label
						>
						<Switch id="prefer-not-to-say" bind:checked={draft.preferNotToSay} />
					</div>
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
								<AlertDialog.Action onclick={onDelete}
									>Delete question</AlertDialog.Action
								>
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
				<input
					class="placeholder:text-muted-foreground w-full bg-transparent text-2xl font-semibold outline-none"
					placeholder="Type a question"
					aria-label="Question wording"
					bind:value={draft.text}
				/>
				<input
					class="placeholder:text-muted-foreground text-muted-foreground w-full bg-transparent text-base outline-none"
					placeholder="Add a description (optional)"
					aria-label="Question description"
					bind:value={draft.description}
				/>

				{#if isChoiceKind(draft.kind)}
					{#if draft.kind === 'multiple_choice'}
						<p class="text-muted-foreground text-sm">
							{selectionHint(draft.kind, draft.maxSelections)}
						</p>
					{/if}
					<ul class="flex flex-col gap-2">
						{#each draft.options as _, index (index)}
							<li
								class="border-border flex h-[3.125rem] items-center gap-2 rounded-lg border px-4"
							>
								<input
									class="placeholder:text-primary/50 h-full w-full bg-transparent text-base outline-none"
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
								class="text-muted-foreground border-border flex h-[3.125rem] items-center rounded-lg border border-dashed px-4 text-base"
							>
								Other (please specify)
							</li>
						{/if}
						{#if draft.preferNotToSay}
							<li
								class="text-muted-foreground border-border flex h-[3.125rem] items-center rounded-lg border border-dashed px-4 text-base"
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
				{:else}
					<QuestionAnswers
						question={draft}
						answer={previewAnswer}
						onAnswer={(next) => (previewAnswer = next)}
					/>
				{/if}
			</PhonePreview>
		</div>

		<Sheet.Footer class="border-border flex-row justify-end gap-2 border-t px-6 py-4">
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			{#if !isNew}
				<Button
					variant="outline"
					disabled={draft.text.trim() === '' || !hasChanges}
					onclick={() => onSaveAsNew($state.snapshot(draft))}
				>
					Save to new question
				</Button>
			{/if}
			<Button disabled={draft.text.trim() === ''} onclick={handleSave}>
				{saveLabel}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

<AlertDialog.Root bind:open={confirmOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>
				This question is used in {usedInFormNames.length}
				{usedInFormNames.length === 1 ? 'form' : 'forms'}
			</AlertDialog.Title>
			<AlertDialog.Description>
				Saving your changes updates it in {usedInFormNames.join(', ')}. Published forms will
				show unpublished changes until you publish them again. To change it for one form
				only, save it as a new question instead.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<Button variant="outline" onclick={() => onSaveAsNew($state.snapshot(draft))}>
				Save as new question
			</Button>
			<AlertDialog.Action onclick={() => onSave($state.snapshot(draft))}>
				Save and update {usedInFormNames.length === 1 ? 'the form' : 'all forms'}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
