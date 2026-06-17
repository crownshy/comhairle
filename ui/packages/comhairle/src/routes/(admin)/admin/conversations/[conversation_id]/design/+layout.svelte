<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidate } from '$app/navigation';
	import { getContext } from 'svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte.js';
	import {
		basic_learn_config,
		basic_polis_config,
		basic_survey_config,
		basic_lived_experience_config,
		basic_elicitation_bot_config,
		basic_thinking_space_config,
		basic_prioritization_config,
		defaultStepCreationParams
	} from '$lib/workflow_templates.js';
	import WorkflowStepStrip from '$lib/components/WorkflowStepStrip.svelte';
	import ToolSelectionModal from '$lib/components/ToolSelectionModal.svelte';
	import {
		CONVERSATION_TAB_EXTRAS_CTX,
		type ConversationTabExtras
	} from '$lib/conversationTabExtras';

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived(data.workflowSteps ?? []);

	let addStepModalOpen = $state(false);

	const tabExtras = getContext<ConversationTabExtras>(CONVERSATION_TAB_EXTRAS_CTX);

	$effect(() => {
		if (!tabExtras) return;
		tabExtras.primary = workflowStripSnippet;
		return () => {
			tabExtras.primary = null;
		};
	});

	$effect(() => {
		if (page.url.searchParams.get('addStep') === 'true') {
			addStepModalOpen = true;
			goto(page.url.pathname, { replaceState: true });
		}
	});

	async function addStep(step: string) {
		const tool_setup = {
			Polis: basic_polis_config,
			Learn: basic_learn_config,
			Survey: basic_survey_config,
			'Lived Experience': basic_lived_experience_config,
			'Elicitation Bot': basic_elicitation_bot_config(conversation),
			'Thinking Space': basic_thinking_space_config(),
			Prioritization: basic_prioritization_config
		}[step];

		const new_step_order =
			workflowSteps.length > 0 ? Math.max(...workflowSteps.map((ws) => ws.stepOrder)) + 1 : 1;

		try {
			await apiClient.CreateConversationWorkflowStep(
				{
					name: defaultStepCreationParams[step]?.name ?? `New ${step} Step`,
					description:
						defaultStepCreationParams[step]?.description ?? `A new ${step} Step`,
					is_offline: false,
					activation_rule: 'manual',
					step_order: new_step_order,
					tool_setup,
					required: true
				},
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			);
			await invalidate('conversation:workflow');
			notifications.send({ priority: 'INFO', message: 'Step Added' });
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to create step' });
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

<ToolSelectionModal
	prompt="Select a step to add"
	onSelection={addStep}
	bind:open={addStepModalOpen}
/>

{@render children()}
