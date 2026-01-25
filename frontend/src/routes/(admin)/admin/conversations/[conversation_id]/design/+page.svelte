<script lang="ts">
	import type { WorkflowStep } from '$lib/api/api.js';
	import { infoURLForTool } from '$lib/utils';
	import { flip } from 'svelte/animate';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import {
		basic_learn_config,
		basic_polis_config,
		basic_survey_config,
		basic_lived_experience_config,
		basic_elicitation_bot_config
	} from '$lib/workflow_templates.js';
	import ToolSelectionModal from '$lib/components/ToolSelectionModal.svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { Plus, BookOpen, ListChecks, Video, MessagesSquare, ChevronDown } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte.js';
	import { ChevronUp } from 'svelte-radix';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte.js';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';

	let { data } = $props();
	let addStepModalOpen = $state(false);

	$effect(() => {
		if (page.url.searchParams.get('addStep') === 'true') {
			addStepModalOpen = true;
			goto(page.url.pathname, { replaceState: true });
		}
	});

	let conversation = $derived(data.conversation);
	let workflow_steps = $derived(data.workflow_steps);
	let workflow = $derived(data.workflows[0]);
	let firstStep = $derived(workflow_steps.find((s) => s.step_order === 1));

	async function addStep(step: string) {
		let tool_setup = {
			Polis: basic_polis_config,
			Learn: basic_learn_config,
			Survey: basic_survey_config,
			'Lived Experience': basic_lived_experience_config,
			'Elicitation Bot': basic_elicitation_bot_config(conversation)
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
					tool_setup,
					required: true
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

	async function decrementStep(step_id: string) {
		let step = workflow_steps.find((ws: WorkflowStep) => ws.id === step_id);
		await apiClient.UpdateWorkflowStep(
			{ step_order: step.step_order - 2 },
			{
				params: {
					conversation_id: conversation.id,
					workflow_id: workflow.id,
					workflow_step_id: step.id
				}
			}
		);
		await invalidateAll();
	}

	async function incrementStep(step_id: string) {
		let step = workflow_steps.find((ws: WorkflowStep) => ws.id === step_id);
		await apiClient.UpdateWorkflowStep(
			{ step_order: step.step_order + 1 },
			{
				params: {
					conversation_id: conversation.id,
					workflow_id: workflow.id,
					workflow_step_id: step.id
				}
			}
		);
		await invalidateAll();
	}

	function activeToolConfig(step: WorkflowStep) {
		return conversation.is_live ? step.tool_config : step.preview_tool_config;
	}
	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Design</h1>
	<AdminPrevNextControls
		next={firstStep && {
			name: firstStep.name,
			url: `/admin/conversations/${conversation.id}/design/step/${firstStep.id}`
		}}
		prev={{ name: 'Configure', url: `/admin/conversations/${conversation.id}/configure` }}
	/>
{/snippet}

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>Design</Breadcrumb.Item>
{/snippet}

<h2 class="mb-5 text-2xl">Process steps</h2>

<p class="mb-10">
	Use this space to design and configure your process. <a
		class="underline"
		href="/admin/info/process_design">Learn what makes for good process design.</a
	>
</p>

<div class="mb-5 flex flex-col gap-y-5">
	{#each workflow_steps as step, index (step.id)}
		<div animate:flip={{ duration: 200 }}>
			<Card.Root class="transition-all">
				<Card.Header>
					<div class="flex flex-row items-center justify-between">
						<div class="flex flex-row items-center gap-x-5">
							{#if activeToolConfig(step).type === 'polis'}
								<MessagesSquare />
							{/if}
							{#if activeToolConfig(step).type === 'stories'}
								<Video />
							{/if}
							{#if activeToolConfig(step).type === 'heyform'}
								<ListChecks />
							{/if}
							{#if activeToolConfig(step).type === 'learn'}
								<BookOpen />
							{/if}
							<h1 class="text-xl">{step.name}</h1>
						</div>
						<div class="flex flex-row items-center gap-2">
							{#if index > 0}
								<Button variant="ghost" onclick={() => decrementStep(step.id)}>
									<ChevronUp />
								</Button>
							{/if}
							{#if index < workflow_steps.length - 1}
								<Button variant="ghost" onclick={() => incrementStep(step.id)}>
									<ChevronDown />
								</Button>
							{/if}
						</div>
					</div>
				</Card.Header>
				<Card.Footer>
					<div class="flex w-full flex-row items-end justify-between capitalize">
						<a href={infoURLForTool(activeToolConfig(step).type)}
							>{activeToolConfig(step).type}</a
						>
						<Button
							href={`/admin/conversations/${conversation.id}/design/step/${step.id}`}
							class="secondary">Configure step</Button
						>
					</div>
				</Card.Footer>
			</Card.Root>
		</div>
	{/each}
</div>

<ToolSelectionModal
	prompt="Select a step to add"
	onSelection={addStep}
	bind:open={addStepModalOpen}
>
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
