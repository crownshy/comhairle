<script lang="ts">
	import { z } from 'zod';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import Input from '$lib/components/ui/input/input.svelte';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';

	type Props = {
		conversation_id: string;
		onSuccess?: () => void;
	};

	let { conversation_id, onSuccess }: Props = $props();
	let registering = $state(false);

	const emailRegistrationSchema = z.object({
		email: z.string().email('Please enter a valid email address')
	});

	async function registerEmail() {
		const result = await validateForm({ update: true });
		if (!result.valid) {
			return;
		}

		try {
			registering = true;
			const response = await apiClient.RegisterEmailForUpdates(
				{ email: result.data.email },
				{ params: { conversation_id } }
			);
			notifications.send({ message: response.message });
			onSuccess?.();
		} catch (e) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to register email for updates'
			});
			$message = 'Failed to register email for updates';
		} finally {
			registering = false;
		}
	}

	const form = superForm(defaults(zod(emailRegistrationSchema)), {
		validators: zodClient(emailRegistrationSchema),
		taintedMessage: false,
		validationMethod: 'oninput',
		onSubmit: registerEmail
	});

	let { form: formData, enhance, validateForm, message } = form;
</script>

<form method="POST" class="space-y-4" use:enhance>
	<Form.Field {form} name="email">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Email</Form.Label>
				<Input type="email" {...props} bind:value={$formData.email} />
			{/snippet}
		</Form.Control>
		<Form.Description class="text-muted-foreground">
			Register your email to receive updates about this conversation.
		</Form.Description>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Button class="w-full" disabled={registering}>
		{registering ? 'Registering...' : 'Register for Updates'}
	</Form.Button>

	{#if $message}
		<p class="text-destructive text-sm">{$message}</p>
	{/if}
</form>

