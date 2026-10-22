<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { apiClient } from '@crownshy/api-client/client';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
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
	const ALL_CONVERSATIONS = 'all';
	const NO_SCOPE_CHOSEN = '';

	// The list page links here with ?type= so Customise opens on the right email.
	let selectedType = $derived<string>(
		page.url.searchParams.get('type') ?? data.schemas[0].email_type
	);
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
	// When the all-conversations version exists already, make the admin pick a conversation.
	let scope = $derived(scopable && existingAllVersion ? NO_SCOPE_CHOSEN : ALL_CONVERSATIONS);
	let scopeLabel = $derived(
		scope === ALL_CONVERSATIONS
			? copy.allScopeLabel
			: (availableConversations.find((conversation) => conversation.id === scope)?.title ??
					'Choose a conversation')
	);
	let blockedByExisting = $derived(scope === ALL_CONVERSATIONS && !!existingAllVersion);
	let saveStatus = $state<SaveStatus>('idle');
	let chosenConversation = $derived(
		scopable ? availableConversations.find((conversation) => conversation.id === scope) : undefined
	);
	let knownVariables = $derived<KnownVariables>(
		chosenConversation ? { conversation_title: chosenConversation.title } : {}
	);

	async function createVersion(submission: EmailTemplateSubmission): Promise<boolean> {
		const conversationId = scopable && scope !== ALL_CONVERSATIONS ? scope : undefined;
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
	<div class="flex flex-wrap gap-4">
		<div class="flex flex-col gap-1.5">
			<Label for="email-type" class="text-sm font-medium">Email</Label>
			<Select.Root
				type="single"
				value={selectedType}
				onValueChange={(value) => (selectedType = value)}
			>
				<Select.Trigger id="email-type" class="min-w-64 text-base">
					{copy.name}
				</Select.Trigger>
				<Select.Content>
					{#each data.schemas as option (option.email_type)}
						<Select.Item value={option.email_type}>
							{EMAIL_TYPE_COPY[option.email_type].name}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
		{#if scopable}
			<div class="flex flex-col gap-1.5">
				<Label for="email-scope" class="text-sm font-medium">Applies to</Label>
				<Select.Root
					type="single"
					value={scope}
					onValueChange={(value) => (scope = value)}
				>
					<Select.Trigger id="email-scope" class="min-w-64 text-base">
						{scopeLabel}
					</Select.Trigger>
					<Select.Content>
						<Select.Item value={ALL_CONVERSATIONS} disabled={!!existingAllVersion}>
							{copy.allScopeLabel}{existingAllVersion ? ' (already customised)' : ''}
						</Select.Item>
						{#each availableConversations as conversation (conversation.id)}
							<Select.Item value={conversation.id}>
								{conversation.title}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		{:else}
			<div class="flex flex-col gap-1.5">
				<span class="text-sm font-medium">Applies to</span>
				<span class="text-foreground flex h-9 items-center text-base">
					{copy.allScopeLabel}
				</span>
			</div>
		{/if}
	</div>
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
			disabled={blockedByExisting || scope === NO_SCOPE_CHOSEN}
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
