<script lang="ts">
	import type { WorkflowStep } from '$lib/api/api.js';
	import { infoURLForTool } from '$lib/utils';
	import {
		basic_learn_config,
		basic_polis_config,
		basic_survey_config,
		basic_lived_experience_config,
		basic_elicitation_bot_config
	} from '$lib/workflow_templates.js';
	import ToolSelectionModal from '$lib/components/ToolSelectionModal.svelte';
	import { Header } from '$lib/components/ui/alert-dialog/index.js';
	let { data } = $props();
	import { draggable, droppable, type DragDropState } from '@thisux/sveltednd';
	import { Plus, Share2, BookOpen, ListChecks, Video, MessagesSquare } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte.js';
	let conversation = $derived(data.conversation);
	let workflow_steps = $derived(data.workflow_steps);
	let workflow = $derived(data.workflows[0]);

	console.log(workflow_steps);

	function handleDrop(state: DragDropState<WorkflowStep>) {
		console.log('drop ', state);
	}

	async function addStep(step: string) {
		console.log('step ', step);
		let tool_setup = {
			Polis: basic_polis_config,
			Learn: basic_learn_config,
			Survey: basic_survey_config,
			'Lived Experience': basic_lived_experience_config,
			'Elicitation Bot': basic_elicitation_bot_config
		}[step];

		let new_step_order =
			workflow_steps.length > 0
				? Math.max(...workflow_steps.map((ws: WorkflowStep) => ws.step_order)) + 1
				: 1;

		try {
			await apiClient.CreateWorkflowStep(
				{
					name: `New ${step} Step`,
					description: 'A new ${step} Step',
					is_offline: false,
					activation_rule: 'manual',
					step_order: new_step_order,
					tool_setup
				},
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			);
			await invalidateAll();
			notifications.send({ priority: 'INFO', message: 'Step Addded' });
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to create step' });
		}
	}
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Share2 /> Design</h1>
<h2 class="mb-5 text-2xl">Process steps</h2>

<p class="mb-10">
	Use this space to design and configure your process. <a
		class="underline"
		href="/admin/info/process_design">Learn what makes for good process design.</a
	>
</p>

<div class="mb-5 flex flex-col gap-y-5">
	{#each workflow_steps as step}
		<!-- Make items draggable -->
		<Card.Root class="border-">
			<Card.Header>
				<div class="flex flex-row items-center gap-x-5">
					{#if step.tool_config.type === 'polis'}
						<MessagesSquare />
					{/if}
					{#if step.tool_config.type === 'stories'}
						<Video />
					{/if}
					{#if step.tool_config.type === 'heyform'}
						<ListChecks />
					{/if}
					{#if step.tool_config.type === 'learn'}
						<BookOpen />
					{/if}
					<h1 class="text-xl">{step.name}</h1>
				</div>
			</Card.Header>
			<Card.Footer>
				<div class="flex w-full flex-row items-end justify-between capitalize">
					<a href={infoURLForTool(step.tool_config.type)}>{step.tool_config.type}</a>
					<Button
						href={`/admin/conversations/${conversation.id}/design/step/${step.id}`}
						class="secondary">Configure step</Button
					>
				</div>
			</Card.Footer>
		</Card.Root>
	{/each}
</div>

<ToolSelectionModal prompt="Select a step to add" onSelection={addStep}>
	<Button variant="secondary"><Plus /> Add Step</Button>
</ToolSelectionModal>

<style>
	.svelte-dnd-dragging {
		opacity: 0.5;
		cursor: grabbing;
	}
	.svelte-dnd-drop-target {
		outline: 2px dashed #4caf50;
	}
</style>
