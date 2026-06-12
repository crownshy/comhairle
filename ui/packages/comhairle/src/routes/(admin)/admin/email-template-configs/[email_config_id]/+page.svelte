<script lang="ts">
	import Label from '$lib/components/ui/label/label.svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { jsonToHtml } from '$lib/utils/rich-text.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils.js';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidateAll } from '$app/navigation';

	const { data } = $props();
	const { emailConfig, schema } = data;

	let formState = $derived.by(() => {
		const output: { [key: string]: string } = {};
		schema.map((slot) => (output[slot.key] = emailConfig.slots[slot.key] as string));
		return output;
	});

	let pageTitle = $derived(`Edit Custom Email: ${emailConfig.emailType}`);

	async function handleSubmit(e: Event) {
		e.preventDefault();

		const toHtml: { [key: string]: string } = {};
		Object.entries(formState).map(([key, value]) => (toHtml[key] = jsonToHtml(value)));

		try {
			await apiClient.UpdateEmailTemplateConfig(
				{
					slots: { type: emailConfig.emailType, ...toHtml }
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
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<h1 class="text-4xl font-bold">Custom email: {snakeToSentenceCase(emailConfig.emailType)}</h1>
<form onsubmit={handleSubmit}>
	{#each schema as slot (slot.key)}
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
