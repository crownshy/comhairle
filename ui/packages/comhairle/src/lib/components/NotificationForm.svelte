<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Alert from '$lib/components/ui/alert';
	import { AlertTriangle } from 'lucide-svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { getBaseExtensions } from '$lib/components/RichTextEditor/editorConfig';
	import { generateHTML } from '@tiptap/core';
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
			content: '',
			delivery_method: 'in_app' as 'in_app' | 'email'
		},
		{
			validators: zodClient(notificationFormSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: sendNotification
		}
	);

	const { form, enhance, validateForm, submitting, reset } = notificationForm;

	let failedRecipients = $state<string[]>([]);
	let lastSendMessage = $state<string | null>(null);
	let lastSendStatus = $state<'partial' | 'failed' | null>(null);

	function jsonToHtml(jsonString: string): string {
		const json = JSON.parse(jsonString);
		return generateHTML(json, getBaseExtensions({ mode: 'renderer' }));
	}

	async function sendNotification({ cancel }: { cancel: () => void }) {
		// Prevent SvelteKit's default form submission — this page has no
		// action handler, and letting it proceed restores the submitted
		// values to the form (defeating our reset below).
		cancel();

		const result = await validateForm({ update: true });
		if (!result.valid) return;

		// Clear any prior failure state at the start of a fresh attempt.
		failedRecipients = [];
		lastSendMessage = null;
		lastSendStatus = null;

		const isEmail = $form.delivery_method === 'email';
		try {
			const payload = {
				title: $form.title,
				content: $form.content,
				notification_type: 'info' as const,
				delivery_method: $form.delivery_method,
				...(isEmail ? { html_content: jsonToHtml($form.content) } : {})
			};

			const response = await apiClient.SendNotificationToParticipants(payload, {
				params: { conversation_id: conversationId }
			});

			const failed = response.failedRecipients ?? [];
			const sentCount = response.participantsNotified ?? 0;

			if (failed.length > 0 && sentCount === 0) {
				failedRecipients = failed;
				lastSendMessage = response.message;
				lastSendStatus = 'failed';
				notifications.send({
					message: response.message,
					priority: 'ERROR'
				});
				return;
			}

			if (failed.length > 0) {
				failedRecipients = failed;
				lastSendMessage = response.message;
				lastSendStatus = 'partial';
				notifications.send({
					message: response.message,
					priority: 'WARNING'
				});
			} else {
				notifications.send({
					message: response.message || 'Sent successfully!',
					priority: 'SUCCESS'
				});
			}

			reset({ data: { title: '', content: '', delivery_method: $form.delivery_method } });
		} catch (error: any) {
			notifications.send({
				message:
					error?.response?.data?.message ||
					`Failed to send ${isEmail ? 'email' : 'notification'}. Please try again.`,
				priority: 'ERROR'
			});
		}
	}

	let isEmail = $derived($form.delivery_method === 'email');

	let lastDeliveryMethod = $state($form.delivery_method);
	$effect(() => {
		if ($form.delivery_method !== lastDeliveryMethod) {
			lastDeliveryMethod = $form.delivery_method;
			$form.content = '';
			failedRecipients = [];
			lastSendMessage = null;
			lastSendStatus = null;
		}
	});
</script>

<div class="flex flex-col gap-6 lg:flex-row lg:items-start">
	<form method="POST" class="w-full max-w-2xl space-y-6" use:enhance>
		<Form.Fieldset form={notificationForm} name="delivery_method" class="space-y-2">
			<Form.Legend class="text-sm font-medium">Delivery method</Form.Legend>
			<RadioGroup.Root
				bind:value={$form.delivery_method}
				class="flex flex-row gap-8"
				name="delivery_method"
			>
				<div class="flex items-center gap-1.5">
					<Form.Control>
						{#snippet children({ props })}
							<RadioGroup.Item value="in_app" {...props} />
							<Form.Label class="font-normal">In-app notification</Form.Label>
						{/snippet}
					</Form.Control>
				</div>
				<div class="flex items-center gap-1.5">
					<Form.Control>
						{#snippet children({ props })}
							<RadioGroup.Item value="email" {...props} />
							<Form.Label class="font-normal">Email</Form.Label>
						{/snippet}
					</Form.Control>
				</div>
			</RadioGroup.Root>
		</Form.Fieldset>

		<Form.Field form={notificationForm} name="title" class="space-y-2">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-medium">
						{isEmail ? 'Email subject' : 'Notification title'}
					</Form.Label>
					<div class="space-y-1">
						<Input
							{...props}
							bind:value={$form.title}
							placeholder={isEmail
								? 'Enter email subject...'
								: 'Enter notification title...'}
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
					<Form.Label class="text-sm font-medium">
						{isEmail ? 'Email body' : 'Message content'}
					</Form.Label>
					<div class="space-y-1">
						{#if isEmail}
							<RichTextEditor
								value={$form.content || null}
								placeholder="Compose your email..."
								editable={!$submitting}
								onChange={(json) => ($form.content = json)}
							/>
							<input type="hidden" {...props} value={$form.content} />
						{:else}
							<Textarea
								{...props}
								bind:value={$form.content}
								placeholder="Enter your notification message here..."
								rows={4}
								disabled={$submitting}
							/>
						{/if}
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>

		<Form.Button class="w-full" disabled={$submitting}>
			<Send class="mr-2 h-4 w-4" />
			{#if $submitting}
				Sending {isEmail ? 'email' : 'notification'}...
			{:else}
				Send {isEmail ? 'email' : 'notification'} to all participants
			{/if}
		</Form.Button>

		<p class="text-muted-foreground text-sm">
			{#if isEmail}
				This will email all participants who have opted in to email updates for this
				conversation.
			{:else}
				This will send the notification to all users who have participated in workflows
				within this conversation.
			{/if}
		</p>
	</form>

	{#if failedRecipients.length > 0}
		<aside class="w-full lg:w-96 lg:shrink-0">
			<Alert.Root variant="destructive">
				<AlertTriangle class="h-4 w-4" />
				<Alert.Title>
					{lastSendStatus === 'failed'
						? 'Email delivery failed'
						: 'Some recipients did not receive the email'}
				</Alert.Title>
				<Alert.Description>
					{#if lastSendMessage}
						<p class="mb-2">{lastSendMessage}</p>
					{/if}
					<p class="font-medium">
						{failedRecipients.length}
						{failedRecipients.length === 1 ? 'address' : 'addresses'} could not be reached:
					</p>
					<ul class="mt-1 list-inside list-disc font-mono text-sm">
						{#each failedRecipients as email (email)}
							<li>{email}</li>
						{/each}
					</ul>
				</Alert.Description>
			</Alert.Root>
		</aside>
	{/if}
</div>
