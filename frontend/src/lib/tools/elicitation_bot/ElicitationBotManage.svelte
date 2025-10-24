<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import TextArea from '$lib/components/ui/textarea/textarea.svelte';
	import { Button } from '$lib/components/ui/button';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { z } from 'zod';
	import { notifications } from '$lib/notifications.svelte';

	const elicitationBotSchema = z.object({
		objective: z.string().min(10, 'Please provide at least 10 characters for your objective'),
		vital_to_collect: z.string().min(10, 'Please provide at least 10 characters for what is vital to collect'),
		other_context: z.string().min(10, 'Please provide at least 10 characters for other important context')
	});

	const form = superForm(
		{
			objective: '',
			vital_to_collect: '',
			other_context: ''
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
		e.preventDefault();
		const result = await validateForm();

		if (result.valid) {
			try {
				// TODO: Implement API call to save elicitation bot configuration
				console.log('Elicitation bot configuration:', result.data);
				notifications.send({ message: 'Elicitation bot configuration saved successfully', priority: 'INFO' });
			} catch (e) {
				console.warn(e);
				notifications.send({ message: 'Failed to save elicitation bot configuration', priority: 'ERROR' });
			}
		}
	}
</script>

<div class="space-y-6">
	<h1 class="text-2xl font-bold">Elicitation Bot Configuration</h1>
	<p class="text-muted-foreground">
		Configure your elicitation bot by providing the key information needed for effective data collection.
	</p>

	{#if $errMessage}
		<p class="text-destructive text-sm">{$errMessage}</p>
	{/if}

	<form onsubmit={handleSubmit} class="space-y-6" method="POST" use:enhance>
		<Form.Field {form} name="objective">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>What is your objective?</Form.Label>
					<TextArea
						{...props}
						bind:value={$formData.objective}
						class="min-h-[100px]"
						placeholder="Describe the main goal or purpose of your elicitation process..."
					/>
				{/snippet}
			</Form.Control>
			<Form.Description>
				Clearly outline what you hope to achieve through this elicitation process.
			</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="vital_to_collect">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>What is vital to collect?</Form.Label>
					<TextArea
						{...props}
						bind:value={$formData.vital_to_collect}
						class="min-h-[100px]"
						placeholder="Specify the critical information, data points, or insights that must be gathered..."
					/>
				{/snippet}
			</Form.Control>
			<Form.Description>
				Identify the essential information that must be collected to meet your objectives.
			</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="other_context">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>What other context is important?</Form.Label>
					<TextArea
						{...props}
						bind:value={$formData.other_context}
						class="min-h-[100px]"
						placeholder="Provide any additional context, constraints, or considerations that should guide the elicitation process..."
					/>
				{/snippet}
			</Form.Control>
			<Form.Description>
				Share any additional context, background information, or special considerations.
			</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Button type="submit" disabled={$submitting} class="w-full">
			{$submitting ? 'Saving...' : 'Save Configuration'}
		</Button>
	</form>
</div>
