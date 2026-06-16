<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import {
		Info,
		LayoutDashboard,
		Plus,
		Settings,
		Home,
		PanelLeftClose,
		PanelLeftOpen
	} from 'lucide-svelte';
	import { Button } from './ui/button';
	import { userInitials } from '$lib/utils';
	import ComhairleLogo from './ComhairleLogo.svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import SidebarResizeHandle from './SidebarResizeHandle.svelte';
	import { sidebarWidth, EXPAND_WIDTH } from './sidebarWidth.svelte.js';
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';

	const sidebar = useSidebar();

	function expandSidebar() {
		if (sidebarWidth.width < EXPAND_WIDTH) sidebarWidth.set(EXPAND_WIDTH);
		sidebar.setOpen(true);
		sidebarWidth.persist();
	}
	let props = $props();
	let path = $derived<string>(props.path);
	let user = $derived(props.user);
	let conversations: LocalizedConversationDto[] = $derived(props.conversations);
	let user_initials = $derived(userInitials(user?.username ?? ''));

	function isConversationActive(conversationId: string): boolean {
		return path.startsWith(`/admin/conversations/${conversationId}`);
	}
</script>

<SideBar.Root collapsible="icon">
	<SideBar.Header
		class="flex flex-row items-center justify-between py-6 pr-3 pl-6 group-data-[collapsible=icon]:px-2 group-data-[collapsible=icon]:py-3"
	>
		<div class="group-data-[collapsible=icon]:hidden">
			<ComhairleLogo color="sidebar-foreground" />
		</div>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar size-7 group-data-[collapsible=icon]:hidden"
			onclick={() => sidebar.toggle()}
		>
			<PanelLeftClose class="size-4" />
			<span class="sr-only">Collapse sidebar</span>
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
			onclick={expandSidebar}
		>
			<PanelLeftOpen class="size-4" />
			<span class="sr-only">Expand sidebar</span>
		</Button>
	</SideBar.Header>

	<SideBar.Content
		class="overflow-hidden pt-4 pl-4 group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:pt-2"
	>
		<!-- Platform section -->
		<SideBar.Group>
			<SideBar.GroupLabel class="text-sidebar-secondary text-xs font-medium">
				Platform
			</SideBar.GroupLabel>
			<SideBar.GroupContent>
				<SideBar.Menu>
					<SideBar.MenuItem>
						<SideBar.MenuButton>
							{#snippet child({ props: btnProps })}
								<a {...btnProps} href="/">
									<Home class="size-4" />
									Home
								</a>
							{/snippet}
						</SideBar.MenuButton>
					</SideBar.MenuItem>
					<SideBar.MenuItem>
						<SideBar.MenuButton>
							{#snippet child({ props: btnProps })}
								<a {...btnProps} href="/admin/">
									<LayoutDashboard class="size-4" />
									Workspace
								</a>
							{/snippet}
						</SideBar.MenuButton>
					</SideBar.MenuItem>
				</SideBar.Menu>
			</SideBar.GroupContent>
		</SideBar.Group>

		<!-- Conversations section -->
		<SideBar.Group
			class="flex min-h-0 flex-1 flex-col pr-1 group-data-[collapsible=icon]:hidden"
		>
			<SideBar.GroupLabel class="text-sidebar-secondary text-xs font-medium">
				Conversations
			</SideBar.GroupLabel>

			<SideBar.GroupContent class="min-h-0 flex-1">
				<ScrollArea.Root class="h-full pr-3" type="always">
					{#if conversations}
						<SideBar.Menu>
							{#each conversations as conversation (conversation.id)}
								{@const active = isConversationActive(conversation.id)}
								<SideBar.MenuItem>
									<SideBar.MenuButton
										class="text-sidebar-foreground/80 data-[active=true]:bg-sidebar-accent h-8 w-full overflow-hidden rounded-lg p-2 data-[active=true]:font-semibold data-[active=true]:text-white"
										isActive={active}
									>
										{#snippet child({ props: btnProps })}
											<a
												{...btnProps}
												href={`/admin/conversations/${conversation.id}/configure`}
												class="hover:bg-sidebar-accent flex w-full items-center rounded-lg px-2 py-1.5 hover:text-white {active
													? 'bg-sidebar-accent font-semibold text-white'
													: ''}"
											>
												{#if conversation.title.length > 29}
													<Tooltip.Root>
														<Tooltip.Trigger>
															{#snippet child({ props: tipProps })}
																<span
																	{...tipProps}
																	class="flex-1 truncate text-left text-sm leading-4 font-medium"
																>
																	{conversation.title}
																</span>
															{/snippet}
														</Tooltip.Trigger>
														<Tooltip.Content side="right">
															{conversation.title}
														</Tooltip.Content>
													</Tooltip.Root>
												{:else}
													<span
														class="flex-1 truncate text-left text-sm leading-4 font-medium"
													>
														{conversation.title}
													</span>
												{/if}
											</a>
										{/snippet}
									</SideBar.MenuButton>
								</SideBar.MenuItem>
							{/each}
						</SideBar.Menu>
					{/if}
				</ScrollArea.Root>
			</SideBar.GroupContent>
		</SideBar.Group>
	</SideBar.Content>

	<div class="shrink-0 px-7 group-data-[collapsible=icon]:px-2">
		<Button
			href="/admin/conversations/new"
			class="w-full group-data-[collapsible=icon]:h-8 group-data-[collapsible=icon]:p-0"
			variant="default"
		>
			<Plus class="size-4" />
			<span class="group-data-[collapsible=icon]:hidden">New conversation</span>
		</Button>
	</div>

	<SideBar.Footer>
		<div class="flex flex-col items-center gap-2 p-2 group-data-[collapsible=icon]:hidden">
			<Avatar.Root class="h-12 w-12">
				{#if user?.avatarUrl}
					<Avatar.Image src={user.avatarUrl} alt={user.username} />
				{/if}
				<Avatar.Fallback>{user_initials}</Avatar.Fallback>
			</Avatar.Root>
			<div class="flex w-full flex-col items-center gap-0.5">
				<span
					class="text-sidebar-foreground w-full truncate text-center text-sm font-semibold"
				>
					{user?.username ?? ''}
				</span>
				{#if user?.email}
					<span class="text-sidebar-foreground/70 w-full truncate text-center text-xs">
						{user.email}
					</span>
				{/if}
			</div>
		</div>
		<SideBar.Menu>
			<SideBar.MenuItem>
				<SideBar.MenuButton>
					{#snippet child({ props: btnProps })}
						<a {...btnProps} href="/settings">
							<Settings class="size-4" />
							Settings
						</a>
					{/snippet}
				</SideBar.MenuButton>
			</SideBar.MenuItem>
			<SideBar.MenuItem>
				<SideBar.MenuButton>
					{#snippet child({ props: btnProps })}
						<a {...btnProps} href="/about">
							<Info class="size-4" />
							About Comhairle
						</a>
					{/snippet}
				</SideBar.MenuButton>
			</SideBar.MenuItem>
		</SideBar.Menu>
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>
