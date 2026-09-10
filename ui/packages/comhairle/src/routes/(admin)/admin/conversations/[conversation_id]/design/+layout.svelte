<script lang="ts">
	import { resolve } from '$app/paths';
	import { notifications } from '$lib/notifications.svelte.js';
	import AddStepDialog from './AddStepDialog.svelte';
	import { Plus, Settings2 } from 'lucide-svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import { setAddStepDialogContext } from './context';

	const { data, children, params } = $props();
	const { conversation_id } = $derived(params);

	let addStepDialogOpen = $state<boolean>(false);
	setAddStepDialogContext({ open: () => void (addStepDialogOpen = true) });
</script>

<TabStripShell ariaLabel="Workflow steps">
	{@const slug = 'design'}
	<TabStripItem
		href={resolve(`/(admin)/admin/conversations/[conversation_id]/${slug}`, {
			conversation_id
		})}
		isActive={(pathname) => pathname.endsWith(slug)}
	>
		<Settings2 class="mr-1 size-4" />
		Design
	</TabStripItem>
	{#await data.streamedWorkflows}
		{#each [1, 2, 3] as i (i)}
			<li class="px-3.5 py-1.5">
				<Skeleton class="h-5 w-24" />
			</li>
		{/each}
	{:then workflows}
		{#if workflows.err !== null}
			{console.error(workflows.err)}
			{notifications.addFlash({
				message: 'Could not load workflows, please try again',
				priority: 'ERROR'
			})}
		{:else}
			{#each workflows.ok.current.steps as step (step.id)}
				<TabStripItem
					href={resolve(
						'/(admin)/admin/conversations/[conversation_id]/design/step/[step_id]',
						{ conversation_id, step_id: step.id }
					)}
					isActive={(pathname) => pathname.includes(step.id)}
				>
					<span class="truncate" title={step.name}>{step.name || 'Unnamed step'}</span>
				</TabStripItem>
			{/each}
			<li>
				<button
					type="button"
					onclick={() => void (addStepDialogOpen = true)}
					class="text-foreground/40 hover:text-foreground inline-flex h-9 items-center gap-1 px-3.5 text-sm font-medium whitespace-nowrap"
				>
					<Plus class="size-4" />
					Add step
				</button>
			</li>
		{/if}
	{/await}
</TabStripShell>

{#await data.streamedWorkflows then workflows}
	{#if workflows.ok !== null}
		<AddStepDialog
			workflowId={workflows.ok.current.workflow.id}
			conversation={data.conversation}
			bind:open={addStepDialogOpen}
		/>
	{/if}
{/await}

{@render children()}
