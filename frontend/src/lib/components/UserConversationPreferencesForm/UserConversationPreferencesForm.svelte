<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Switch } from '$lib/components/ui/switch';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';
	import { Settings } from 'lucide-svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { userConversationPreferencesSchema } from './schema';
	import type { UserConversationPreferences } from '$lib/api/api';
	import { onMount } from 'svelte';

	let {
		conversationId
	}: {
		conversationId: string;
	} = $props();

	let loading = $state(true);
	let saving = $state(false);
	let loadError = $state<string | null>(null);

	const form = superForm(
		{
			receive_updates_by_notification: false,
			receive_updates_by_email: false,
			receive_similar_conversation_updates_by_email: false,
			receive_similar_conversation_updates_by_notification: false
		},
		{
			validators: zodClient(userConversationPreferencesSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onChange: savePreferences
		}
	);

	const { form: formData, enhance, validateForm, errors } = form;

	onMount(async () => {
		await loadPreferences();
	});

	function updateFormWithPreferences(prefs: UserConversationPreferences) {
		$formData.receive_updates_by_notification = prefs.receive_updates_by_notification;
		$formData.receive_updates_by_email = prefs.receive_updates_by_email;
		$formData.receive_similar_conversation_updates_by_email =
			prefs.receive_similar_conversation_updates_by_email;
		$formData.receive_similar_conversation_updates_by_notification =
			prefs.receive_similar_conversation_updates_by_notification;
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
			loadError = error?.response?.data?.message || 'Failed to load preferences. Please try again.';
		} finally {
			loading = false;
		}
	}

	async function savePreferences() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

		try {
			saving = true;
			const updatedPreferences = await apiClient.UpdateUserPreferenceForConversation(result.data, {
				params: { conversation_id: conversationId }
			});
		} catch (error: any) {
			notifications.send({
				message: error?.response?.data?.message || 'Failed to save preferences. Please try again.',
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
			<div class=" p-4">
				<h4 class="text-base font-medium">Conversation Updates</h4>
				<p class="text-muted-foreground text-sm">
					Get notified about new activities in this conversation.
				</p>

				<Form.Field {form} name="receive_updates_by_notification" class="space-y-2">
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-sm font-normal">In-app notifications</Form.Label>
								<Switch
									{...props}
									bind:checked={$formData.receive_updates_by_notification}
									disabled={saving}
								/>
							</div>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>

				<Form.Field {form} name="receive_updates_by_email" class="space-y-2">
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-sm font-normal">Email notifications</Form.Label>
								<Switch
									{...props}
									bind:checked={$formData.receive_updates_by_email}
									disabled={saving}
								/>
							</div>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>

			<div class=" p-4">
				<h4 class="text-base font-medium">Similar Conversations</h4>
				<p class="text-muted-foreground text-sm">
					Get notified about updates in similar conversations that might interest you.
				</p>

				<Form.Field
					{form}
					name="receive_similar_conversation_updates_by_notification"
					class="space-y-2"
				>
					<Form.Control>
						{#snippet children({ props })}
							<div class="flex items-center justify-between">
								<Form.Label class="text-sm font-normal">In-app notifications</Form.Label>
								<Switch
									{...props}
									bind:checked={$formData.receive_similar_conversation_updates_by_notification}
									disabled={saving}
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
								<Form.Label class="text-sm font-normal">Email notifications</Form.Label>
								<Switch
									{...props}
									bind:checked={$formData.receive_similar_conversation_updates_by_email}
									disabled={saving}
								/>
							</div>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>
		</div>
	</form>
{/if}

