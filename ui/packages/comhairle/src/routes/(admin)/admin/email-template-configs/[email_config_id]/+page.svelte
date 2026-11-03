<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { apiClient } from '@crownshy/api-client/client';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import {
		initialSlotValues,
		type EmailTemplateSubmission,
		type KnownVariables,
		type PreviewConversation,
		type SaveStatus
	} from '$lib/utils/emailTemplateForm';
	import EmailTemplateForm from '../EmailTemplateForm.svelte';
	import SaveActions from '../SaveActions.svelte';
	import { EMAIL_TYPE_COPY } from '../emailTypeCopy';
	import { emailTemplateConfigTour } from './tour';

	useTour(() => emailTemplateConfigTour);

	let { data } = $props();

	let emailConfig = $derived(data.emailConfig);
	let schema = $derived(data.schema);
	let copy = $derived(EMAIL_TYPE_COPY[emailConfig.emailType]);
	let isConversationVersion = $derived(!!emailConfig.conversationId);
	let scopeLabel = $derived(
		isConversationVersion
			? (emailConfig.conversationTitle ?? 'Untitled conversation')
			: copy.allScopeLabel
	);
	let knownVariables = $derived<KnownVariables>(
		emailConfig.conversationTitle ? { conversation_title: emailConfig.conversationTitle } : {}
	);
	let previewConversation = $derived<PreviewConversation | null>(
		emailConfig.conversationTitle
			? { title: emailConfig.conversationTitle, imageUrl: data.conversationImageUrl }
			: null
	);
	let removeConsequence = $derived(
		isConversationVersion
			? `Participants in ${scopeLabel} will get the version for ${copy.allScopeLabel.toLowerCase()}, or Comhairle's default if there isn't one.`
			: "Participants will get Comhairle's default email."
	);

	const FORM_ID = 'email-template-form';

	let saveStatus = $state<SaveStatus>('idle');
	let removeDialogOpen = $state(false);
	let removing = $state(false);

	async function saveChanges(submission: EmailTemplateSubmission): Promise<boolean> {
		const result = await tryCatchAsync(() =>
			apiClient.UpdateEmailTemplateConfig(
				{
					slots: { type: emailConfig.emailType, ...submission.slots },
					...(submission.subject && { subject: submission.subject })
				},
				{ params: { email_config_id: emailConfig.id } }
			)
		);
		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Something went wrong saving this email. Please try again.'
			});
			return false;
		}
		return true;
	}

	async function removeVersion() {
		removing = true;
		const result = await tryCatchAsync(() =>
			apiClient.DeleteEmailTemplateConfig(undefined, {
				params: { email_config_id: emailConfig.id }
			})
		);
		removing = false;
		if (result.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Something went wrong removing this email. Please try again.'
			});
			return;
		}
		removeDialogOpen = false;
		await goto(resolve('/(admin)/admin/email-template-configs'), {
			invalidate: [key('admin/email-template-config')]
		});
	}
</script>

<svelte:head>
	<title>{copy.name}, {scopeLabel} - Comhairle Admin</title>
</svelte:head>

{#snippet settings()}
	<div class="flex flex-col gap-1.5">
		<span class="text-sm font-medium">Applies to</span>
		<span class="text-foreground text-base font-semibold">{scopeLabel}</span>
		{#if isConversationVersion}
			<span class="text-muted-foreground text-sm">
				Only participants in this conversation get this version.
			</span>
		{/if}
	</div>
{/snippet}

{#snippet footer()}
	<div
		class="border-border mt-10 flex flex-wrap items-center justify-between gap-3 border-t pt-6"
	>
		<div class="flex flex-col gap-0.5">
			<span class="text-foreground text-base font-semibold">Remove this custom version</span>
			<span class="text-muted-foreground text-sm">{removeConsequence}</span>
		</div>
		<Button
			type="button"
			variant="destructiveOutline"
			size="sm"
			onclick={() => (removeDialogOpen = true)}
		>
			Remove
		</Button>
	</div>
{/snippet}

<SectionHeader
	content={{
		back: { href: resolve('/(admin)/admin/email-template-configs'), label: 'Email templates' },
		title: copy.name,
		description: copy.sentWhen
	}}
>
	{#snippet actions()}
		<SaveActions
			formId={FORM_ID}
			status={saveStatus}
			submitLabel="Save changes"
			cancelHref={resolve('/(admin)/admin/email-template-configs')}
		/>
	{/snippet}
</SectionHeader>
<TabContent>
	{#key emailConfig.id}
		<EmailTemplateForm
			formId={FORM_ID}
			{schema}
			initialSubject={emailConfig.subject ?? schema.default_subject}
			initialSlots={initialSlotValues(schema, emailConfig.slots)}
			bind:saveStatus
			{knownVariables}
			{previewConversation}
			onSubmit={saveChanges}
			{settings}
			{footer}
		/>
	{/key}
</TabContent>

<AlertDialog.Root bind:open={removeDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Remove this custom version?</AlertDialog.Title>
			<AlertDialog.Description>{removeConsequence}</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={removing}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action disabled={removing} onclick={removeVersion}>
				Remove
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
