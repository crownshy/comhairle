<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { notifications } from '$lib/notifications.svelte.js';
	import { createWorkflowStep } from '$lib/createWorkflowStep';
	import type { CreationKey } from '$lib/tool_meta';
	import { addStepDialog } from '$lib/stores/addStepDialog.svelte';
	import { newStepHighlight } from '$lib/stores/newStepHighlight.svelte';
	import AddStepDialog from './AddStepDialog.svelte';
	import { Plus, Settings2 } from 'lucide-svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import type { WorkflowDto } from '@crownshy/api-client/api';
	import { onMount } from 'svelte';

	const { data, children, params } = $props();
	const { conversation_id } = $derived(params);

	let adding = $state(false);

	async function addStep(creationKey: CreationKey, workflowId: WorkflowDto['id']) {
		if (adding) return;
		adding = true;
		try {
			const created = await createWorkflowStep({
				conversation: data.conversation,
				workflowId,
				creationKey,
				existingSteps: workflowSteps
			});
			if (!created) return;
			await invalidate(key('conversation/design/workflow'));
			notifications.send({ priority: 'INFO', message: 'Step added' });
			newStepHighlight.flag(created.id);
			addStepDialog.open = false;

			// Return to the design board (works from either entry point: the board's own Add step
			// button or the workflow strip's while inside a step editor). The design page reacts to
			// `newStepHighlight` by scrolling the new card into view and briefly highlighting it, so
			// the operator sees exactly which step was just created instead of landing in its editor.
			await goto(
				resolve('/(admin)/admin/conversations/[conversation_id]/design', {
					conversation_id
				})
			);
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to create step' });
		} finally {
			adding = false;
		}
	}

	// The "Online video conference" palette entry has no backing workflow tool, so
	// adding it creates a conversation Event instead. Hand off to the create-event
	// flow, where the organiser sets the required date, time, and details.
	function addEvent() {
		addStepDialog.open = false;
		goto(
			resolve('/(admin)/admin/conversations/[conversation_id]/events/new', {
				conversation_id
			})
		);
	}
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
					onclick={() => (addStepDialog.open = true)}
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
			bind:open={addStepDialog.open}
			{adding}
			onAdd={(creationKey) => addStep(creationKey, workflows.ok.current.workflow.id)}
			onAddEvent={addEvent}
		/>
	{/if}
{/await}

{@render children()}
