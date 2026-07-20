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

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived(data.workflowSteps ?? []);

	let adding = $state(false);

	// The workflow step strip (Row 3) is rendered by the shared conversation layout, not
	// injected from here: rendering it there from `data.workflowSteps` puts it on the SSR
	// path instead of a post-hydration `$effect`, so it no longer lags the page content.

	// Deep link: /design?addStep=true opens the dialog, then drops the query param.
	$effect(() => {
		if (page.url.searchParams.get('addStep') === 'true') {
			addStepDialog.open = true;
			goto(page.url.pathname, { replaceState: true });
		}
	});

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

{@render children()}
