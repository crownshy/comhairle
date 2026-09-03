<script lang="ts">
	import { page } from '$app/state';
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

	let { data, children } = $props();
	let { step_id } = page.params;

	let conversationId = $derived(data.conversation.id);
	let steps = $derived(data.workflowSteps);

	let basePath = $derived(`/admin/conversations/${conversationId}/design`);
	let orderedSteps = $derived(steps ? steps.toSorted((a, b) => a.stepOrder - b.stepOrder) : []);
	let loading = $derived(steps === undefined);
	let manageActive = $derived(page.url.pathname === basePath);

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived(data.workflowSteps ?? []);

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
	<li>
		<a
			href={resolve('/(admin)/admin/conversations/[conversation_id]/design', {
				conversation_id: conversation.id
			})}
			class="text-foreground inline-flex h-9 items-center gap-1.5 px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
			class:text-primary={manageActive}
			class:opacity-70={!manageActive}
			class:hover:opacity-100={!manageActive}
			aria-current={manageActive ? 'page' : undefined}
		>
			<Settings2 class="size-4" />
			Design
		</a>
	</li>
	{#if loading}
		{#each [1, 2, 3] as i (i)}
			<li class="px-3.5 py-1.5">
				<Skeleton class="h-5 w-24" />
			</li>
		{/each}
	{:else}
		{#each orderedSteps as step (step.id)}
			{@const active = step.id === step_id}
			<li>
				<a
					href={resolve(
						'/(admin)/admin/conversations/[conversation_id]/design/step/[step_id]',
						{ conversation_id: conversation.id, step_id: step.id }
					)}
					title={step.name || 'Unnamed step'}
					class="text-foreground inline-flex h-9 max-w-55 items-center px-3.5 text-sm font-medium transition-opacity"
					class:text-primary={active}
					class:opacity-70={!active}
					class:hover:opacity-100={!active}
					aria-current={active ? 'page' : undefined}
				>
					<span class="truncate">{step.name || 'Unnamed step'}</span>
				</a>
			</li>
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
