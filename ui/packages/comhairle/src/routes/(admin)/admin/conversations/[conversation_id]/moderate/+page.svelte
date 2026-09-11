<script lang="ts">
	import { BookOpen, ListChecks, MessagesSquare, Video } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TabContent from '../TabContent.svelte';
	import Skeleton from '$lib/components/ui/skeleton/skeleton.svelte';
	import { resolve } from '$app/paths';

	const { data, params } = $props();
</script>

<svelte:head>
	<title>Moderate Conversation - Comhairle Admin</title>
</svelte:head>

<TabContent>
	<PageHeader title="Moderate" description="Use this space to moderate the conversation" />

	<div class="mb-5 flex flex-col gap-y-5">
		{#await data.streamedWorkflowSteps}
			{#each [1, 2] as i (i)}
				<div class="border-border bg-card flex flex-col gap-y-9 rounded-xl border p-6">
					<div class="flex items-center gap-x-5">
						<Skeleton class="size-7 rounded" />
						<Skeleton class="h-7 w-50 rounded-lg" />
					</div>
					<Skeleton class="h-9 w-50 rounded-2xl" />
				</div>
			{/each}
		{:then workflowSteps}
			{#if workflowSteps.err !== null}
				<span class="text-destructive">Could not load workflow steps, please try again</span
				>
			{:else}
				{#each workflowSteps.ok as step (step.id)}
					{@const type = step.previewToolConfig.type}
					<Card.Root class="transition-all">
						<Card.Header>
							<div class="flex flex-row items-center justify-between">
								<div class="flex flex-row items-center gap-x-5">
									{#if type === 'polis'}
										<MessagesSquare />
									{/if}
									{#if type === 'stories'}
										<Video />
									{/if}
									{#if type === 'heyform'}
										<ListChecks />
									{/if}
									{#if type === 'learn'}
										<BookOpen />
									{/if}
									<h2 class="text-xl">{step.name}</h2>
								</div>
							</div>
						</Card.Header>
						<Card.Footer>
							<div class="flex w-full flex-row items-end justify-between capitalize">
								{#if type === 'polis'}
									<Button
										href={resolve(
											`/(admin)/admin/conversations/[conversation_id]/moderate/step/[step_id]`,
											{
												conversation_id: params.conversation_id,
												step_id: step.id
											}
										)}
										class="secondary">Moderate step</Button
									>
								{:else}
									<Button disabled class="secondary"
										>No moderation for this step</Button
									>
								{/if}
							</div>
						</Card.Footer>
					</Card.Root>
				{/each}
			{/if}
		{/await}
	</div>
</TabContent>
