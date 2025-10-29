<script lang="ts">
	import {
		Binoculars,
		BookOpen,
		ChevronUp,
		ListChecks,
		MessagesSquare,
		Video
	} from 'lucide-svelte';
	let { data } = $props();
	let { workflow_steps, conversation } = data;
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Binoculars /> Moderate</h1>
<p class="mb-10">Use this space to moderate the conversation</p>

<div class="mb-5 flex flex-col gap-y-5">
	{#each workflow_steps as step (step.id)}
		<Card.Root class="transition-all">
			<Card.Header>
				<div class="flex flex-row items-center justify-between">
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
				</div>
			</Card.Header>
			<Card.Footer>
				<div class="flex w-full flex-row items-end justify-between capitalize">
					{#if step.tool_config.type === 'polis'}
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
