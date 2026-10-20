<script lang="ts">
	import { z } from 'zod';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	type Props = {
		conversation_id: string;
		onSuccess?: () => void;
	};

	let { conversation_id, onSuccess }: Props = $props();
	let registering = $state(false);
	// Success toasts are switched off app-wide, so the confirmation replaces the form instead.
	let confirmation = $state<string | null>(null);

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

		registering = true;
		const registration = await tryCatchAsync(() =>
			apiClient.RegisterEmailForUpdates(
				{
					email: result.data.email,
					receive_updates_by_email: result.data.receive_updates_by_email,
					receive_similar_conversation_updates_by_email:
						result.data.receive_similar_conversation_updates_by_email
				},
				{ params: { conversation_id } }
			)
		);
		registering = false;

		if (registration.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to register email for updates'
			});
			$message = 'Failed to register email for updates';
			return;
		}

		confirmation = registration.ok.message;
		onSuccess?.();
	}

	const form = superForm(defaults(zod(emailRegistrationSchema)), {
		validators: zodClient(emailRegistrationSchema),
		taintedMessage: false,
		validationMethod: 'oninput',
		// Submit through the API only; the page has no form action to post to.
		onSubmit: async ({ cancel }) => {
			cancel();
			await registerEmail();
		}
	});

	let { form: formData, enhance, validateForm, message } = form;
</script>

{#if confirmation}
	<p class="bg-background border p-5 text-base" role="status">{confirmation}</p>
{:else}
	<form class="bg-background space-y-4 border p-5" method="POST" use:enhance>
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
							<Form.Label class="text-sm font-normal"
								>Receive updates by email</Form.Label
							>
							<Form.Description class="text-muted-foreground text-xs">
								By ticking this box I agree to receive updates relating to this
								engagement.
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
								By ticking this box I agree to be contacted about future engagments.
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
{/if}
