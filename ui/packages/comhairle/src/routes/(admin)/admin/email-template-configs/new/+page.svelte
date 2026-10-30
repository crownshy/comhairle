<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { apiClient } from '@crownshy/api-client/client';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import { canScopeToConversation } from '$lib/utils/emailTemplateGroups';
	import {
		initialSlotValues,
		type EmailTemplateSubmission,
		type KnownVariables,
		type SaveStatus
	} from '$lib/utils/emailTemplateForm';
	import EmailTemplateForm from '../EmailTemplateForm.svelte';
	import SaveActions from '../SaveActions.svelte';
	import { EMAIL_TYPE_COPY } from '../emailTypeCopy';
	import { newEmailTemplateConfigTour } from './tour';

	useTour(() => newEmailTemplateConfigTour);

	let { data } = $props();

	const FORM_ID = 'email-template-form';

	// The list page fixes the email with ?type= and, for one conversation, ?conversation=.
	let selectedType = $derived(page.url.searchParams.get('type') ?? data.schemas[0].email_type);
	let conversationParam = $derived(page.url.searchParams.get('conversation'));
	let schema = $derived(
		data.schemas.find((candidate) => candidate.email_type === selectedType) ?? data.schemas[0]
	);
	let copy = $derived(EMAIL_TYPE_COPY[schema.email_type]);
	let scopable = $derived(canScopeToConversation(schema.email_type));
	let configsOfType = $derived(
		data.emailConfigs.filter((config) => config.emailType === schema.email_type)
	);
	let existingAllVersion = $derived(configsOfType.find((config) => !config.conversationId));
	let availableConversations = $derived(
		data.conversations.filter(
			(conversation) =>
				!configsOfType.some((config) => config.conversationId === conversation.id)
		)
	);
	let chosenConversation = $derived(
		scopable && conversationParam
			? availableConversations.find((conversation) => conversation.id === conversationParam)
			: undefined
	);
	let conversationUnavailable = $derived(!!conversationParam && !chosenConversation);
	let blockedByExisting = $derived(!conversationParam && !!existingAllVersion);
	let saveStatus = $state<SaveStatus>('idle');
	let knownVariables = $derived<KnownVariables>(
		chosenConversation ? { conversation_title: chosenConversation.title } : {}
	);

	async function createVersion(submission: EmailTemplateSubmission): Promise<boolean> {
		const conversationId = chosenConversation?.id;
		const result = await tryCatchAsync(() =>
			apiClient.CreateEmailTemplateConfig({
				slots: { type: schema.email_type, ...submission.slots },
				...(submission.subject && { subject: submission.subject }),
				...(conversationId && { conversation_id: conversationId })
			})
		);
		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Something went wrong saving this email. Please try again.'
			});
			return false;
		}

		await goto(
			resolve('/(admin)/admin/email-template-configs/[email_config_id]', {
				email_config_id: result.ok.id
			}),
			{ invalidate: [key('admin/email-template-config')] }
		);
		return true;
	}
</script>

<svelte:head>
	<title>Customise {copy.name} - Comhairle Admin</title>
</svelte:head>

{#snippet settings()}
	<div class="flex flex-col gap-1">
		<span class="text-muted-foreground text-sm font-medium">Applies to</span>
		<span class="text-foreground text-base font-medium">
			{chosenConversation?.title ?? copy.allScopeLabel}
		</span>
	</div>
	{#if conversationUnavailable}
		<p role="status" class="bg-muted text-foreground rounded-lg px-4 py-3 text-base">
			This conversation already has its own version, or you can't edit its emails.
			<a
				href={resolve('/(admin)/admin/email-template-configs')}
				class="text-primary font-medium underline underline-offset-4"
			>
				Back to email templates
			</a>
		</p>
	{/if}
	{#if blockedByExisting && existingAllVersion}
		<p role="status" class="bg-muted text-foreground rounded-lg px-4 py-3 text-base">
			This email is already customised for {copy.allScopeLabel.toLowerCase()}.
			<a
				href={resolve('/(admin)/admin/email-template-configs/[email_config_id]', {
					email_config_id: existingAllVersion.id
				})}
				class="text-primary font-medium underline underline-offset-4"
			>
				Edit that version
			</a>
			instead.
		</p>
	{/if}
{/snippet}

<SectionHeader
	content={{
		back: { href: resolve('/(admin)/admin/email-template-configs'), label: 'Email templates' },
		title: `Customise ${copy.name.toLowerCase()}`,
		description: copy.sentWhen
	}}
>
	{#snippet actions()}
		<SaveActions
			formId={FORM_ID}
			status={saveStatus}
			submitLabel="Save custom email"
			cancelHref={resolve('/(admin)/admin/email-template-configs')}
			disabled={blockedByExisting || conversationUnavailable}
		/>
	{/snippet}
</SectionHeader>
<TabContent>
	{#key schema.email_type}
		<EmailTemplateForm
			formId={FORM_ID}
			{schema}
			initialSubject={schema.default_subject}
			initialSlots={initialSlotValues(schema)}
			bind:saveStatus
			{knownVariables}
			previewConversation={chosenConversation
				? { title: chosenConversation.title, imageUrl: chosenConversation.imageUrl }
				: null}
			onSubmit={createVersion}
			{settings}
		/>
	{/key}
</TabContent>
