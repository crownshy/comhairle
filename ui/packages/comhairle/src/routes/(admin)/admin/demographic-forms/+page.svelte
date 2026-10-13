<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { page } from '$app/state';
	import { apiClient } from '@crownshy/api-client/client';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import * as Tabs from '$lib/components/ui/tabs';
	import { notifications } from '$lib/notifications.svelte.js';
	import { key } from '$lib/utils/invalidationKey';
	import FormSheet from './FormSheet.svelte';
	import FormsTab from './FormsTab.svelte';
	import QuestionBankTab from './QuestionBankTab.svelte';
	import QuestionSheet from './QuestionSheet.svelte';
	import {
		archiveForm,
		archiveQuestion,
		createDemographicTag,
		persistForm,
		persistQuestion
	} from './demographicApi';
	import {
		blankForm,
		blankQuestion,
		duplicateQuestionDraft,
		formsUsingQuestion,
		type DemographicForm,
		type DemographicQuestion,
		type Tag
	} from './demographicModel';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	type FormEdit = { form: DemographicForm; original: DemographicForm; isNew: boolean };
	type QuestionEdit = {
		question: DemographicQuestion;
		isNew: boolean;
		addsToForm: boolean;
		focusConsent: boolean;
	};
	type QuestionSaveOptions = { publish: boolean; asNew: boolean };

	let tab = $state('forms');
	let formEdit = $state<FormEdit | null>(null);
	let questionEdit = $state<QuestionEdit | null>(null);
	// Id of the row saved most recently. Shown highlighted until the next thing the user does.
	let highlightId = $state<string | null>(null);

	const reload = () => invalidate(key('admin/demographic-forms'));

	const selectedOrganizationName = $derived(
		data.organizations.find((org) => org.id === data.organizationId)?.name ??
			'Choose an organisation'
	);

	function selectOrganization(organizationId: string) {
		formEdit = null;
		questionEdit = null;
		highlightId = null;
		const url = new URL(page.url);
		url.searchParams.set('organization', organizationId);
		goto(url, { replaceState: true, noScroll: true, keepFocus: true });
	}

	const usedInFormList = $derived(
		questionEdit?.question.id
			? formsUsingQuestion(questionEdit.question.id, data.forms).map((form) => ({
					id: form.id,
					name: form.name
				}))
			: []
	);

	function notifyError(message: string) {
		notifications.send({ priority: 'ERROR', message });
	}

	function clearHighlight() {
		highlightId = null;
	}

	function newForm() {
		const form = blankForm();
		formEdit = { form, original: $state.snapshot(form), isNew: true };
	}

	function editForm(formId: string) {
		const form = data.forms.find((f) => f.id === formId);
		if (form) {
			const snapshot = $state.snapshot(form);
			formEdit = { form: snapshot, original: $state.snapshot(snapshot), isNew: false };
		}
	}

	function copyForm(formId: string) {
		const source = data.forms.find((f) => f.id === formId);
		if (!source) return;
		const form: DemographicForm = {
			...blankForm(),
			name: `${source.name} (copy)`,
			questions: source.questions.map((entry) => ({ ...entry }))
		};
		formEdit = { form, original: $state.snapshot(form), isNew: true };
	}

	const formHasChanges = $derived(
		formEdit !== null &&
			(formEdit.isNew ||
				JSON.stringify(formEdit.original) !==
					JSON.stringify($state.snapshot(formEdit.form)))
	);

	async function commitForm(form: DemographicForm, publish: boolean) {
		if (data.organizationId === null) return;
		const result = await persistForm(apiClient, data.organizationId, form, publish);
		if (result.err !== null) {
			notifyError('Could not save the form. Try again.');
			return;
		}
		if (form.id === null) highlightId = result.ok;
		formEdit = null;
		await reload();
	}

	function saveForm(publish: boolean) {
		if (!formEdit) return;
		return commitForm($state.snapshot(formEdit.form), publish);
	}

	function saveFormAsNew() {
		if (!formEdit) return;
		const draft = $state.snapshot(formEdit.form);
		const unchangedName = draft.name === formEdit.original.name;
		return commitForm(
			{
				...draft,
				id: null,
				name: unchangedName ? `${draft.name} (copy)` : draft.name,
				publishedVersion: null,
				draftVersion: null
			},
			false
		);
	}

	async function deleteForm() {
		const id = formEdit?.form.id;
		if (!id || data.organizationId === null) return;
		const result = await archiveForm(apiClient, data.organizationId, id);
		if (result.err !== null) {
			notifyError('Could not delete the form. Try again.');
			return;
		}
		formEdit = null;
		await reload();
	}

	function newQuestion(addsToForm: boolean) {
		questionEdit = { question: blankQuestion(), isNew: true, addsToForm, focusConsent: false };
	}

	function editQuestion(questionId: string, focusConsent = false) {
		const question = data.questions.find((q) => q.id === questionId);
		if (question?.isDefault) {
			notifyError('Duplicate the default question in the question bank before editing it.');
			return;
		}
		if (question) {
			questionEdit = {
				question: $state.snapshot(question),
				isNew: false,
				addsToForm: false,
				focusConsent
			};
		}
	}

	// Opens a pre-filled new question so nothing is written until the user saves it.
	function duplicateQuestion(questionId: string) {
		const source = data.questions.find((q) => q.id === questionId);
		if (!source) return;
		questionEdit = {
			question: duplicateQuestionDraft($state.snapshot(source)),
			isNew: true,
			addsToForm: false,
			focusConsent: false
		};
	}

	async function saveQuestion(question: DemographicQuestion, options: QuestionSaveOptions) {
		if (data.organizationId === null) return;
		const addsToForm = questionEdit?.addsToForm ?? false;
		const original = data.questions.find((q) => q.id === question.id);
		const unchangedText =
			original !== undefined && question.text.trim() === original.text.trim();
		const toPersist: DemographicQuestion = options.asNew
			? {
					...question,
					id: null,
					text: unchangedText ? `${question.text.trim()} (copy)` : question.text
				}
			: question;
		const result = await persistQuestion(
			apiClient,
			data.organizationId,
			toPersist,
			options.publish
		);
		if (result.err !== null) {
			notifyError('Could not save the question. Try again.');
			return;
		}
		if (addsToForm && options.publish && formEdit) {
			formEdit.form.questions.push({
				questionId: result.ok,
				required: false,
				text: question.text.trim()
			});
		}
		if (toPersist.id === null) highlightId = result.ok;
		questionEdit = null;
		await reload();
	}

	async function deleteQuestion() {
		const id = questionEdit?.question.id;
		if (!id || data.organizationId === null) return;
		const result = await archiveQuestion(apiClient, data.organizationId, id);
		if (result.err !== null) {
			notifyError('Could not delete the question. Try again.');
			return;
		}
		if (formEdit) {
			formEdit.form.questions = formEdit.form.questions.filter((q) => q.questionId !== id);
		}
		questionEdit = null;
		await reload();
	}

	async function createTag(name: string): Promise<Tag | null> {
		if (data.organizationId === null) return null;
		const result = await createDemographicTag(apiClient, data.organizationId, name);
		if (result.err !== null) {
			notifyError('Could not create the category. Try again.');
			return null;
		}
		await reload();
		return result.ok;
	}
</script>

<svelte:window onpointerdown={clearHighlight} onkeydown={clearHighlight} />

<svelte:head>
	<title>Demographic forms - Comhairle Admin</title>
</svelte:head>

<div class="bg-nav-background min-h-full">
	<div class="mx-auto w-11/12 max-w-6xl p-10">
		<header class="flex flex-wrap items-start justify-between gap-4">
			<div class="flex flex-col gap-2">
				<h1 class="text-4xl font-bold">Demographic forms</h1>
				<p class="text-muted-foreground max-w-2xl text-base">
					Build questions, put them together into forms, then add a form to any
					conversation as a demographic step.
				</p>
			</div>
			{#if data.canEdit}
				{#if tab === 'forms'}
					<Button onclick={newForm}>New form</Button>
				{:else}
					<Button onclick={() => newQuestion(false)}>New question</Button>
				{/if}
			{/if}
		</header>

		{#if data.organizations.length > 0}
			<div class="mt-6 flex flex-wrap items-center gap-3">
				<span class="text-base font-medium">Organisation</span>
				<Select.Root
					type="single"
					value={data.organizationId ?? ''}
					onValueChange={selectOrganization}
				>
					<Select.Trigger id="organization-picker" class="w-72">
						{selectedOrganizationName}
					</Select.Trigger>
					<Select.Content>
						{#each data.organizations as organization (organization.id)}
							<Select.Item value={organization.id} label={organization.name}>
								{organization.name}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		{/if}

		{#if data.organizationId === null}
			<div class="border-border bg-card mt-6 flex flex-col gap-2 rounded-xl border p-8">
				<h2 class="text-lg font-semibold">
					{data.organizations.length === 0
						? 'No organisation yet'
						: 'Choose an organisation'}
				</h2>
				<p class="text-muted-foreground text-base">
					{data.organizations.length === 0
						? 'You are not a member of an organisation, so there are no demographic forms to manage yet.'
						: 'Pick an organisation above to see and manage its demographic forms and question bank.'}
				</p>
			</div>
		{:else}
			<Tabs.Root bind:value={tab} class="mt-6">
				<Tabs.List>
					<Tabs.Trigger value="forms">Forms ({data.forms.length})</Tabs.Trigger>
					<Tabs.Trigger value="questions"
						>Question bank ({data.questions.length})</Tabs.Trigger
					>
				</Tabs.List>
				<Tabs.Content value="forms" class="mt-6">
					<FormsTab
						forms={data.forms}
						questions={data.questions}
						canEdit={data.canEdit}
						{highlightId}
						onNew={newForm}
						onEdit={editForm}
						onCopy={copyForm}
					/>
				</Tabs.Content>
				<Tabs.Content value="questions" class="mt-6">
					<QuestionBankTab
						questions={data.questions}
						canEdit={data.canEdit}
						{highlightId}
						onEdit={editQuestion}
						onDuplicate={duplicateQuestion}
					/>
				</Tabs.Content>
			</Tabs.Root>
		{/if}
	</div>
</div>

{#if formEdit}
	<FormSheet
		bind:form={formEdit.form}
		isNew={formEdit.isNew}
		questions={data.questions}
		onEditQuestion={editQuestion}
		onCreateQuestion={() => newQuestion(true)}
		hasChanges={formHasChanges}
		onSaveDraft={() => saveForm(false)}
		onPublish={() => saveForm(true)}
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
		focusConsent={questionEdit.focusConsent}
		tags={data.tags}
		{usedInFormList}
		onCreateTag={createTag}
		onSave={saveQuestion}
		onDelete={deleteQuestion}
		onClose={() => (questionEdit = null)}
	/>
{/if}
