<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import FormSheet from './FormSheet.svelte';
	import FormsTab from './FormsTab.svelte';
	import QuestionBankTab from './QuestionBankTab.svelte';
	import QuestionSheet from './QuestionSheet.svelte';
	import { createStarterForm, createStarterQuestions } from './starterSet';
	import {
		blankForm,
		blankQuestion,
		createInitialForms,
		createInitialQuestions,
		usedInFormsCount,
		CURRENT_USER,
		type DemographicForm,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type QuestionEdit = {
		question: DemographicQuestion;
		isNew: boolean;
		addsToForm: boolean;
		focusConsent?: boolean;
	};

	let tab = $state('forms');

	// Prototype only: lets you show the app with sample data or as a brand new organisation.
	let dataState = $state<'sample' | 'new'>('sample');
	let forms = $state<DemographicForm[]>(createInitialForms());
	let questions = $state<DemographicQuestion[]>(createInitialQuestions());
	let formEdit = $state<{
		form: DemographicForm;
		original: DemographicForm;
		isNew: boolean;
	} | null>(null);
	let questionEdit = $state<QuestionEdit | null>(null);
	// Id of the row added most recently. Shown highlighted until the next thing the user does.
	let highlightId = $state<string | null>(null);

	function setDataState(next: 'sample' | 'new') {
		if (next === dataState) return;
		dataState = next;
		forms = next === 'sample' ? createInitialForms() : [createStarterForm()];
		questions = next === 'sample' ? createInitialQuestions() : createStarterQuestions();
		formEdit = null;
		questionEdit = null;
		highlightId = null;
		tab = 'forms';
	}

	function clearHighlight() {
		highlightId = null;
	}

	function newForm() {
		const form = blankForm();
		formEdit = { form, original: $state.snapshot(form), isNew: true };
	}

	function editForm(formId: string) {
		const form = forms.find((f) => f.id === formId);
		if (form) {
			formEdit = {
				form: $state.snapshot(form),
				original: $state.snapshot(form),
				isNew: false
			};
		}
	}

	// Adds a draft copy straight to the table, marked "New", and leaves the original untouched.
	function duplicateForm(formId: string) {
		const source = forms.find((f) => f.id === formId);
		if (!source) return;
		const id = blankForm().id;
		forms.unshift({
			...$state.snapshot(source),
			id,
			name: `${source.name} (copy)`,
			createdBy: CURRENT_USER,
			status: 'draft',
			version: 0,
			hasUnpublishedChanges: false,
			usage: [],
			versions: [],
			editedLabel: `Draft created just now by ${CURRENT_USER}`,
			isNewlyCreated: true
		});
		highlightId = id;
		tab = 'forms';
	}

	const formHasChanges = $derived(
		formEdit !== null &&
			(formEdit.isNew ||
				JSON.stringify(formEdit.original) !==
					JSON.stringify($state.snapshot(formEdit.form)))
	);

	function upsertForm(saved: DemographicForm) {
		const index = forms.findIndex((f) => f.id === saved.id);
		if (index >= 0) forms[index] = saved;
		else {
			forms.unshift(saved);
			highlightId = saved.id;
		}
		formEdit = null;
	}

	function saveDraft() {
		if (!formEdit) return;
		const draft = $state.snapshot(formEdit.form);
		upsertForm({
			...draft,
			hasUnpublishedChanges: draft.version > 0,
			editedLabel: `Draft saved just now by ${CURRENT_USER}`,
			isNewlyCreated: formEdit.isNew || draft.isNewlyCreated
		});
	}

	function publishForm() {
		if (!formEdit) return;
		const draft = $state.snapshot(formEdit.form);
		upsertForm({
			...draft,
			status: 'published',
			version: draft.version + 1,
			versions: [
				{
					version: draft.version + 1,
					label: `Published just now by ${CURRENT_USER}`,
					questions: draft.questions.map((q) => ({ ...q }))
				},
				...draft.versions
			],
			hasUnpublishedChanges: false,
			editedLabel: `Published just now by ${CURRENT_USER}`,
			isNewlyCreated: formEdit.isNew || draft.isNewlyCreated
		});
	}

	function deleteForm() {
		if (!formEdit) return;
		const id = formEdit.form.id;
		forms = forms.filter((f) => f.id !== id);
		formEdit = null;
	}

	function newQuestion(addsToForm: boolean) {
		questionEdit = { question: blankQuestion(), isNew: true, addsToForm };
	}

	function editQuestion(questionId: string, focusConsent = false) {
		const question = questions.find((q) => q.id === questionId);
		if (question) {
			questionEdit = {
				question: $state.snapshot(question),
				isNew: false,
				addsToForm: false,
				focusConsent
			};
		}
	}

	function duplicateQuestion(questionId: string) {
		const source = questions.find((q) => q.id === questionId);
		if (!source) return;
		const copy = $state.snapshot(source);
		const id = blankQuestion().id;
		questions.unshift({ ...copy, id, text: `${copy.text} (copy)` });
		highlightId = id;
	}

	function deleteQuestion() {
		if (!questionEdit) return;
		const id = questionEdit.question.id;
		questions = questions.filter((q) => q.id !== id);
		for (const form of forms) {
			if (form.questions.some((q) => q.questionId === id)) {
				form.questions = form.questions.filter((q) => q.questionId !== id);
				if (form.version > 0) form.hasUnpublishedChanges = true;
			}
		}
		if (formEdit)
			formEdit.form.questions = formEdit.form.questions.filter((q) => q.questionId !== id);
		questionEdit = null;
	}

	// Keeps the original question untouched and adds the edited version to the bank as a new one.
	function saveQuestionAsNew(edited: DemographicQuestion) {
		const original = questions.find((q) => q.id === edited.id);
		const text =
			original && edited.text.trim() === original.text.trim()
				? `${edited.text.trim()} (copy)`
				: edited.text;
		const id = blankQuestion().id;
		questions.unshift({ ...edited, id, text });
		highlightId = id;
		questionEdit = null;
	}

	const formsUsing = (questionId: string) =>
		forms.filter((f) => f.questions.some((q) => q.questionId === questionId));

	function saveQuestion(saved: DemographicQuestion) {
		const index = questions.findIndex((q) => q.id === saved.id);
		if (index >= 0) {
			questions[index] = saved;
			// Published forms now differ from what participants see, so flag them.
			for (const form of formsUsing(saved.id)) {
				if (form.version > 0) form.hasUnpublishedChanges = true;
			}
		} else {
			questions.unshift(saved);
			highlightId = saved.id;
		}
		if (questionEdit?.addsToForm && formEdit) {
			formEdit.form.questions.push({ questionId: saved.id, required: false });
		}
		questionEdit = null;
	}
</script>

<svelte:window onpointerdown={clearHighlight} onkeydown={clearHighlight} />

<svelte:head>
	<title>Demographic forms - Comhairle Admin</title>
</svelte:head>

<div class="bg-nav-background min-h-full">
	<div class="mx-auto w-11/12 max-w-6xl p-10">
		<div
			class="border-border mb-6 flex flex-wrap items-center justify-end gap-2 rounded-lg border border-dashed px-3 py-2"
		>
			<span class="text-muted-foreground mr-2 text-sm">Prototype data</span>
			<Button
				size="sm"
				variant={dataState === 'sample' ? 'default' : 'outline'}
				aria-pressed={dataState === 'sample'}
				onclick={() => setDataState('sample')}
			>
				Sample organisation
			</Button>
			<Button
				size="sm"
				variant={dataState === 'new' ? 'default' : 'outline'}
				aria-pressed={dataState === 'new'}
				onclick={() => setDataState('new')}
			>
				New organisation
			</Button>
		</div>
		<header class="flex flex-wrap items-start justify-between gap-4">
			<div class="flex flex-col gap-2">
				<h1 class="text-4xl font-bold">Demographic forms</h1>
				<p class="text-muted-foreground max-w-2xl text-base">
					Build questions, put them together into forms, then add a form to any
					conversation as a demographic step.
				</p>
			</div>
			{#if tab === 'forms'}
				<Button onclick={newForm}>New form</Button>
			{:else}
				<Button onclick={() => newQuestion(false)}>New question</Button>
			{/if}
		</header>

		<Tabs.Root bind:value={tab} class="mt-6">
			<Tabs.List>
				<Tabs.Trigger value="forms">Forms ({forms.length})</Tabs.Trigger>
				<Tabs.Trigger value="questions">Question bank ({questions.length})</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="forms" class="mt-6">
				<FormsTab
					{forms}
					{questions}
					{highlightId}
					onNew={newForm}
					onEdit={editForm}
					onDuplicate={duplicateForm}
				/>
			</Tabs.Content>
			<Tabs.Content value="questions" class="mt-6">
				<QuestionBankTab
					{questions}
					{forms}
					{highlightId}
					onEdit={editQuestion}
					onDuplicate={duplicateQuestion}
				/>
			</Tabs.Content>
		</Tabs.Root>
	</div>
</div>

{#if formEdit}
	<FormSheet
		bind:form={formEdit.form}
		isNew={formEdit.isNew}
		{questions}
		onEditQuestion={editQuestion}
		onCreateQuestion={() => newQuestion(true)}
		hasChanges={formHasChanges}
		onSaveDraft={saveDraft}
		onPublish={publishForm}
		onDelete={deleteForm}
		onClose={() => (formEdit = null)}
	/>
{/if}

{#if questionEdit}
	<QuestionSheet
		question={questionEdit.question}
		isNew={questionEdit.isNew}
		addsToForm={questionEdit.addsToForm}
		focusConsent={questionEdit.focusConsent ?? false}
		onSave={saveQuestion}
		onSaveAsNew={saveQuestionAsNew}
		onDelete={deleteQuestion}
		usedInForms={usedInFormsCount(questionEdit.question.id, forms)}
		usedInFormNames={formsUsing(questionEdit.question.id).map((f) => f.name)}
		onClose={() => (questionEdit = null)}
	/>
{/if}
