<script lang="ts">
	// PROTOTYPE variant "drill-in". Outside a conversation: workspace nav. Inside one: the
	// sidebar is replaced by that conversation's sections (GitHub repo / Vercel project pattern),
	// with a back link to leave. Workflow steps nest under Process design.
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { page } from '$app/state';
	import { ArrowLeft, ChevronRight, Lock, Plus } from 'lucide-svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import SidebarResizeHandle from '$lib/components/SidebarResizeHandle.svelte';
	import type { UserDto } from '@crownshy/api-client/api';
	import CollapseToggle from './CollapseToggle.svelte';
	import UserMenu from './UserMenu.svelte';
	import WorkspaceNav from './WorkspaceNav.svelte';
	import { conversationStatus, sectionLinks, type ConversationLike, type StepLike } from './nav';

	let { user }: { user: UserDto } = $props();

	let conversation = $derived(page.data.conversation as ConversationLike | undefined);
	let steps = $derived(
		[...((page.data.workflowSteps as StepLike[] | undefined) ?? [])].sort(
			(a, b) => a.stepOrder - b.stepOrder
		)
	);
	let links = $derived(conversation ? sectionLinks(conversation, page.url.pathname) : []);
	let status = $derived(conversation ? conversationStatus(conversation) : null);
</script>

<SideBar.Root collapsible="icon">
	<SideBar.Header class="gap-3 p-3">
		<div class="flex items-center justify-between gap-2">
			<div class="pl-2 group-data-[collapsible=icon]:hidden">
				<ComhairleLogo color="sidebar-foreground" logoSize="sm" />
			</div>
			<CollapseToggle />
		</div>

		{#if conversation}
			<SideBar.Menu>
				<SideBar.MenuItem>
					<SideBar.MenuButton tooltipContent="All conversations">
						{#snippet child({ props })}
							<a href="/admin" {...props}>
								<ArrowLeft />
								<span>All conversations</span>
							</a>
						{/snippet}
					</SideBar.MenuButton>
				</SideBar.MenuItem>
			</SideBar.Menu>
			<div class="flex flex-col gap-1.5 px-2 group-data-[collapsible=icon]:hidden">
				<span
					class="text-sidebar-foreground line-clamp-2 text-base leading-snug font-semibold"
				>
					{conversation.title}
				</span>
				<span
					class="flex w-fit items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-medium {status ===
					'Live'
						? 'bg-primary text-primary-foreground'
						: 'bg-sidebar-accent text-sidebar-accent-foreground'}"
				>
					<span class="size-1.5 rounded-full bg-current"></span>
					{status}
				</span>
			</div>
		{/if}
	</SideBar.Header>

	<SideBar.Content>
		{#if conversation}
			<SideBar.Group>
				<SideBar.Menu>
					{#each links as link (link.path)}
						<SideBar.MenuItem>
							{#if link.path === 'design'}
								<Collapsible.Root open class="group/collapsible">
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
									<Collapsible.Trigger>
										{#snippet child({ props })}
											<SideBar.MenuAction
												{...props}
												class="data-[state=open]:rotate-90"
											>
												<ChevronRight />
												<span class="sr-only">Toggle steps</span>
											</SideBar.MenuAction>
										{/snippet}
									</Collapsible.Trigger>
									<Collapsible.Content>
										<SideBar.MenuSub>
											{#each steps as step (step.id)}
												{@const stepHref = `${link.href}/step/${step.id}`}
												<SideBar.MenuSubItem>
													<SideBar.MenuSubButton
														href={stepHref}
														isActive={page.url.pathname.startsWith(
															stepHref
														)}
													>
														<span>{step.name || 'Unnamed step'}</span>
													</SideBar.MenuSubButton>
												</SideBar.MenuSubItem>
											{/each}
											<SideBar.MenuSubItem>
												<SideBar.MenuSubButton
													href={link.href}
													class="text-sidebar-foreground/60"
												>
													<Plus />
													<span>Add step</span>
												</SideBar.MenuSubButton>
											</SideBar.MenuSubItem>
										</SideBar.MenuSub>
									</Collapsible.Content>
								</Collapsible.Root>
							{:else if link.locked}
								<SideBar.MenuButton
									aria-disabled="true"
									class="text-sidebar-foreground/45 cursor-not-allowed"
									tooltipContent={`${link.label}: available after launch`}
									title="Available after launch"
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
		{:else}
			<WorkspaceNav />
			<div class="px-4 pt-2 group-data-[collapsible=icon]:px-2">
				<NewConversationButton
					class="w-full group-data-[collapsible=icon]:h-8 group-data-[collapsible=icon]:p-0"
					labelClass="group-data-[collapsible=icon]:hidden"
				/>
			</div>
		{/if}
	</SideBar.Content>

	<SideBar.Footer>
		<UserMenu {user} />
	</SideBar.Footer>
	<SidebarResizeHandle />
</SideBar.Root>
