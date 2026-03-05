<script lang="ts">
	import { BookOpen, ListChecks, MessagesSquare, Video } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { BreadcrumbItem } from '$lib/components/ui/breadcrumb/index.js';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte.js';
	import type { ToolConfig, WorkflowStepWithTranslations } from '@crownshy/api-client/api';

	let { data } = $props();
	let { workflowSteps, conversation } = data;

	function activeToolConfig(step: WorkflowStepWithTranslations): ToolConfig {
		return conversation.isLive ? step.toolConfig : step.previewToolConfig;
	}

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

<svelte:head>
	<title>Moderate Conversation - Comhairle Admin</title>
</svelte:head>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Moderate</h1>
{/snippet}

{#snippet breadcrumbSnippet()}
	<BreadcrumbItem>Moderate</BreadcrumbItem>
{/snippet}

<p class="mb-10">Use this space to moderate the conversation</p>

<div class="mb-5 flex flex-col gap-y-5">
	{#each workflowSteps as step (step.id)}
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
				</div>
			</Card.Header>
			<Card.Footer>
				<div class="flex w-full flex-row items-end justify-between capitalize">
					{#if activeToolConfig(step).type === 'polis'}
						<Button
							href={`/admin/conversations/${conversation.id}/moderate/step/${step.id}`}
							class="secondary">Moderate step</Button
						>
					{:else}
						<Button disabled class="secondary">No moderation for this step</Button>
					{/if}
				</div>
			</Card.Footer>
		</Card.Root>
	{/each}
</div>
