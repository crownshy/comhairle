<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import * as Form from '$lib/components/ui/form/';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { conversationConfigSchema } from './schema';
	import TeamManager from '$lib/components/TeamManager.svelte';
	import TranslationDialog from '$lib/components/Translation/TranslationDialog.svelte';
	import TranslatableFormField from '$lib/components/Translation/TranslatableFormField.svelte';
	import { createTranslationManager } from '$lib/components/Translation/useTranslations.svelte';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import type { LocalizedConversationDto, WorkflowStep } from '$lib/api/api';
	import type { Workflow } from 'lucide-svelte';
	import { snakeCaseKeys } from '$lib/utils/snakeCaseKeys';

	let {
		data
	}: {
		data: {
			conversation: LocalizedConversationDto;
			workflows: Workflow[];
			workflow_steps: WorkflowStep[];
		};
	} = $props();
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);

	let primaryLanguage = $state(data.conversation.primaryLocale ?? 'en');
	let supportedLanguages = $state(data.conversation.supportedLanguages ?? ['en']);

	const translations = createTranslationManager(
		() => conversation,
		(field) => {
			if (field === 'title') return $form.title;
			if (field === 'shortDescription') return $form.shortDescription;
			if (field === 'description') return $form.description;
			return undefined;
		}
	);

	function updateFormForLanguage(newLanguage: string) {
		const fields = ['title', 'shortDescription', 'description'] as const;

		for (const field of fields) {
			const content = translations.getFieldContentForLocale(field, newLanguage);
			$form[field] = content ?? '';
		}
	}

	async function handlePrimaryLanguageChange(newPrimary: string) {
		updateFormForLanguage(newPrimary);

		try {
			await apiClient.UpdateConversation(
				{
					primary_locale: newPrimary,
					supported_languages: supportedLanguages
				},
				{ params: { conversation_id: conversation.id } }
			);
			await invalidateAll();
			notifications.send({ message: 'Primary language updated', priority: 'INFO' });
		} catch (e) {
			notifications.send({ message: 'Failed to update primary language', priority: 'ERROR' });
		}
	}

	async function handleSupportedLanguagesChange(newSupported: string[]) {
		const currentSupported = conversation.supportedLanguages ?? [];
		const newlyAddedLanguages = newSupported.filter((lang) => !currentSupported.includes(lang));

		try {
			await apiClient.UpdateConversation(
				{
					primary_locale: primaryLanguage,
					supported_languages: newSupported
				},
				{ params: { conversation_id: conversation.id } }
			);
			await invalidateAll();
			notifications.send({ message: 'Languages updated', priority: 'INFO' });

			if (newlyAddedLanguages.length > 0) {
				const textContentIds = getTranslatableTextContentIds();
				if (textContentIds.length > 0) {
					notifications.send({ message: 'Generating translations...', priority: 'INFO' });

					for (const locale of newlyAddedLanguages) {
						await translations.autoTranslateNewLanguage(locale, textContentIds);
					}

					notifications.send({ message: 'Translations generated', priority: 'INFO' });
				}
			}
		} catch (e) {
			notifications.send({ message: 'Failed to update languages', priority: 'ERROR' });
		}
	}

	function getTranslatableTextContentIds(): string[] {
		const translationsData = conversation.translations;
		if (!translationsData) return [];

		return Object.values(translationsData)
			.map((field) => field.text_content?.id)
			.filter((id): id is string => !!id);
	}

	let conversationForm = superForm(
		{
			title: data.conversation.title,
			shortDescription: data.conversation.shortDescription,
			description: data.conversation.description,
			imageUrl: data.conversation.imageUrl,
			isPublic: data.conversation.isPublic,
			isInviteOnly: data.conversation.isInviteOnly,
			autoLogin: data.workflows[0].auto_login
		},
		{
			validators: zodClient(conversationConfigSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: updateConversation
		}
	);

	let { form, enhance, validateForm, submitting, tainted } = conversationForm;

	async function updateConversation() {
		const result = await validateForm({ update: true });

		if (!result.valid) return;

		try {
			const {
				title: _title /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				shortDescription:
					_short_description /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				description: _description, /* eslint-disable-line @typescript-eslint/no-unused-vars */
				autoLogin: _auto_login, /* eslint-disable-line @typescript-eslint/no-unused-vars */
				...conversationData
			} = result.data;

			// Convert to snake case for update params
			const conversationSnakeCase = snakeCaseKeys(conversationData);

			await apiClient.UpdateConversation(
				{
					...conversationSnakeCase,
					primary_locale: primaryLanguage,
					supported_languages: supportedLanguages
				},
				{ params: { conversation_id: conversation.id } }
			);

			await apiClient.UpdateWorkflow(
				{ auto_login: result.data.autoLogin },
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			);

			await invalidateAll();
			notifications.send({ message: 'Updated conversation', priority: 'INFO' });
		} catch (e) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}
	}

	function handleContentChange(language: string, content: string) {
		const field = translations.activeField;
		const isPrimary = language === primaryLanguage;

		// Update form if primary language changed
		if (isPrimary && field) {
			if (field === 'title') $form.title = content;
			else if (field === 'shortDescription') $form.shortDescription = content;
			else if (field === 'description') $form.description = content;
		}

		translations.updateContent(language, content);
	}

	useAdminLayoutSlots({
		title: titleContentSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>Configure</Breadcrumb.Item>
{/snippet}

{#snippet titleContentSnippet()}
	<h1 class="text-4xl font-bold">Configure</h1>
	<AdminPrevNextControls
		next={{ name: 'design', url: `/admin/conversations/${conversation.id}/design` }}
	/>
{/snippet}

<p class="mb-10">Use this space to set up the project and manage the team supporting it</p>

<TranslationDialog
	bind:open={translations.modalOpen}
	translations={translations.workingTranslations}
	activeLanguage={translations.activeLanguage}
	isTranslating={translations.isTranslating}
	onClose={translations.closeDialog}
	onContentChange={handleContentChange}
	onStatusChange={translations.updateStatus}
	onActiveLanguageChange={translations.setActiveLanguage}
	onAiTranslate={translations.handleAiTranslate}
/>

<form method="POST" onsubmit={updateConversation} class="flex flex-col" use:enhance>
	<TranslatableFormField
		form={conversationForm}
		name="title"
		label="Title"
		value={$form.title}
		onValueChange={(v) => ($form.title = v)}
		onEditTranslations={(lang) => translations.openDialog('title', lang)}
		onPrimaryChange={() => translations.handlePrimaryContentChange('title')}
		translations={translations.getFieldTranslations('title')}
	/>

	<TranslatableFormField
		form={conversationForm}
		name="short_description"
		label="Short Description"
		value={$form.shortDescription}
		onValueChange={(v) => ($form.shortDescription = v)}
		onEditTranslations={(lang) => translations.openDialog('shortDescription', lang)}
		onPrimaryChange={() => translations.handlePrimaryContentChange('shortDescription')}
		translations={translations.getFieldTranslations('shortDescription')}
		inputType="textarea"
	/>

	<TranslatableFormField
		form={conversationForm}
		name="description"
		label="Description"
		value={$form.description}
		onValueChange={(v) => ($form.description = v)}
		onEditTranslations={(lang) => translations.openDialog('description', lang)}
		onPrimaryChange={() => translations.handlePrimaryContentChange('description')}
		translations={translations.getFieldTranslations('description')}
		inputType="textarea"
	/>

	<div class="item-start grid grid-cols-[200px_1fr] gap-6 border-t py-6">
		<p class="pt-2 font-semibold">Language options</p>
		<div class="max-w-md">
			<LanguageSelector
				bind:primaryLanguage
				bind:supportedLanguages
				onPrimaryChange={handlePrimaryLanguageChange}
				onSupportedChange={handleSupportedLanguagesChange}
			/>
		</div>
	</div>

	<div class="flex flex-row gap-4">
		<div class="grow">
			<Form.Field
				class="flex w-full flex-row justify-between border-t-1 py-5"
				form={conversationForm}
				name="imageUrl"
			>
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex w-full flex-row justify-between border-t-1 py-5">
							<div class="flex w-60 flex-col gap-2">
								<Form.Label class="font-bold">Banner Image URL</Form.Label>
								{#if $form.imageUrl}
									<img
										width="200px"
										alt="Conversation Banner"
										src={$form.imageUrl}
									/>
								{/if}
							</div>
							<div class="grow flex-col gap-2">
								<Input {...props} bind:value={$form.imageUrl} />
								<Form.FieldErrors />
							</div>
						</div>
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>
	</div>

	<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
		<p class="font-bold lg:w-60 lg:shrink-0">Access</p>
		<div class="flex grow flex-col gap-5">
			<Form.Field form={conversationForm} name="isPublic">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.isPublic} />
							<Form.Label>Show conversation publicly</Form.Label>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="isInviteOnly">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.isInviteOnly} />
							<Form.Label>Only allow participation by invite</Form.Label>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="autoLogin">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.autoLogin} />
							<Form.Label
								>Automatically log in a user with an annon account if not logged in</Form.Label
							>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
		</div>
	</div>

	<TeamManager />

	<Form.Button variant="secondary" class="my-5" disabled={$submitting || !$tainted}>
		Save Changes
	</Form.Button>
</form>
