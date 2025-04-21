<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '$lib/api/client';
	import TextArea from '$lib/components/ui/textarea/textarea.svelte';
	import { notifications } from '$lib/notifications.svelte.js';
	import { invalidateAll } from '$app/navigation';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflow_steps = $derived(data.workflow_steps);
	let step = $derived(workflow_steps.find((s) => s.id === step_id));
	let localStep = $state(step);

	// Keep in sync with navigation changes
	$effect(() => {
		localStep = step;
	});

	let dirty = $state(false);

	// Check to see if our local copy has changed
	$effect(() => {
		dirty = JSON.stringify(localStep) !== JSON.stringify(step);
	});

	async function updateStep() {
		dirty = true;
		try {
			let updatedStep = await apiClient.UpdateWorkflowStep(localStep, {
				params: {
					conversation_id: conversation.id,
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

<div>
	<Label for="name">Step Name</Label>
	<Input name="name" bind:value={localStep.name} />

	<Label for="description">Step Description</Label>
	<TextArea name="description" bind:value={localStep.description} />
	<Button onclick={updateStep} disabled={!dirty}>Save</Button>
</div>

{#if step.tool_config.type === 'learn'}
	<LearnManage
		conversation_id={conversation.id}
		pages={step.tool_config.pages}
		workflow_step={step}
	/>
{/if}

{#if step.tool_config.type === 'polis'}
	<PolisManage
		polis_id={step.tool_config.poll_id}
		polis_url={step.tool_config.server_url}
		admin_user={step.tool_config.admin_user}
		admin_password={step.tool_config.admin_password}
		workflow_step_id={step.id}
	/>
{/if}

{#if step.tool_config.type === 'heyform'}
	<HeyFormManage survey_url={step.tool_config.survey_url} survey_id={step.tool_config.survey_id} />
{/if}
