<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import * as Form from '$lib/components/ui/form';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import CollapsibleRichField from '../CollapsibleRichField.svelte';
	import FieldLabel from '../FieldLabel.svelte';
	import { contentSchema } from './schema';
	import type { Locale } from '$lib/paraglide/runtime';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import {
		autoTranslateNewLanguage,
		type TranslationSource
	} from '$lib/components/Translation/translationUtils';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { camelToSentenceCase, camelToSnakeCase } from '$lib/utils/casingUtils';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import TranslatableFieldWithSkeleton from '$lib/components/Translation/TranslatableFieldWithSkeleton.svelte';

	let { data } = $props();
	const { conversation, streamedAvailableDocuments } = $derived(data);

	let primaryLocale = $derived<Locale>((data.conversation.primaryLocale as Locale) ?? 'en');
	let supportedLanguages = $derived<Locale[]>(
		(data.conversation.supportedLanguages as Locale[]) ?? ['en']
	);

	let contentForm = superForm(
		{
			privacyPolicy: data.conversation.privacyPolicy,
			shortPrivacyPolicy: data.conversation.shortPrivacyPolicy,
			faqs: data.conversation.faqs,
			thankYouMessage: data.conversation.thankYouMessage,
			callToAction: data.conversation.callToAction
		},
		{
			validators: zodClient(contentSchema),
			taintedMessage: false,
			validationMethod: 'oninput'
		}
	);

	const { form } = $derived(contentForm);

	// Only 1 rich content field can be expanded at a time. Holds the field name or null
	let openContentField = $state<string | null>(null);

	const fieldSource = (
		field: 'privacyPolicy' | 'shortPrivacyPolicy' | 'faqs' | 'thankYouMessage' | 'callToAction',
		ensureTextContentId: (content: string) => Promise<void>
	): TranslationSource =>
		createTextContentSource({
			getTranslation: () => conversation.translations?.[field] ?? undefined,
			getPrimaryLocale: () => primaryLocale,
			getSupportedLanguages: () => supportedLanguages,
			getPrimaryFallback: () => $form[field] ?? '',
			onEdit: (content) => ($form[field] = content),
			ensureTextContentId
		});

	async function handleInitOptionalTranslationField(
		content: string,
		field: string,
		format: 'plain' | 'rich' = 'rich',
		autoTranslate: boolean = false
	) {
		if (!conversation) return;

		const textContent = await tryCatchAsync(() =>
			apiClient.CreateTextContent({
				content,
				format,
				primary_locale: conversation.primaryLocale
			})
		);

		if (textContent.err !== null) {
			console.error(textContent.err);
			notifications.send({
				message: `Failed to create ${camelToSentenceCase(field)}`,
				priority: 'ERROR'
			});
			return;
		}

		const updateConversation = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{ [camelToSnakeCase(field)]: textContent.ok.id },
				{ params: { conversation_id: conversation.id } }
			)
		);

		if (updateConversation.err !== null) {
			console.error(updateConversation.err);
			notifications.send({
				message: `Failed to update conversation`,
				priority: 'ERROR'
			});
			return;
		}

		if (autoTranslate) {
			const targetLocales = supportedLanguages.filter((lang) => lang !== primaryLocale);
			if (targetLocales.length > 0) {
				notifications.send({ message: 'Generating translations...', priority: 'INFO' });
				for (const locale of targetLocales) {
					await autoTranslateNewLanguage(locale, [textContent.ok.id]);
				}
				notifications.send({ message: 'Translations generated', priority: 'INFO' });
			}
		}

		await invalidate(key('conversation'));
	}

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
</script>

<PageHeader
	title="Content"
	description="Participant-facing copy shown throughout the conversation."
/>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<Form.Field form={contentForm} name="privacyPolicy" class="contents">
		<Form.Control>
			{#snippet children({ props })}
				<FieldLabel
					label="Privacy Policy"
					info="The full policy, shown on the Privacy Policy page and the 'Find out more' panel. Leave blank to use Comhairle's default."
				/>
				<div class="flex-1">
					<CollapsibleRichField
						label="Privacy policy"
						content={$form.privacyPolicy}
						open={openContentField === 'privacyPolicy'}
						onOpenChange={(o) => (openContentField = o ? 'privacyPolicy' : null)}
					>
						<TranslatableFieldWithSkeleton
							source={privacyPolicySource}
							editorType="rich"
							placeholder="The full policy, shown on the Privacy Policy page and the 'Find out more' panel. Leave blank to use Comhairle's default."
							{primaryLocale}
							{supportedLanguages}
							{streamedAvailableDocuments}
							conversationId={conversation.id}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</CollapsibleRichField>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>
</div>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<Form.Field form={contentForm} name="shortPrivacyPolicy" class="contents">
		<Form.Control>
			{#snippet children({ props })}
				<FieldLabel
					label="Short Privacy Policy"
					info="Shown in the consent dialog participants accept before joining. Leave blank to use Comhairle's default."
				/>
				<div class="flex-1">
					<CollapsibleRichField
						label="Short privacy policy"
						content={$form.shortPrivacyPolicy}
						open={openContentField === 'shortPrivacyPolicy'}
						onOpenChange={(o) => (openContentField = o ? 'shortPrivacyPolicy' : null)}
					>
						<TranslatableFieldWithSkeleton
							source={shortPrivacyPolicySource}
							editorType="rich"
							placeholder="Shown in the consent dialog participants accept before joining. Leave blank to use Comhairle's default."
							{primaryLocale}
							{supportedLanguages}
							{streamedAvailableDocuments}
							conversationId={conversation.id}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</CollapsibleRichField>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>
</div>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<Form.Field form={contentForm} name="faqs" class="contents">
		<Form.Control>
			{#snippet children({ props })}
				<FieldLabel
					label="FAQs"
					info="Shown on the FAQ page and the 'Find out more' panel. Leave blank to use Comhairle's default FAQs."
				/>
				<div class="flex-1">
					<CollapsibleRichField
						label="FAQs"
						content={$form.faqs}
						open={openContentField === 'faqs'}
						onOpenChange={(o) => (openContentField = o ? 'faqs' : null)}
					>
						<TranslatableFieldWithSkeleton
							source={faqsSource}
							editorType="rich"
							placeholder="Shown on the FAQ page and the 'Find out more' panel. Leave blank to use Comhairle's default FAQs."
							{primaryLocale}
							{supportedLanguages}
							{streamedAvailableDocuments}
							conversationId={conversation.id}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</CollapsibleRichField>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>
</div>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<Form.Field form={contentForm} name="thankYouMessage" class="contents">
		<Form.Control>
			{#snippet children({ props })}
				<FieldLabel
					label="Thank you message"
					info="Shown on the thank-you page after someone finishes. Leave blank for the default 'Thank you for participating' message."
				/>
				<div class="flex-1">
					<CollapsibleRichField
						label="Thank you message"
						content={$form.thankYouMessage}
						open={openContentField === 'thankYouMessage'}
						onOpenChange={(o) => (openContentField = o ? 'thankYouMessage' : null)}
					>
						<TranslatableFieldWithSkeleton
							source={thankYouMessageSource}
							editorType="rich"
							placeholder="Shown on the thank-you page after someone finishes. Leave blank for the default 'Thank you for participating' message."
							{primaryLocale}
							{supportedLanguages}
							{streamedAvailableDocuments}
							conversationId={conversation.id}
							inputProps={props}
						/>
						<Form.FieldErrors />
					</CollapsibleRichField>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>
</div>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<Form.Field form={contentForm} name="callToAction" class="contents">
		<Form.Control>
			{#snippet children({ props })}
				<FieldLabel
					label="Call to action"
					info="The label on the main join button. Leave blank for 'Join the conversation'."
				/>
				<div class="flex-1">
					<TranslatableField
						source={callToActionSource}
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
