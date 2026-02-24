<script lang="ts">
	import { z } from 'zod';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';

	type Props = {
		conversation_id: string;
		onSuccess?: () => void;
	};

	let { conversation_id, onSuccess }: Props = $props();
	let registering = $state(false);

	const emailRegistrationSchema = z.object({
		email: z.string().email('Please enter a valid email address'),
		receive_updates_by_email: z.boolean().default(false),
		receive_similar_conversation_updates_by_email: z.boolean().default(false)
	});

	async function registerEmail() {
		const result = await validateForm({ update: true });
		if (!result.valid) {
			return;
		}

		try {
			registering = true;
			const response = await apiClient.RegisterEmailForUpdates(
				{
					email: result.data.email,
					receive_updates_by_email: result.data.receive_updates_by_email,
					receive_similar_conversation_updates_by_email:
						result.data.receive_similar_conversation_updates_by_email
				},
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

<form class="space-y-4 border-1 bg-white p-5" method="POST" use:enhance>
	<Form.Field {form} name="email">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Email</Form.Label>
				<Input type="email" {...props} bind:value={$formData.email} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="receive_updates_by_email" class="space-y-2">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex items-center justify-between">
					<div class="space-y-1 leading-none">
						<Form.Label class="text-sm font-normal">Receive updates by email</Form.Label>
						<Form.Description class="text-muted-foreground text-xs">
							By ticking this box I allow Scottish Government or an organisation acting on behalf of
							Scottish Government to contact me to provide me updates relating to this engagement on
							the National Performance Framework.
						</Form.Description>
					</div>
					<Switch
						{...props}
						bind:checked={$formData.receive_updates_by_email}
						disabled={registering}
					/>
				</div>
				<Form.FieldErrors />
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Field {form} name="receive_similar_conversation_updates_by_email" class="space-y-2">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex items-center justify-between">
					<div class="space-y-1 leading-none">
						<Form.Label class="text-sm font-normal"
							>Receive emails about future engagments</Form.Label
						>
						<Form.Description class="text-muted-foreground text-xs">
							By ticking this box I agree that I am willing to be contacted for future engagement or
							research purposes relating to the National Performance Framework.
						</Form.Description>
					</div>
					<Switch
						{...props}
						bind:checked={$formData.receive_similar_conversation_updates_by_email}
						disabled={registering}
					/>
				</div>
				<Form.FieldErrors />
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Button class="w-full" disabled={registering}>
		{registering ? 'Registering...' : 'Register for Updates'}
	</Form.Button>

	{#if $message}
		<p class="text-destructive text-sm">{$message}</p>
	{/if}
</form>
