<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { detailsSchema } from './schema';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import FieldLabel from '../FieldLabel.svelte';
	import { MediaUpload, MediaLibraryDialog } from '$lib/components/Media';
	import { LanguageSelector } from '$lib/components/ui/language-selector';
	import type { Locale } from '$lib/paraglide/runtime';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import {
		autoTranslateNewLanguage,
		type TranslationSource
	} from '$lib/components/Translation/translationUtils';
	import type { MediaDto } from '@crownshy/api-client/api';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { Image } from '@lucide/svelte';
	import { addToCache } from '$lib/components/Media/MediaLibraryDialog.svelte';
	import { GLOSSARY_METADATA_KEY } from '$lib/glossary/parseGlossary';
	import { translateGlossaryToLocale } from '$lib/glossary/translateGlossary';
	import { localizedGlossaryFromMetadata } from '$lib/glossary/localizedGlossary';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { DEFAULT_LOCALE } from '$lib/utils/constants';

	const { data } = $props();
	const { conversation, streamedMedia } = $derived(data);

	let primaryLocale = $derived<Locale>(
		(data.conversation.primaryLocale as Locale) ?? DEFAULT_LOCALE
	);
	let supportedLanguages = $derived<Locale[]>(
		(data.conversation.supportedLanguages as Locale[]) ?? [DEFAULT_LOCALE]
	);

	let conversationForm = superForm(
		{
			title: data.conversation.title,
			shortDescription: data.conversation.shortDescription,
			description: data.conversation.description
		},
		{
			validators: zodClient(detailsSchema),
			taintedMessage: false,
			validationMethod: 'oninput'
		}
	);

	let { form } = conversationForm;

	function requiredFieldValidator(field: 'title' | 'shortDescription' | 'description') {
		return (value: string) => detailsSchema.shape[field].safeParse(value).success;
	}

	const fieldSource = (field: 'title' | 'shortDescription' | 'description'): TranslationSource =>
		createTextContentSource({
			getTranslation: () => conversation.translations?.[field] ?? undefined,
			getPrimaryLocale: () => primaryLocale,
			getSupportedLanguages: () => supportedLanguages,
			getPrimaryFallback: () => $form[field] ?? '',
			onEdit: (content) => ($form[field] = content)
		});

	const titleSource = fieldSource('title');
	const shortDescriptionSource = fieldSource('shortDescription');
	const descriptionSource = fieldSource('description');

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

		await invalidate(key('conversation'));
	}

	//#Region Translations
	function getTranslatableTextContentIds(): string[] {
		const translationsData = conversation.translations;
		if (!translationsData) return [];

		return Object.values(translationsData)
			.map((field) => field.textContent?.id)
			.filter((id): id is string => !!id);
	}

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

		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{
					primary_locale: newPrimary,
					supported_languages: supportedLanguages
				},
				{ params: { conversation_id: conversation.id } }
			)
		);

		if (result.err !== null) {
			notifications.send({ message: 'Failed to update primary language', priority: 'ERROR' });
			return;
		}

		await invalidate(key('conversation'));
		notifications.send({ message: 'Primary language updated', priority: 'INFO' });
	}

	async function handleSupportedLanguagesChange(newSupported: string[]) {
		const currentSupported = conversation.supportedLanguages ?? [];
		const newlyAddedLanguages = newSupported.filter((lang) => !currentSupported.includes(lang));

		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{
					primary_locale: primaryLocale,
					supported_languages: newSupported
				},
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (result.err !== null) {
			notifications.send({ message: 'Failed to update languages', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Languages updated', priority: 'INFO' });

		if (newlyAddedLanguages.length > 0) {
			notifications.send({ message: 'Generating translations...', priority: 'INFO' });

			const textContentIds = getTranslatableTextContentIds();
			for (const locale of newlyAddedLanguages) {
				if (textContentIds.length > 0) {
					await autoTranslateNewLanguage(locale, textContentIds);
				}
			}

			// The glossary lives in metadata, so it's auto-translated separately from the
			// TextContent-backed fields above.
			let glossary = localizedGlossaryFromMetadata(conversation.metadata, primaryLocale);
			if (glossary.length > 0) {
				for (const locale of newlyAddedLanguages) {
					const translation = await tryCatchAsync(() =>
						translateGlossaryToLocale(glossary, locale, primaryLocale)
					);
					if (translation.err !== null) {
						notifications.send({
							message: `Failed to generate translation for ${glossary}`,
							priority: 'WARNING'
						});
					}
				}
				const patchConversationMetadata = await tryCatchAsync(() =>
					apiClient.PatchConversationMetadata(
						{ [GLOSSARY_METADATA_KEY]: glossary },
						{ params: { conversation_id: conversation.id } }
					)
				);
				if (patchConversationMetadata.err !== null) {
					notifications.send({
						message: `Failed to update glossary. Please try again`,
						priority: 'WARNING'
					});
				}
			}
			notifications.send({ message: 'Translations generated', priority: 'INFO' });
		}

		await invalidate(key('conversation'));
	}
	//#EndRegion Translations
</script>

<PageHeader title="Details" description="Title, description, language and banner." />

<!-- One superForm still backs every field (its inline validation + $form binding); there is
	 no submit — every field autosaves (see ADR-0004), so this is a plain container. -->
<div class="flex flex-col">
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={conversationForm} name="title" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<FieldLabel
						label="Title"
						info="The conversation's name. Shown as the heading participants see, and on listing cards, invitations, and the thank-you and report pages."
					/>
					<div class="flex-1" id="conversation-title-field">
						<TranslatableField
							source={titleSource}
							canSave={requiredFieldValidator('title')}
							{primaryLocale}
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
					<FieldLabel
						type="form"
						label="Short description"
						info=" A one-line summary, shown under the title on the landing page and on conversation cards in listings. "
					/>
					<div class="flex-1">
						<TranslatableField
							source={shortDescriptionSource}
							canSave={requiredFieldValidator('shortDescription')}
							{primaryLocale}
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
					<FieldLabel
						label="Description"
						info="A fuller introduction, shown beside the banner image on the landing and invitation pages."
					/>
					<div class="flex-1">
						<TranslatableField
							source={descriptionSource}
							canSave={requiredFieldValidator('description')}
							{primaryLocale}
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
		<FieldLabel
			type="plain"
			label="Language options"
			info="The primary language plus any others you support. Adding a language lets you translate the other fields into it."
		/>
		<div class="max-w-md flex-1">
			<LanguageSelector
				bind:primaryLanguage={primaryLocale}
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
			<FieldLabel
				type="plain"
				label="Banner image"
				info="Shown beside the description on the landing and invitation pages."
			/>
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
							updateConversationMedia(media, 'image');
							addToCache(media);
						}}
					/>
				</div>
				{#if streamedMedia === null}
					<span class="text-muted-foreground">No image</span>
				{:else}
					{#await streamedMedia}
						<!-- TODO: Try using a CSS mask here -->
						<div class="pile">
							<Skeleton class="h-40 w-40 rounded-4xl" />
							<Image class="z-2 h-full w-auto" strokeWidth={0.9} opacity={0.5} />
						</div>
					{:then media}
						{#if media?.err !== null}
							{notifications.addFlash({
								message: 'Could not load image. Please try again',
								priority: 'ERROR'
							})}
						{:else}
							<div class="h-70 w-auto">
								<img
									src={media.ok.url}
									alt="Conversation"
									class="h-full w-auto object-contain"
								/>
							</div>
						{/if}
					{/await}
				{/if}
			</div>
		</div>
	</div>
</div>
