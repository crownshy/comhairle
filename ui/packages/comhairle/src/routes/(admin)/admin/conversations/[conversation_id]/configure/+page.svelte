<script lang="ts">
	import { onMount } from 'svelte';
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
	import * as HoverCard from '$lib/components/ui/hover-card';
	import CollapsibleRichField from './CollapsibleRichField.svelte';
	import ExampleDialog from './ExampleDialog.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import { hasUnsavedChanges } from '$lib/components/Translation/translationUtils';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import { autoTranslateNewLanguage } from '$lib/components/Translation/translationUtils';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import type {
		ConversationWithTranslations,
		MediaDto,
		UserDto,
		UserWithPermissionDto,
		WorkflowDto
	} from '@crownshy/api-client/api';
	import { camelToSentenceCase, camelToSnakeCase } from '$lib/utils/casingUtils';
	import { Image as ImageIcon, Info } from 'lucide-svelte';
	import MediaLibraryDialog, {
		addToCache
	} from '$lib/components/Media/MediaLibraryDialog.svelte';
	import MediaUpload from '$lib/components/Media/MediaUpload.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	let {
		data
	}: {
		data: {
			conversation: ConversationWithTranslations;
			workflows: WorkflowDto[];
			media: MediaDto | null;
			user: UserDto;
			usersWithPermission: UserWithPermissionDto[];
			configureTabs: { id: string; label: string }[];
		};
	} = $props();
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let imageMedia = $derived(data.media);
	let permittedUsers = $derived(data.usersWithPermission);

	let primaryLanguage = $state(data.conversation.primaryLocale ?? 'en');
	let supportedLanguages = $state(data.conversation.supportedLanguages ?? ['en']);
	let pageTitle = $derived(`Configure ${conversation.title}`);

	// The sub-tab strip (Row 3) is server-rendered by the conversation layout from `configureTabs`;
	// this page reads the same list to pick the default tab and its header copy.
	const tabHeaders: Record<string, { title: string; description: string }> = {
		details: { title: 'Details', description: 'Title, description, language and banner.' },
		content: {
			title: 'Content',
			description: 'Participant-facing copy shown throughout the conversation.'
		},
		access: { title: 'Access', description: 'Visibility, invites and participation.' },
		team: { title: 'Team', description: 'Manage collaborators.' }
	};
	let activeTab = $derived(page.url.searchParams.get('tab') ?? data.configureTabs[0].id);
	let header = $derived(tabHeaders[activeTab] ?? tabHeaders.details);

	// "See example" opens a modal with a static screenshot of where the field appears for
	// participants. Images are hand-maintained assets under static/examples/; a missing one
	// falls back to "Example coming soon" (see ExampleDialog).
	const examples: Record<string, { title: string; src: string }> = {
		shortDescription: { title: 'Short description', src: '/examples/short-description.png' },
		privacyPolicy: { title: 'Privacy Policy', src: '/examples/privacy-policy.png' },
		shortPrivacyPolicy: {
			title: 'Short Privacy Policy',
			src: '/examples/short-privacy-policy.png'
		},
		faqs: { title: 'FAQs', src: '/examples/faqs.png' },
		thankYouMessage: { title: 'Thank you message', src: '/examples/thank-you.png' },
		callToAction: { title: 'Call to action', src: '/examples/call-to-action.png' }
	};
	let exampleKey = $state<string | null>(null);
	let exampleOpen = $state(false);
	let exampleEntry = $derived(exampleKey ? examples[exampleKey] : null);

	function openExample(key: string) {
		exampleKey = key;
		exampleOpen = true;
	}

	// The rich Content fields behave as an accordion: at most one is expanded at a time.
	// Holds the field name of the open one, or null when all are collapsed.
	let openContentField = $state<string | null>(null);

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

	// Gate the inline autosave for required text fields: TranslatableField saves on every change,
	// so without this an empty title/description would persist even while the form shows its
	// "required" error. Optional fields get no guard and keep autosaving blank values.
	function requiredFieldValidator(field: 'title' | 'shortDescription' | 'description') {
		return (value: string) => conversationConfigSchema.shape[field].safeParse(value).success;
	}

	// Each translatable field is driven by a TranslationSource (ADR-0005). `onEdit` mirrors the primary
	// value into `$form` so superForm's inline validation (Form.FieldErrors) keeps working; the nullable
	// rich fields add `ensureTextContentId` so their TextContent is created + linked on first edit.
	function fieldSource(
		field:
			| 'title'
			| 'shortDescription'
			| 'description'
			| 'privacyPolicy'
			| 'shortPrivacyPolicy'
			| 'faqs'
			| 'thankYouMessage'
			| 'callToAction',
		ensureTextContentId?: (content: string) => Promise<void>
	) {
		return createTextContentSource({
			getTranslation: () => conversation.translations?.[field] ?? undefined,
			getPrimaryLocale: () => primaryLanguage,
			getSupportedLanguages: () => supportedLanguages,
			getPrimaryFallback: () => $form[field] ?? '',
			onEdit: (content) => ($form[field] = content),
			ensureTextContentId
		});
	}

	const titleSource = fieldSource('title');
	const shortDescriptionSource = fieldSource('shortDescription');
	const descriptionSource = fieldSource('description');
	const privacyPolicySource = fieldSource('privacyPolicy', (content) =>
		handleInitOptionalTranslationField(content, 'privacyPolicy', 'rich', true)
	);
	const shortPrivacyPolicySource = fieldSource('shortPrivacyPolicy', (content) =>
		handleInitOptionalTranslationField(content, 'shortPrivacyPolicy', 'rich', true)
	);
	const faqsSource = fieldSource('faqs', (content) =>
		handleInitOptionalTranslationField(content, 'faqs')
	);
	const thankYouMessageSource = fieldSource('thankYouMessage', (content) =>
		handleInitOptionalTranslationField(content, 'thankYouMessage')
	);
	const callToActionSource = fieldSource('callToAction', (content) =>
		handleInitOptionalTranslationField(content, 'callToAction', 'plain')
	);

	// Warn on refresh / navigate-away while any field is still autosaving.
	const fieldSources = [
		titleSource,
		shortDescriptionSource,
		descriptionSource,
		privacyPolicySource,
		shortPrivacyPolicySource,
		faqsSource,
		thankYouMessageSource,
		callToActionSource
	];
	guardUnsavedChanges(() => fieldSources.some(hasUnsavedChanges));

	// Access toggles autosave on change (the page has no Save button — see ADR-0004).
	// `$form.<field>` is updated optimistically by `bind:checked`; on failure we revert it.
	// TODO: replace the per-toggle toast with a quiet inline "Saving → Saved" indicator per
	// row (like TranslatableField's), which is the intended end state for toggle feedback.
	type ConversationToggle =
		| 'isPublic'
		| 'isInviteOnly'
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

<!-- The (i) affordance: a hover card (bits-ui LinkPreview, via our HoverCard wrapper) that shows
	 the field's description on hover, plus an optional "See example" button that opens the image
	 modal. Keeping the heavy image behind an explicit click means an accidental hover only ever
	 reveals a light text card. `exampleKey` indexes the examples map. -->
{#snippet infoPreview(info: string, exampleKey: string = '')}
	<HoverCard.Root openDelay={150} closeDelay={100}>
		<HoverCard.Trigger
			class="text-muted-foreground hover:text-foreground inline-flex cursor-help"
			aria-label="More information"
		>
			<Info class="size-4" />
		</HoverCard.Trigger>
		<HoverCard.Content class="w-72 text-sm" side="top" sideOffset={6}>
			<p>{info}</p>
			{#if exampleKey}
				<button
					type="button"
					onclick={() => openExample(exampleKey)}
					class="text-primary mt-3 inline-flex items-center gap-1 text-sm font-medium hover:underline"
				>
					<ImageIcon class="size-3.5" />
					See example
				</button>
			{/if}
		</HoverCard.Content>
	</HoverCard.Root>
{/snippet}

<!-- Left-column label for a form field. `label` stays a <Form.Label> so it keeps its `for`
	 association with the control. The (i) beside it carries the description and the example. -->
{#snippet fieldLabel(label: string, exampleKey: string = '', info: string = '')}
	<div class="lg:w-50 lg:shrink-0 lg:pt-2">
		<div class="flex items-center gap-1.5">
			<Form.Label class="text-sm font-semibold">{label}</Form.Label>
			{#if info || exampleKey}
				{@render infoPreview(info, exampleKey)}
			{/if}
		</div>
	</div>
{/snippet}

<!-- Same, for rows whose control isn't a Form.Field (Banner, Language), so there's no label to
	 associate. -->
{#snippet plainLabel(label: string, exampleKey: string = '', info: string = '')}
	<div class="lg:w-50 lg:shrink-0 lg:pt-2">
		<div class="flex items-center gap-1.5">
			<p class="text-sm font-semibold">{label}</p>
			{#if info || exampleKey}
				{@render infoPreview(info, exampleKey)}
			{/if}
		</div>
	</div>
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
						{@render fieldLabel(
							'Title',
							'',
							"The conversation's name. Shown as the heading participants see, and on listing cards, invitations, and the thank-you and report pages."
						)}
						<div class="flex-1" id="conversation-title-field">
							<TranslatableField
								source={titleSource}
								canSave={requiredFieldValidator('title')}
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
						{@render fieldLabel(
							'Short description',
							'shortDescription',
							'A one-line summary, shown under the title on the landing page and on conversation cards in listings.'
						)}
						<div class="flex-1">
							<TranslatableField
								source={shortDescriptionSource}
								canSave={requiredFieldValidator('shortDescription')}
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
						{@render fieldLabel(
							'Description',
							'',
							'A fuller introduction, shown beside the banner image on the landing and invitation pages.'
						)}
						<div class="flex-1">
							<TranslatableField
								source={descriptionSource}
								canSave={requiredFieldValidator('description')}
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
			{@render plainLabel(
				'Language options',
				'',
				'The primary language plus any others you support. Adding a language lets you translate the other fields into it.'
			)}
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
				{@render plainLabel(
					'Banner image',
					'',
					'Shown beside the description on the landing and invitation pages.'
				)}
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
						{@render fieldLabel(
							'Privacy Policy',
							'privacyPolicy',
							"The full policy, shown on the Privacy Policy page and the 'Find out more' panel. Leave blank to use Comhairle's default."
						)}
						<div class="flex-1">
							<CollapsibleRichField
								label="Privacy policy"
								content={$form.privacyPolicy}
								open={openContentField === 'privacyPolicy'}
								onOpenChange={(o) =>
									(openContentField = o ? 'privacyPolicy' : null)}
							>
								<TranslatableField
									source={privacyPolicySource}
									editorType="rich"
									placeholder="The full policy, shown on the Privacy Policy page and the 'Find out more' panel. Leave blank to use Comhairle's default."
									primaryLocale={primaryLanguage}
									{supportedLanguages}
									inputProps={props}
								/>
								<Form.FieldErrors />
							</CollapsibleRichField>
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
						{@render fieldLabel(
							'Short Privacy Policy',
							'shortPrivacyPolicy',
							"Shown in the consent dialog participants accept before joining. Leave blank to use Comhairle's default."
						)}
						<div class="flex-1">
							<CollapsibleRichField
								label="Short privacy policy"
								content={$form.shortPrivacyPolicy}
								open={openContentField === 'shortPrivacyPolicy'}
								onOpenChange={(o) =>
									(openContentField = o ? 'shortPrivacyPolicy' : null)}
							>
								<TranslatableField
									source={shortPrivacyPolicySource}
									editorType="rich"
									placeholder="Shown in the consent dialog participants accept before joining. Leave blank to use Comhairle's default."
									primaryLocale={primaryLanguage}
									{supportedLanguages}
									inputProps={props}
								/>
								<Form.FieldErrors />
							</CollapsibleRichField>
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
						{@render fieldLabel(
							'FAQs',
							'faqs',
							"Shown on the FAQ page and the 'Find out more' panel. Leave blank to use Comhairle's default FAQs."
						)}
						<div class="flex-1">
							<CollapsibleRichField
								label="FAQs"
								content={$form.faqs}
								open={openContentField === 'faqs'}
								onOpenChange={(o) => (openContentField = o ? 'faqs' : null)}
							>
								<TranslatableField
									source={faqsSource}
									editorType="rich"
									placeholder="Shown on the FAQ page and the 'Find out more' panel. Leave blank to use Comhairle's default FAQs."
									primaryLocale={primaryLanguage}
									{supportedLanguages}
									inputProps={props}
								/>
								<Form.FieldErrors />
							</CollapsibleRichField>
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
						{@render fieldLabel(
							'Thank you message',
							'thankYouMessage',
							"Shown on the thank-you page after someone finishes. Leave blank for the default 'Thank you for participating' message."
						)}
						<div class="flex-1">
							<CollapsibleRichField
								label="Thank you message"
								content={$form.thankYouMessage}
								open={openContentField === 'thankYouMessage'}
								onOpenChange={(o) =>
									(openContentField = o ? 'thankYouMessage' : null)}
							>
								<TranslatableField
									source={thankYouMessageSource}
									editorType="rich"
									placeholder="Shown on the thank-you page after someone finishes. Leave blank for the default 'Thank you for participating' message."
									primaryLocale={primaryLanguage}
									{supportedLanguages}
									inputProps={props}
								/>
								<Form.FieldErrors />
							</CollapsibleRichField>
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
						{@render fieldLabel(
							'Call to action',
							'callToAction',
							"The label on the main join button. Leave blank for 'Join the conversation'."
						)}
						<div class="flex-1">
							<TranslatableField
								source={callToActionSource}
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
									<div class="flex items-center gap-1.5">
										<Form.Label class="text-sm font-medium"
											>Show conversation publicly</Form.Label
										>
										{@render infoPreview(
											'When this conversation is launched, anyone (even without an account) can open its documents and data. Off means only you, collaborators, and participants can.'
										)}
									</div>
									<p class="text-muted-foreground text-sm">
										Let anyone view this conversation's data once it's launched.
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
									<div class="flex items-center gap-1.5">
										<Form.Label class="text-sm font-medium"
											>Only allow participation by invite</Form.Label
										>
										{@render infoPreview(
											'Only people you invite can take part. With this off, anyone with the link can participate.'
										)}
									</div>
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
									<div class="flex items-center gap-1.5">
										<Form.Label class="text-sm font-medium"
											>Automatically log in with an anonymous account</Form.Label
										>
										{@render infoPreview(
											'Visitors who are not signed in get a temporary anonymous account automatically, so they can take part without registering. They can upgrade to a real account later.'
										)}
									</div>
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

				<Form.Field form={conversationForm} name="enableSignupPrompts">
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between gap-4">
								<div class="flex flex-col gap-1">
									<div class="flex items-center gap-1.5">
										<Form.Label class="text-sm font-medium"
											>Enable signup prompts</Form.Label
										>
										{@render infoPreview(
											'Shows prompts encouraging participants to create an account on the thank-you page after they finish.'
										)}
									</div>
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
									<div class="flex items-center gap-1.5">
										<Form.Label class="text-sm font-medium"
											>Show thank you page anonymous instructions</Form.Label
										>
										{@render infoPreview(
											'On the thank-you page, shows anonymous participants their temporary ID and how to log back in later to see the results.'
										)}
									</div>
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
			<div class="flex-1">
				<TeamManager conversationId={conversation.id} {permittedUsers} />
			</div>
		</div>
	{/if}
</div>

<ExampleDialog
	bind:open={exampleOpen}
	title={exampleEntry?.title ?? ''}
	src={exampleEntry?.src ?? null}
/>
