<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidate } from '$app/navigation';
	import { getContext } from 'svelte';
	import { notifications } from '$lib/notifications.svelte.js';
	import { createWorkflowStep } from '$lib/createWorkflowStep';
	import type { CreationKey } from '$lib/tool_meta';
	import { addStepDialog } from '$lib/stores/addStepDialog.svelte';
	import { newStepHighlight } from '$lib/stores/newStepHighlight.svelte';
	import WorkflowStepStrip from '$lib/components/WorkflowStepStrip.svelte';
	import AddStepDialog from '$lib/components/AddStepDialog.svelte';
	import {
		CONVERSATION_TAB_EXTRAS_CTX,
		type ConversationTabExtras
	} from '$lib/conversationTabExtras';

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived(data.workflowSteps ?? []);

	let addStepModalOpen = $state(false);
	let adding = $state(false);

	const tabExtras = getContext<ConversationTabExtras>(CONVERSATION_TAB_EXTRAS_CTX);

	$effect(() => {
		if (!tabExtras) return;
		tabExtras.primary = workflowStripSnippet;
		return () => {
			tabExtras.primary = null;
		};
	});

	// Deep link: /design?addStep=true opens the dialog, then drops the query param.
	$effect(() => {
		if (page.url.searchParams.get('addStep') === 'true') {
			addStepModalOpen = true;
			goto(page.url.pathname, { replaceState: true });
		}
	});

	// Cross-component opener: the board's empty-state / footer button asks the layout
	// to open the dialog by bumping the shared request counter.
	let seenRequestCount = addStepDialog.requestCount;
	$effect(() => {
		if (addStepDialog.requestCount !== seenRequestCount) {
			seenRequestCount = addStepDialog.requestCount;
			addStepModalOpen = true;
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
			addStepModalOpen = false;
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to create step' });
		} finally {
			adding = false;
		}
	}
</script>

{#snippet workflowStripSnippet()}
	<WorkflowStepStrip
		conversationId={conversation.id}
		steps={workflowSteps}
		onAddStep={() => (addStepModalOpen = true)}
	/>
{/snippet}

<AddStepDialog bind:open={addStepModalOpen} {adding} onAdd={addStep} />

{@render children()}
