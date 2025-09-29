<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import TextArea from '$lib/components/ui/textarea/textarea.svelte';
	import * as Form from '../ui/form/';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import type { WorkflowStep } from '$lib/api/api';
	import { apiClient } from '$lib/api/client';
	import { superForm } from 'sveltekit-superforms';
	import { commonStepSchema } from './schema';

	type Props = {
		conversation_id: string;
		step: WorkflowStep;
	};

	let { step, conversation_id }: Props = $props();

	let commonStepForm = superForm(
		{ name: step.name, description: step.description },
		{
			validators: zodClient(commonStepSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: updateStep
		}
	);

	let { form, enhance, validateForm, message, submitting } = commonStepForm;

	async function updateStep() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;
		try {
			await apiClient.UpdateWorkflowStep(result.data, {
				params: {
					conversation_id: conversation_id,
					workflow_id: step.workflow_id,
					workflow_step_id: step.id
				}
			});
			invalidateAll();
			notifications.send({ message: 'Updated workflow step', priority: 'INFO' });
		} catch (e) {
			notifications.send({ message: 'Failed to update workflow step', priority: 'ERROR' });
		}
	}
</script>

<form method="POST" onsubmit={updateStep} class="mt-10 flex flex-col gap-y-5" use:enhance>
	<Form.Field form={commonStepForm} name="name">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Name</Form.Label>
				<Input {...props} bind:value={$form.name} />
			{/snippet}
		</Form.Control>
		<Form.Description class="text-black"
			>The name of the step that will be shown to participants.</Form.Description
		>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field form={commonStepForm} name="description">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Description</Form.Label>
				<TextArea class="bg-white" {...props} bind:value={$form.description} />
			{/snippet}
		</Form.Control>
		<Form.Description class="text-black"
			>A description of this step that will inform users of it's intent.</Form.Description
		>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Button variant="secondary" class="my-5" disabled={$submitting}>Submit</Form.Button>
</form>
