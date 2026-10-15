<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Check, Lock, Rocket } from 'lucide-svelte';
	import { permissions } from '$lib/permissions.svelte';
	import {
		conversationStages,
		sectionLockLabels,
		type ConversationStage,
		type StageConversation
	} from './conversationStages';

	type Props = { conversation: StageConversation; stepCount: number; path: string };
	let { conversation, stepCount, path }: Props = $props();

	let canAdmin = $derived(permissions.can('conversation', 'conversation_admin', conversation.id));
	let stages = $derived(conversationStages({ conversation, pathname: path, canAdmin }));
	let sections = $derived(stages.flatMap((stage) => stage.sections));

	const markerClass: Record<ConversationStage['state'], string> = {
		done: 'bg-primary text-primary-foreground',
		current: 'bg-primary text-primary-foreground',
		locked: 'bg-sidebar-foreground/10 text-sidebar-foreground/60'
	};
</script>

<!-- Section hrefs come from conversationStages, built from the conversation id. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<ol class="flex flex-col gap-5 pr-4 group-data-[collapsible=icon]:hidden">
	{#each stages as stage, index (stage.id)}
		<li class="flex flex-col gap-2">
			<div class="flex items-center gap-3">
				<span
					class="flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold {stage.id ===
						'launch' && stage.state !== 'done'
						? 'bg-sidebar-foreground/10 text-sidebar-foreground'
						: markerClass[stage.state]}"
					aria-hidden="true"
				>
					{#if stage.state === 'done'}
						<Check class="size-4" strokeWidth={3} />
					{:else if stage.id === 'launch'}
						<Rocket class="size-3.5" />
					{:else}
						{index + 1}
					{/if}
				</span>
				<span
					class="text-base font-semibold {stage.state === 'locked'
						? 'text-sidebar-foreground/60'
						: 'text-sidebar-foreground'}"
				>
					{stage.label}
					{#if stage.state === 'done'}<span class="sr-only">, done</span>{/if}
				</span>
			</div>

			{#if stage.id === 'launch' && stage.state !== 'done'}
				<p class="text-sidebar-foreground/70 pl-10 text-sm">
					Use Launch conversation in the header when setup is ready.
				</p>
			{/if}

			{#if stage.sections.length > 0}
				<ul class="border-sidebar-foreground/20 ml-3.5 flex flex-col gap-0.5 border-l pl-2">
					{#each stage.sections as link (link.path)}
						<li>
							{#if link.lock}
								{@const lockLabel = sectionLockLabels[link.lock]}
								<Tooltip.Root>
									<Tooltip.Trigger>
										{#snippet child({ props })}
											<span
												{...props}
												tabindex="0"
												aria-disabled="true"
												class="text-sidebar-foreground/40 flex h-10 cursor-not-allowed items-center gap-2 rounded-lg px-2 text-base"
											>
												<link.icon class="size-4 shrink-0" />
												<span class="flex-1 truncate">{link.name}</span>
												<Lock
													class="size-3.5 shrink-0"
													aria-hidden="true"
												/>
												<span class="sr-only">{lockLabel}</span>
											</span>
										{/snippet}
									</Tooltip.Trigger>
									<Tooltip.Content
										side="top"
										align="start"
										sideOffset={4}
										class="bg-popover text-popover-foreground border-border border text-sm"
										arrowClasses="bg-popover"
									>
										{lockLabel}
									</Tooltip.Content>
								</Tooltip.Root>
							{:else}
								<a
									href={link.href}
									aria-current={link.active ? 'page' : undefined}
									class="flex h-10 items-center gap-2 rounded-lg px-2 text-base {link.active
										? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium'
										: 'text-sidebar-foreground/80 hover:bg-sidebar-foreground/10 hover:text-sidebar-foreground'}"
								>
									<link.icon class="size-4 shrink-0" />
									<span class="flex-1 truncate">{link.name}</span>
									{#if link.path === 'design'}
										<span
											class="shrink-0 rounded-full bg-current/10 px-2 text-sm"
										>
											{stepCount}<span class="sr-only">
												{stepCount === 1 ? 'step' : 'steps'}</span
											>
										</span>
									{/if}
								</a>
							{/if}
						</li>
					{/each}
				</ul>
			{/if}
		</li>
	{/each}
</ol>

<!-- Collapsed: one icon per section, in the same order. -->
<SideBar.Menu class="hidden items-center group-data-[collapsible=icon]:flex">
	{#each sections as link (link.path)}
		<SideBar.MenuItem>
			<SideBar.MenuButton
				isActive={link.active}
				tooltipContent={link.lock
					? `${link.name} (${sectionLockLabels[link.lock]})`
					: link.name}
				aria-disabled={link.lock ? 'true' : undefined}
			>
				{#snippet child({ props })}
					<a {...props} href={link.lock ? undefined : link.href}>
						<link.icon />
						<span class="sr-only">{link.name}</span>
					</a>
				{/snippet}
			</SideBar.MenuButton>
		</SideBar.MenuItem>
	{/each}
</SideBar.Menu>
