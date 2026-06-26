<script lang="ts">
	import Label from '$lib/components/ui/label/label.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { jsonToHtml } from '$lib/utils/rich-text.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils.js';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { goto, invalidateAll } from '$app/navigation';
	import EmailTemplateVariables from '../EmailTemplateVariables.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { useDebounce } from 'runed';
	import { LoaderCircle } from 'lucide-svelte';

	const { data } = $props();
	const { emailConfig, schema } = data;

	let formState = $derived.by(() => {
		const output: { slots: { [key: string]: string }; subject?: string } = {
			slots: {}
		};
		schema.slots.map(
			(slot) => (output.slots[slot.key] = emailConfig.slots[slot.key] as string)
		);

		if (emailConfig.subject) output.subject = emailConfig.subject;

		return output;
	});

	let pageTitle = $derived(`Edit Custom Email: ${emailConfig.emailType}`);
	let previewHtml = $state('');
	let openPreviewModal = $state(false);
	let previewIframeSrc = $state('');
	let openDeleteModal = $state(false);
	let deleting = $state(false);

	$effect(() => {
		if (!previewHtml) return;

		const blob = new Blob([previewHtml], { type: 'text/html' });
		const url = URL.createObjectURL(blob);
		previewIframeSrc = url;

		return () => URL.revokeObjectURL(url);
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();

		const parsed: { [key: string]: string } = {};
		Object.entries(formState.slots).map(([key, value]) => {
			const matched = schema.slots.find((slot) => slot.key === key);
			if (!matched) return;

			if (matched.content_type === 'rich_text') {
				parsed[key] = jsonToHtml(value);
			} else {
				parsed[key] = value;
			}
		});

		try {
			await apiClient.UpdateEmailTemplateConfig(
				{
					slots: { type: emailConfig.emailType, ...parsed },
					...(formState.subject && { subject: formState.subject })
				},
				{ params: { email_config_id: emailConfig.id } }
			);

			notifications.send({
				priority: 'INFO',
				message: 'Successfully updated custom email'
			});

			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message: 'Something went wrong updating your custom email'
			});
		}
	}

	async function handleDelete() {
		try {
			apiClient.DeleteEmailTemplateConfig(undefined, {
				params: { email_config_id: emailConfig.id }
			});

			goto('/admin/email-template-configs', { invalidateAll: true });
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Something went wrong deleting your custom email',
				priority: 'ERROR'
			});
		}
	}

	const debouncedPreviewEmail = useDebounce(async () => {
		try {
			const parsed: { [key: string]: string } = {};
			Object.entries(formState.slots).map(([key, value]) => {
				const matched = schema.slots.find((slot) => slot.key === key);
				if (!matched) return;

				if (matched.content_type === 'rich_text') {
					parsed[key] = jsonToHtml(value);
				} else {
					parsed[key] = value;
				}
			});

			const response = await apiClient.PreviewEmailTemplateConfig({
				slots: { type: schema.email_type, ...parsed }
			});

			previewHtml = response.html;
		} catch (e) {
			console.error(e);
		}
	}, 300);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<h1 class="text-4xl font-bold">Custom email: {snakeToSentenceCase(emailConfig.emailType)}</h1>

<div class="flex justify-end gap-2">
	<Button variant="destructive" type="button" onclick={() => (openDeleteModal = true)}
		>Delete</Button
	>
	<Button
		variant="default"
		type="button"
		onclick={() => {
			debouncedPreviewEmail();
			openPreviewModal = true;
		}}>Preview</Button
	>
</div>

{#if schema.variables.length > 0}
	<EmailTemplateVariables templateVariables={schema.variables} />
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
	{#each schema.slots as slot (slot.key)}
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

	<div class="flex justify-center">
		<Button type="submit">Submit</Button>
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

<AlertDialog.Root open={openDeleteModal}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete custom email?</AlertDialog.Title>
			<AlertDialog.Description>
				This will permanently remove the custom email for this email type.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={deleting}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action disabled={deleting} onclick={handleDelete}>
				{#if deleting}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
