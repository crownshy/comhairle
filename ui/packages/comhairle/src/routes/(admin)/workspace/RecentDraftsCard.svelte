<script lang="ts">
	import { resolve } from '$app/paths';
	import { Eye } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Button } from '$lib/components/ui/button';
	import type { PermittedConversationDto } from '@crownshy/api-client/api';
	import SeeAllLabel from './SeeAllLabel.svelte';

	type Props = { conversations: PermittedConversationDto[]; seeAllHref: string };
	let { conversations, seeAllHref }: Props = $props();
</script>

<Card.Root class="h-full gap-3 py-5">
	<Card.Header class="px-5">
		<Card.Title class="text-lg">Recent drafts</Card.Title>
		<Card.Action>
			<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
			<a href={seeAllHref} class="group"><SeeAllLabel /></a>
		</Card.Action>
	</Card.Header>
	<Card.Content class="flex flex-col gap-1 px-3">
		{#each conversations as conversation (conversation.id)}
			<div
				class="hover:bg-muted flex items-center gap-2 rounded-lg px-2 py-1 transition-colors"
			>
				<a
					href={resolve('/(admin)/admin/conversations/[conversation_id]/configure', {
						conversation_id: conversation.id
					})}
					class="min-w-0 flex-1 truncate text-base"
				>
					{conversation.title}
				</a>
				<Tooltip.Root>
					<Tooltip.Trigger>
						{#snippet child({ props })}
							<Button
								{...props}
								size="icon"
								variant="ghost"
								target="_blank"
								href={resolve(
									'/(public)/conversations/[conversation_id]/[[preview]]',
									{
										conversation_id: conversation.id,
										preview: 'preview'
									}
								)}
								class="size-8"
							>
								<Eye class="size-4" /><span class="sr-only"
									>Preview {conversation.title}</span
								>
							</Button>
						{/snippet}
					</Tooltip.Trigger>
					<Tooltip.Content>Preview</Tooltip.Content>
				</Tooltip.Root>
			</div>
		{:else}
			<p class="text-muted-foreground px-2 text-base">No drafts.</p>
		{/each}
	</Card.Content>
</Card.Root>
