<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { Send } from 'lucide-svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { notificationFormSchema } from './NotificationForm/schema';

	let { conversationId }: { conversationId: string } = $props();

	const notificationForm = superForm(
		{
			title: '',
			content: ''
		},
		{
			validators: zodClient(notificationFormSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: sendNotification
		}
	);

	const { form, enhance, validateForm, submitting, errors } = notificationForm;

	async function sendNotification() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

		try {
			const response = await apiClient.SendNotificationToParticipants(
				{ ...result.data, notification_type: 'info', delivery_method: 'in_app' },
				{
					params: { conversation_id: conversationId }
				}
			);

			notifications.send({
				message: response.message || 'Notification sent successfully!',
				priority: 'SUCCESS'
			});

			// Reset form
			$form.title = '';
			$form.content = '';
		} catch (error: any) {
			notifications.send({
				message:
					error?.response?.data?.message ||
					'Failed to send notification. Please try again.',
				priority: 'ERROR'
			});
		}
	}
</script>

<form method="POST" class="space-y-6" use:enhance>
	<Form.Field form={notificationForm} name="title" class="space-y-2">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label class="text-sm font-medium">Notification Title</Form.Label>
				<div class="space-y-1">
					<Input
						{...props}
						bind:value={$form.title}
						placeholder="Enter notification title..."
						disabled={$submitting}
					/>
					<Form.FieldErrors />
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Field form={notificationForm} name="content" class="space-y-2">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label class="text-sm font-medium">Message Content</Form.Label>
				<div class="space-y-1">
					<Textarea
						{...props}
						bind:value={$form.content}
						placeholder="Enter your notification message here..."
						rows={4}
						disabled={$submitting}
					/>
					<Form.FieldErrors />
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Button class="w-full" disabled={$submitting}>
		<Send class="mr-2 h-4 w-4" />
		{$submitting ? 'Sending notification...' : 'Send Notification to All Participants'}
	</Form.Button>

	<p class="text-muted-foreground text-sm">
		This will send the notification to all users who have participated in workflows within this
		conversation. They will receive the notification via the selected delivery method.
	</p>
</form>
