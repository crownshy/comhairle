<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import * as Form from '$lib/components/ui/form/';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidateAll } from '$app/navigation';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { conversationConfigSchema } from './schema';
	import TeamManager from '$lib/components/TeamManager.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { autoTranslateNewLanguage } from '$lib/components/Translation/translationUtils';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import type { ConversationWithTranslations, WorkflowDto } from '@crownshy/api-client/api';
	import { camelToSentenceCase, camelToSnakeCase, snakeCaseKeys } from '$lib/utils/casingUtils';

	let {
		data
	}: {
		data: {
			conversation: ConversationWithTranslations;
			workflows: WorkflowDto[];
		};
	} = $props();
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);

	let primaryLanguage = $state(data.conversation.primaryLocale ?? 'en');
	let supportedLanguages = $state(data.conversation.supportedLanguages ?? ['en']);
	let pageTitle = $derived(`Configure ${conversation.title}`);

	$effect(() => {
		primaryLanguage = data.conversation.primaryLocale ?? 'en';
		supportedLanguages = data.conversation.supportedLanguages ?? ['en'];
		$form.title = data.conversation.title;
		$form.shortDescription = data.conversation.shortDescription;
		$form.description = data.conversation.description;
		$form.imageUrl = data.conversation.imageUrl;
		$form.isPublic = data.conversation.isPublic;
		$form.isInviteOnly = data.conversation.isInviteOnly;
		$form.privacyPolicy = data.conversation.privacyPolicy;
		$form.shortPrivacyPolicy = data.conversation.shortPrivacyPolicy;
		$form.faqs = data.conversation.faqs;
		$form.thankYouMessage = data.conversation.thankYouMessage;
		$form.callToAction = data.conversation.callToAction;
		$form.autoLogin = data.workflows[0]?.autoLogin;
		$form.enableQaChatBot = data.conversation.enableQaChatBot;
		$form.enableSignupPrompts = data.conversation.enableSignupPrompts;
		$form.showThankYouPageAnnonInstructions =
			data.conversation.showThankYouPageAnnonInstructions;
	});

	function updateFormForLanguage(newLanguage: string) {
		const t = conversation.translations;
		if (!t) return;
		const fields = {
			title: t.title,
			shortDescription: t.shortDescription,
			description: t.description
		} as const;
		for (const [key, field] of Object.entries(fields)) {
			const trans = field?.textTranslations?.find(
				(tt: { locale: string }) => tt.locale === newLanguage
			);
			if (trans) {
				$form[key as keyof typeof fields] = trans.content;
			}
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
						await autoTranslateNewLanguage(locale, textContentIds);
					}

					await invalidateAll();
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
			.map((field) => field.textContent?.id)
			.filter((id): id is string => !!id);
	}

	let conversationForm = superForm(
		{
			title: data.conversation.title,
			shortDescription: data.conversation.shortDescription,
			description: data.conversation.description,
			imageUrl: data.conversation.imageUrl,
			privacyPolicy: data.conversation.privacyPolicy,
			shortPrivacyPolicy: data.conversation.shortPrivacyPolicy,
			faqs: data.conversation.faqs,
			thankYouMessage: data.conversation.thankYouMessage,
			callToAction: data.conversation.callToAction,
			isPublic: data.conversation.isPublic,
			isInviteOnly: data.conversation.isInviteOnly,
			autoLogin: data.workflows[0].autoLogin,
			enableQaChatBot: data.conversation.enableQaChatBot,
			enableSignupPrompts: data.conversation.enableSignupPrompts,
			showThankYouPageAnnonInstructions: data.conversation.showThankYouPageAnnonInstructions
		},
		{
			validators: zodClient(conversationConfigSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: updateConversation
		}
	);

	async function handleInitOptionalTranslationField(
		content: string,
		field: string,
		format: 'plain' | 'rich' = 'rich'
	) {
		try {
			if (!conversation) return;

			const textContentRes = await apiClient.CreateTextContent({
				content,
				format,
				primary_locale: conversation.primaryLocale
			});

			await apiClient.UpdateConversation(
				{ [camelToSnakeCase(field)]: textContentRes.id },
				{ params: { conversation_id: conversation.id } }
			);
		} catch (e) {
			console.error(e);
			notifications.send({
				message: `Failed to create ${camelToSentenceCase(field)}`,
				priority: 'ERROR'
			});
		}
	}

	let { form, enhance, validateForm, submitting, tainted } = conversationForm;

	async function updateConversation() {
		const result = await validateForm({ update: true });

		if (!result.valid) return;

		try {
			const {
				title: _title /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				shortDescription:
					_short_description /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				description:
					_description /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				privacyPolicy:
					_privacyPolicy /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				shortPrivacyPolicy:
					_shortPrivacyPolicy /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				faqs: _faqs /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				thankYouMessage:
					_thankYouMessage /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				callToAction:
					_callToAction /* eslint-disable-line @typescript-eslint/no-unused-vars */,
				autoLogin: _auto_login /* eslint-disable-line @typescript-eslint/no-unused-vars */,
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

			await apiClient.UpdateConversationWorkflow(
				{ auto_login: result.data.autoLogin },
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			);

			await invalidateAll();
			notifications.send({ message: 'Updated conversation', priority: 'INFO' });
		} catch (e) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}
	}

	useAdminLayoutSlots({
		title: titleContentSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>Configure</Breadcrumb.Item>
{/snippet}

{#snippet titleContentSnippet()}
	<h1 class="text-4xl font-bold">Configure</h1>
	<AdminPrevNextControls
		next={{ name: 'design', url: `/admin/conversations/${conversation.id}/design` }}
	/>
{/snippet}

<div class="flex flex-col gap-4">
	<h2 class="text-card-foreground text-xl font-semibold">Conversation configuration</h2>
	<p class="text-muted-foreground text-sm">Personal details and general information.</p>
</div>

<form method="POST" onsubmit={updateConversation} class="mt-8 flex flex-col" use:enhance>
	<!-- Title -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="title" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Title</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.title}
							onValueChange={(v) => ($form.title = v)}
							translation={conversation.translations?.title}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Short Description -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="shortDescription" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Short description</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.shortDescription}
							onValueChange={(v) => ($form.shortDescription = v)}
							translation={conversation.translations?.shortDescription}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputType="textarea"
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Description -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="description" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Description</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.description}
							onValueChange={(v) => ($form.description = v)}
							translation={conversation.translations?.description}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputType="textarea"
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Language options -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Language options</p>
		<div class="max-w-md flex-1">
			<LanguageSelector
				bind:primaryLanguage
				bind:supportedLanguages
				onPrimaryChange={handlePrimaryLanguageChange}
				onSupportedChange={handleSupportedLanguagesChange}
			/>
		</div>
	</div>

	<!-- Banner Image URL -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="imageUrl" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Banner image URL</Form.Label
					>
					<div class="flex flex-1 flex-col gap-4">
						<Input {...props} bind:value={$form.imageUrl} />
						<Form.FieldErrors />
						{#if $form.imageUrl}
							<img
								class="bg-muted w-full max-w-md rounded-lg object-cover"
								alt="Conversation Banner"
								src={$form.imageUrl}
							/>
						{/if}
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Privacy policy -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="privacyPolicy" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Privacy Policy</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.privacyPolicy || null}
							onValueChange={(v) => ($form.privacyPolicy = v)}
							translation={conversation.translations?.privacyPolicy ?? undefined}
							editorType="rich"
							onSaveSource={(content: string) =>
								handleInitOptionalTranslationField(content, 'privacyPolicy')}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Short privacy policy -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="shortPrivacyPolicy" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Short Privacy Policy</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.shortPrivacyPolicy || null}
							onValueChange={(v) => ($form.shortPrivacyPolicy = v)}
							translation={conversation.translations?.shortPrivacyPolicy ?? undefined}
							editorType="rich"
							onSaveSource={(content: string) =>
								handleInitOptionalTranslationField(content, 'shortPrivacyPolicy')}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- FAQs -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="faqs" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>FAQs</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.faqs || null}
							onValueChange={(v) => ($form.faqs = v)}
							translation={conversation.translations?.faqs ?? undefined}
							editorType="rich"
							onSaveSource={(content: string) =>
								handleInitOptionalTranslationField(content, 'faqs')}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Thank you message -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="thankYouMessage" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Thank you message</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.thankYouMessage || null}
							onValueChange={(v) => ($form.thankYouMessage = v)}
							translation={conversation.translations?.thankYouMessage ?? undefined}
							editorType="rich"
							onSaveSource={(content: string) =>
								handleInitOptionalTranslationField(content, 'thankYouMessage')}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Call to action -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="callToAction" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Call to action</Form.Label
					>
					<div class="flex-1">
						<TranslatableField
							value={$form.callToAction || null}
							onValueChange={(v) => ($form.callToAction = v)}
							translation={conversation.translations?.callToAction ?? undefined}
							onSaveSource={(content: string) =>
								handleInitOptionalTranslationField(
									content,
									'callToAction',
									'plain'
								)}
							primaryLocale={primaryLanguage}
							{supportedLanguages}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Access / Other configuration -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Other configuration</p>
		<div class="flex flex-1 flex-col gap-6">
			<Form.Field form={conversationForm} name="isPublic">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center justify-between gap-4">
							<div class="flex flex-col gap-1">
								<Form.Label class="text-sm font-medium"
									>Show conversation publicly</Form.Label
								>
								<p class="text-muted-foreground text-sm">
									Allow export of personal data and backups.
								</p>
							</div>
							<Switch {...props} bind:checked={$form.isPublic} />
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="isInviteOnly">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center justify-between gap-4">
							<div class="flex flex-col gap-1">
								<Form.Label class="text-sm font-medium"
									>Only allow participation by invite</Form.Label
								>
								<p class="text-muted-foreground text-sm">
									Admins can invite and manage members.
								</p>
							</div>
							<Switch {...props} bind:checked={$form.isInviteOnly} />
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="autoLogin">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center justify-between gap-4">
							<div class="flex flex-col gap-1">
								<Form.Label class="text-sm font-medium"
									>Automatically log in with an anonymous account</Form.Label
								>
								<p class="text-muted-foreground text-sm">
									Creates a temporary account for unauthenticated users.
								</p>
							</div>
							<Switch {...props} bind:checked={$form.autoLogin} />
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="enableQaChatBot">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center justify-between gap-4">
							<div class="flex flex-col gap-1">
								<Form.Label class="text-sm font-medium">Show Tutor Bot</Form.Label>
								<p class="text-muted-foreground text-sm">
									Display a Q&A Tutor Bot on the conversation.<br />
									{#if !conversation.isLive}
										(Configure Tutor Bot on the
										<a
											href={`/admin/conversations/${conversation.id}/knowledge-base`}
											class="underline">Knowledge Base page</a
										>)
									{/if}
								</p>
							</div>
							<Switch {...props} bind:checked={$form.enableQaChatBot} />
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="enableSignupPrompts">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center justify-between gap-4">
							<div class="flex flex-col gap-1">
								<Form.Label class="text-sm font-medium"
									>Enable signup prompts</Form.Label
								>
								<p class="text-muted-foreground text-sm">
									Toggle whether to display signup prompts on thank you page.
								</p>
							</div>
							<Switch {...props} bind:checked={$form.enableSignupPrompts} />
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="showThankYouPageAnnonInstructions">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center justify-between gap-4">
							<div class="flex flex-col gap-1">
								<Form.Label class="text-sm font-medium"
									>Show thank you page anonymous instructions</Form.Label
								>
								<p class="text-muted-foreground text-sm">
									Display instructions for anonymous users on the thank you page.
								</p>
							</div>
							<Switch
								{...props}
								bind:checked={$form.showThankYouPageAnnonInstructions}
							/>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
		</div>
	</div>

	<!-- Collaborators -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Collaborators</p>
		<div class="flex-1">
			<TeamManager />
		</div>
	</div>

	<!-- Save Button -->
	<div class="border-border flex justify-center border-t py-6">
		<Form.Button variant="default" class="px-12" disabled={$submitting || !$tainted}>
			Save Changes
		</Form.Button>
	</div>
</form>
