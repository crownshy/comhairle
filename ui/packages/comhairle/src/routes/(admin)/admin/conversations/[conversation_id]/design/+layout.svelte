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

	let { data, children } = $props();
	let { conversation, workflowSteps = [] } = $derived(data);

	let workflow = $derived(data.workflows[0]);

	let orderedSteps = $derived(workflowSteps.toSorted((a, b) => a.stepOrder - b.stepOrder));
	let loading = $derived(workflowSteps === undefined);

	let adding = $state(false);

	async function addStep(creationKey: CreationKey) {
		if (adding) return;
		adding = true;
		try {
			const created = await createWorkflowStep({
				conversation,
				workflowId: workflow.id,
				creationKey,
				existingSteps: workflowSteps
			});
			if (!created) return;
			await invalidate('conversation:workflow');
			notifications.send({ priority: 'INFO', message: 'Step added' });
			newStepHighlight.flag(created.id);
			addStepDialog.open = false;

			// Return to the design board (works from either entry point: the board's own Add step
			// button or the workflow strip's while inside a step editor). The design page reacts to
			// `newStepHighlight` by scrolling the new card into view and briefly highlighting it, so
			// the operator sees exactly which step was just created instead of landing in its editor.
			await goto(
				resolve('/(admin)/admin/conversations/[conversation_id]/design', {
					conversation_id: conversation.id
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
				conversation_id: conversation.id
			})
		);
	}
</script>

<AddStepDialog bind:open={addStepDialog.open} {adding} onAdd={addStep} onAddEvent={addEvent} />

<TabStripShell ariaLabel="Workflow steps">
	{@const slug = 'design'}
	<TabStripItem
		href={resolve(`/(admin)/admin/conversations/[conversation_id]/${slug}`, {
			conversation_id: conversation.id
		})}
		isActive={(pathname) => pathname.endsWith(slug)}
	>
		<Settings2 class="mr-1 size-4" />
		Design
	</TabStripItem>
	{#if loading}
		{#each [1, 2, 3] as i (i)}
			<li class="px-3.5 py-1.5">
				<Skeleton class="h-5 w-24" />
			</li>
		{/each}
	{:else}
		{#each orderedSteps as step (step.id)}
			<TabStripItem
				href={resolve(
					'/(admin)/admin/conversations/[conversation_id]/design/step/[step_id]',
					{ conversation_id: conversation.id, step_id: step.id }
				)}
				isActive={(pathname) => pathname.includes(step.id)}
			>
				<span class="truncate">{step.name || 'Unnamed step'}</span>
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
</TabStripShell>

{@render children()}
