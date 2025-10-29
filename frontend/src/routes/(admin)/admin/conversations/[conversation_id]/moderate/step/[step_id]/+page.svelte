<script lang="ts">
	import PolisModerate from '$lib/tools/polis/PolisModerate.svelte';

	import type { WorkflowStep } from '$lib/api/api.js';
	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflow_steps = $derived(data.workflow_steps);
	let step = $derived(workflow_steps.find((s: WorkflowStep) => s.id === step_id));
	console.log('Step ', step);
</script>

{#if step.tool_config.type === 'polis'}
	<PolisModerate
		polis_id={step.tool_config.poll_id}
		polis_url={step.tool_config.server_url}
		workflow_step_id={step.id}
		admin_user={step.tool_config.admin_user}
		admin_password={step.tool_config.admin_password}
	/>
{:else}
	<h1>No moderation available for this step</h1>
{/if}
