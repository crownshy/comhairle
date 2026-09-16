<script lang="ts">
	// PROTOTYPE variant "switcher". A searchable conversation picker sits at the top (Slack
	// workspace / Supabase project pattern, Cmd+K). Conversations only appear inside the picker,
	// never as nav. Sections below are grouped by phase; workspace links pin to the bottom.
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Popover from '$lib/components/ui/popover';
	import * as Command from '$lib/components/ui/command';
	import { page } from '$app/state';
	import { Check, ChevronsUpDown, LayoutGrid, Lock, MessagesSquare } from 'lucide-svelte';
	import SidebarResizeHandle from '$lib/components/SidebarResizeHandle.svelte';
	import type { LocalizedConversationDto, UserDto } from '@crownshy/api-client/api';
	import CollapseToggle from './CollapseToggle.svelte';
	import UserMenu from './UserMenu.svelte';
	import {
		conversationStatus,
		groupByPhase,
		isNavActive,
		libraryNav,
		sectionLinks,
		workspaceNav,
		type ConversationLike
	} from './nav';

	let {
		user,
		ownedConversations,
		permittedConversations
	}: {
		user: UserDto;
		ownedConversations: LocalizedConversationDto[];
		permittedConversations: LocalizedConversationDto[];
	} = $props();

	let open = $state(false);
	let conversation = $derived(page.data.conversation as ConversationLike | undefined);
	let phaseGroups = $derived(
		conversation ? groupByPhase(sectionLinks(conversation, page.url.pathname)) : []
	);

	function onKeydown(event: KeyboardEvent) {
		if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') {
			event.preventDefault();
			open = !open;
		}
	}
</script>

<svelte:window onkeydown={onKeydown} />

{#snippet conversationItems(heading: string, items: LocalizedConversationDto[])}
	{#if items.length > 0}
		<Command.Group {heading}>
			{#each items as item (item.id)}
				<Command.LinkItem
					href={`/admin/conversations/${item.id}/configure`}
					value={`${item.title} ${item.id}`}
					onSelect={() => (open = false)}
				>
					<Check
						class="size-4 {item.id === conversation?.id ? '' : 'text-transparent'}"
					/>
					<span class="flex-1 truncate">{item.title}</span>
					<span class="text-muted-foreground text-xs">{conversationStatus(item)}</span>
				</Command.LinkItem>
			{/each}
		</Command.Group>
	{/if}
{/snippet}

<SideBar.Root collapsible="icon">
	<SideBar.Header class="gap-2 p-3">
		<div class="flex justify-end group-data-[collapsible=icon]:justify-center">
			<CollapseToggle />
		</div>
		<SideBar.Menu>
			<SideBar.MenuItem>
				<Popover.Root bind:open>
					<Popover.Trigger>
						{#snippet child({ props })}
							<SideBar.MenuButton
								size="lg"
								variant="outline"
								class="h-14"
								{...props}
								aria-keyshortcuts="Meta+K"
							>
								<span
									class="bg-primary text-primary-foreground flex size-8 shrink-0 items-center justify-center rounded-lg"
								>
									<MessagesSquare class="size-4" />
								</span>
								<span class="flex min-w-0 flex-1 flex-col text-left leading-tight">
									{#if conversation}
										<span class="truncate text-sm font-semibold"
											>{conversation.title}</span
										>
										<span class="text-sidebar-foreground/70 text-xs">
											{conversationStatus(conversation)} conversation
										</span>
									{:else}
										<span class="truncate text-sm font-semibold"
											>Choose conversation</span
										>
										<span class="text-sidebar-foreground/70 text-xs"
											>⌘K to search</span
										>
									{/if}
								</span>
								<ChevronsUpDown class="ml-auto size-4" />
							</SideBar.MenuButton>
						{/snippet}
					</Popover.Trigger>
					<Popover.Content side="right" align="start" class="w-80 p-0">
						<Command.Root>
							<Command.Input placeholder="Search conversations" />
							<Command.List class="max-h-80">
								<Command.Empty>No conversations match.</Command.Empty>
								{@render conversationItems('Owned', ownedConversations)}
								{@render conversationItems(
									'Shared with you',
									permittedConversations
								)}
								<Command.Separator />
								<Command.Group>
									<Command.LinkItem href="/admin" onSelect={() => (open = false)}>
										<LayoutGrid class="size-4" />
										All conversations
									</Command.LinkItem>
								</Command.Group>
							</Command.List>
						</Command.Root>
					</Popover.Content>
				</Popover.Root>
			</SideBar.MenuItem>
		</SideBar.Menu>
	</SideBar.Header>

	<SideBar.Content>
		{#if conversation}
			{#each phaseGroups as phase (phase.id)}
				<SideBar.Group>
					<SideBar.GroupLabel>{phase.label}</SideBar.GroupLabel>
					<SideBar.Menu>
						{#each phase.links as link (link.path)}
							<SideBar.MenuItem>
								{#if link.locked}
									<SideBar.MenuButton
										aria-disabled="true"
										class="text-sidebar-foreground/45"
										tooltipContent={`${link.label}: available after launch`}
									>
										<link.icon />
										<span>{link.label}</span>
									</SideBar.MenuButton>
									<SideBar.MenuBadge class="text-sidebar-foreground/45">
										<Lock class="size-3.5" />
									</SideBar.MenuBadge>
								{:else}
									<SideBar.MenuButton
										isActive={link.active}
										tooltipContent={link.label}
									>
										{#snippet child({ props })}
											<a href={link.href} {...props}>
												<link.icon />
												<span>{link.label}</span>
											</a>
										{/snippet}
									</SideBar.MenuButton>
								{/if}
							</SideBar.MenuItem>
						{/each}
					</SideBar.Menu>
				</SideBar.Group>
			{/each}
		{:else}
			<div
				class="text-sidebar-foreground/70 mx-3 mt-2 rounded-lg border border-dashed p-4 text-base group-data-[collapsible=icon]:hidden"
			>
				Pick a conversation above to see its tools.
			</div>
		{/if}

		<SideBar.Group class="mt-auto">
			<SideBar.GroupLabel>Workspace</SideBar.GroupLabel>
			<SideBar.Menu>
				{#each [...workspaceNav, ...libraryNav] as item (item.href)}
					<SideBar.MenuItem>
						<SideBar.MenuButton
							size="sm"
							isActive={isNavActive(item.href, page.url.pathname)}
							tooltipContent={item.label}
						>
							{#snippet child({ props })}
								<a href={item.href} {...props}>
									<item.icon />
									<span>{item.label}</span>
								</a>
							{/snippet}
						</SideBar.MenuButton>
					</SideBar.MenuItem>
				{/each}
			</SideBar.Menu>
		</SideBar.Group>
	</SideBar.Content>

	<SideBar.Footer>
		<UserMenu {user} />
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>
