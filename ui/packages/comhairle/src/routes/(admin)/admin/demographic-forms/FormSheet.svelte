<script lang="ts">
	import { EllipsisVertical, Plus, RotateCcw, Trash2 } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import TagBadge from './TagBadge.svelte';
	import {
		emptyAnswer,
		isAnswered,
		PREFER_NOT_TO_SAY,
		summariseAnswer,
		type Answer
	} from './demographicAnswers';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import * as Sheet from '$lib/components/ui/sheet';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import AddQuestionsDialog from './AddQuestionsDialog.svelte';
	import PublishDialog from './PublishDialog.svelte';
	import ConsentCheck from './ConsentCheck.svelte';
	import CreatedBy from './CreatedBy.svelte';
	import VersionUsage from './VersionUsage.svelte';
	import PhonePreview from './PhonePreview.svelte';
	import QuestionAnswers from './QuestionAnswers.svelte';
	import {
		conversationCount,
		COUNTRY_FLAGS,
		COUNTRY_OPTIONS,
		usageForVersion,
		type DemographicForm,
		type DemographicQuestion,
		type FormQuestion
	} from './demographicPrototypeData';

	type Props = {
		form: DemographicForm;
		isNew: boolean;
		questions: DemographicQuestion[];
		onEditQuestion: (questionId: string, focusConsent?: boolean) => void;
		onCreateQuestion: () => void;
		hasChanges: boolean;
		onSaveDraft: () => void;
		onPublish: () => void;
		onDelete: () => void;
		onClose: () => void;
	};

	let {
		form = $bindable(),
		isNew,
		questions,
		onEditQuestion,
		onCreateQuestion,
		hasChanges,
		onSaveDraft,
		onPublish,
		onDelete,
		onClose
	}: Props = $props();

	let addOpen = $state(false);
	let publishOpen = $state(false);
	let consents = $state<Record<string, boolean>>({});
	let answers = $state<Record<string, Answer>>({});
	let previewId = $state<string | null>(null);
	let finished = $state(false);
	let moveNote = $state('');

	const toRows = (list: FormQuestion[]) =>
		list.flatMap((formQuestion) => {
			const question = questions.find((q) => q.id === formQuestion.questionId);
			return question ? [{ id: question.id, question, required: formQuestion.required }] : [];
		});

	const rows = $derived(toRows(form.questions));

	// The preview shows the working draft by default, or any published version (read-only).
	let viewing = $state<'draft' | number>('draft');
	const viewedVersion = $derived(
		viewing === 'draft' ? undefined : form.versions.find((v) => v.version === viewing)
	);
	// An older version shows the name it was published under.
	const displayName = $derived(viewedVersion?.name ?? form.name);
	const previewRows = $derived(viewedVersion ? toRows(viewedVersion.questions) : rows);
	const hasDraftEdits = $derived(hasChanges || form.hasUnpublishedChanges);

	const previewIndex = $derived(
		Math.max(
			0,
			previewRows.findIndex((row) => row.question.id === previewId)
		)
	);
	const preview = $derived(previewRows[previewIndex]?.question);

	const consentMissing = $derived(
		!!preview?.specialCategory &&
			!consents[preview.id] &&
			!answers[preview.id]?.selected.includes(PREFER_NOT_TO_SAY)
	);

	const isLast = $derived(previewIndex >= previewRows.length - 1);
	const requiredMissing = $derived(
		!!preview &&
			(previewRows[previewIndex]?.required ?? false) &&
			!isAnswered(answers[preview.id] ?? emptyAnswer())
	);
	const results = $derived(
		previewRows.map((row) => ({
			id: row.id,
			text: row.question.text,
			required: row.required,
			summary: summariseAnswer(answers[row.id] ?? emptyAnswer())
		}))
	);

	function resetTest() {
		answers = {};
		consents = {};
		finished = false;
		previewId = previewRows[0]?.question.id ?? null;
	}

	const draftLabel = $derived(
		hasDraftEdits ? 'Draft (unpublished changes)' : `Current draft (v${form.version})`
	);
	const versionLabel = (version: number) => {
		const count = usageForVersion(form, version).length;
		return `v${version}${version === form.version ? ' (latest)' : ''} · ${count} ${count === 1 ? 'conversation' : 'conversations'}`;
	};

	function viewVersion(value: string) {
		viewing = value === 'draft' ? 'draft' : Number(value);
		resetTest();
	}

	function selectQuestion(id: string) {
		finished = false;
		previewId = id;
	}

	function advance() {
		if (isLast) finished = true;
		else showNextQuestion();
	}

	const hasName = $derived(form.name.trim() !== '');
	const canPublish = $derived(
		hasName &&
			form.questions.length > 0 &&
			(hasChanges || form.hasUnpublishedChanges || form.status === 'draft')
	);

	function toggleRequired(index: number) {
		form.questions[index].required = !form.questions[index].required;
	}

	let dragRows = $state<typeof rows | null>(null);

	function reorderWhileDragging(next: typeof rows) {
		dragRows = next;
	}

	function commitReorder(next: typeof rows) {
		form.questions = next.map((row) => ({ questionId: row.id, required: row.required }));
		dragRows = null;
	}

	function showNextQuestion() {
		const next = previewRows[previewIndex + 1];
		if (next) previewId = next.question.id;
	}

	function move(index: number, offset: number) {
		const target = index + offset;
		if (target < 0 || target >= form.questions.length) return;
		[form.questions[index], form.questions[target]] = [
			form.questions[target],
			form.questions[index]
		];
		moveNote = `${rows[index]?.question.text ?? 'Question'} moved to position ${target + 1} of ${form.questions.length}`;
	}

	function addQuestions(ids: string[]) {
		form.questions.push(...ids.map((questionId) => ({ questionId, required: false })));
		addOpen = false;
	}

	function createFromDialog() {
		addOpen = false;
		onCreateQuestion();
	}
</script>

<Sheet.Root open onOpenChange={(open) => !open && onClose()}>
	<Sheet.Content
		side="right"
		class="flex w-full flex-col gap-0 p-0 sm:max-w-none lg:w-[64rem] [&>button.absolute]:hidden"
	>
		<div
			class="grid min-h-0 flex-1 gap-6 p-6 lg:grid-cols-[20rem_1fr] {viewedVersion
				? 'bg-primary/5'
				: ''}"
		>
			<div class="-mx-3 -my-1 flex min-h-0 flex-col gap-5 overflow-y-auto px-3 py-1">
				<Sheet.Header class="border-border gap-3 border-b p-0 pb-4">
					<Sheet.Title class="sr-only">
						{displayName.trim() || (isNew ? 'New form' : 'Form')}
					</Sheet.Title>
					<Sheet.Description class="sr-only">
						Edit the form name and questions. Save a draft or publish a new version.
					</Sheet.Description>
					<Input
						id="form-name"
						aria-label="Form name"
						value={displayName}
						oninput={(event) => (form.name = event.currentTarget.value)}
						placeholder="Name this form"
						disabled={!!viewedVersion}
						class="placeholder:text-muted-foreground hover:border-input focus-visible:border-ring -mx-2 h-auto border-transparent bg-transparent px-2 py-1 text-2xl font-semibold shadow-none md:text-2xl"
					/>
					{#if form.versions.length > 0}
						<Select.Root
							type="single"
							value={viewedVersion ? String(viewedVersion.version) : 'draft'}
							onValueChange={viewVersion}
						>
							<Select.Trigger
								size="sm"
								aria-label="Version"
								class="w-fit max-w-full text-sm font-medium {viewedVersion
									? 'bg-primary text-primary-foreground border-primary [&_svg]:text-primary-foreground'
									: 'bg-primary/15 border-primary/30'}"
							>
								<span class="truncate">
									{viewedVersion
										? versionLabel(viewedVersion.version)
										: draftLabel}
								</span>
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="draft" label={draftLabel}>
									{draftLabel}
								</Select.Item>
								{#each form.versions as version (version.version)}
									<Select.Item
										value={String(version.version)}
										label={versionLabel(version.version)}
									>
										{versionLabel(version.version)}
									</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
						{#if viewedVersion}
							<p class="text-muted-foreground text-sm">
								Read-only. {viewedVersion.label}. Question wording shown is the
								current wording.
								<Button
									variant="link"
									class="h-auto p-0 text-sm"
									onclick={() => viewVersion('draft')}
								>
									Back to draft
								</Button>
							</p>
						{/if}
					{/if}
				</Sheet.Header>

				<div class="flex flex-col gap-2">
					<Label for="form-country" class="text-base">Country</Label>
					<Select.Root
						type="single"
						disabled={!!viewedVersion}
						value={form.country}
						onValueChange={(value) => (form.country = value)}
					>
						<Select.Trigger id="form-country" class="w-full">
							<span class="flex items-center gap-2">
								<span aria-hidden="true">{COUNTRY_FLAGS[form.country]}</span>
								<span class="text-muted-foreground text-sm">{form.country}</span>
							</span>
						</Select.Trigger>
						<Select.Content>
							{#each COUNTRY_OPTIONS as country (country)}
								<Select.Item value={country} label={country}>
									<span aria-hidden="true">{COUNTRY_FLAGS[country]}</span>
									<span class="text-muted-foreground text-sm">{country}</span>
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>

				<div class="border-border flex flex-col gap-3 border-t pt-4">
					<h3 class="text-base font-semibold">
						Questions in {viewedVersion ? `v${viewedVersion.version}` : 'this form'}
					</h3>
					{#if viewedVersion}
						<ul class="flex flex-col gap-2">
							{#each previewRows as row, index (row.id)}
								<li
									class="bg-card rounded-lg border {index === previewIndex
										? 'border-primary'
										: 'border-border'}"
								>
									<button
										type="button"
										class="flex w-full items-center gap-2 px-3 py-3 text-left"
										onclick={() => selectQuestion(row.question.id)}
									>
										<span
											class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
										>
											{index + 1}
										</span>
										<span class="min-w-0 flex-1 truncate text-base">
											{row.question.text}
										</span>
										{#if row.required}
											<span
												class="text-destructive font-semibold"
												title="Required">*</span
											>
											<span class="sr-only">Required</span>
										{/if}
										{#if row.question.tags[0]}
											<TagBadge tag={row.question.tags[0]} class="shrink-0" />
										{/if}
									</button>
									{#if row.question.specialCategory}
										<div class="pr-3 pb-2 pl-11">
											<TagBadge tag="Special category" />
										</div>
									{/if}
								</li>
							{/each}
						</ul>
					{:else}
						{#if rows.length === 0}
							<p class="text-muted-foreground text-base">No questions yet.</p>
						{/if}
						<p class="sr-only" role="status" aria-live="polite">{moveNote}</p>
						<DraggableList
							items={dragRows ?? rows}
							onReorder={reorderWhileDragging}
							onCommit={commitReorder}
							class="flex flex-col gap-2"
						>
							{#snippet children(row, index)}
								<div
									class="bg-card rounded-lg border {!viewedVersion &&
									index === previewIndex
										? 'border-primary'
										: 'border-border'}"
								>
									<div class="flex items-center gap-2 py-1 pr-1 pl-3">
										<button
											type="button"
											class="flex min-w-0 flex-1 items-center gap-2 py-2 text-left"
											onclick={() => selectQuestion(row.question.id)}
										>
											<span
												class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
											>
												{index + 1}
											</span>
											<span class="truncate text-base"
												>{row.question.text}</span
											>
											{#if row.required}
												<span
													class="text-destructive font-semibold"
													title="Required">*</span
												>
												<span class="sr-only">Required</span>
											{/if}
										</button>
										{#if row.question.tags[0]}
											<TagBadge tag={row.question.tags[0]} class="shrink-0" />
										{/if}
										<DropdownMenu.Root>
											<DropdownMenu.Trigger
												class={buttonVariants({
													variant: 'ghost',
													size: 'icon'
												})}
												aria-label={`Actions for ${row.question.text}`}
											>
												<EllipsisVertical class="size-4" />
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end" class="w-52">
												<DropdownMenu.Item
													onSelect={() => toggleRequired(index)}
												>
													{row.required
														? 'Set to optional'
														: 'Set to required'}
												</DropdownMenu.Item>
												<DropdownMenu.Item
													onSelect={() => onEditQuestion(row.question.id)}
												>
													Edit question
												</DropdownMenu.Item>
												{#if row.question.specialCategory}
													<DropdownMenu.Item
														onSelect={() =>
															onEditQuestion(row.question.id, true)}
													>
														Edit consent wording
													</DropdownMenu.Item>
												{/if}
												<DropdownMenu.Item
													disabled={index === 0}
													onSelect={() => move(index, -1)}
												>
													Move up
												</DropdownMenu.Item>
												<DropdownMenu.Item
													disabled={index === rows.length - 1}
													onSelect={() => move(index, 1)}
												>
													Move down
												</DropdownMenu.Item>
												<DropdownMenu.Separator />
												<DropdownMenu.Item
													class="text-destructive"
													onSelect={() => form.questions.splice(index, 1)}
												>
													Remove from form
												</DropdownMenu.Item>
											</DropdownMenu.Content>
										</DropdownMenu.Root>
									</div>
									{#if row.question.specialCategory}
										<div class="flex items-center gap-2 pr-3 pb-2 pl-11">
											<TagBadge tag="Special category" />
											<Button
												variant="link"
												class="h-auto p-0 text-sm"
												onclick={() =>
													onEditQuestion(row.question.id, true)}
											>
												Edit consent
											</Button>
										</div>
									{/if}
								</div>
							{/snippet}
						</DraggableList>
						<Button onclick={() => (addOpen = true)}
							><Plus class="size-4" />Add question</Button
						>
					{/if}
				</div>

				{#if !isNew && conversationCount(form) > 0}
					<VersionUsage
						{form}
						viewing={viewedVersion ? viewedVersion.version : 'draft'}
					/>
				{/if}

				<div class="mt-auto flex items-center gap-2">
					<span class="text-base font-semibold">Created by</span>
					<CreatedBy name={form.createdBy} />
				</div>

				{#if !isNew && !viewedVersion}
					<AlertDialog.Root>
						<AlertDialog.Trigger
							class={buttonVariants({ variant: 'destructiveOutline' })}
						>
							<Trash2 class="size-4" />Delete this form
						</AlertDialog.Trigger>
						<AlertDialog.Content>
							<AlertDialog.Header>
								<AlertDialog.Title>Delete this form?</AlertDialog.Title>
								<AlertDialog.Description>
									{conversationCount(form) > 0
										? `${form.name} is used in ${conversationCount(form)} ${conversationCount(form) === 1 ? 'conversation' : 'conversations'}. Those conversations will lose their demographic step.`
										: `${form.name} will be removed. This cannot be undone.`}
								</AlertDialog.Description>
							</AlertDialog.Header>
							<AlertDialog.Footer>
								<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
								<AlertDialog.Action onclick={onDelete}
									>Delete form</AlertDialog.Action
								>
							</AlertDialog.Footer>
						</AlertDialog.Content>
					</AlertDialog.Root>
				{/if}
			</div>

			<div class="flex h-full min-h-0 flex-col gap-3">
				{#snippet consentFooter()}
					{#if preview}
						<ConsentCheck
							id={`consent-${preview.id}`}
							text={preview.consentText}
							checked={consents[preview.id] ?? false}
							onChange={(checked) => (consents[preview.id] = checked)}
						/>
					{/if}
				{/snippet}
				<div class="min-h-0 flex-1">
					<div class="flex h-full min-h-0 flex-col gap-3">
						<div class="flex items-center justify-end gap-3">
							<Button variant="outline" size="sm" onclick={resetTest}>
								<RotateCcw class="size-4" />Reset answers
							</Button>
						</div>
						<PhonePreview
							note={viewedVersion
								? `Trying v${viewedVersion.version} as a participant.`
								: 'Try it as a participant. Nothing is saved.'}
							step={finished ? Math.max(previewRows.length, 1) : previewIndex + 1}
							steps={Math.max(previewRows.length, 1)}
							showNext={!finished}
							onNext={advance}
							nextLabel={isLast ? 'Finish test' : 'Next'}
							nextDisabled={!preview || consentMissing || requiredMissing}
							footer={!finished && preview?.specialCategory
								? consentFooter
								: undefined}
						>
							{#if finished}
								<h2 class="text-2xl font-semibold">Test complete</h2>
								<ul class="flex flex-col gap-3">
									{#each results as result (result.id)}
										<li class="flex flex-col">
											<span class="text-base font-medium">
												{result.text}{result.required ? ' *' : ''}
											</span>
											<span class="text-muted-foreground text-base">
												{result.summary || 'Skipped'}
											</span>
										</li>
									{/each}
								</ul>
								<Button variant="outline" onclick={resetTest}>Start again</Button>
							{:else if preview}
								{@const current = preview}
								<h2 class="text-2xl font-semibold">{preview.text}</h2>
								{#if preview.description}
									<p class="text-muted-foreground text-base">
										{preview.description}
									</p>
								{/if}
								<QuestionAnswers
									question={current}
									answer={answers[current.id] ?? emptyAnswer()}
									onAnswer={(next) => (answers[current.id] = next)}
								/>
							{:else}
								<p class="text-muted-foreground text-base">
									Add a question to see how participants will see it.
								</p>
							{/if}
						</PhonePreview>
					</div>
				</div>
			</div>
		</div>

		<Sheet.Footer
			class="border-border flex-row flex-wrap items-center justify-end gap-2 border-t px-6 py-4"
		>
			<span class="text-muted-foreground mr-auto text-sm">
				{#if viewedVersion}Viewing v{viewedVersion.version}. Go back to the draft to save or
					publish.{:else}{#if form.hasUnpublishedChanges}Unpublished changes saved as
						draft.
					{/if}Publishing creates v{form.version + 1}{/if}
			</span>
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			<Button
				variant="outline"
				disabled={!hasName || !hasChanges || !!viewedVersion}
				onclick={onSaveDraft}
			>
				Save draft
			</Button>
			<Button disabled={!canPublish || !!viewedVersion} onclick={() => (publishOpen = true)}
				>Publish</Button
			>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

{#if publishOpen}
	<PublishDialog
		{form}
		onConfirm={() => {
			publishOpen = false;
			onPublish();
		}}
		onClose={() => (publishOpen = false)}
	/>
{/if}

{#if addOpen}
	<AddQuestionsDialog
		{questions}
		inFormIds={form.questions.map((q) => q.questionId)}
		onAdd={addQuestions}
		onCreateQuestion={createFromDialog}
		onClose={() => (addOpen = false)}
	/>
{/if}
