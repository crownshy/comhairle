<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import ThinkingSpaceManage from '$lib/tools/thinking_space/ThinkingSpaceManage.svelte';
	import ElicitationBotManage from '$lib/tools/elicitation_bot/ElicitationBotManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import * as Prioritization from '$lib/tools/prioritization';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);
	let workflow = $derived(data.workflows[0]);
	let toolConfig = $derived(data.toolConfig);
</script>

{#if step && toolConfig?.type === 'learn'}
	<LearnManage
		conversationId={conversation.id}
		{conversation}
		isLive={conversation.isLive}
		workflowStep={step}
	/>
{/if}

{#if step && toolConfig?.type === 'polis'}
	<PolisManage
		{toolConfig}
		conversationId={conversation.id}
		workflowId={workflow.id}
		workflowStepId={step.id}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && toolConfig?.type === 'heyform'}
	<HeyFormManage
		conversation_id={conversation.id}
		workflow_id={step.workflowId}
		workflow_step_id={step.id}
		survey_url={toolConfig.server_url}
		survey_id={toolConfig.survey_id}
		admin_user={toolConfig.admin_user}
		admin_password={toolConfig.admin_password}
		workspace_id={toolConfig.workspace_id}
		project_id={toolConfig.project_id}
	/>
{/if}

{#if toolConfig?.type === 'stories'}
	<LivedExperienceManage />
{/if}

{#if step && toolConfig?.type === 'thinkingspace'}
	<ThinkingSpaceManage
		{conversation}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && toolConfig?.type === 'elicitationbot'}
	<ElicitationBotManage
		conversationId={conversation.id}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && toolConfig?.type === Prioritization.TOOL_NAME}
	{#key step.id}
		<Prioritization.ManageUI {conversation} workflowId={step.workflowId} workflowStep={step} />
	{/key}
{/if}
