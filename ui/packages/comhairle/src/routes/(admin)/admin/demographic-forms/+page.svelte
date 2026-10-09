<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import FormSheet from './FormSheet.svelte';
	import FormsTab from './FormsTab.svelte';
	import QuestionBankTab from './QuestionBankTab.svelte';
	import QuestionSheet from './QuestionSheet.svelte';
	import {
		blankForm,
		blankQuestion,
		createInitialForms,
		createInitialQuestions,
		CURRENT_USER,
		type DemographicForm,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type QuestionEdit = { question: DemographicQuestion; isNew: boolean; addsToForm: boolean };

	let tab = $state('forms');
	let forms = $state<DemographicForm[]>(createInitialForms());
	let questions = $state<DemographicQuestion[]>(createInitialQuestions());
	let formEdit = $state<{
		form: DemographicForm;
		original: DemographicForm;
		isNew: boolean;
	} | null>(null);
	let questionEdit = $state<QuestionEdit | null>(null);

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

	function copyForm(formId: string) {
		const source = forms.find((f) => f.id === formId);
		if (!source) return;
		const form: DemographicForm = {
			...$state.snapshot(source),
			id: blankForm().id,
			name: `${source.name} (copy)`,
			createdBy: CURRENT_USER,
			status: 'draft',
			version: 0,
			usedInConversations: 0,
			editedLabel: 'Not yet published'
		};
		formEdit = { form, original: $state.snapshot(form), isNew: true };
	}

	const formHasChanges = $derived(
		formEdit !== null &&
			(formEdit.isNew ||
				JSON.stringify(formEdit.original) !==
					JSON.stringify($state.snapshot(formEdit.form)))
	);

	function saveForm() {
		if (!formEdit) return;
		const draft = $state.snapshot(formEdit.form);
		const saved: DemographicForm = {
			...draft,
			status: 'published',
			version: draft.version + 1,
			editedLabel: `Edited just now by ${CURRENT_USER}`,
			isNewlyCreated: formEdit.isNew || draft.isNewlyCreated
		};
		const index = forms.findIndex((f) => f.id === saved.id);
		if (index >= 0) forms[index] = saved;
		else forms.push(saved);
		formEdit = null;
	}

	function saveFormAsNew() {
		if (!formEdit) return;
		const draft = $state.snapshot(formEdit.form);
		const renamed = draft.name === formEdit.original.name;
		forms.push({
			...draft,
			id: blankForm().id,
			name: renamed ? `${draft.name} (copy)` : draft.name,
			createdBy: CURRENT_USER,
			status: 'published',
			version: 1,
			usedInConversations: 0,
			editedLabel: `Created just now by ${CURRENT_USER}`,
			isNewlyCreated: true
		});
		formEdit = null;
		tab = 'forms';
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

	function editQuestion(questionId: string) {
		const question = questions.find((q) => q.id === questionId);
		if (question) {
			questionEdit = { question: $state.snapshot(question), isNew: false, addsToForm: false };
		}
	}

	function saveQuestion(saved: DemographicQuestion) {
		const index = questions.findIndex((q) => q.id === saved.id);
		if (index >= 0) questions[index] = saved;
		else questions.push(saved);
		if (questionEdit?.addsToForm && formEdit) {
			formEdit.form.questions.push({ questionId: saved.id, required: false });
		}
		questionEdit = null;
	}
</script>

<svelte:head>
	<title>Demographic forms - Comhairle Admin</title>
</svelte:head>

<div class="mx-auto w-11/12 max-w-6xl p-10">
	<header class="flex flex-wrap items-start justify-between gap-4">
		<div class="flex flex-col gap-2">
			<h1 class="text-4xl font-bold">Demographic forms</h1>
			<p class="text-muted-foreground max-w-2xl text-base">
				Build questions, put them together into forms, then add a form to any conversation
				as a demographic step.
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
			<FormsTab {forms} {questions} onNew={newForm} onEdit={editForm} onCopy={copyForm} />
		</Tabs.Content>
		<Tabs.Content value="questions" class="mt-6">
			<QuestionBankTab {questions} {forms} onEdit={editQuestion} />
		</Tabs.Content>
	</Tabs.Root>
</div>

{#if formEdit}
	<FormSheet
		bind:form={formEdit.form}
		isNew={formEdit.isNew}
		{questions}
		onEditQuestion={editQuestion}
		onCreateQuestion={() => newQuestion(true)}
		hasChanges={formHasChanges}
		onSave={saveForm}
		onSaveAsNew={saveFormAsNew}
		onDelete={deleteForm}
		onClose={() => (formEdit = null)}
	/>
{/if}

{#if questionEdit}
	<QuestionSheet
		question={questionEdit.question}
		isNew={questionEdit.isNew}
		addsToForm={questionEdit.addsToForm}
		onSave={saveQuestion}
		onClose={() => (questionEdit = null)}
	/>
{/if}
