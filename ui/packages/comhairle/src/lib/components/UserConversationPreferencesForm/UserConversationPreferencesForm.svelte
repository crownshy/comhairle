<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Switch } from '$lib/components/ui/switch';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { userConversationPreferencesSchema } from './schema';
	import type { UserConversationPreferencesDto } from '@crownshy/api-client/api';
	import { onMount } from 'svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { m } from '$lib/paraglide/messages';

	type ApiError = { response?: { data?: { message?: string } } };

	type Props = {
		conversationId: string;
		isGuest: boolean;
	};
	let { conversationId, isGuest }: Props = $props();

	let loading = $state(true);
	let saving = $state(false);
	let loadError = $state<string | null>(null);

	const form = superForm(
		{
			receiveUpdatesByNotification: false,
			receiveUpdatesByEmail: false,
			receiveSimilarConversationUpdatesByEmail: false,
			receiveSimilarConversationUpdatesByNotification: false
		},
		{
			validators: zodClient(userConversationPreferencesSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onChange: savePreferences
		}
	);

	const { form: formData, enhance, validateForm } = form;

	onMount(async () => {
		await loadPreferences();
	});

	function updateFormWithPreferences(prefs: UserConversationPreferencesDto) {
		$formData.receiveUpdatesByNotification = prefs.receiveUpdatesByNotification;
		$formData.receiveUpdatesByEmail = prefs.receiveUpdatesByEmail;
		$formData.receiveSimilarConversationUpdatesByEmail =
			prefs.receiveSimilarConversationUpdatesByEmail;
		$formData.receiveSimilarConversationUpdatesByNotification =
			prefs.receiveSimilarConversationUpdatesByNotification;
	}

	async function loadPreferences() {
		loading = true;
		loadError = null;
		const loaded = await tryCatchAsync<UserConversationPreferencesDto, ApiError>(() =>
			apiClient.GetUserPreferenceForConversation({
				params: { conversation_id: conversationId }
			})
		);
		loading = false;

		if (loaded.err !== null) {
			loadError = loaded.err.response?.data?.message || m.update_preferences_load_failed();
			return;
		}
		updateFormWithPreferences(loaded.ok);
	}

	async function savePreferences() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

		saving = true;
		const saved = await tryCatchAsync<UserConversationPreferencesDto, ApiError>(() =>
			apiClient.UpdateUserPreferenceForConversation(result.data, {
				params: { conversation_id: conversationId }
			})
		);
		saving = false;

		if (saved.err !== null) {
			notifications.send({
				message: saved.err.response?.data?.message || m.update_preferences_save_failed(),
				priority: 'ERROR'
			});
		}
	}
</script>

{#if loading}
	<div class="flex items-center justify-center p-6">
		<div class="border-primary h-8 w-8 animate-spin rounded-full border-b-2"></div>
		<span class="text-muted-foreground ml-2">{m.update_preferences_loading()}</span>
	</div>
{:else if loadError}
	<div class="flex flex-col items-center justify-center space-y-4 p-6">
		<div class="text-destructive text-center">
			<h3 class="mb-2 font-semibold">{m.update_preferences_load_failed_heading()}</h3>
			<p class="text-base">{loadError}</p>
		</div>
		<button
			class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-full px-4 py-2 transition-colors"
			onclick={() => loadPreferences()}
			disabled={loading}
		>
			{loading ? m.update_preferences_retrying() : m.try_again()}
		</button>
	</div>
{:else}
	<form method="POST" class="w-full" use:enhance>
		<div class="flex flex-col gap-6">
			<div class="flex flex-col gap-3">
				<h4 class="text-xl font-semibold">{m.update_preferences_conversation_heading()}</h4>
				<p class="text-muted-foreground text-lg">
					{m.update_preferences_conversation_body()}
				</p>

				<Form.Field {form} name="receiveUpdatesByNotification" class="space-y-2">
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-lg font-normal"
									>{m.update_preferences_in_app()}</Form.Label
								>
								<Switch
									{...props}
									aria-label={`${m.update_preferences_conversation_heading()}: ${m.update_preferences_in_app()}`}
									bind:checked={$formData.receiveUpdatesByNotification}
									disabled={saving}
								/>
							</div>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
				{#if !isGuest}
					<Form.Field {form} name="receiveUpdatesByEmail" class="space-y-2">
						<Form.Control>
							{#snippet children({ props })}
								<div class="flex items-center justify-between">
									<Form.Label class="text-lg font-normal"
										>{m.update_preferences_email()}</Form.Label
									>
									<Switch
										{...props}
										aria-label={`${m.update_preferences_conversation_heading()}: ${m.update_preferences_email()}`}
										bind:checked={$formData.receiveUpdatesByEmail}
										disabled={saving}
									/>
								</div>
								<Form.FieldErrors />
							{/snippet}
						</Form.Control>
					</Form.Field>
				{/if}
			</div>

			<div class="flex flex-col gap-3">
				<h4 class="text-xl font-semibold">{m.update_preferences_similar_heading()}</h4>
				<p class="text-muted-foreground text-lg">
					{m.update_preferences_similar_body()}
				</p>

				<Form.Field
					{form}
					name="receiveSimilarConversationUpdatesByNotification"
					class="space-y-2"
				>
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-lg font-normal"
									>{m.update_preferences_in_app()}</Form.Label
								>
								<Switch
									{...props}
									aria-label={`${m.update_preferences_similar_heading()}: ${m.update_preferences_in_app()}`}
									bind:checked={
										$formData.receiveSimilarConversationUpdatesByNotification
									}
									disabled={saving}
								/>
							</div>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>

				{#if !isGuest}
					<Form.Field
						{form}
						name="receiveSimilarConversationUpdatesByEmail"
						class="space-y-2"
					>
						<Form.Control>
							{#snippet children({ props })}
								<div class="flex items-center justify-between">
									<Form.Label class="text-lg font-normal"
										>{m.update_preferences_email()}</Form.Label
									>
									<Switch
										{...props}
										aria-label={`${m.update_preferences_similar_heading()}: ${m.update_preferences_email()}`}
										bind:checked={
											$formData.receiveSimilarConversationUpdatesByEmail
										}
										disabled={saving}
									/>
								</div>
								<Form.FieldErrors />
							{/snippet}
						</Form.Control>
					</Form.Field>
				{/if}
			</div>
		</div>
	</form>
{/if}
