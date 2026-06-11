<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import Label from '$lib/components/ui/label/label.svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { jsonToHtml } from '$lib/utils/rich-text.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { goto, invalidateAll } from '$app/navigation';

	let { data } = $props();
	const { schemas } = data;

	let selectedSchema = $state(schemas[0]);
	let formState = $derived.by(() => {
		const output: { [key: string]: string } = {};
		selectedSchema.slots.map((slot) => (output[slot.key] = ''));
		return output;
	});

	function handleSelectSchema(value: string) {
		selectedSchema = schemas.find((s) => s.email_type === value);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		const toHtml: { [key: string]: string } = {};
		Object.entries(formState).map(([key, value]) => (toHtml[key] = jsonToHtml(value)));

		try {
			const emailConfig = await apiClient.CreateEmailTemplateConfig({
				slots: { type: selectedSchema.email_type, ...toHtml }
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
</script>

<Select.Root type="single" value={selectedSchema.email_type} onValueChange={handleSelectSchema}>
	<Select.Trigger class=""
		>Email type: {snakeToSentenceCase(selectedSchema.email_type)}</Select.Trigger
	>
	<Select.Content>
		{#each schemas as schema (schema.email_type)}
			<Select.Item value={schema.email_type}
				>{snakeToSentenceCase(schema.email_type)}</Select.Item
			>
		{/each}
	</Select.Content>
</Select.Root>

<form onsubmit={handleSubmit}>
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
				value={formState[slot.key]}
				placeholder="Enter email content..."
				onChange={(json) => (formState[slot.key] = json)}
			/>
		</div>
	{/each}

	<div class="flex justify-center">
		<Button type="submit">Submit</Button>
	</div>
</form>
