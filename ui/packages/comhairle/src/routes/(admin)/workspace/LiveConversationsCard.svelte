<script lang="ts">
	import { resolve } from '$app/paths';
	import { BarChart3 } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import type { PermittedConversationDto } from '@crownshy/api-client/api';
	import SeeAllLabel from './SeeAllLabel.svelte';

	type Props = { conversations: PermittedConversationDto[]; total: number; seeAllHref: string };
	let { conversations, total, seeAllHref }: Props = $props();
</script>

<Card.Root class="h-full gap-4 py-5">
	<Card.Header class="px-5">
		<Card.Title class="text-lg">Live now</Card.Title>
		<Card.Action>
			<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
			<a href={seeAllHref} class="group"><SeeAllLabel label="See all {total}" /></a>
		</Card.Action>
	</Card.Header>
	<Card.Content class="flex flex-col gap-1 px-3">
		{#each conversations as conversation (conversation.id)}
			<div
				class="hover:bg-muted group/row flex items-center gap-2 rounded-lg px-2 py-1.5 transition-colors"
			>
				<a
					href={resolve('/(admin)/admin/conversations/[conversation_id]/configure', {
						conversation_id: conversation.id
					})}
					class="min-w-0 flex-1 truncate text-base"
				>
					{conversation.title}
				</a>
				<Button
					size="sm"
					variant="ghost"
					href={resolve('/(admin)/admin/conversations/[conversation_id]/monitor', {
						conversation_id: conversation.id
					})}
					class="text-primary opacity-70 group-hover/row:opacity-100"
				>
					<BarChart3 class="size-4" /> Monitor
				</Button>
			</div>
		{:else}
			<p class="text-muted-foreground px-2 text-base">Nothing live right now.</p>
		{/each}
	</Card.Content>
</Card.Root>
