<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import {
		Info,
		LayoutDashboard,
		Plus,
		Settings,
		ChevronRight,
		Home,
		PanelLeftClose
	} from 'lucide-svelte';
	import { conversationSteps, type ConversationStep } from '$lib/config/conversation-steps';
	import { Button } from './ui/button';
	import { page } from '$app/state';
	import { userInitials } from '$lib/utils';
	import ComhairleLogo from './ComhairleLogo.svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import type { LocalizedConversationDto } from '$lib/api/api';
	const sidebar = useSidebar();
	let props = $props();
	let path = $derived(props.path);
	let user = $derived(props.user);
	let conversations: LocalizedConversationDto[] = $derived(props.conversations);
	let workflowSteps = $derived(page.data?.workflowSteps ?? []);
	let user_initials = $derived(userInitials(user?.username ?? ''));

	function shouldActivateStep(isLive: boolean, step: ConversationStep): boolean {
		return isLive ? !step.activeOnLive : step.activeOnLive;
	}

	// TODO We need to use data-sveltekit-reload as the
	// component isn't relaoading on navigation when we use
	// page.ts for the data fetching
</script>

<SideBar.Root class="w-72">
	<SideBar.Header class="flex flex-row items-center justify-between py-6 pl-6 pr-3">
		<ComhairleLogo />
		<Button
			variant="ghost"
			size="icon"
			class="text-sidebar-foreground/70 hover:text-sidebar size-7"
			onclick={() => sidebar.toggle()}
		>
			<PanelLeftClose class="size-4" />
			<span class="sr-only">Collapse sidebar</span>
		</Button>
	</SideBar.Header>

	<SideBar.Content class="overflow-hidden py-4 pl-4">
		<!-- Platform section -->
		<SideBar.Group class="">
			<!-- todo: hook up to style variable + add translations -->
			<SideBar.GroupLabel class="text-sidebar-secondary text-xs font-medium"
				>Platform</SideBar.GroupLabel
			>
			<SideBar.GroupContent>
				<SideBar.Menu>
					<SideBar.MenuItem>
						<SideBar.MenuButton>
							{#snippet child({ props: btnProps })}
								<a {...btnProps} href="/admin/">
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
									Dashboard
								</a>
							{/snippet}
						</SideBar.MenuButton>
					</SideBar.MenuItem>
				</SideBar.Menu>
			</SideBar.GroupContent>
		</SideBar.Group>

		<!-- Conversations section -->
		<SideBar.Group class="flex min-h-0 flex-1 flex-col pr-1">
			<!-- todo: add var for sky blue -->
			<SideBar.GroupLabel class="text-sidebar-secondary text-xs font-medium"
				>Conversations</SideBar.GroupLabel
			>

			<SideBar.GroupContent class="min-h-0 flex-1">
				<ScrollArea.Root class="h-full pr-3" type="always">
					{#if conversations}
						<SideBar.Menu>
							{#each conversations as conversation (conversation.id)}
								<Collapsible.Root
									open={path.includes(conversation.id)}
									class="group/collapsible"
								>
									<SideBar.MenuItem class="">
										<Collapsible.Trigger>
											{#snippet child({ props: triggerProps })}
												<SideBar.MenuButton
													class=" text-sidebar-primary-foreground data-[active=true]:bg-mutted data-[active=true]:text-sidebar-accent-foreground h-8 w-full overflow-hidden rounded-lg p-2"
													isActive={path.includes(conversation.id)}
													{...triggerProps}
												>
													{#snippet child({ props: btnProps })}
														<a
															{...btnProps}
															href={`/admin/conversations/${conversation.id}/configure`}
															class="group-data-[state=open]/collapsible:bg-sidebar-accent group-data-[state=open]/collapsible:text-sidebar-accent-foreground active:text-sidebar-accent-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground flex w-full items-center rounded-lg px-2 py-1.5"
														>
															{#if conversation.title.length > 29}
																<Tooltip.Root>
																	<Tooltip.Trigger>
																		{#snippet child({
																			props: tipProps
																		})}
																			<span
																				{...tipProps}
																				class="flex-1 truncate text-left text-sm font-medium leading-4"
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
																	class="flex-1 truncate text-left text-sm font-medium leading-4"
																>
																	{conversation.title}
																</span>
															{/if}
															<ChevronRight
																class="size-4 shrink-0 transition-transform group-data-[state=open]/collapsible:rotate-90"
															/>
														</a>
													{/snippet}
												</SideBar.MenuButton>
											{/snippet}
										</Collapsible.Trigger>
										<Collapsible.Content>
											<div class="bg-sidebar-active-bg mt-1 rounded-lg p-1">
												{#each conversationSteps as step (step.path)}
													{#if step.path === 'design'}
														<Collapsible.Root
															open={path.includes('design')}
															class="group/design"
														>
															<SideBar.MenuSub class="w-full">
																<SideBar.MenuSubItem>
																	<Collapsible.Trigger
																		class="w-full"
																	>
																		<SideBar.MenuSubButton
																			href={`/admin/conversations/${conversation.id}/design`}
																			class="
																						stroke-sidebar-foreground 
																						{path.includes('design') ? 'font-bold' : ''} hover:text-sidebar-accent-foreground"
																			aria-disabled={shouldActivateStep(
																				conversation.isLive,
																				step
																			)}
																		>
																			<step.icon
																				class="stroke-sidebar-foreground size-4 shrink-0"
																			/>
																			<span
																				class="flex-1 truncate text-left"
																				>{step.name}</span
																			>
																			<ChevronRight
																				class="stroke-sidebar-foreground size-4 shrink-0 transition-transform group-data-[state=open]/design:rotate-90"
																			/>
																		</SideBar.MenuSubButton>
																	</Collapsible.Trigger>
																</SideBar.MenuSubItem>
															</SideBar.MenuSub>
															<Collapsible.Content class="pl-4">
																<div
																	class="border-sidebar-foreground relative ml-6 mr-2 border-l py-0.5 pl-2"
																>
																	{#if path.includes(conversation.id) && workflowSteps?.length > 0}
																		{#each workflowSteps as wfStep (wfStep.id)}
																			<a
																				href={!shouldActivateStep(
																					conversation.isLive,
																					step
																				)
																					? `/admin/conversations/${conversation.id}/design/step/${wfStep.id}`
																					: ''}
																				aria-disabled={shouldActivateStep(
																					conversation.isLive,
																					step
																				)}
																				class="text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground block truncate rounded-lg px-2 py-1 text-sm disabled:pointer-events-none disabled:opacity-50"
																				class:font-bold={path.includes(
																					wfStep.id
																				)}
																				class:opacity-50={shouldActivateStep(
																					conversation.isLive,
																					step
																				)}
																			>
																				{wfStep.name}
																			</a>
																		{/each}
																	{/if}
																	<a
																		href={!shouldActivateStep(
																			conversation.isLive,
																			step
																		)
																			? `/admin/conversations/${conversation.id}/design?addStep=true`
																			: ''}
																		aria-disabled={shouldActivateStep(
																			conversation.isLive,
																			step
																		)}
																		class:opacity-50={shouldActivateStep(
																			conversation.isLive,
																			step
																		)}
																		class="text-sidebar-foreground/40 hover:bg-sidebar-accent hover:text-sidebar-accent-foreground block rounded-lg px-2 py-1 text-sm"
																	>
																		+ Add new
																	</a>
																</div>
															</Collapsible.Content>
														</Collapsible.Root>
													{:else}
														<SideBar.MenuSub class="w-full">
															<SideBar.MenuSubItem>
																<SideBar.MenuSubButton
																	href={`/admin/conversations/${conversation.id}/${step.path}`}
																	aria-disabled={shouldActivateStep(
																		conversation.isLive,
																		step
																	)}
																	class="{path.includes(step.path)
																		? 'font-bold'
																		: ''} hover:text-sidebar-accent-foreground"
																>
																	<step.icon
																		class="stroke-sidebar-foreground size-4 shrink-0"
																	/>
																	<span class="truncate"
																		>{step.name}</span
																	>
																</SideBar.MenuSubButton>
															</SideBar.MenuSubItem>
														</SideBar.MenuSub>
													{/if}
												{/each}
											</div>
										</Collapsible.Content>
									</SideBar.MenuItem>
								</Collapsible.Root>
							{/each}
						</SideBar.Menu>
					{/if}
				</ScrollArea.Root>
			</SideBar.GroupContent>

			<div class="shrink-0 p-2">
				<Button href="/admin/conversations/new" class="w-full" variant="default">
					<Plus class="size-4" />
					New conversation
				</Button>
			</div>
		</SideBar.Group>
	</SideBar.Content>

	<SideBar.Footer>
		<div class="flex flex-col items-center gap-2 p-2">
			<Avatar.Root class="h-12 w-12">
				{#if user?.avatarUrl}
					<Avatar.Image src={user.avatarUrl} alt={user.username} />
				{/if}
				<Avatar.Fallback>{user_initials}</Avatar.Fallback>
			</Avatar.Root>
			<div class="flex w-full flex-col items-center gap-0.5">
				<span
					class="text-sidebar-primary-foreground w-full truncate text-center text-sm font-semibold"
				>
					{user?.username ?? ''}
				</span>
				{#if user?.email}
					<span
						class="text-sidebar-primary-foreground/70 w-full truncate text-center text-xs"
					>
						{user.email}
					</span>
				{/if}
			</div>
		</div>
		<SideBar.Menu>
			<SideBar.MenuItem>
				<SideBar.MenuButton>
					{#snippet child({ props: btnProps })}
						<a {...btnProps} href="/admin/settings">
							<Settings class="size-4" />
							Settings
						</a>
					{/snippet}
				</SideBar.MenuButton>
			</SideBar.MenuItem>
			<SideBar.MenuItem>
				<SideBar.MenuButton>
					{#snippet child({ props: btnProps })}
						<a {...btnProps} href="/admin/about">
							<Info class="size-4" />
							About Comhairle
						</a>
					{/snippet}
				</SideBar.MenuButton>
			</SideBar.MenuItem>
		</SideBar.Menu>
	</SideBar.Footer>
	<SideBar.Rail />
</SideBar.Root>
