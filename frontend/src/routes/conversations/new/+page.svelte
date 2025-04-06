<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';
	import NewConversationSchema from './NewConversationSchema';
	import { notifications } from '$lib/notifications.svelte';
	import { goto } from '$app/navigation';
	import { z } from 'zod';
	import { conversation_url } from '$lib/urls';
	import { apiClient } from '$lib/api/client';

	let { data } = $props();

	const form = superForm(data.form, {
		SPA: true,
		validators: zodClient(NewConversationSchema),
		resetForm: false
	});

	const { form: formData, enhance, message: errMessage, validateForm } = form;

	let submitting = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		submitting = true;
		const result = await validateForm();

		if (result.valid) {
			try {
				let convo = await apiClient.CreateConversation(result.data);
				notifications.addFlash({ message: 'Conversastion Created' });
				goto(conversation_url(convo.id));
			} catch (e) {
				console.warn(e);
				notifications.send({ message: 'Something went wrong creating the conversation' });
			}
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4" method="POST" use:enhance>
	{#if $errMessage}
		<p class="text-sm text-destructive">{$errMessage}</p>
	{/if}

	<Form.Field {form} name="title">
		<Form.Control let:attrs>
			<Form.Label>{m.enter_a_title_for_the_conversation()}</Form.Label>
			<Input {...attrs} bind:value={$formData.title} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="short_description">
		<Form.Control let:attrs>
			<Form.Label>{m.short_description()}</Form.Label>
			<Input {...attrs} bind:value={$formData.short_description} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="description">
		<Form.Control let:attrs>
			<Form.Label>{m.description()}</Form.Label>
			<Input {...attrs} bind:value={$formData.description} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="image_url">
		<Form.Control let:attrs>
			<Form.Label>{m.image_url()}</Form.Label>
			<Input {...attrs} bind:value={$formData.image_url} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="video_url">
		<Form.Control let:attrs>
			<Form.Label>{m.video_url()}</Form.Label>
			<Input {...attrs} bind:value={$formData.video_url} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="is_public">
		<Form.Control let:attrs>
			<Form.Label>{m.is_public()}</Form.Label>
			<Input {...attrs} bind:value={$formData.is_public} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="is_invite_only">
		<Form.Control let:attrs>
			<Form.Label>{m.is_invite_only()}</Form.Label>
			<Input {...attrs} bind:value={$formData.is_invite_only} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Button disabled={submitting} fullWidth variant="default">Create</Form.Button>
</form>
