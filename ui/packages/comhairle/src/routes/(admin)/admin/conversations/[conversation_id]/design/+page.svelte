<script lang="ts">
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';
	import { infoURLForTool } from '$lib/utils';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import {
		basic_learn_config,
		basic_polis_config,
		basic_survey_config,
		basic_lived_experience_config,
		basic_elicitation_bot_config,
		basic_thinking_space_config,
		defaultStepCreationParams
	} from '$lib/workflow_templates.js';
	import ToolSelectionModal from '$lib/components/ToolSelectionModal.svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import {
		Plus,
		BookOpen,
		ListChecks,
		Video,
		MessagesSquare,
		Bot,
		GripVertical,
		ChevronUp,
		ChevronDown
	} from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte.js';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte.js';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import DraggableList from '$lib/components/DraggableList.svelte';

	let { data } = $props();
	let addStepModalOpen = $state(false);
	let reorderedSteps = $state<WorkflowStepWithTranslations[]>([]);

	$effect(() => {
		if (page.url.searchParams.get('addStep') === 'true') {
			addStepModalOpen = true;
			goto(page.url.pathname, { replaceState: true });
		}
	});

	let conversation = $derived(data.conversation);
	let workflowSteps = $derived(data.workflowSteps);
	let workflow = $derived(data.workflows[0]);
	let firstStep = $derived(workflowSteps.find((s) => s.stepOrder === 1));

	$effect(() => {
		reorderedSteps = [...workflowSteps];
	});

	async function addStep(step: string) {
		let tool_setup = {
			Polis: basic_polis_config,
			Learn: basic_learn_config,
			Survey: basic_survey_config,
			'Lived Experience': basic_lived_experience_config,
			'Elicitation Bot': basic_elicitation_bot_config(conversation),
			'Thinking Space': basic_thinking_space_config()
		}[step];

		let new_step_order =
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
			await invalidateAll();
			notifications.send({ priority: 'INFO', message: 'Step Added' });
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to create step' });
		}
	}

	function handleReorder(next: WorkflowStepWithTranslations[]) {
		reorderedSteps = next;
	}

	async function handleCommit(next: WorkflowStepWithTranslations[]) {
		for (let i = 0; i < next.length; i++) {
			const step = next[i];
			const newOrder = i + 1;

			if (step.stepOrder !== newOrder) {
				try {
					await apiClient.UpdateConversationWorkflowStep(
						{ step_order: newOrder },
						{
							params: {
								conversation_id: conversation.id,
								workflow_id: workflow.id,
								workflow_step_id: step.id
							}
						}
					);
				} catch (e) {
					console.error(e);
					notifications.send({ priority: 'ERROR', message: 'Failed to reorder steps' });
					await invalidateAll();
					return;
				}
			}
		}
		await invalidateAll();
		notifications.send({ priority: 'INFO', message: 'Steps reordered' });
	}

	async function moveStep(index: number, direction: -1 | 1) {
		const target = index + direction;
		if (target < 0 || target >= reorderedSteps.length) return;
		const next = [...reorderedSteps];
		[next[index], next[target]] = [next[target], next[index]];
		reorderedSteps = next;
		await handleCommit(next);
	}

	function activeToolConfig(step: WorkflowStepWithTranslations) {
		return conversation.isLive ? step.toolConfig : step.previewToolConfig;
	}
	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
	let pageTitle = $derived(`Design ${conversation.title}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

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
	<DraggableList
		items={reorderedSteps}
		onReorder={handleReorder}
		onCommit={handleCommit}
		class="flex flex-col gap-y-5"
		flipDurationMs={200}
	>
		{#snippet children(step, index)}
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
							{#if activeToolConfig(step).type === 'thinkingspace'}
								<Bot />
							{/if}
							{#if activeToolConfig(step).type === 'elicitationbot'}
								<Bot />
							{/if}
							<h1 class="text-xl">{step.name}</h1>
						</div>
						<div class="flex flex-row items-center gap-2">
							{#if index > 0}
								<Button
									variant="ghost"
									size="icon"
									aria-label="Move step up"
									onclick={() => moveStep(index, -1)}
								>
									<ChevronUp />
								</Button>
							{/if}
							{#if index < reorderedSteps.length - 1}
								<Button
									variant="ghost"
									size="icon"
									aria-label="Move step down"
									onclick={() => moveStep(index, 1)}
								>
									<ChevronDown />
								</Button>
							{/if}
							<GripVertical class="text-muted-foreground cursor-grab" />
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
		{/snippet}
	</DraggableList>
</div>

<ToolSelectionModal
	prompt="Select a step to add"
	onSelection={addStep}
	bind:open={addStepModalOpen}
>
	<Button variant="outline"><Plus /> Add Step</Button>
</ToolSelectionModal>
