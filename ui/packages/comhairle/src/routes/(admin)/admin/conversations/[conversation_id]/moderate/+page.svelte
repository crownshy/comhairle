<script lang="ts">
	import { resolve } from '$app/paths';
	import { ChevronRight, MessagesSquare } from 'lucide-svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { moderateTour } from './tour';

	useTour(() => moderateTour);

	let { data } = $props();
	let conversation = $derived(data.conversation);
	let polisSteps = $derived(data.polisSteps);
</script>

<svelte:head>
	<title>Moderate - Comhairle Admin</title>
</svelte:head>

{#if polisSteps.length === 0}
	<p class="text-muted-foreground">
		There is nothing to moderate yet. Add a Polis step in Process design.
	</p>
{:else}
	<ul
		data-tour="moderate-steps"
		class="bg-background divide-border border-border max-w-3xl divide-y rounded-xl border"
	>
		{#each polisSteps as step (step.id)}
			<li>
				<a
					href={resolve(
						'/(admin)/admin/conversations/[conversation_id]/moderate/step/[step_id]',
						{
							conversation_id: conversation.id,
							step_id: step.id
						}
					)}
					class="hover:bg-muted/60 flex items-center gap-4 px-5 py-4"
				>
					<MessagesSquare class="text-muted-foreground size-5 shrink-0" />
					<span class="flex-1 font-medium">{step.name}</span>
					<ChevronRight class="text-muted-foreground size-4 shrink-0" />
				</a>
			</li>
		{/each}
	</ul>
{/if}
