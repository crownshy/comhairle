<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Dialog from '$lib/components/ui/dialog';
	import { snakeToStartCase } from '$lib/utils/casingUtils';
	import Label from '$lib/components/ui/label/label.svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { jsonToHtml } from '$lib/utils/rich-text.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import EmailTemplateVariables from '../EmailTemplateVariables.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { useDebounce } from 'runed';
	import { key } from '$lib/utils/invalidationKey';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { LocalizedConversationDto } from '@crownshy/api-client/api';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { newEmailTemplateConfigTour } from './tour';

	useTour(() => newEmailTemplateConfigTour);

	type FormState = {
		subject?: string;
		slots: { [key: string]: string };
	};

	let { data } = $props();
	const { schemas, conversations, emailConfigs } = data;

	let selectedSchema = $state(schemas[0]);
	let filteredConversations = $derived(
		conversations.filter(
			(conversation) =>
				!emailConfigs.find((config) => config.conversationId === conversation.id)
		)
	);
	let selectedConversation = $state<LocalizedConversationDto | null>(null);
	let formState = $derived.by(() => {
		const output: FormState = {
			slots: {},
			subject: selectedSchema.default_subject
		};
		selectedSchema.slots.map((slot) => (output.slots[slot.key] = slot.default_content));
		return output;
	});
	let previewHtml = $state('');
	let openPreviewModal = $state(false);
	let previewIframeSrc = $state('');

	$effect(() => {
		if (!previewHtml) return;

		const blob = new Blob([previewHtml], { type: 'text/html' });
		const url = URL.createObjectURL(blob);
		previewIframeSrc = url;

		return () => URL.revokeObjectURL(url);
	});

	function handleSelectSchema(value: string) {
		selectedSchema = schemas.find((s) => s.email_type === value);
	}

	function handleSelectConversation(value: string) {
		selectedConversation = conversations.find((c) => c.id === value) ?? null;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		const parsed: { [key: string]: string } = {};
		Object.entries(formState.slots).map(([key, value]) => {
			const matched = selectedSchema.slots.find((slot) => slot.key === key);
			if (!matched) return;

			if (matched.content_type === 'rich_text') {
				parsed[key] = jsonToHtml(value);
			} else {
				parsed[key] = value;
			}
		});

		try {
			const emailConfig = await apiClient.CreateEmailTemplateConfig({
				slots: {
					type: selectedSchema.email_type,
					...parsed
				},
				...(formState.subject && { subject: formState.subject }),
				...(selectedConversation &&
					selectedSchema.email_type === 'conversation_invite' && {
						conversation_id: selectedConversation.id
					})
			});

			notifications.send({
				priority: 'INFO',
				message: 'Successfully create new custom email'
			});

			goto(
				resolve('/(admin)/admin/email-template-configs/[email_config_id]', {
					email_config_id: emailConfig.id
				}),
				{
					invalidate: [key('admin/email-template-config')]
				}
			);
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message:
					"Something went wrong creating your custom email. Please ensure you haven't already created a custom email for this type."
			});
		}
	}

	const debouncedPreviewEmail = useDebounce(async () => {
		try {
			const parsed: { [key: string]: string } = {};
			Object.entries(formState.slots).map(([key, value]) => {
				const matched = selectedSchema.slots.find((slot) => slot.key === key);
				if (!matched) return;

				if (matched.content_type === 'rich_text') {
					parsed[key] = jsonToHtml(value);
				} else {
					parsed[key] = value;
				}
			});

			const response = await apiClient.PreviewEmailTemplateConfig({
				slots: { type: selectedSchema.email_type, ...parsed }
			});

			previewHtml = response.html;
		} catch (e) {
			console.error(e);
		}
	}, 300);
</script>

<svelte:head>
	<title>New Custom Email - Comhairle Admin</title>
</svelte:head>

<SectionHeader
	content={{
		eyebrow: 'Library',
		title: 'New custom email',
		description: 'Choose an email type, then write the subject and content.'
	}}
>
	{#snippet actions()}
		<Button
			variant="outline"
			type="button"
			onclick={() => {
				debouncedPreviewEmail();
				openPreviewModal = true;
			}}
		>
			Preview
		</Button>
	{/snippet}
</SectionHeader>

<TabContent>
	<div class="mb-4 flex justify-between">
		<div class="flex flex-col gap-2">
			<h2 class="text-xl font-semibold">Email type</h2>
			<Select.Root
				type="single"
				value={selectedSchema.email_type}
				onValueChange={handleSelectSchema}
			>
				<Select.Trigger class="w-fit"
					>{snakeToStartCase(selectedSchema.email_type)}</Select.Trigger
				>
				<Select.Content>
					{#each schemas as schema (schema.email_type)}
						<Select.Item value={schema.email_type}>
							{snakeToStartCase(schema.email_type)}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
		{#if selectedSchema.email_type === 'conversation_invite' && filteredConversations.length > 0}
			<div class="flex flex-col gap-2">
				<h2 class="text-xl font-semibold">Scoped Conversation</h2>
				<Select.Root
					type="single"
					value={selectedConversation?.id}
					onValueChange={handleSelectConversation}
				>
					<Select.Trigger class="w-fit"
						>{selectedConversation?.title ??
							'All conversations (fallback)'}</Select.Trigger
					>
					<Select.Content>
						<Select.Item value="">All conversations (fallback)</Select.Item>
						{#each filteredConversations as conversation (conversation.id)}
							<Select.Item value={conversation.id}>
								{conversation.title}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		{/if}
	</div>
	{#if selectedSchema.variables.length > 0}
		<EmailTemplateVariables templateVariables={selectedSchema.variables} />
	{/if}

	<form onsubmit={handleSubmit}>
		<div class="flex flex-col gap-4 py-6 lg:flex-row lg:items-start lg:gap-6">
			<Label
				class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
				for="subject">Email subject</Label
			>
			<Input
				placeholder="Subject"
				name="subject"
				defaultValue={formState.subject}
				onchange={(e) => (formState.subject = e.target?.value ?? undefined)}
			/>
		</div>
		{#each selectedSchema.slots as slot (slot.key)}
			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Label
					class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					for={slot.key}
				>
					<span>{slot.label}</span>
					<span class="text-sm font-normal">{slot.hint}</span>
				</Label>
				{#if slot.content_type === 'plain_text'}
					<Input
						placeholder="Enter email content..."
						defaultValue={formState.slots[slot.key]}
						onchange={(e) => (formState.slots[slot.key] = e.target?.value)}
					/>
				{:else}
					<RichTextEditor
						width="100%"
						value={formState.slots[slot.key]}
						placeholder="Enter email content..."
						onChange={(json) => (formState.slots[slot.key] = json)}
					/>
				{/if}
			</div>
		{/each}

		<div class="flex">
			<Button type="submit">Create email</Button>
		</div>
	</form>

	<Dialog.Root bind:open={openPreviewModal}>
		<Dialog.Content class="h-[70vh] w-full max-w-6xl sm:w-full sm:max-w-6xl">
			<Dialog.Header>
				<Dialog.Title class="flex items-center gap-2">Email preview</Dialog.Title>
				<Dialog.Description>
					{#if previewIframeSrc}
						<iframe
							src={previewIframeSrc}
							class="h-[63vh] w-full"
							title="Email preview"
							sandbox="allow-same-origin"
						></iframe>
					{/if}
				</Dialog.Description>
			</Dialog.Header>
		</Dialog.Content>
	</Dialog.Root>
</TabContent>
