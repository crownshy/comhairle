<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Alert from '$lib/components/ui/alert';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { AlertTriangle, ChevronDown, Users } from 'lucide-svelte';
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

	let participantCount = $state<number | null>(null);
	let emailRecipients = $state<string[]>([]);
	let recipientsError = $state<string | null>(null);
	let recipientsLoading = $state(true);
	let recipientsOpen = $state(false);

	async function loadRecipients() {
		recipientsLoading = true;
		recipientsError = null;
		try {
			const response = await apiClient.GetNotificationRecipients({
				params: { conversation_id: conversationId }
			});
			participantCount = response.participantCount;
			emailRecipients = response.emailRecipients;
		} catch (error: any) {
			recipientsError = error?.response?.data?.message || 'Could not load recipient preview.';
		} finally {
			recipientsLoading = false;
		}
	}

	$effect(() => {
		loadRecipients();
	});

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
			loadRecipients();
		} catch (error: any) {
			notifications.send({
				message:
					error?.response?.data?.message ||
					`Failed to send ${isEmail ? 'email' : 'notification'}. Please try again.`,
				priority: 'ERROR'
			});
		}
	}

	let testDialogOpen = $state(false);
	let testEmailAddress = $state('');
	let testEmailError = $state<string | null>(null);
	let testSending = $state(false);

	const EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

	async function sendTestEmail() {
		testEmailError = null;

		const result = await validateForm({ update: true });
		if (!result.valid) {
			testEmailError = 'Fill in the email subject and body before sending a test.';
			return;
		}

		const recipient = testEmailAddress.trim();
		if (!EMAIL_PATTERN.test(recipient)) {
			testEmailError = 'Enter a valid email address.';
			return;
		}

		testSending = true;
		try {
			const response = await apiClient.SendNotificationToParticipants(
				{
					title: $form.title,
					content: $form.content,
					notification_type: 'info' as const,
					delivery_method: 'email',
					html_content: jsonToHtml($form.content),
					test_email_recipient: recipient
				},
				{ params: { conversation_id: conversationId } }
			);

			notifications.send({
				message: response.message || `Test email sent to ${recipient}`,
				priority: 'SUCCESS'
			});

			testDialogOpen = false;
			testEmailAddress = '';
		} catch (error: any) {
			testEmailError =
				error?.response?.data?.message || 'Failed to send test email. Please try again.';
		} finally {
			testSending = false;
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

		<div class="flex flex-col gap-2 sm:flex-row">
			<Form.Button class="flex-1" disabled={$submitting}>
				<Send class="mr-2 h-4 w-4" />
				{#if $submitting}
					Sending {isEmail ? 'email' : 'notification'}...
				{:else}
					Send {isEmail ? 'email' : 'notification'} to all participants
				{/if}
			</Form.Button>

			{#if isEmail}
				<Button
					type="button"
					variant="outline"
					disabled={$submitting || testSending}
					onclick={() => {
						testEmailError = null;
						testDialogOpen = true;
					}}
				>
					Send test email
				</Button>
			{/if}
		</div>

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

	<aside class="w-full space-y-4 lg:w-96 lg:shrink-0">
		<div class="rounded-lg border p-4">
			<div class="mb-3 flex items-center gap-2">
				<Users class="h-4 w-4" />
				<h2 class="text-sm font-semibold">Recipients preview</h2>
			</div>

			{#if recipientsLoading}
				<p class="text-muted-foreground text-sm">Loading…</p>
			{:else if recipientsError}
				<p class="text-destructive text-sm">{recipientsError}</p>
			{:else if isEmail}
				{#if emailRecipients.length === 0}
					<p class="text-muted-foreground text-sm">
						No participants have opted in to email updates yet.
					</p>
				{:else}
					<p class="text-sm">
						This email will be sent to
						<span class="font-semibold">{emailRecipients.length}</span>
						{emailRecipients.length === 1 ? 'recipient' : 'recipients'}.
					</p>
					<Collapsible.Root bind:open={recipientsOpen} class="mt-2">
						<Collapsible.Trigger
							class="text-primary inline-flex items-center gap-1 text-sm hover:underline"
						>
							{recipientsOpen ? 'Hide' : 'Show'} addresses
							<ChevronDown
								class="h-3 w-3 transition-transform {recipientsOpen
									? 'rotate-180'
									: ''}"
							/>
						</Collapsible.Trigger>
						<Collapsible.Content>
							<ul
								class="bg-muted/40 mt-2 max-h-64 list-inside list-disc overflow-y-auto rounded-md p-3 font-mono text-xs"
							>
								{#each emailRecipients as email (email)}
									<li>{email}</li>
								{/each}
							</ul>
						</Collapsible.Content>
					</Collapsible.Root>
				{/if}
			{:else if participantCount === 0}
				<p class="text-muted-foreground text-sm">
					No participants have joined any workflows in this conversation yet.
				</p>
			{:else}
				<p class="text-sm">
					This notification will be sent to
					<span class="font-semibold">{participantCount}</span>
					workflow {participantCount === 1 ? 'participant' : 'participants'}.
				</p>
			{/if}
		</div>

		{#if failedRecipients.length > 0}
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
		{/if}
	</aside>
</div>

<Dialog.Root bind:open={testDialogOpen}>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Send test email</Dialog.Title>
			<Dialog.Description>
				Send the current subject and body to a single address. Recipients opted in to this
				conversation will not receive this preview.
			</Dialog.Description>
		</Dialog.Header>
		<div class="space-y-2 py-2">
			<Label for="test-email-recipient">Email address</Label>
			<Input
				id="test-email-recipient"
				type="email"
				placeholder="you@example.com"
				bind:value={testEmailAddress}
				disabled={testSending}
				onkeydown={(e) => {
					if (e.key === 'Enter') {
						e.preventDefault();
						sendTestEmail();
					}
				}}
			/>
			{#if testEmailError}
				<p class="text-destructive text-sm">{testEmailError}</p>
			{/if}
		</div>
		<Dialog.Footer>
			<Button
				type="button"
				variant="outline"
				disabled={testSending}
				onclick={() => (testDialogOpen = false)}
			>
				Cancel
			</Button>
			<Button type="button" onclick={sendTestEmail} disabled={testSending}>
				{testSending ? 'Sending…' : 'Send test'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
