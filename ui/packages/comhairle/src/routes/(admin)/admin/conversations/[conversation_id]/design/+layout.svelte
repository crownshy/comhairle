<script lang="ts">
	import { page } from '$app/state';
	import { goto, invalidate } from '$app/navigation';
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

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived(data.workflowSteps ?? []);

	let addStepModalOpen = $state(false);

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

<!-- Strip breaks out of the parent layout's padded content wrapper so it sits flush
     below the section tab bar, matching the chrome treatment above it. -->
<div class="-mx-4 -mt-8 mb-8 sm:-mx-8 sm:-mt-10 lg:-mx-16">
	<WorkflowStepStrip
		conversationId={conversation.id}
		steps={workflowSteps}
		onAddStep={() => (addStepModalOpen = true)}
	/>
</div>

<ToolSelectionModal
	prompt="Select a step to add"
	onSelection={addStep}
	bind:open={addStepModalOpen}
/>

{@render children()}
