<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Switch } from '$lib/components/ui/switch';
	import * as Form from '$lib/components/ui/form/';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { conversationConfigSchema } from './schema';
	import TeamManager from '$lib/components/TeamManager.svelte';
	import { TerminalSquare } from 'lucide-svelte';

	let { data } = $props();
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);

	let conversationForm = superForm(
		{
			title: conversation.title,
			short_description: conversation.short_description,
			description: conversation.description,
			image_url: conversation.image_url,
			is_public: conversation.is_public,
			is_invite_only: conversation.is_invite_only,
			auto_login: workflow.auto_login
		},
		{
			validators: zodClient(conversationConfigSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: updateConversation
		}
	);

	let { form, enhance, validateForm, submitting, tainted } = conversationForm;

	async function updateConversation() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

		try {
			await apiClient.UpdateConversation(result.data, {
				params: { conversation_id: conversation.id }
			});

			await apiClient.UpdateWorkflow(
				{ auto_login: result.data.auto_login },
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			);

			invalidateAll();
			notifications.send({ message: 'Updated conversation', priority: 'INFO' });
		} catch (e) {
			notifications.send({ message: 'Failed to save changes', priority: 'ERROR' });
		}
	}
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><TerminalSquare /> Configure</h1>
<p class="mb-10">Use this space to set up the project and manage the team supporting it</p>

<form method="POST" onsubmit={updateConversation} class="flex flex-col gap-4" use:enhance>
	<Form.Field form={conversationForm} name="title">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-row justify-between border-t-1 py-5">
					<Form.Label class="w-60 font-bold">Title</Form.Label>
					<div class="grow flex-col gap-2">
						<Input class="max-w-5xl" {...props} bind:value={$form.title} />
						<Form.FieldErrors />
					</div>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Field form={conversationForm} name="short_description">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-row justify-between border-t-1 py-5">
					<Form.Label class="w-60 font-bold">Short Description</Form.Label>
					<div class="grow flex-col gap-2">
						<Textarea class="max-w-3xl bg-white " {...props} bind:value={$form.short_description} />
						<Form.FieldErrors />
					</div>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Field form={conversationForm} name="description">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex w-full flex-row justify-between border-t-1 py-5">
					<Form.Label class="w-60 font-bold">Description</Form.Label>
					<div class="grow flex-col gap-2">
						<Textarea
							class=" w-full min-w-2xl bg-white "
							{...props}
							bind:value={$form.description}
						/>
						<Form.FieldErrors />
					</div>
				</div>
			{/snippet}
		</Form.Control>
	</Form.Field>

	<div class="flex flex-row gap-4">
		<div class="grow">
			<Form.Field
				class="flex w-full flex-row justify-between border-t-1 py-5"
				form={conversationForm}
				name="image_url"
			>
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex w-full flex-row justify-between border-t-1 py-5">
							<div class="flex w-60 flex-col gap-2">
								<Form.Label class="font-bold">Banner Image URL</Form.Label>
								{#if $form.image_url}
									<img width="200px" alt="Conversation Banner" src={$form.image_url} />
								{/if}
							</div>
							<div class="grow flex-col gap-2">
								<Input {...props} bind:value={$form.image_url} />

								<Form.FieldErrors />
							</div>
						</div>
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>
	</div>

	<div class="flex w-full flex-row justify-between border-t-1 py-5">
		<p class="font-bold">Access</p>
		<div class="flex flex-col gap-5">
			<Form.Field form={conversationForm} name="is_public">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.is_public} />
							<Form.Label>Show conversation publicly</Form.Label>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="is_invite_only">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.is_invite_only} />
							<Form.Label>Only allow participation by invite</Form.Label>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field form={conversationForm} name="auto_login">
				<Form.Control>
					{#snippet children({ props })}
						<div class="flex items-center space-x-2">
							<Switch {...props} bind:checked={$form.auto_login} />
							<Form.Label
								>Automatically log in a user with an annon account if not logged in</Form.Label
							>
						</div>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
		</div>
	</div>
	<TeamManager />

	<Form.Button variant="secondary" class="my-5" disabled={$submitting || !$tainted}
		>Save Changes</Form.Button
	>
</form>
