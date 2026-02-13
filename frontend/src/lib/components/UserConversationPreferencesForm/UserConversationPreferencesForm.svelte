<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Switch } from '$lib/components/ui/switch';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { userConversationPreferencesSchema } from './schema';
	import type { UserConversationPreferencesDto } from '$lib/api/api';
	import { onMount } from 'svelte';

	let {
		conversationId,
		isAnnon
	}: {
		conversationId: string;
		isAnnon: boolean;
	} = $props();

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
		try {
			loading = true;
			loadError = null;
			const loadedPreferences = await apiClient.GetUserPreferenceForConversation({
				params: { conversation_id: conversationId }
			});
			updateFormWithPreferences(loadedPreferences);
		} catch (error: any) {
			loadError =
				error?.response?.data?.message || 'Failed to load preferences. Please try again.';
		} finally {
			loading = false;
		}
	}

	async function savePreferences() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

		try {
			saving = true;
			const updatedPreferences = await apiClient.UpdateUserPreferenceForConversation(
				result.data,
				{
					params: { conversation_id: conversationId }
				}
			);
		} catch (error: any) {
			console.error(error);
			notifications.send({
				message:
					error?.response?.data?.message ||
					'Failed to save preferences. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

{#if loading}
	<div class="flex items-center justify-center p-6">
		<div class="border-primary h-8 w-8 animate-spin rounded-full border-b-2"></div>
		<span class="text-muted-foreground ml-2">Loading preferences...</span>
	</div>
{:else if loadError}
	<div class="flex flex-col items-center justify-center space-y-4 p-6">
		<div class="text-destructive text-center">
			<h3 class="mb-2 font-semibold">Failed to Load Preferences</h3>
			<p class="text-sm">{loadError}</p>
		</div>
		<button
			class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2 transition-colors"
			onclick={() => loadPreferences()}
			disabled={loading}
		>
			{loading ? 'Retrying...' : 'Try Again'}
		</button>
	</div>
{:else}
	<form method="POST" class="w-full" use:enhance>
		<div class="flex flex-col gap-2 md:flex-row">
			<div class="flex-1 p-4">
				<h4 class="text-base font-medium">Conversation Updates</h4>
				<p class="text-muted-foreground my-2 text-sm">
					Get notified about new activities in this conversation.
				</p>

				<Form.Field {form} name="receiveUpdatesByNotification" class="space-y-2">
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-sm font-normal"
									>In-app notifications</Form.Label
								>
								<Switch
									{...props}
									bind:checked={$formData.receiveUpdatesByNotification}
									disabled={saving}
								/>
							</div>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
				{#if !isAnnon}
					<Form.Field {form} name="receiveUpdatesByEmail" class="space-y-2">
						<Form.Control>
							{#snippet children({ props })}
								<div class="flex items-center justify-between">
									<Form.Label class="text-sm font-normal"
										>Email notifications</Form.Label
									>
									<Switch
										{...props}
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

			<div class="flex-1 p-4">
				<h4 class="text-base font-medium">Similar Conversations</h4>
				<p class="text-muted-foreground my-2 text-sm">
					Get notified about updates in similar conversations that might interest you.
				</p>

				<Form.Field
					{form}
					name="receiveSimilarConversationUpdatesByNotification"
					class="space-y-2"
				>
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-sm font-normal"
									>In-app notifications</Form.Label
								>
								<Switch
									{...props}
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

				{#if !isAnnon}
					<Form.Field
						{form}
						name="receive_similar_conversation_updates_by_email"
						class="space-y-2"
					>
						<Form.Control>
							{#snippet children({ props })}
								<div class="flex items-center justify-between">
									<Form.Label class="text-sm font-normal"
										>Email notifications</Form.Label
									>
									<Switch
										{...props}
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
