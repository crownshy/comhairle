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
	import { TerminalSquare } from 'lucide-svelte';

	let { data } = $props();
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);

	let primaryLanguage = $state(data.conversation.primary_locale ?? 'en');
	let supportedLanguages = $state(data.conversation.supported_languages ?? ['en']);

	const translations = createTranslationManager(
		() => conversation,
		(field) => {
			if (field === 'title') return $form.title;
			if (field === 'short_description') return $form.short_description;
			if (field === 'description') return $form.description;
			return undefined;
		}
	);

	function updateFormForLanguage(newLanguage: string) {
		const fields = ['title', 'short_description', 'description'] as const;
		
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
		const currentSupported = conversation.supported_languages ?? [];
		const newlyAddedLanguages = newSupported.filter(lang => !currentSupported.includes(lang));

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
			.map(field => field.text_content?.id)
			.filter((id): id is string => !!id);
	}

	let conversationForm = superForm(
		{
			title: data.conversation.title,
			short_description: data.conversation.short_description,
			description: data.conversation.description,
			image_url: data.conversation.image_url,
			is_public: data.conversation.is_public,
			is_invite_only: data.conversation.is_invite_only,
			auto_login: data.workflows[0].auto_login
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
			await apiClient.UpdateConversation(
				{
					...result.data,
					primary_locale: primaryLanguage,
					supported_languages: supportedLanguages
				},
				{ params: { conversation_id: conversation.id } }
			);

			await apiClient.UpdateWorkflow(
				{ auto_login: result.data.auto_login },
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
			else if (field === 'short_description') $form.short_description = content;
			else if (field === 'description') $form.description = content;
		}
		
		translations.updateContent(language, content);
	}

</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl">
	<TerminalSquare /> Configure
</h1>
<p class="mb-10">Use this space to set up the project and manage the team supporting it</p>

<form method="POST" onsubmit={updateConversation} class="flex flex-col gap-4" use:enhance>
	<Form.Field form={conversationForm} name="title">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
					<Form.Label class="font-bold lg:w-60 lg:shrink-0">Title</Form.Label>
					<div class="grow flex-col gap-2">
						<Input {...props} bind:value={$form.title} />
						<Form.FieldErrors />
					</div>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

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

	<Form.Field form={conversationForm} name="description">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
					<Form.Label class="font-bold lg:w-60 lg:shrink-0">Description</Form.Label>
					<div class="grow flex-col gap-2">
						<Textarea class="bg-white" {...props} bind:value={$form.description} />
						<Form.FieldErrors />
					</div>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Field form={conversationForm} name="image_url">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
					<div class="flex flex-col gap-2 lg:w-60 lg:shrink-0">
						<Form.Label class="font-bold">Banner Image URL</Form.Label>
						{#if $form.image_url}
							<img width="200px" alt="Conversation Banner" src={$form.image_url} />
						{/if}
					</div>
					<div class="grow flex-col gap-2">
						<Input {...props} bind:value={$form.image_url} />
						<Form.FieldErrors />
					</div>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<div class="flex w-full flex-col gap-2 border-t py-5 lg:flex-row lg:justify-between">
		<p class="font-bold lg:w-60 lg:shrink-0">Access</p>
		<div class="flex grow flex-col gap-5">
			<Form.Field form={conversationForm} name="is_public">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.is_public} />
							<Form.Label>Show conversation publicly</Form.Label>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="is_invite_only">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.is_invite_only} />
							<Form.Label>Only allow participation by invite</Form.Label>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="auto_login">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.auto_login} />
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

	<Form.Button 
		variant="secondary" 
		class="my-5" 
		disabled={$submitting || !$tainted}
	>
		Save Changes
	</Form.Button>
</form>

