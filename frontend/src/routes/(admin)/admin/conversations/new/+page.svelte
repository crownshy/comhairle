<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';
	import NewConversationSchema from './NewConversationSchema';
	import { notifications } from '$lib/notifications.svelte';
	import { goto } from '$app/navigation';
	import { manage_conversation_url } from '$lib/urls';
	import { apiClient } from '$lib/api/client';
	import PlaceholderConvo from '$lib/assets/placeholder_convo.png';
	import { invalidateAll } from '$app/navigation';
	import { workflow_templates } from '$lib/workflow_templates';

	import * as Card from '$lib/components/ui/card';
	import { page } from '$app/stores';
	let url = $page.url;

	let { data } = $props();

	const form = superForm(data.form, {
		SPA: true,
		validators: zodClient(NewConversationSchema),
		resetForm: false
	});

	const { form: formData, enhance, message: errMessage, validateForm } = form;

	let submitting = $state(false);
	let selectedWorkflowTemplate = $state('empty');

	async function handleSubmit(e: Event) {
		e.preventDefault();
		submitting = true;
		const result = await validateForm();

		if (result.valid) {
			try {
				let fullConversation = {
					...result.data,
					description:
						'This should be a longer description about the conversation. It should introduce people to what is being discussed and outline-solid the actions that might be taken as a result of the conversation',
					tags: [],
					image_url: PlaceholderConvo,
					is_public: false,
					is_invite_only: false
				};

				let conversation = await apiClient.CreateConversation(fullConversation);

				let workflow = await apiClient.CreateWorkflow(
					{
						name: 'Default Workflow',
						description: 'The default workflow',
						is_active: true,
						is_public: true,
						auto_login: false
					},
					{ params: { conversation_id: conversation.id } }
				);

				//@ts-ignore
				let template = workflow_templates[selectedWorkflowTemplate];
				for (let step of template) {
					await apiClient.CreateWorkflowStep(step, {
						params: {
							conversation_id: conversation.id,
							workflow_id: workflow.id
						}
					});
				}

				notifications.addFlash({ message: 'Conversastion Created' });
				await invalidateAll();

				goto(manage_conversation_url(conversation.id));
			} catch (e) {
				console.warn(e);
				notifications.send({ message: 'Something went wrong creating the conversation' });
			}
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4" method="POST" use:enhance>
	<h2 class="text-xl font-bold">Create a new conversation</h2>

	{#if $errMessage}
		<p class="text-destructive text-sm">{$errMessage}</p>
	{/if}

	<Form.Field {form} name="title">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.enter_a_title_for_the_conversation()}</Form.Label>
				<Input {...props} bind:value={$formData.title} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="short_description">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.short_description()}</Form.Label>
				<Input {...props} bind:value={$formData.short_description} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<h2 class="text-xl font-bold">Select a workflow template</h2>
	<p>
		Select a workflow template from the options bellow. You will have the opertunity to customise
		the workflow in the next step
	</p>
	<div class="m:grid-cols-3 grid h-auto w-full grid-cols-2 gap-4">
		<Card.Root
			class={selectedWorkflowTemplate === 'empty'
				? 'cursor-pointer bg-green-100'
				: 'cursor-pointer'}
			onclick={() => (selectedWorkflowTemplate = 'empty')}
		>
			<Card.Header class="grow">
				<Card.Title>Empty Workflow</Card.Title>
			</Card.Header>
			<Card.Content class="line-clamp4">
				An empty workflow for you to configure as you will
			</Card.Content>
		</Card.Root>

		<Card.Root
			class={selectedWorkflowTemplate === 'learn_polis'
				? 'cursor-pointer bg-green-100'
				: 'cursor-pointer'}
			onclick={() => (selectedWorkflowTemplate = 'learn_polis')}
		>
			<Card.Header class="grow">
				<Card.Title>Learn step + Polis</Card.Title>
			</Card.Header>
			<Card.Content class="line-clamp4">
				For when you want to introduce a topic and then map out the landscape of opinions on it
			</Card.Content>
		</Card.Root>

		<Card.Root
			class={selectedWorkflowTemplate === 'learn_survey'
				? 'cursor-pointer bg-green-100'
				: 'cursor-pointer'}
			onclick={() => (selectedWorkflowTemplate = 'learn_survey')}
		>
			<Card.Header class="grow">
				<Card.Title>Learn step + survey</Card.Title>
			</Card.Header>
			<Card.Content class="line-clamp4">
				A more traditional workflow where we want to ask a introduce participants to a specific
				subject and then conduct a survey
			</Card.Content>
		</Card.Root>

		<Card.Root
			class={selectedWorkflowTemplate === 'learn_survey_polis'
				? 'cursor-pointer bg-green-100'
				: 'cursor-pointer'}
			onclick={() => (selectedWorkflowTemplate = 'learn_survey_polis')}
		>
			<Card.Header class="grow">
				<Card.Title>Learn step + survey + polis</Card.Title>
			</Card.Header>
			<Card.Content class="line-clamp4">
				A full end to end workflow where we do a little teaching a survey and then a opinion mapping
				step
			</Card.Content>
		</Card.Root>
	</div>

	<Form.Button disabled={submitting} fullWidth variant="default">Create</Form.Button>
</form>
