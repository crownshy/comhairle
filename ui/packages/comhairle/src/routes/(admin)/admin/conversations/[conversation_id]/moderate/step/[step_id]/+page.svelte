<script lang="ts">
	import PolisModerate from '$lib/tools/polis/PolisModerate.svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflowSteps = $derived(data.workflowSteps);
	let step = $derived(workflowSteps.find((s) => s.id === step_id));

	let toolConfig = $derived(
		step ? (conversation.isLive ? step.toolConfig : step.previewToolConfig) : null
	);
</script>

{#if toolConfig.type === 'polis'}
	<PolisModerate
		polis_id={toolConfig.poll_id}
		polis_url={toolConfig.server_url}
		workflow_step_id={step.id}
		admin_user={toolConfig.admin_user}
		admin_password={toolConfig.admin_password}
	/>
{:else}
	<h1>No moderation available for this step</h1>
{/if}
