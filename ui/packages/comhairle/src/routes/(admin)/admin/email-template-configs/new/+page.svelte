<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Dialog from '$lib/components/ui/dialog';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import Label from '$lib/components/ui/label/label.svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { jsonToHtml } from '$lib/utils/rich-text.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { goto, invalidateAll } from '$app/navigation';
	import EmailTemplateVariables from '../EmailTemplateVariables.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { useDebounce } from 'runed';

	type FormState = {
		subject?: string;
		slots: { [key: string]: string };
	};

	let { data } = $props();
	const { schemas } = data;

	let selectedSchema = $state(schemas[0]);
	let formState = $derived.by(() => {
		const output: FormState = {
			slots: {}
		};
		selectedSchema.slots.map((slot) => (output.slots[slot.key] = ''));
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

	async function handleSubmit(e: Event) {
		e.preventDefault();

		const toHtml: { [key: string]: string } = {};
		Object.entries(formState.slots).map(([key, value]) => (toHtml[key] = jsonToHtml(value)));

		try {
			const emailConfig = await apiClient.CreateEmailTemplateConfig({
				slots: {
					type: selectedSchema.email_type,
					...toHtml
				},
				...(formState.subject && { subject: formState.subject })
			});

			notifications.send({
				priority: 'INFO',
				message: 'Successfully create new custom email'
			});

			await invalidateAll();

			goto(`/admin/email-template-configs/${emailConfig.id}`);
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message: 'Something went wrong creating your custom email'
			});
		}
	}

	const debouncedPreviewEmail = useDebounce(async () => {
		try {
			const toHtml: { [key: string]: string } = {};
			Object.entries(formState.slots).forEach(([key, value]) => {
				if (value) {
					toHtml[key] = jsonToHtml(value);
				}
			});

			const response = await apiClient.PreviewEmailTemplateConfig({
				slots: { type: selectedSchema.email_type, ...toHtml }
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

<h1 class="text-4xl font-bold">Create new custom email</h1>

<div class="flex justify-between">
	<div class="flex flex-col gap-4">
		<h2 class="text-xl font-semibold">Email type:</h2>
		<Select.Root
			type="single"
			value={selectedSchema.email_type}
			onValueChange={handleSelectSchema}
		>
			<Select.Trigger class=""
				>{snakeToSentenceCase(selectedSchema.email_type)}</Select.Trigger
			>
			<Select.Content>
				{#each schemas as schema (schema.email_type)}
					<Select.Item value={schema.email_type}
						>{snakeToSentenceCase(schema.email_type)}</Select.Item
					>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>
	<Button
		variant="default"
		type="button"
		onclick={() => {
			debouncedPreviewEmail();
			openPreviewModal = true;
		}}>Preview</Button
	>
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
			<RichTextEditor
				width="100%"
				value={formState.slots[slot.key]}
				placeholder="Enter email content..."
				onChange={(json) => (formState.slots[slot.key] = json)}
			/>
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
