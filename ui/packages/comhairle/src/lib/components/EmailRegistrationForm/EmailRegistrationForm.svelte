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
	import { Button } from '$lib/components/ui/button';
	import { m } from '$lib/paraglide/messages';

	type Props = {
		conversation_id: string;
		onSuccess?: () => void;
		/** Shows a Done button under the confirmation, e.g. to close the dialog around the form. */
		onDone?: () => void;
	};

	let { conversation_id, onSuccess, onDone }: Props = $props();
	let registering = $state(false);
	// Success toasts are switched off app-wide, so the confirmation replaces the form instead.
	let confirmation = $state<string | null>(null);

	const emailRegistrationSchema = z.object({
		email: z.string().email(m.please_enter_a_valid_email()),
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
			notifications.send({ priority: 'ERROR', message: m.email_updates_register_failed() });
			$message = m.email_updates_register_failed();
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
	<div class="flex flex-col gap-6">
		<p class="text-lg" role="status">{confirmation}</p>
		{#if onDone}
			<Button size="xl" class="w-full" onclick={onDone}>
				{m.done()}
			</Button>
		{/if}
	</div>
{:else}
	<form class="flex flex-col gap-6" method="POST" use:enhance>
		<Form.Field {form} name="email">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-lg">{m.email()}</Form.Label>
					<Input
						type="email"
						class="h-12 text-lg md:text-lg"
						{...props}
						bind:value={$formData.email}
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="receive_updates_by_email" class="space-y-2">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-6">
						<div class="flex flex-col gap-1">
							<Form.Label class="text-lg leading-snug"
								>{m.email_updates_this_conversation()}</Form.Label
							>
							<Form.Description class="text-muted-foreground text-base">
								{m.email_updates_this_conversation_consent()}
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
					<div class="flex items-center justify-between gap-6">
						<div class="flex flex-col gap-1">
							<Form.Label class="text-lg leading-snug"
								>{m.email_updates_future()}</Form.Label
							>
							<Form.Description class="text-muted-foreground text-base">
								{m.email_updates_future_consent()}
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

		<Form.Button size="xl" class="w-full" disabled={registering}>
			{registering ? m.email_updates_registering() : m.email_updates_register()}
		</Form.Button>

		{#if $message}
			<p class="text-destructive text-base">{$message}</p>
		{/if}
	</form>
{/if}
