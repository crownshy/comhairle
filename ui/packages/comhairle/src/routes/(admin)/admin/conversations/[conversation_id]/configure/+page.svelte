<script lang="ts">
	import { onMount, getContext } from 'svelte';
	import { page } from '$app/state';
	import { Switch } from '$lib/components/ui/switch';
	import * as Form from '$lib/components/ui/form/';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate, invalidateAll } from '$app/navigation';
	import { justCreatedConversation } from '$lib/stores/justCreatedConversation.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { conversationConfigSchema } from './schema';
	import TeamManager from '$lib/components/TeamManager.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import ConfigureTabStrip, { type ConfigureTab } from './ConfigureTabStrip.svelte';
	import {
		CONVERSATION_TAB_EXTRAS_CTX,
		type ConversationTabExtras
	} from '$lib/conversationTabExtras';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { autoTranslateNewLanguage } from '$lib/components/Translation/translationUtils';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import type {
		ConversationWithTranslations,
		MediaDto,
		WorkflowDto
	} from '@crownshy/api-client/api';
	import { camelToSentenceCase, camelToSnakeCase } from '$lib/utils/casingUtils';
	import { Image as ImageIcon } from 'lucide-svelte';
	import MediaLibraryDialog, {
		addToCache
	} from '$lib/components/Media/MediaLibraryDialog.svelte';
	import MediaUpload from '$lib/components/Media/MediaUpload.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	let {
		data
	}: {
		data: {
			conversation: ConversationWithTranslations;
			workflows: WorkflowDto[];
			media: MediaDto | null;
		};
	} = $props();
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let imageMedia = $derived(data.media);

	let primaryLanguage = $state(data.conversation.primaryLocale ?? 'en');
	let supportedLanguages = $state(data.conversation.supportedLanguages ?? ['en']);
	let pageTitle = $derived(`Configure ${conversation.title}`);

	// Sub-tabs. `id` is the `?tab=` value; the first entry is the default when absent.
	const tabs: ConfigureTab[] = [
		{ id: 'details', label: 'Details' },
		{ id: 'content', label: 'Content' },
		{ id: 'access', label: 'Access' },
		{ id: 'team', label: 'Team' }
	];
	const tabHeaders: Record<string, { title: string; description: string }> = {
		details: { title: 'Details', description: 'Title, description, language and banner.' },
		content: {
			title: 'Content',
			description: 'Participant-facing copy shown throughout the conversation.'
		},
		access: { title: 'Access', description: 'Visibility, invites and participation.' },
		team: { title: 'Team', description: 'Manage collaborators.' }
	};
	let activeTab = $derived(page.url.searchParams.get('tab') ?? tabs[0].id);
	let header = $derived(tabHeaders[activeTab] ?? tabHeaders.details);

	// Inject the full-bleed sub-tab strip into "Row 3" of the conversation layout,
	// the same slot the workflow step strip uses. Cleared on unmount.
	const tabExtras = getContext<ConversationTabExtras>(CONVERSATION_TAB_EXTRAS_CTX);
	$effect(() => {
		if (!tabExtras) return;
		tabExtras.primary = configureStripSnippet;
		return () => {
			tabExtras.primary = null;
		};
	});

	// When we land here straight after creating this conversation, focus and select the
	// auto-generated "Untitled …" title so it's obvious this is a brand-new conversation
	// ready to be named, rather than looking identical to the one the user came from.
	onMount(() => {
		if (justCreatedConversation.id !== conversation.id) return;
		justCreatedConversation.clear();
		// Defer a frame so the title field is mounted and populated before we select it.
		requestAnimationFrame(() => {
			const input = document.querySelector<HTMLInputElement>(
				'#conversation-title-field input'
			);
			input?.focus();
			input?.select();
		});
	});

	$effect(() => {
		primaryLanguage = data.conversation.primaryLocale ?? 'en';
		supportedLanguages = data.conversation.supportedLanguages ?? ['en'];
		$form.title = data.conversation.title;
		$form.shortDescription = data.conversation.shortDescription;
		$form.description = data.conversation.description;
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
			validationMethod: 'oninput'
		}
	);

	async function handleInitOptionalTranslationField(
		content: string,
		field: string,
		format: 'plain' | 'rich' = 'rich',
		autoTranslate: boolean = false
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

			if (autoTranslate) {
				const targetLocales = supportedLanguages.filter((lang) => lang !== primaryLanguage);
				if (targetLocales.length > 0) {
					notifications.send({ message: 'Generating translations...', priority: 'INFO' });
					for (const locale of targetLocales) {
						await autoTranslateNewLanguage(locale, [textContentRes.id]);
					}
					notifications.send({ message: 'Translations generated', priority: 'INFO' });
				}
			}

			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				message: `Failed to create ${camelToSentenceCase(field)}`,
				priority: 'ERROR'
			});
		}
	}

	let { form } = conversationForm;

	// Access toggles autosave on change (the page has no Save button — see ADR-0004).
	// `$form.<field>` is updated optimistically by `bind:checked`; on failure we revert it.
	// TODO: replace the per-toggle toast with a quiet inline "Saving → Saved" indicator per
	// row (like TranslatableField's), which is the intended end state for toggle feedback.
	type ConversationToggle =
		| 'isPublic'
		| 'isInviteOnly'
		| 'enableQaChatBot'
		| 'enableSignupPrompts'
		| 'showThankYouPageAnnonInstructions';

	async function saveConversationToggle(field: ConversationToggle, value: boolean) {
		const res = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{ [camelToSnakeCase(field)]: value },
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (res.err !== null) {
			console.error(res.err);
			$form[field] = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
		await invalidate('conversation:meta');
	}

	// `autoLogin` lives on the workflow, not the conversation, so it saves via its own route.
	async function saveAutoLogin(value: boolean) {
		const res = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflow(
				{ auto_login: value },
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			)
		);
		if (res.err !== null) {
			console.error(res.err);
			$form.autoLogin = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
		await invalidate('conversation:meta');
	}

	async function updateConversationMedia(media: MediaDto, field: string) {
		const response = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{
					[field]: media.id
				},
				{ params: { conversation_id: conversation.id } }
			)
		);

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Something went wrong updating conversation media',
				priority: 'ERROR'
			});

			return;
		}

		notifications.send({
			message: 'Successfully updated conversation media',
			priority: 'INFO'
		});

		await invalidate('conversation:meta');
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

{#snippet configureStripSnippet()}
	<ConfigureTabStrip {tabs} />
{/snippet}

<PageHeader title={header.title} description={header.description} />

<!-- One superForm still backs every field (its inline validation + $form binding); there is
	 no submit — every field autosaves (see ADR-0004), so this is a plain container. -->
<div class="flex flex-col">
	{#if activeTab === 'details'}
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
						<div class="flex-1" id="conversation-title-field">
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
								placeholder="A short description for this conversation."
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
								placeholder="Introduce people to what is being discussed and outline the actions that might be taken as a result of the conversation."
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
			<div class="contents">
				<Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>Banner image URL</Label
				>
				<div class="align-start flex w-full flex-col gap-4">
					<div class="flex flex-1 gap-4">
						<MediaLibraryDialog
							onconfirm={(media) => {
								updateConversationMedia(media, 'image');
							}}
						/>
						<MediaUpload
							clientSide
							size="sm"
							oncomplete={(media) => {
								if (!media.length) return;
								updateConversationMedia(media[0], 'image');
								for (const m of media) {
									addToCache(m);
								}
							}}
						/>
					</div>
					{#if imageMedia}
						<div class="h-70 w-auto">
							<img
								src={imageMedia.url}
								alt="Conversation"
								class="h-full w-auto object-contain"
							/>
						</div>
					{:else}
						<div class="relative h-40 w-fit rounded-3xl bg-white/60">
							<ImageIcon class="h-full w-auto" />
							<span
								class="absolute top-1/2 left-1/2 z-10 -translate-1/2 text-center text-xl font-bold text-gray-600"
								>Awaiting image</span
							>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}

	{#if activeTab === 'content'}
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
									handleInitOptionalTranslationField(
										content,
										'privacyPolicy',
										'rich',
										true
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
								translation={conversation.translations?.shortPrivacyPolicy ??
									undefined}
								editorType="rich"
								onSaveSource={(content: string) =>
									handleInitOptionalTranslationField(
										content,
										'shortPrivacyPolicy',
										'rich',
										true
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
								translation={conversation.translations?.thankYouMessage ??
									undefined}
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
	{/if}

	{#if activeTab === 'access'}
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
								<Switch
									{...props}
									bind:checked={$form.isPublic}
									onCheckedChange={(v) => saveConversationToggle('isPublic', v)}
								/>
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
								<Switch
									{...props}
									bind:checked={$form.isInviteOnly}
									onCheckedChange={(v) =>
										saveConversationToggle('isInviteOnly', v)}
								/>
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
								<Switch
									{...props}
									bind:checked={$form.autoLogin}
									onCheckedChange={(v) => saveAutoLogin(v)}
								/>
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
									<Form.Label class="text-sm font-medium"
										>Show Learning Assistant</Form.Label
									>
									<p class="text-muted-foreground text-sm">
										Display a Q&A Learning Assistant on the conversation.<br />
										{#if !conversation.isLive}
											(Configure Learning Assistant on the
											<a
												href={`/admin/conversations/${conversation.id}/knowledge-base`}
												class="underline">Knowledge Base page</a
											>)
										{/if}
									</p>
								</div>
								<Switch
									{...props}
									bind:checked={$form.enableQaChatBot}
									onCheckedChange={(v) =>
										saveConversationToggle('enableQaChatBot', v)}
								/>
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
								<Switch
									{...props}
									bind:checked={$form.enableSignupPrompts}
									onCheckedChange={(v) =>
										saveConversationToggle('enableSignupPrompts', v)}
								/>
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
										Display instructions for anonymous users on the thank you
										page.
									</p>
								</div>
								<Switch
									{...props}
									bind:checked={$form.showThankYouPageAnnonInstructions}
									onCheckedChange={(v) =>
										saveConversationToggle(
											'showThankYouPageAnnonInstructions',
											v
										)}
								/>
							</div>
						{/snippet}
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>
			</div>
		</div>
	{/if}

	{#if activeTab === 'team'}
		<!-- Collaborators -->
		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Collaborators</p>
			<div class="flex-1">
				<TeamManager />
			</div>
		</div>
	{/if}
</div>
