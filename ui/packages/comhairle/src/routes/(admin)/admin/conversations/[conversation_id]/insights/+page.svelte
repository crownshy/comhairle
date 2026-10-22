<script lang="ts">
	import { resolve } from '$app/paths';
	import { ChevronRight, Lightbulb } from 'lucide-svelte';
	import { toolMeta } from '$lib/tool_meta';

	let { data } = $props();
	let conversation = $derived(data.conversation);
	let insightsSteps = $derived(data.insightsSteps);
</script>

<svelte:head>
	<title>Insights - Comhairle Admin</title>
</svelte:head>

{#if insightsSteps.length === 0}
	<p class="text-muted-foreground">
		None of this conversation's steps have Insights yet. Polls, surveys, thinking spaces and
		prioritisation steps do.
	</p>
{:else}
	<ul class="bg-background divide-border border-border max-w-3xl divide-y rounded-xl border">
		{#each insightsSteps as step (step.id)}
			{@const tool = toolMeta(step.toolType)}
			{@const Icon = tool?.icon ?? Lightbulb}
			<li>
				<a
					href={resolve(
						'/(admin)/admin/conversations/[conversation_id]/insights/step/[step_id]',
						{
							conversation_id: conversation.id,
							step_id: step.id
						}
					)}
					class="hover:bg-muted/60 flex items-center gap-4 px-5 py-4"
				>
					<Icon class="text-muted-foreground size-5 shrink-0" />
					<span class="flex-1">
						<span class="block font-medium">{step.name}</span>
						{#if tool}
							<span class="text-muted-foreground block">{tool.displayName}</span>
						{/if}
					</span>
					<ChevronRight class="text-muted-foreground size-4 shrink-0" />
				</a>
			</li>
		{/each}
	</ul>
{/if}
