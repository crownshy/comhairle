<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflow_steps = $derived(data.workflow_steps);
	let step = $derived(workflow_steps.find((s) => s.id === step_id));
</script>

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
