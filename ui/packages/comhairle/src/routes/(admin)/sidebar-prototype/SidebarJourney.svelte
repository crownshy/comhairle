<script lang="ts">
	// PROTOTYPE variant "journey". Inside a conversation the nav reads as the conversation's
	// lifecycle: numbered stages with done / current / locked state and Launch as an explicit
	// milestone between setup and engagement (onboarding-checklist pattern). Outside a
	// conversation it falls back to the plain workspace nav.
	import * as SideBar from '$lib/components/ui/sidebar';
	import { page } from '$app/state';
	import { ArrowLeft, Check, Lock, Rocket } from 'lucide-svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import SidebarResizeHandle from '$lib/components/SidebarResizeHandle.svelte';
	import type { UserDto } from '@crownshy/api-client/api';
	import CollapseToggle from './CollapseToggle.svelte';
	import UserMenu from './UserMenu.svelte';
	import WorkspaceNav from './WorkspaceNav.svelte';
	import { sectionLinks, type ConversationLike, type SectionLink, type StepLike } from './nav';

	let { user }: { user: UserDto } = $props();

	type StageState = 'done' | 'current' | 'locked';

	let conversation = $derived(page.data.conversation as ConversationLike | undefined);
	let stepCount = $derived((page.data.workflowSteps as StepLike[] | undefined)?.length ?? 0);
	let links = $derived(conversation ? sectionLinks(conversation, page.url.pathname) : []);

	let stages = $derived.by(() => {
		if (!conversation) return [];
		const pick = (paths: string[]) =>
			paths
				.map((path) => links.find((link) => link.path === path))
				.filter((link): link is SectionLink => !!link);
		const live = conversation.isLive;
		const closed = conversation.isComplete;
		return [
			{
				label: 'Set up',
				state: (live ? 'done' : 'current') as StageState,
				links: pick(['configure', 'design', 'learning-assistant'])
			},
			{
				label: live ? 'Launched' : 'Launch',
				state: (live ? 'done' : 'current') as StageState,
				milestone: true,
				links: []
			},
			{
				label: 'Engage',
				state: (!live ? 'locked' : closed ? 'done' : 'current') as StageState,
				links: pick(['events', 'invites', 'notifications'])
			},
			{
				label: 'Learn',
				state: (!live ? 'locked' : 'current') as StageState,
				links: pick(['monitor', 'report'])
			}
		];
	});
	let doneCount = $derived(stages.filter((stage) => stage.state === 'done').length);
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
			<a
				href="/admin"
				class="text-sidebar-foreground/70 hover:text-sidebar-foreground flex items-center gap-1.5 px-2 text-sm group-data-[collapsible=icon]:hidden"
			>
				<ArrowLeft class="size-4" /> Conversations
			</a>
			<div class="flex flex-col gap-2 px-2 group-data-[collapsible=icon]:hidden">
				<span
					class="text-sidebar-foreground line-clamp-2 text-base leading-snug font-semibold"
				>
					{conversation.title}
				</span>
				<div class="bg-sidebar-accent h-1.5 overflow-hidden rounded-full">
					<div
						class="bg-primary h-full rounded-full transition-all"
						style="width: {(doneCount / stages.length) * 100}%"
					></div>
				</div>
				<span class="text-sidebar-foreground/70 text-xs">
					{doneCount} of {stages.length} stages done
				</span>
			</div>
		{/if}
	</SideBar.Header>

	<SideBar.Content>
		{#if conversation}
			<ol class="flex flex-col px-3 py-2 group-data-[collapsible=icon]:hidden">
				{#each stages as stage, index (stage.label)}
					<li class="relative flex gap-3 pb-5 last:pb-0">
						{#if index < stages.length - 1}
							<span
								class="absolute top-7 bottom-0 left-3.5 w-px {stage.state === 'done'
									? 'bg-primary'
									: 'bg-sidebar-border'}"
								aria-hidden="true"
							></span>
						{/if}
						<span
							class="relative z-10 flex size-7 shrink-0 items-center justify-center rounded-full text-xs font-semibold {stage.state ===
							'done'
								? 'bg-primary text-primary-foreground'
								: stage.state === 'current'
									? 'border-primary text-primary bg-sidebar border-2'
									: 'bg-sidebar-accent text-sidebar-foreground/50'}"
						>
							{#if stage.state === 'done'}
								<Check class="size-4" strokeWidth={3} />
							{:else if stage.state === 'locked'}
								<Lock class="size-3.5" />
							{:else if stage.milestone}
								<Rocket class="size-3.5" />
							{:else}
								{index + 1}
							{/if}
						</span>
						<div class="flex min-w-0 flex-1 flex-col gap-1 pt-0.5">
							<span
								class="text-sm font-semibold {stage.state === 'locked'
									? 'text-sidebar-foreground/50'
									: 'text-sidebar-foreground'}"
							>
								{stage.label}
							</span>
							{#if stage.milestone && stage.state !== 'done'}
								<span class="text-sidebar-foreground/70 text-sm">
									Use Launch conversation in the header when setup is ready.
								</span>
							{/if}
							{#if stage.state === 'locked'}
								<span class="text-sidebar-foreground/50 text-xs"
									>Opens after launch</span
								>
							{/if}
							{#each stage.links as link (link.path)}
								{#if link.locked}
									<span
										class="text-sidebar-foreground/40 flex h-8 items-center gap-2 rounded-lg px-2 text-sm"
									>
										<link.icon class="size-4" />
										{link.label}
									</span>
								{:else}
									<a
										href={link.href}
										aria-current={link.active ? 'page' : undefined}
										class="flex h-8 items-center gap-2 rounded-lg px-2 text-sm {link.active
											? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium'
											: 'text-sidebar-foreground/80 hover:bg-sidebar-accent/60'}"
									>
										<link.icon class="size-4" />
										<span class="flex-1">{link.label}</span>
										{#if link.path === 'design'}
											<span class="text-sidebar-foreground/60 text-xs">
												{stepCount} steps
											</span>
										{/if}
									</a>
								{/if}
							{/each}
						</div>
					</li>
				{/each}
			</ol>
			<!-- Collapsed icon mode: a flat icon list. -->
			<SideBar.Group class="hidden group-data-[collapsible=icon]:flex">
				<SideBar.Menu>
					{#each links as link (link.path)}
						<SideBar.MenuItem>
							<SideBar.MenuButton
								isActive={link.active}
								aria-disabled={link.locked}
								tooltipContent={link.label}
							>
								{#snippet child({ props })}
									<a href={link.href} {...props}><link.icon /></a>
								{/snippet}
							</SideBar.MenuButton>
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
