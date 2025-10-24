<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import TextArea from '$lib/components/ui/textarea/textarea.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Form from '../ui/form/';
	import * as Dialog from '$lib/components/ui/dialog';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import type { WorkflowStep } from '$lib/api/api';
	import { apiClient } from '$lib/api/client';
	import { superForm } from 'sveltekit-superforms';
	import { commonStepSchema } from './schema';
	import { Switch } from '../ui/switch';

	type Props = {
		conversation_id: string;
		step: WorkflowStep;
	};

	let open = $state(false);

	let { step, conversation_id }: Props = $props();

	let commonStepForm = superForm(
		{ name: step.name, description: step.description, required: step.required },
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
			open = false;
		} catch (e) {
			notifications.send({ message: 'Failed to update workflow step', priority: 'ERROR' });
		}
	}
</script>

<div class="mb-10 flex flex-row items-start justify-between">
	<div class="flex flex-col gap-2">
		<div class="flex flex-row items-end gap-2">
			<h2 class="text-2xl">{step.name}</h2>
			{#if step.required}
				<p class="text-red-900">(Required)</p>
			{:else}
				<p class="text-green-900">(Skippable)</p>
			{/if}
		</div>
		<p>{step.description}</p>
	</div>
	<Dialog.Root bind:open>
		<Dialog.Trigger><Button variant="secondary">Edit Metadata</Button></Dialog.Trigger>

		<Dialog.Content>
			<Dialog.Header>
				<Dialog.Title>Edit the metadata?</Dialog.Title>
				<Dialog.Description>
					This is the name and description that will be shown to participants when they get to this
					step
				</Dialog.Description>
			</Dialog.Header>
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
				<Form.Field form={commonStepForm} name="required">
					<Form.Control>
						{#snippet children({ props })}
							<Switch {...props} bind:checked={$form.required} />
							<Form.Label>Required</Form.Label>
							<Form.Description>Are users allowed to skip this step?</Form.Description>
						{/snippet}
					</Form.Control>
					<Form.Description class="text-black"
						>A description of this step that will inform users of it's intent.</Form.Description
					>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Button variant="default" class="my-5" disabled={$submitting}>Submit</Form.Button>
			</form>
		</Dialog.Content>
	</Dialog.Root>
</div>
