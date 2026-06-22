<script lang="ts">
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';
	import { infoURLForTool } from '$lib/utils';
	import { page } from '$app/state';
	import { goto, invalidate } from '$app/navigation';
	import {
		Plus,
		BookOpen,
		ListChecks,
		ListOrdered,
		Video,
		MessagesSquare,
		Bot,
		GripVertical,
		ChevronUp,
		ChevronDown
	} from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte.js';
	import DraggableList from '$lib/components/DraggableList.svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived<WorkflowStepWithTranslations[] | undefined>(data.workflowSteps);
	let loading = $derived(workflowSteps === undefined);

	let reorderedSteps = $state<WorkflowStepWithTranslations[]>([]);

	$effect(() => {
		reorderedSteps = workflowSteps
			? [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder)
			: [];
	});

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
					await invalidate('conversation:workflow');
					return;
				}
			}
		}
		await invalidate('conversation:workflow');
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

	function openAddStep() {
		goto(`${page.url.pathname}?addStep=true`);
	}

	let pageTitle = $derived(`Design ${conversation.title}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<div class="mx-auto max-w-4xl">
	<h2 class="mb-5 text-2xl">Process steps</h2>

	<p class="mb-10">
		Use this space to design and configure your process.
		<a class="underline" href="/admin/info/process_design">
			Learn what makes for good process design.
		</a>
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
								{#if activeToolConfig(step).type === 'prioritization'}
									<ListOrdered />
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
							<a href={infoURLForTool(activeToolConfig(step).type)}>
								{activeToolConfig(step).type}
							</a>
							<Button
								href={`/admin/conversations/${conversation.id}/design/step/${step.id}`}
								class="secondary"
							>
								Configure step
							</Button>
						</div>
					</Card.Footer>
				</Card.Root>
			{/snippet}
		</DraggableList>
	</div>

	<Button variant="outline" onclick={openAddStep}>
		<Plus />
		Add Step
	</Button>
</div>
