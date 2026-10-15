<script lang="ts">
	import { resolve } from '$app/paths';
	import * as SideBar from '$lib/components/ui/sidebar';
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { conversationStatus } from '$lib/utils/conversationStatus';

	type Props = { latestConversations: LocalizedConversationDto[]; path: string };
	let { latestConversations, path }: Props = $props();

	function isOpen(conversationId: string): boolean {
		return path.startsWith(`/admin/conversations/${conversationId}`);
	}
</script>

{#if permissions.can('system', 'conversation_create')}
	<div class="pr-3 group-data-[collapsible=icon]:pr-0">
		<NewConversationButton
			class="w-full group-data-[collapsible=icon]:h-8 group-data-[collapsible=icon]:w-8 group-data-[collapsible=icon]:p-0"
			labelClass="group-data-[collapsible=icon]:hidden"
		/>
	</div>
{/if}

<SideBar.Group class="px-0 group-data-[collapsible=icon]:hidden">
	<SideBar.GroupLabel class="text-sidebar-secondary text-sm font-medium"
		>Latest</SideBar.GroupLabel
	>
	<SideBar.GroupContent>
		<SideBar.Menu class="pr-3">
			{#each latestConversations as conversation (conversation.id)}
				{@const live = conversationStatus(conversation) === 'live'}
				<SideBar.MenuItem>
					<SideBar.MenuButton
						isActive={isOpen(conversation.id)}
						tooltipContent={conversation.title}
						class="text-base"
					>
						{#snippet child({ props })}
							<a
								{...props}
								href={resolve(
									'/(admin)/admin/conversations/[conversation_id]/configure',
									{
										conversation_id: conversation.id
									}
								)}
							>
								<span
									class="size-2 shrink-0 rounded-full {live
										? 'bg-primary'
										: 'bg-sidebar-foreground/40'}"
									aria-hidden="true"
								></span>
								<span class="truncate">{conversation.title}</span>
							</a>
						{/snippet}
					</SideBar.MenuButton>
				</SideBar.MenuItem>
			{/each}
			<SideBar.MenuItem>
				<a
					href={resolve('/admin/conversations')}
					class="text-sidebar-secondary hover:text-sidebar-foreground flex px-2 py-1.5 text-base"
				>
					See more
				</a>
			</SideBar.MenuItem>
		</SideBar.Menu>
	</SideBar.GroupContent>
</SideBar.Group>
