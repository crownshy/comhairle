<script lang="ts">
	import type { PageProps } from './$types';
	import * as Card from '$lib/components/ui/card';
	import * as Tabs from '$lib/components/ui/tabs';
	import Button from '$lib/components/ui/button/button.svelte';
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';

	let props: PageProps = $props();
	let conversation = $derived(props.data.conversation);

	let workflows = $derived(props.data.workflows);
	let workflow_steps = $derived(props.data.workflow_steps);
	let stats = $derived(props.data.stats);
</script>

<h1 class="text-2xl">Admin for <span class="text-bold">{conversation!.title}</span></h1>

{#if stats}
	<Card.Root>
		<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
			<Card.Title class="text-sm font-medium">Total Participants</Card.Title>
		</Card.Header>

		<Card.Content>
			<div class="text-2xl font-bold">{stats.total_users}</div>
		</Card.Content>
	</Card.Root>
{/if}

<Tabs.Root value="Landing Page" class="space-y-4">
	<Tabs.List>
		<Tabs.Trigger value="Landing Page">Landing Page</Tabs.Trigger>
		<Tabs.Trigger value="Workflow">Workflow</Tabs.Trigger>
		<Tabs.Trigger value="Report">Report</Tabs.Trigger>
	</Tabs.List>

	<Tabs.Content value="Landing Page">
		<h1>Landing Page</h1>
	</Tabs.Content>

	<Tabs.Content value="Workflow">
		<h1>Workflow</h1>
		<div class="flex flex-col">
			<Tabs.Root value={workflow_steps[0].id} class="space-y-4">
				<Tabs.List>
					{#each workflow_steps as step}
						<Tabs.Trigger value={step.id}>{step.name}</Tabs.Trigger>
					{/each}
				</Tabs.List>
				{#each workflow_steps as step}
					<Tabs.Content value={step.id}>
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
					</Tabs.Content>
				{/each}
			</Tabs.Root>
		</div>
	</Tabs.Content>

	<Tabs.Content value="Report">
		<h1>Report</h1>

		{#if workflows && workflows.length > 0}
			<p>WE have a workflow</p>
		{:else}
			<p>Create a new workflow</p>
		{/if}
		<h2>Workflows</h2>
	</Tabs.Content>
</Tabs.Root>
