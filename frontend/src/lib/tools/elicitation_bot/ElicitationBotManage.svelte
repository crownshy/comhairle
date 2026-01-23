<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Button } from '$lib/components/ui/button';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { z } from 'zod';
	import { notifications } from '$lib/notifications.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { apiClient } from '$lib/api/client';
	import type { WorkflowStep } from '$lib/api/api';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStep: WorkflowStep;
	};
	let { conversationId, workflowId, workflowStep }: Props = $props();

	const elicitationBotSchema = z.object({
		topic: z.string().min(3, 'Please provide at least 3 characters for your topic')
	});

	const form = superForm(
		{
			topic: workflowStep.tool_config.topic
		},
		{
			SPA: true,
			validators: zodClient(elicitationBotSchema),
			resetForm: false,
			onSubmit: handleSubmit
		}
	);

	const { form: formData, enhance, message: errMessage, validateForm, submitting } = form;

	async function handleSubmit(e: Event) {
		// e.preventDefault();
		const result = await validateForm();

		if (result.valid) {
			try {
				await apiClient.UpdateElicitationBotWorkflowStep(
					{ tool_config: { ...workflowStep.tool_config, ...result.data } },
					{
						params: {
							conversation_id: conversationId,
							workflow_id: workflowId,
							workflow_step_id: workflowStep.id
						}
					}
				);
				notifications.send({
					message: 'Elicitation bot configuration saved successfully',
					priority: 'INFO'
				});
			} catch (e) {
				console.error(e);
				notifications.send({
					message: 'Failed to save elicitation bot configuration',
					priority: 'ERROR'
				});
			}
		}
	}
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold">Elicitation Bot Configuration</h1>
	<p class="text-muted-foreground">
		Configure your elicitation bot by providing the key information needed for effective data
		collection.
	</p>

	{#if $errMessage}
		<p class="text-destructive text-sm">{$errMessage}</p>
	{/if}

	<form onsubmit={handleSubmit} class="space-y-6" method="POST" use:enhance>
		<Form.Field {form} name="topic">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						>What topic do you want your elicitation bot help users gather their
						opinions about?</Form.Label
					>
					<Input
						{...props}
						bind:value={$formData.topic}
						placeholder="Enter the main topic of the conversation you hope to gather opinions on..."
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Button type="submit" disabled={$submitting} class="w-full">
			{$submitting ? 'Saving...' : 'Save Configuration'}
		</Button>
	</form>
</div>
