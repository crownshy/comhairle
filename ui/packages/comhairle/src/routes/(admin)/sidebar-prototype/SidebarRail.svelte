<script lang="ts">
	// PROTOTYPE variant "rail". Two tiers (Figma / Supabase / Discord pattern): a thin icon rail of
	// workspace destinations that never changes, plus a panel that only exists inside a
	// conversation. Desktop only; no mobile handling in this sketch.
	import * as Tooltip from '$lib/components/ui/tooltip';
	import * as Avatar from '$lib/components/ui/avatar';
	import { page } from '$app/state';
	import { ArrowUpRight, Info, Lock, Plus, Settings } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';
	import type { UserDto } from '@crownshy/api-client/api';
	import { userInitials } from '$lib/utils';
	import comhairleMark from '$lib/assets/comhairle_logo.svg';
	import {
		conversationStatus,
		groupByPhase,
		isNavActive,
		libraryNav,
		sectionLinks,
		workspaceNav,
		type ConversationLike,
		type StepLike
	} from './nav';

	let { user }: { user: UserDto } = $props();

	let pathname = $derived(page.url.pathname);
	let conversation = $derived(page.data.conversation as ConversationLike | undefined);
	let steps = $derived(
		[...((page.data.workflowSteps as StepLike[] | undefined) ?? [])].sort(
			(a, b) => a.stepOrder - b.stepOrder
		)
	);
	let phaseGroups = $derived(
		conversation ? groupByPhase(sectionLinks(conversation, pathname)) : []
	);

	// Inside a conversation the Conversations rail icon stays lit, since that is where you are.
	function railActive(href: string) {
		if (href === '/admin' && pathname.startsWith('/admin/conversations/')) return true;
		return isNavActive(href, pathname);
	}
</script>

{#snippet railLink(label: string, href: string, Icon: ComponentType, active: boolean)}
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				<a
					{...props}
					{href}
					aria-label={label}
					aria-current={active ? 'page' : undefined}
					class="flex size-10 items-center justify-center rounded-xl transition-colors {active
						? 'bg-sidebar-accent text-sidebar-accent-foreground'
						: 'text-sidebar-foreground/70 hover:bg-sidebar-accent/60 hover:text-sidebar-foreground'}"
				>
					<Icon class="size-5" />
				</a>
			{/snippet}
		</Tooltip.Trigger>
		<Tooltip.Content side="right">{label}</Tooltip.Content>
	</Tooltip.Root>
{/snippet}

<div class="hidden h-svh shrink-0 md:flex">
	<nav
		class="bg-sidebar border-sidebar-border flex w-16 flex-col items-center gap-1 border-r py-3"
		aria-label="Workspace"
	>
		<a
			href="/admin"
			class="mb-3 flex size-10 items-center justify-center"
			aria-label="Comhairle"
		>
			<img src={comhairleMark} alt="" class="size-8" />
		</a>
		{#each workspaceNav as item (item.href)}
			{@render railLink(item.label, item.href, item.icon, railActive(item.href))}
		{/each}
		<div class="bg-sidebar-border my-2 h-px w-8"></div>
		{#each libraryNav as item (item.href)}
			{@render railLink(item.label, item.href, item.icon, railActive(item.href))}
		{/each}

		<div class="mt-auto flex flex-col items-center gap-1">
			{@render railLink('About Comhairle', '/about', Info, false)}
			{@render railLink('Settings', '/settings', Settings, pathname.startsWith('/settings'))}
			<Avatar.Root class="mt-2 size-9" title={user?.username}>
				{#if user?.avatarUrl}
					<Avatar.Image src={user.avatarUrl} alt={user.username} />
				{/if}
				<Avatar.Fallback>{userInitials(user?.username ?? '')}</Avatar.Fallback>
			</Avatar.Root>
		</div>
	</nav>

	{#if conversation}
		<nav
			class="bg-sidebar border-sidebar-border flex w-64 flex-col border-r"
			aria-label="Conversation"
		>
			<div class="border-sidebar-border flex flex-col gap-2 border-b p-4">
				<span
					class="text-sidebar-foreground/60 text-xs font-medium tracking-wide uppercase"
				>
					Conversation
				</span>
				<span
					class="text-sidebar-foreground line-clamp-2 text-base leading-snug font-semibold"
				>
					{conversation.title}
				</span>
				<div class="flex items-center justify-between">
					<span class="text-sidebar-foreground/70 flex items-center gap-1.5 text-sm">
						<span
							class="size-2 rounded-full {conversation.isLive &&
							!conversation.isComplete
								? 'bg-primary'
								: 'bg-sidebar-foreground/30'}"
						></span>
						{conversationStatus(conversation)}
					</span>
					<a
						href={`/conversations/${conversation.id}/preview`}
						target="_blank"
						class="text-sidebar-foreground/70 hover:text-sidebar-foreground flex items-center gap-1 text-sm"
					>
						Preview <ArrowUpRight class="size-3.5" />
					</a>
				</div>
			</div>

			<div class="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-3">
				{#each phaseGroups as phase (phase.id)}
					<div class="flex flex-col gap-0.5">
						<span class="text-sidebar-foreground/60 px-2 pb-1 text-xs font-medium">
							{phase.label}
						</span>
						{#each phase.links as link (link.path)}
							{#if link.locked}
								<span
									class="text-sidebar-foreground/40 flex h-9 items-center gap-2.5 rounded-lg px-2 text-sm"
									title="Available after launch"
								>
									<link.icon class="size-4" />
									<span class="flex-1">{link.label}</span>
									<Lock class="size-3.5" />
								</span>
							{:else}
								<a
									href={link.href}
									aria-current={link.active ? 'page' : undefined}
									class="flex h-9 items-center gap-2.5 rounded-lg px-2 text-sm font-medium {link.active
										? 'bg-sidebar-accent text-sidebar-accent-foreground'
										: 'text-sidebar-foreground/80 hover:bg-sidebar-accent/60'}"
								>
									<link.icon class="size-4" />
									{link.label}
								</a>
								{#if link.path === 'design' && link.active}
									<div
										class="border-sidebar-border ml-4 flex flex-col border-l pl-2"
									>
										{#each steps as step (step.id)}
											{@const stepHref = `${link.href}/step/${step.id}`}
											<a
												href={stepHref}
												class="truncate rounded-md px-2 py-1.5 text-sm {pathname.startsWith(
													stepHref
												)
													? 'text-sidebar-accent-foreground font-semibold'
													: 'text-sidebar-foreground/70 hover:text-sidebar-foreground'}"
											>
												{step.name || 'Unnamed step'}
											</a>
										{/each}
										<a
											href={link.href}
											class="text-sidebar-foreground/50 hover:text-sidebar-foreground flex items-center gap-1 px-2 py-1.5 text-sm"
										>
											<Plus class="size-3.5" /> Add step
										</a>
									</div>
								{/if}
							{/if}
						{/each}
					</div>
				{/each}
			</div>
		</nav>
	{/if}
</div>
