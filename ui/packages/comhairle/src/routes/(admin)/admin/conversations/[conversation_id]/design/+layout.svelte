<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import AddStepDialog from './AddStepDialog.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { Plus, Settings2 } from '@lucide/svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { sectionTabClass } from '$lib/components/sectionTabClass';
	import { setAddStepDialogContext } from './context';
	import { max } from '$lib/utils/maths';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';

	const { data, children, params } = $props();
	const { conversation_id } = $derived(params);
	const isStepPage = $derived(page.params.step_id !== undefined);

	const hasPermission = $derived(
		permissions.can('conversation', 'conversation_update', conversation_id)
	);

	function open() {
		if (!hasPermission) return;
		addStepDialogOpen = true;
	}

	let addStepDialogOpen = $state<boolean>(false);
	setAddStepDialogContext({ open });
</script>

<!-- A step page has its own header and tabs, so the board's step strip only shows on the board. -->
{#if !isStepPage}
	<TabStripShell ariaLabel="Workflow steps">
		{@const slug = 'design'}
		<TabStripItem
			href={resolve(`/(admin)/admin/conversations/[conversation_id]/${slug}`, {
				conversation_id
			})}
			isActive={(pathname) => pathname.endsWith(slug)}
		>
			<Settings2 class="size-4" />
			Design
		</TabStripItem>
		{#await data.streamedWorkflows}
			{#each { length: 3 }}
				<li class="flex h-11 items-center">
					<Skeleton class="h-5 w-24" />
				</li>
			{/each}
		{:then workflows}
			{#if workflows.err !== null}
				{console.error(workflows.err)}
				<ErrorMessage>Could not load workflows, please try again</ErrorMessage>
			{:else}
				{#each workflows.ok.current.steps as step (step.id)}
					<TabStripItem
						href={resolve(
							'/(admin)/admin/conversations/[conversation_id]/design/step/[step_id]',
							{ conversation_id, step_id: step.id }
						)}
						isActive={(pathname) => pathname.includes(step.id)}
					>
						<span class="truncate" title={step.name}>{step.name || 'Unnamed step'}</span
						>
					</TabStripItem>
				{/each}
				<li>
					<button
						type="button"
						disabled={!hasPermission}
						title={hasPermission ? undefined : 'Edit permission required'}
						onclick={open}
						class={sectionTabClass(
							false,
							'disabled:cursor-not-allowed! disabled:opacity-50'
						)}
					>
						<Plus class="size-4" />
						Add step
					</button>
				</li>
			{/if}
		{/await}
	</TabStripShell>
{/if}

{#await data.streamedWorkflows then workflows}
	{#if workflows.ok !== null && hasPermission}
		<AddStepDialog
			workflowId={workflows.ok.current.workflow.id}
			conversationId={conversation_id}
			organizationId={data.conversation.organizationId ?? null}
			highestStepOrder={max(workflows.ok.current.steps, (s) => s.stepOrder)}
			bind:open={addStepDialogOpen}
		/>
	{/if}
{/await}

{@render children()}
