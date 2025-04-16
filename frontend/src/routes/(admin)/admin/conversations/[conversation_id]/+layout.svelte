<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import Button from '$lib/components/ui/button/button.svelte';

	let { children, data }: LayoutProps = $props();
	let conversation = $derived(data.conversation);

	let workflow_steps = $derived(data.workflow_steps);
	let stats = $derived(data.stats);
</script>

<h1 class="text-bold mb-4 text-2xl">{conversation!.title}</h1>

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

<div class="mt-4 flex flex-row gap-4 divide-x-2">
	<nav class="br-gray flex flex-col gap-2 py-4">
		<h3>Landing Page</h3>
		<Button variant="outline" href={`/admin/conversations/${conversation.id}/landing/`}
			>Landing Page</Button
		>

		<h3>Workflow Steps</h3>
		{#each workflow_steps as step}
			<Button variant="outline" href={`/admin/conversations/${conversation.id}/workflow/${step.id}`}
				>{step.name}</Button
			>
		{/each}

		<Button variant="default">+ Add Step</Button>

		<h3>Report</h3>
		<Button variant="outline" href={`/admin/conversations/${conversation.id}/report/`}
			>Report</Button
		>
	</nav>
	<div class="grow p-4">
		{@render children()}
	</div>
</div>
