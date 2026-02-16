<script lang="ts">
	import PolisModerate from '$lib/tools/polis/PolisModerate.svelte';

	import type { WorkflowStepWithTranslations, ConversationWithTranslations } from '$lib/api/api.js';
	let { data } = $props();

	let conversation = $derived(data.conversation as ConversationWithTranslations);
	let step_id = $derived(data.step_id);
	let workflow_steps = $derived((data.workflow_steps ?? []) as WorkflowStepWithTranslations[]);
	let step = $derived(workflow_steps.find((s) => s.id === step_id));

	let toolConfig = $derived(step ? (conversation.isLive ? step.toolConfig : step.previewToolConfig) : null);
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
