<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ArrowUpRight, MoreHorizontal, Eye, ExternalLink, Check, CircleX } from 'lucide-svelte';
	import { resolve } from '$app/paths';
	import { MOBILE_BREAKPOINT } from '$lib/utils/constants';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';
	import type { ConversationDto } from '@crownshy/api-client/api';

	interface Props {
		conversation: ConversationDto;
	}

	let { conversation }: Props = $props();

	let endModalOpen = $state(false);
	let launchModalOpen = $state(false);
	let innerWidth = $state<number | null>(null);
</script>

<svelte:window bind:innerWidth />

<!-- Modals (triggered programmatically from mobile dropdown or desktop UI) -->
<EndConversationModal {conversation} bind:open={endModalOpen} hideTrigger />
<LaunchConversationModal
	conversation_id={conversation.id}
	bind:open={launchModalOpen}
	hideTrigger
/>

{#if innerWidth && innerWidth <= MOBILE_BREAKPOINT}
	<!-- Mobile actions: single more-menu -->
	<div class="flex shrink-0 items-center">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class="bg-primary/20 hover:bg-primary/30 inline-flex size-9 items-center justify-center rounded-full"
				aria-label="Actions"
			>
				<MoreHorizontal class="size-4" />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end" class="w-56">
				<DropdownMenu.Item>
					<a
						href={resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
							conversation_id: conversation.id
						})}
						target="_blank"
						class="flex w-full items-center gap-2"
					>
						<Eye class="size-4" />
						Preview
					</a>
				</DropdownMenu.Item>
				{#if conversation.isLive}
					<DropdownMenu.Item>
						<a
							href={resolve('/(public)/conversations/[conversation_id]', {
								conversation_id: conversation.id
							})}
							class="flex w-full items-center gap-2"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					{#if !conversation.isComplete}
						<DropdownMenu.Item
							class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
							onclick={() => void (endModalOpen = true)}
						>
							<CircleX class="text-destructive size-4" />
							End Conversation
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => void (endModalOpen = true)}>
							<Check class="size-4" />
							Re-open Conversation
						</DropdownMenu.Item>
					{/if}
				{:else}
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={() => void (launchModalOpen = true)}>
						<ArrowUpRight class="size-4" />
						Launch Conversation
					</DropdownMenu.Item>
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
{:else}
	<!-- Desktop actions -->
	<div class="flex shrink-0 items-center gap-4">
		<Button
			href={`/conversations/${conversation.id}/preview`}
			target="_blank"
			variant="secondary"
			class="bg-primary/20 text-foreground hover:bg-primary/30 inline-flex h-10 rounded-full px-4 text-sm"
		>
			Preview
			<ArrowUpRight class="size-4" />
		</Button>

		{#if conversation.isLive}
			{#if !conversation.isComplete}
				<span
					class="bg-primary text-primary-foreground inline-flex h-10 items-center gap-2 rounded-full py-1 pr-1 pl-5 text-sm font-medium"
				>
					Launched
					<span
						class="bg-primary-foreground text-primary inline-flex size-8 items-center justify-center rounded-full"
					>
						<Check class="size-4" strokeWidth={3} />
					</span>
				</span>
			{/if}

			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="bg-primary/20 hover:bg-primary/30 inline-flex size-10 items-center justify-center rounded-full"
					aria-label="More actions"
				>
					<MoreHorizontal class="size-4" />
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					<DropdownMenu.Item>
						<a
							href={resolve('/(public)/conversations/[conversation_id]', {
								conversation_id: conversation.id
							})}
							class="flex w-full items-center gap-2"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					{#if !conversation.isComplete}
						<DropdownMenu.Item
							class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
							onclick={() => void (endModalOpen = true)}
						>
							<CircleX class="text-destructive size-4" />
							End Conversation
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => void (endModalOpen = true)}>
							<Check class="size-4" />
							Re-open Conversation
						</DropdownMenu.Item>
					{/if}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{:else}
			<Button variant="default" class="h-10" onclick={() => void (launchModalOpen = true)}>
				Launch Conversation
			</Button>
		{/if}
	</div>
{/if}
