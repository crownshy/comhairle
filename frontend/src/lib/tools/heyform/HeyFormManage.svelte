<script lang="ts">
	import Label from '$lib/components/ui/label/label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';
	let {
		survey_id,
		survey_url,
		conversation_id,
		workflow_id,
		workflow_step_id
	}: {
		survey_id: string;
		survey_url: string;
		conversation_id: string;
		workflow_id: string;
		workflow_step_id: string;
	} = $props();
	$effect(() => {
		apiClient
			.UpdateWorkflowStep(
				{ tool_config: { survey_id, survey_url, type: 'heyform' } },
				{ params: { conversation_id, workflow_id, workflow_step_id } }
			)
			.then(() => {
				notifications.send({ message: 'Updated HeyForm ID', priority: 'INFO' });
			})
			.catch((e) => {
				console.warn(e);
				notifications.send({ message: 'Failed to update HeyForm ID', priority: 'ERROR' });
			});
	});
</script>

<Label for="heyform_id">HeyForm ID</Label>
<Input name="heyform_id" bind:value={survey_id} />
