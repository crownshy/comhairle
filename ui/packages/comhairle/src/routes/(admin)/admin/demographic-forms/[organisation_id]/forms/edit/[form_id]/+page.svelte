<script lang="ts">
	import { EllipsisVertical, GripVertical, Plus, RotateCcw, Trash2 } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import TagBadge from '../../../TagBadge.svelte';
	import ConsentCheck from '../../../ConsentCheck.svelte';
	import {
		emptyAnswer,
		isAnswered,
		PREFER_NOT_TO_SAY,
		summariseAnswer,
		type Answer
	} from '../../../../demographicAnswers';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Form, Input } from '$lib/components/EasyForm';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import AddQuestionsDialog from '../../../AddQuestionsDialog.svelte';
	import PublishDialog from '../../../PublishDialog.svelte';
	import PhonePreview from '../../../PhonePreview.svelte';
	import QuestionAnswers from '../../../QuestionAnswers.svelte';
	import {
		nextVersionNumber,
		type DemographicForm,
		type SavedQuestion
	} from '../../../../demographicModel';
	import TabContent from '$lib/components/TabContent.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import type { DemographicFormEntryDto, QuestionLatestDto } from '@crownshy/api-client/api';

	// type Props = {
	// 	form: DemographicForm;
	// 	isNew: boolean;
	// 	questions: SavedQuestion[];
	// 	hasChanges: boolean;
	// 	onEditQuestion: (questionId: string, focusConsent?: boolean) => void;
	// 	onCreateQuestion: () => void;
	// 	onSaveDraft: () => void;
	// 	onPublish: () => void;
	// 	onSaveAsNew: () => void;
	// 	onDelete: () => void;
	// 	onClose: () => void;
	// };
	//
	// let addOpen = $state(false);
	// let publishOpen = $state(false);
	// let consents = $state<Record<string, boolean>>({});
	// let answers = $state<Record<string, Answer>>({});
	// let previewId = $state<string | null>(null);
	// let finished = $state(false);
	// let dragRows = $state<Row[] | null>(null);
	//
	// let formEdit = $state<FormEdit | null>(null);
	//
	// function newForm() {
	// 	const form = blankForm();
	// 	formEdit = { form, original: $state.snapshot(form), isNew: true };
	// }
	//
	// function editForm(formId: string) {
	// 	const form = data.forms.find((f) => f.id === formId);
	// 	if (form) {
	// 		const snapshot = $state.snapshot(form);
	// 		formEdit = { form: snapshot, original: $state.snapshot(snapshot), isNew: false };
	// 	}
	// }
	//
	// function copyForm(formId: string) {
	// 	const source = data.forms.find((f) => f.id === formId);
	// 	if (!source) return;
	// 	const form: DemographicForm = {
	// 		...blankForm(),
	// 		name: `${source.name} (copy)`,
	// 		questions: source.questions.map((entry) => ({ ...entry }))
	// 	};
	// 	formEdit = { form, original: $state.snapshot(form), isNew: true };
	// }
	//
	// const formHasChanges = $derived(
	// 	formEdit !== null &&
	// 		(formEdit.isNew ||
	// 			JSON.stringify(formEdit.original) !==
	// 				JSON.stringify($state.snapshot(formEdit.form)))
	// );
	//
	// async function commitForm(form: DemographicForm, publish: boolean) {
	// 	if (data.organizationId === null) return;
	// 	const result = await persistForm(apiClient, data.organizationId, form, publish);
	// 	if (result.err !== null) {
	// 		notifyError('Could not save the form. Try again.');
	// 		return;
	// 	}
	// 	formEdit = null;
	// 	await reload();
	// }
	//
	// function onSaveDraft(publish: boolean) {
	// 	if (!formEdit) return;
	// 	return commitForm($state.snapshot(formEdit.form), publish);
	// }
	//
	// function onSaveFormAsNew() {
	// 	if (!formEdit) return;
	// 	const draft = $state.snapshot(formEdit.form);
	// 	const unchangedName = draft.name === formEdit.original.name;
	// 	return commitForm(
	// 		{
	// 			...draft,
	// 			id: null,
	// 			name: unchangedName ? `${draft.name} (copy)` : draft.name,
	// 			publishedVersion: null,
	// 			draftVersion: null
	// 		},
	// 		false
	// 	);
	// }
	//
	// async function onDelete() {
	// 	const id = formEdit?.form.id;
	// 	if (!id || data.organizationId === null) return;
	// 	const result = await archiveForm(apiClient, data.organizationId, id);
	// 	if (result.err !== null) {
	// 		notifyError('Could not delete the form. Try again.');
	// 		return;
	// 	}
	// 	formEdit = null;
	// 	await reload();
	// }
	//
	// function onCreateQuestion() {
	// 	questionEdit = {
	// 		question: createNewQuestion(),
	// 		isNew: true,
	// 		addsToForm: true,
	// 		focusConsent: false
	// 	};
	// }
	//
	// function onEditQuestion(questionId: string, focusConsent = false) {
	// 	const question = data.questions.find((q) => q.id === questionId);
	// 	if (question?.isDefault) {
	// 		notifyError('Duplicate the default question in the question bank before editing it.');
	// 		return;
	// 	}
	// 	if (question) {
	// 		questionEdit = {
	// 			question: $state.snapshot(question),
	// 			isNew: false,
	// 			addsToForm: false,
	// 			focusConsent
	// 		};
	// 	}
	// }
	//
	// // bind:form={formEdit.form}
	// // isNew={formEdit.isNew}
	// // questions={data.questions}
	// // onEditQuestion={editQuestion}
	// // hasChanges={formHasChanges}
	// // onSaveDraft={() => saveForm(false)}
	// // onPublish={() => saveForm(true)}
	//
	// type Row = { id: string; question: SavedQuestion; required: boolean };
	//
	// const rows = $derived<Row[]>(
	// 	form.questions.flatMap((formQuestion) => {
	// 		const question = questions.find((q) => q.id === formQuestion.questionId);
	// 		return question ? [{ id: question.id, question, required: formQuestion.required }] : [];
	// 	})
	// );
	// const previewIndex = $derived(
	// 	Math.max(
	// 		0,
	// 		rows.findIndex((row) => row.id === previewId)
	// 	)
	// );
	// const preview = $derived(rows[previewIndex]?.question);
	// const versionLabel = $derived(`v${nextVersionNumber(form)}`);
	//
	// const consentMissing = $derived(
	// 	!!preview?.specialCategory &&
	// 		!consents[preview.id] &&
	// 		!answers[preview.id]?.selected.includes(PREFER_NOT_TO_SAY)
	// );
	// const isLast = $derived(previewIndex >= rows.length - 1);
	// const requiredMissing = $derived(
	// 	!!preview &&
	// 		(rows[previewIndex]?.required ?? false) &&
	// 		!isAnswered(answers[preview.id] ?? emptyAnswer())
	// );
	// const results = $derived(
	// 	rows.map((row) => ({
	// 		id: row.id,
	// 		text: row.question.text,
	// 		required: row.required,
	// 		summary: summariseAnswer(answers[row.id] ?? emptyAnswer())
	// 	}))
	// );
	//
	// const hasName = $derived(form.name.trim() !== '');
	// const canPublish = $derived(
	// 	hasName &&
	// 		form.questions.length > 0 &&
	// 		(hasChanges || form.draftVersion !== null || form.publishedVersion === null)
	// );
	//
	// function toggleRequired(index: number) {
	// 	form.questions[index].required = !form.questions[index].required;
	// }
	//
	// function reorderWhileDragging(next: Row[]) {
	// 	dragRows = next;
	// }
	//
	// function commitReorder(next: Row[]) {
	// 	form.questions = next.map((row) => ({
	// 		questionId: row.id,
	// 		required: row.required,
	// 		text: row.question.text
	// 	}));
	// 	dragRows = null;
	// }
	//
	// function showNextQuestion() {
	// 	const next = rows[previewIndex + 1];
	// 	if (next) previewId = next.id;
	// }
	//
	// function advance() {
	// 	if (isLast) finished = true;
	// 	else showNextQuestion();
	// }
	//
	// function resetTest() {
	// 	answers = {};
	// 	consents = {};
	// 	finished = false;
	// 	previewId = rows[0]?.id ?? null;
	// }
	//
	// function selectQuestion(id: string) {
	// 	finished = false;
	// 	previewId = id;
	// }
	//
	// function move(index: number, offset: number) {
	// 	const target = index + offset;
	// 	if (target < 0 || target >= form.questions.length) return;
	// 	[form.questions[index], form.questions[target]] = [
	// 		form.questions[target],
	// 		form.questions[index]
	// 	];
	// }
	//
	// function addQuestions(ids: string[]) {
	// 	const added = ids.flatMap((questionId) => {
	// 		const question = questions.find((q) => q.id === questionId);
	// 		return question ? [{ questionId, required: false, text: question.text }] : [];
	// 	});
	// 	form.questions.push(...added);
	// 	addOpen = false;
	// }
	//
	// function createFromDialog() {
	// 	addOpen = false;
	// 	onCreateQuestion();
	// }

	const { data } = $props();

	let order = $state<QuestionLatestDto[]>([]);
	// TODO: Swap to use $derived when Svelte is updated
	$effect(() => {
		data.streamed.then((res) => {
			if (res.ok !== null) {
				order = res.ok.questions;
			}
		});
	});
</script>

{#await data.streamed}
	<div class="flex h-full w-full items-center justify-center">
		<Spinner
			class="text-muted-foreground h-1/4 w-1/4"
			strokeWidth={0.5}
			aria-label="Loading form"
		/>
	</div>
{:then demographics}
	{#if demographics.err !== null}
		<ErrorMessage>Could not load form, please try again</ErrorMessage>
	{:else}
		<!-- TODO: Copied from SectionHeader, might be good to unify at some point -->
		<header class="border-border flex w-full justify-between border-b pt-6 pb-5">
			<h1 class="text-foreground pl-5 text-3xl font-bold">{demographics.ok.form.name}</h1>
			<div class="mr-10">
				<Button variant="outline" class="mr-2">Save as draft</Button>
				<Button>Publish</Button>
			</div>
		</header>

		<section class="pt-page-top px-5">
			<div class="grid grid-cols-[1fr_3fr_2fr] gap-x-5">
				<section>
					<Input
						label="Name"
						placeholder="Default form"
						value={demographics.ok.form.name}
					/>
					<h3 class="mb-2 pt-5 text-sm font-semibold">Questions</h3>
					{#if demographics.ok.form.entries.length === 0}
						<div class="text-muted-foreground text-sm font-medium italic">
							No questions yet
						</div>
					{:else if order.length !== 0}
						<DraggableList
							items={order}
							onReorder={(next) => void (order = next)}
							class="flex flex-col gap-2"
						>
							{#snippet children(question, index)}
								<div
									class="border-border bg-card flex items-center rounded-xl border px-3 py-3"
								>
									<GripVertical class="text-muted-foreground/70 size-4" />
									<div class="flex flex-col">
										<div class="flex items-center gap-1">
											<div
												class="text-muted-foreground ml-2 text-sm font-semibold"
											>
												{index + 1}.
											</div>
											<h4
												class="w-45 truncate text-sm font-medium"
												title={question.text}
											>
												{question.text}
											</h4>
										</div>
									</div>
								</div>
							{/snippet}
						</DraggableList>
					{/if}
				</section>
				<section>Preview</section>
				<section>Question</section>
			</div>

			<!-- <div class="border-border flex flex-col gap-3 border-t pt-4"> -->
			<!-- 	<h3 class="text-base font-semibold">Questions in this form</h3> -->
			<!-- 	{#if rows.length === 0} -->
			<!-- 		<p class="text-muted-foreground text-base">No questions yet.</p> -->
			<!-- 	{/if} -->
			<!-- 	<DraggableList -->
			<!-- 		items={dragRows ?? rows} -->
			<!-- 		onReorder={reorderWhileDragging} -->
			<!-- 		onCommit={commitReorder} -->
			<!-- 		class="flex flex-col gap-2" -->
			<!-- 	> -->
			<!-- 		{#snippet children(row, index)} -->
			<!-- 			<div class="flex flex-col gap-1"> -->
			<!-- 				<div -->
			<!-- 					class="bg-card flex items-center gap-2 rounded-lg border py-1 pr-1 pl-3 {index === -->
			<!-- 					previewIndex -->
			<!-- 						? 'border-primary' -->
			<!-- 						: 'border-border'}" -->
			<!-- 				> -->
			<!-- 					<button -->
			<!-- 						type="button" -->
			<!-- 						class="flex min-w-0 flex-1 items-center gap-2 py-2 text-left" -->
			<!-- 						onclick={() => selectQuestion(row.id)} -->
			<!-- 					> -->
			<!-- 						<span -->
			<!-- 							class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-full text-sm font-semibold" -->
			<!-- 						> -->
			<!-- 							{index + 1} -->
			<!-- 						</span> -->
			<!-- 						<span class="truncate text-base">{row.question.text}</span> -->
			<!-- 						{#if row.required} -->
			<!-- 							<span class="text-destructive font-semibold" title="Required"> -->
			<!-- 								* -->
			<!-- 							</span> -->
			<!-- 							<span class="sr-only">Required</span> -->
			<!-- 						{/if} -->
			<!-- 					</button> -->
			<!-- 					{#if row.question.tags[0]} -->
			<!-- 						<TagBadge tag={row.question.tags[0].name} class="shrink-0" /> -->
			<!-- 					{/if} -->
			<!-- 					<DropdownMenu.Root> -->
			<!-- 						<DropdownMenu.Trigger -->
			<!-- 							class={buttonVariants({ -->
			<!-- 								variant: 'ghost', -->
			<!-- 								size: 'icon' -->
			<!-- 							})} -->
			<!-- 							aria-label={`Actions for ${row.question.text}`} -->
			<!-- 						> -->
			<!-- 							<EllipsisVertical class="size-4" /> -->
			<!-- 						</DropdownMenu.Trigger> -->
			<!-- 						<DropdownMenu.Content align="end" class="w-52"> -->
			<!-- 							<DropdownMenu.Item onSelect={() => toggleRequired(index)}> -->
			<!-- 								{row.required ? 'Set to optional' : 'Set to required'} -->
			<!-- 							</DropdownMenu.Item> -->
			<!-- 							{#if row.question.isDefault} -->
			<!-- 								<DropdownMenu.Label>Default question</DropdownMenu.Label> -->
			<!-- 							{:else} -->
			<!-- 								<DropdownMenu.Item onSelect={() => onEditQuestion(row.id)}> -->
			<!-- 									Edit question -->
			<!-- 								</DropdownMenu.Item> -->
			<!-- 								{#if row.question.specialCategory} -->
			<!-- 									<DropdownMenu.Item -->
			<!-- 										onSelect={() => onEditQuestion(row.id, true)} -->
			<!-- 									> -->
			<!-- 										Edit consent wording -->
			<!-- 									</DropdownMenu.Item> -->
			<!-- 								{/if} -->
			<!-- 							{/if} -->
			<!-- 							<DropdownMenu.Item -->
			<!-- 								disabled={index === 0} -->
			<!-- 								onSelect={() => move(index, -1)} -->
			<!-- 							> -->
			<!-- 								Move up -->
			<!-- 							</DropdownMenu.Item> -->
			<!-- 							<DropdownMenu.Item -->
			<!-- 								disabled={index === rows.length - 1} -->
			<!-- 								onSelect={() => move(index, 1)} -->
			<!-- 							> -->
			<!-- 								Move down -->
			<!-- 							</DropdownMenu.Item> -->
			<!-- 							<DropdownMenu.Separator /> -->
			<!-- 							<DropdownMenu.Item -->
			<!-- 								class="text-destructive" -->
			<!-- 								onSelect={() => form.questions.splice(index, 1)} -->
			<!-- 							> -->
			<!-- 								Remove from form -->
			<!-- 							</DropdownMenu.Item> -->
			<!-- 						</DropdownMenu.Content> -->
			<!-- 					</DropdownMenu.Root> -->
			<!-- 				</div> -->
			<!-- 				{#if row.question.specialCategory} -->
			<!-- 					<div class="flex items-center gap-2 pr-3 pb-2 pl-11"> -->
			<!-- 						<TagBadge tag="Special category" /> -->
			<!-- 						{#if row.question.isDefault} -->
			<!-- 							<Badge variant="secondary" class="text-sm">Default</Badge> -->
			<!-- 						{:else} -->
			<!-- 							<Button -->
			<!-- 								variant="link" -->
			<!-- 								class="h-auto p-0 text-sm" -->
			<!-- 								onclick={() => onEditQuestion(row.id, true)} -->
			<!-- 							> -->
			<!-- 								Edit consent -->
			<!-- 							</Button> -->
			<!-- 						{/if} -->
			<!-- 					</div> -->
			<!-- 				{/if} -->
			<!-- 			</div> -->
			<!-- 		{/snippet} -->
			<!-- 	</DraggableList> -->
			<!-- 	<Button onclick={() => (addOpen = true)}><Plus class="size-4" />Add question</Button> -->
			<!-- </div> -->
			<!---->
			<!-- {#if !isNew} -->
			<!-- 	<AlertDialog.Root> -->
			<!-- 		<AlertDialog.Trigger -->
			<!-- 			class="{buttonVariants({ variant: 'destructiveOutline' })} mt-auto" -->
			<!-- 		> -->
			<!-- 			<Trash2 class="size-4" />Delete this form -->
			<!-- 		</AlertDialog.Trigger> -->
			<!-- 		<AlertDialog.Content> -->
			<!-- 			<AlertDialog.Header> -->
			<!-- 				<AlertDialog.Title>Delete this form?</AlertDialog.Title> -->
			<!-- 				<AlertDialog.Description> -->
			<!-- 					{form.name} will be archived and hidden from your forms. -->
			<!-- 				</AlertDialog.Description> -->
			<!-- 			</AlertDialog.Header> -->
			<!-- 			<AlertDialog.Footer> -->
			<!-- 				<AlertDialog.Cancel>Cancel</AlertDialog.Cancel> -->
			<!-- 				<AlertDialog.Action onclick={onDelete}>Delete form</AlertDialog.Action> -->
			<!-- 			</AlertDialog.Footer> -->
			<!-- 		</AlertDialog.Content> -->
			<!-- 	</AlertDialog.Root> -->
			<!-- {/if} -->
			<!---->
			<!-- <div class="flex h-full min-h-0 flex-col gap-3"> -->
			<!-- 	{#snippet consentFooter()} -->
			<!-- 		{#if preview} -->
			<!-- 			<ConsentCheck -->
			<!-- 				id={`consent-${preview.id}`} -->
			<!-- 				text={preview.consentText} -->
			<!-- 				checked={consents[preview.id] ?? false} -->
			<!-- 				onChange={(checked) => (consents[preview.id] = checked)} -->
			<!-- 			/> -->
			<!-- 		{/if} -->
			<!-- 	{/snippet} -->
			<!-- 	<div class="flex items-center justify-end"> -->
			<!-- 		<Button variant="outline" size="sm" onclick={resetTest}> -->
			<!-- 			<RotateCcw class="size-4" />Reset answers -->
			<!-- 		</Button> -->
			<!-- 	</div> -->
			<!-- 	<PhonePreview -->
			<!-- 		note="Try it as a participant. Nothing is saved." -->
			<!-- 		step={finished ? Math.max(rows.length, 1) : previewIndex + 1} -->
			<!-- 		steps={Math.max(rows.length, 1)} -->
			<!-- 		showNext={!finished} -->
			<!-- 		onNext={advance} -->
			<!-- 		nextLabel={isLast ? 'Finish test' : 'Next'} -->
			<!-- 		nextDisabled={!preview || consentMissing || requiredMissing} -->
			<!-- 		footer={!finished && preview?.specialCategory ? consentFooter : undefined} -->
			<!-- 	> -->
			<!-- 		{#if finished} -->
			<!-- 			<h2 class="text-2xl font-semibold">Test complete</h2> -->
			<!-- 			<ul class="flex flex-col gap-3"> -->
			<!-- 				{#each results as result (result.id)} -->
			<!-- 					<li class="flex flex-col"> -->
			<!-- 						<span class="text-base font-medium"> -->
			<!-- 							{result.text}{result.required ? ' *' : ''} -->
			<!-- 						</span> -->
			<!-- 						<span class="text-muted-foreground text-base"> -->
			<!-- 							{result.summary || 'Skipped'} -->
			<!-- 						</span> -->
			<!-- 					</li> -->
			<!-- 				{/each} -->
			<!-- 			</ul> -->
			<!-- 			<Button variant="outline" onclick={resetTest}>Start again</Button> -->
			<!-- 		{:else if preview} -->
			<!-- 			{@const current = preview} -->
			<!-- 			<h2 class="text-2xl font-semibold">{preview.text}</h2> -->
			<!-- 			<QuestionAnswers -->
			<!-- 				question={current} -->
			<!-- 				answer={answers[current.id] ?? emptyAnswer()} -->
			<!-- 				onAnswer={(next) => (answers[current.id] = next)} -->
			<!-- 			/> -->
			<!-- 		{:else} -->
			<!-- 			<p class="text-muted-foreground text-base"> -->
			<!-- 				Add a question to see how participants will see it. -->
			<!-- 			</p> -->
			<!-- 		{/if} -->
			<!-- 	</PhonePreview> -->
			<!-- </div> -->
		</section>
	{/if}
{/await}
<!-- <span class="text-muted-foreground mr-auto text-sm"> -->
<!-- 	{#if !hasName}Name this form to publish it. -->
<!-- 	{:else if form.questions.length === 0}Add a question to publish this form. -->
<!-- 	{:else}Publishing creates {versionLabel}. -->
<!-- 	{/if} -->
<!-- </span> -->
<!-- <Button variant="outline" onclick={onClose}>Cancel</Button> -->
<!-- {#if !isNew} -->
<!-- 	<Button variant="outline" disabled={!hasChanges} onclick={onSaveAsNew}>Save to new form</Button> -->
<!-- {/if} -->
<!-- <Button variant="outline" disabled={!hasName || !hasChanges} onclick={onSaveDraft}> -->
<!-- 	Save draft -->
<!-- </Button> -->
<!-- <Button disabled={!canPublish} onclick={() => (publishOpen = true)}>Publish</Button> -->
<!---->
<!-- {#if publishOpen} -->
<!-- 	<PublishDialog -->
<!-- 		title="Publish {form.name} as {versionLabel}?" -->
<!-- 		description="Publishing creates a new version. Earlier published versions are kept." -->
<!-- 		actionLabel="Publish {versionLabel}" -->
<!-- 		onConfirm={() => { -->
<!-- 			publishOpen = false; -->
<!-- 			onPublish(); -->
<!-- 		}} -->
<!-- 		onClose={() => (publishOpen = false)} -->
<!-- 	/> -->
<!-- {/if} -->
<!---->
<!-- {#if addOpen} -->
<!-- 	<AddQuestionsDialog -->
<!-- 		{questions} -->
<!-- 		inFormIds={form.questions.map((q) => q.questionId)} -->
<!-- 		onAdd={addQuestions} -->
<!-- 		onCreateQuestion={createFromDialog} -->
<!-- 		onClose={() => (addOpen = false)} -->
<!-- 	/> -->
<!-- {/if} -->
