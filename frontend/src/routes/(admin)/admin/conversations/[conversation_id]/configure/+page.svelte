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
	import { LanguageSelector } from '$lib/components/ui/language-selector';

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
		} catch (e) {
			notifications.send({ message: 'Failed to update languages', priority: 'ERROR' });
		}
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

	async function handleLanguageToggle(language: string, enabled: boolean) {
		translations.handleLanguageToggle(
			language, 
			enabled, 
			supportedLanguages, 
			(newSupported) => { supportedLanguages = newSupported; }
		);
	}

	function handlePrimaryContentChange(content: string) {
		const field = translations.activeField;
		if (field === 'title') {
			$form.title = content;
		} else if (field === 'short_description') {
			$form.short_description = content;
		} else if (field === 'description') {
			$form.description = content;
		}
		
		if (field) {
			translations.handlePrimaryContentChange(field);
		}
	}

</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl">
	<TerminalSquare /> Configure
</h1>
<p class="mb-10">Use this space to set up the project and manage the team supporting it</p>


<TranslationDialog
	bind:open={translations.modalOpen}
	translations={translations.activeTranslations}
	initialLanguage={translations.initialLanguage}
	onSave={translations.handleSave}
	onAutoSave={translations.handleAutoSave}
	onAiTranslate={translations.handleAiTranslate}
	onLanguageToggle={handleLanguageToggle}
	onClose={translations.closeDialog}
	onPrimaryContentChange={handlePrimaryContentChange}
/>

<form 
	method="POST" 
	onsubmit={updateConversation} 
	class="flex flex-col" 
	use:enhance
>
	<TranslatableFormField
		form={conversationForm}
		name="title"
		label="Title"
		value={$form.title}
		onValueChange={(v) => $form.title = v}
		onEditTranslations={(lang) => translations.openDialog('title', lang)}
		onPrimaryChange={() => translations.handlePrimaryContentChange('title')}
		translations={translations.getFieldTranslations('title')}
	/>

	<TranslatableFormField
		form={conversationForm}
		name="short_description"
		label="Short Description"
		value={$form.short_description}
		onValueChange={(v) => $form.short_description = v}
		onEditTranslations={(lang) => translations.openDialog('short_description', lang)}
		onPrimaryChange={() => translations.handlePrimaryContentChange('short_description')}
		translations={translations.getFieldTranslations('short_description')}
		inputType="textarea"
	/>

	<TranslatableFormField
		form={conversationForm}
		name="description"
		label="Description"
		value={$form.description}
		onValueChange={(v) => $form.description = v}
		onEditTranslations={(lang) => translations.openDialog('description', lang)}
		onPrimaryChange={() => translations.handlePrimaryContentChange('description')}
		translations={translations.getFieldTranslations('description')}
		inputType="textarea"
	/>


	<div class="grid grid-cols-[200px_1fr] gap-6 border-t py-6">
		<p class="font-semibold pt-2">Language options</p>
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
				name="image_url"
			>
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex w-full flex-row justify-between border-t-1 py-5">
							<div class="flex w-60 flex-col gap-2">
								<Form.Label class="font-bold">Banner Image URL</Form.Label>
								{#if $form.image_url}
									<img 
										width="200px" 
										alt="Conversation Banner" 
										src={$form.image_url} 
									/>
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
		</div>
	</div>

	<div class="flex w-full flex-row justify-between border-t-1 py-5">
		<p class="font-bold">Access</p>
		<div class="flex flex-col gap-5">
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
							<Form.Label>
								Automatically log in a user with an anon account if not logged in
							</Form.Label>
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

