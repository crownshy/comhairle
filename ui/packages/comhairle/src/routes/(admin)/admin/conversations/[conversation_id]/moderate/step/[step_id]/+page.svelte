<script lang="ts">
	import { getToolConfig } from '$lib/tool_meta';
	import PolisModerate from '$lib/tools/polis/PolisModerate.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { TabContent } from './external';

	let { data, params } = $props();
</script>

<svelte:head>
	<title>Moderate Step - Comhairle Admin</title>
</svelte:head>

<TabContent>
	{#await data.streamedWorkflowSteps}
		<Skeleton />
	{:then workflowSteps}
		{#if workflowSteps.err !== null}
			<span class="text-destructive">Could not load workflow steps, please try again</span>
		{:else}
			{@const step = workflowSteps.ok.find((s) => s.id === params.step_id)}
			{#if step === undefined}
				<span class="text-destructive">Could not find step, please try again</span>
			{:else}
				{@const toolConfig = getToolConfig(data.conversation.isLive, step)}
				{#if toolConfig.type === 'polis'}
					<PolisModerate
						polis_id={toolConfig.poll_id}
						polis_url={toolConfig.server_url}
						workflow_step_id={params.step_id}
						admin_user={toolConfig.admin_user}
						admin_password={toolConfig.admin_password}
					/>
				{:else}
					<h1>No moderation available for this step</h1>
				{/if}
			{/if}
		{/if}
	{/await}
</TabContent>
