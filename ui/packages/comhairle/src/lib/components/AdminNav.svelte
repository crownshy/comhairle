<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import {
		Info,
		LayoutDashboard,
		Settings,
		Home,
		Mail,
		Images,
		PanelLeftClose,
		PanelLeftOpen
	} from 'lucide-svelte';
	import { Button } from './ui/button';
	import NewConversationButton from './NewConversationButton.svelte';
	import { userInitials } from '$lib/utils';
	import ComhairleLogo from './ComhairleLogo.svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import SidebarResizeHandle from './SidebarResizeHandle.svelte';
	import { sidebarWidth, EXPAND_WIDTH } from './sidebarWidth.svelte.js';
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import { SIDEBAR_KEYBOARD_SHORTCUT } from './ui/sidebar/constants';

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
			title={`Open / close (Ctrl+${SIDEBAR_KEYBOARD_SHORTCUT} or Cmd+${SIDEBAR_KEYBOARD_SHORTCUT})`}
			onclick={() => sidebar.toggle()}
		>
			<PanelLeftClose class="size-4" />
			<span class="sr-only">Collapse sidebar</span>
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
			title={`Open / close (Ctrl+${SIDEBAR_KEYBOARD_SHORTCUT} or Cmd+${SIDEBAR_KEYBOARD_SHORTCUT})`}
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

		<SideBar.Group>
			<SideBar.GroupLabel class="text-sidebar-secondary text-xs font-medium">
				Configuration
			</SideBar.GroupLabel>
			<SideBar.GroupContent>
				<SideBar.Menu>
					<SideBar.MenuItem>
						<SideBar.MenuButton>
							{#snippet child({ props: btnProps })}
								<a {...btnProps} href="/admin/email-template-configs">
									<Mail class="size-4" />
									Emails
								</a>
							{/snippet}
						</SideBar.MenuButton>
					</SideBar.MenuItem>
					<SideBar.MenuItem>
						<SideBar.MenuButton>
							{#snippet child({ props: btnProps })}
								<a {...btnProps} href="/admin/media-library">
									<Images class="size-4" />
									Media library
								</a>
							{/snippet}
						</SideBar.MenuButton>
					</SideBar.MenuItem>
				</SideBar.Menu>
			</SideBar.GroupContent>
		</SideBar.Group>

		<!-- Conversations section (collapsed: dot list, active pill-highlighted) -->
		<SideBar.Group class="hidden min-h-0 flex-1 group-data-[collapsible=icon]:flex">
			<SideBar.GroupContent class="min-h-0 flex-1">
				<ScrollArea.Root class="h-full" type="hover">
					<SideBar.Menu class="items-center gap-0.5">
						{#each conversations ?? [] as conversation (conversation.id)}
							{@const active = isConversationActive(conversation.id)}
							<SideBar.MenuItem>
								<SideBar.MenuButton
									isActive={active}
									tooltipContent={conversation.title}
									class="flex h-8 w-8 items-center justify-center p-0"
								>
									{#snippet child({ props: btnProps })}
										<a
											{...btnProps}
											href={`/admin/conversations/${conversation.id}/configure`}
											aria-label={conversation.title}
										>
											<span
												class="size-1.5 rounded-full {active
													? 'bg-sidebar-accent-foreground'
													: 'bg-sidebar-foreground/40'}"
												aria-hidden="true"
											></span>
										</a>
									{/snippet}
								</SideBar.MenuButton>
							</SideBar.MenuItem>
						{/each}
					</SideBar.Menu>
				</ScrollArea.Root>
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
										class="text-sidebar-foreground/80 data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground h-8 w-full overflow-hidden rounded-lg p-2 data-[active=true]:font-semibold"
										isActive={active}
									>
										{#snippet child({ props: btnProps })}
											<a
												{...btnProps}
												href={`/admin/conversations/${conversation.id}/configure`}
												class="hover:bg-sidebar-accent hover:text-sidebar-accent-foreground flex w-full items-center rounded-lg px-2 py-1.5 {active
													? 'bg-sidebar-accent text-sidebar-accent-foreground font-semibold'
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
		<NewConversationButton
			class="w-full group-data-[collapsible=icon]:h-8 group-data-[collapsible=icon]:p-0"
			labelClass="group-data-[collapsible=icon]:hidden"
		/>
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
