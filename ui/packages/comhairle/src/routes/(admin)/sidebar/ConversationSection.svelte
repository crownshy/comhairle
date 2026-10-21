<script lang="ts">
	import { resolve } from '$app/paths';
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Check, ChevronDown, Eye, Lock, Rocket } from 'lucide-svelte';
	import type { Snippet } from 'svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { cn } from '$lib/utils';
	import {
		conversationStages,
		sectionLockLabels,
		type ConversationStage,
		type SectionLink,
		type StageConversation
	} from './conversationStages';
	import { LINK_ROW, LINK_ROW_IDLE, ROW_ACTIVE, ROW_HOVER } from './sidebarRows';

	type Props = {
		conversation: StageConversation;
		stepCount: number;
		/** Adds Moderate to Engage. */
		hasPolisStep: boolean;
		path: string;
	};
	let { conversation, stepCount, hasPolisStep, path }: Props = $props();

	let previewHref = $derived(
		resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
			conversation_id: conversation.id,
			preview: 'preview'
		})
	);
	let canAdmin = $derived(permissions.can('conversation', 'conversation_admin', conversation.id));
	let stages = $derived(
		conversationStages({ conversation, pathname: path, canAdmin, hasPolisStep })
	);
	// Collapsed view: every link in order, milestones (Launch, Close) included.
	let sections = $derived(
		stages.flatMap((stage) => (stage.link ? [stage.link] : stage.sections))
	);
	let lastSetUpSection = $derived(stages.find((stage) => stage.id === 'set-up')?.sections.at(-1));

	// Engage and Results fold. Locked stages start folded, but admins can still open them to
	// look. Set up stays open and editable after launch (ADR-0001). A stage holding the page
	// you're on always starts open.
	function isFoldable(stage: ConversationStage): boolean {
		return stage.id === 'engage' || stage.id === 'results';
	}
	let folded = $state<Partial<Record<ConversationStage['id'], boolean>>>({});
	function isOpen(stage: ConversationStage): boolean {
		if (!isFoldable(stage)) return true;
		const startsFolded =
			stage.state === 'locked' && !stage.sections.some((link) => link.active);
		return !(folded[stage.id] ?? startsFolded);
	}
	function toggle(stage: ConversationStage) {
		folded[stage.id] = isOpen(stage);
	}

	const MARKER_CLASS: Record<ConversationStage['state'], string> = {
		done: 'bg-primary text-primary-foreground',
		current: 'bg-primary text-primary-foreground',
		locked: 'border-sidebar-foreground/40 text-sidebar-foreground/70 border'
	};
	const ACTIVE_MARKER_CLASS =
		'border-sidebar-accent-foreground/40 text-sidebar-accent-foreground border';
	const MILESTONE_ROW = '-ml-1 flex h-10 items-center gap-3 rounded-lg pr-2 pl-1';

	// The Launch marker stays an outlined rocket until the conversation is launched.
	function markerClass(stage: ConversationStage, active: boolean): string {
		if (stage.state === 'done') return MARKER_CLASS.done;
		if (active) return ACTIVE_MARKER_CLASS;
		if (stage.id === 'launch') return MARKER_CLASS.locked;
		return MARKER_CLASS[stage.state];
	}

	function labelClass(stage: ConversationStage, active: boolean): string {
		if (active) return 'text-sidebar-accent-foreground';
		if (stage.state === 'locked') return 'text-sidebar-foreground/60';
		return 'text-sidebar-foreground';
	}
</script>

<!-- Section hrefs come from conversationStages, built from the conversation id. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

{#snippet stageHeading(stage: ConversationStage, index: number, active = false)}
	<span
		class={cn(
			'flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold',
			markerClass(stage, active)
		)}
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
	<span class={cn('text-base font-semibold', labelClass(stage, active))}>
		{stage.label}
		{#if stage.state === 'done'}<span class="sr-only">, done</span>{/if}
	</span>
{/snippet}

{#snippet lockedRow(lock: NonNullable<SectionLink['lock']>, rowClass: string, body: Snippet)}
	{@const lockLabel = sectionLockLabels[lock]}
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				<span {...props} tabindex="0" aria-disabled="true" class={rowClass}>
					{@render body()}
					<Lock class="ml-auto size-3.5 shrink-0" aria-hidden="true" />
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
{/snippet}

<ol class="flex flex-col gap-5 pr-4 group-data-[collapsible=icon]:hidden">
	{#each stages as stage, index (stage.id)}
		{@const open = isOpen(stage)}
		<li class="flex flex-col gap-2">
			{#if stage.link && !stage.link.lock}
				{@const active = stage.link.active}
				<a
					href={stage.link.href}
					aria-current={active ? 'page' : undefined}
					class={cn(MILESTONE_ROW, active ? ROW_ACTIVE : ROW_HOVER)}
				>
					{@render stageHeading(stage, index, active)}
				</a>
			{:else if stage.link?.lock}
				{#snippet milestoneBody()}
					{@render stageHeading(stage, index)}
				{/snippet}
				{@render lockedRow(
					stage.link.lock,
					cn(MILESTONE_ROW, 'text-sidebar-foreground/40 cursor-not-allowed'),
					milestoneBody
				)}
			{:else if isFoldable(stage)}
				<button
					type="button"
					aria-expanded={open}
					aria-controls="stage-{stage.id}"
					onclick={() => toggle(stage)}
					class={cn(MILESTONE_ROW, 'text-left', ROW_HOVER)}
				>
					{@render stageHeading(stage, index)}
					<ChevronDown
						class={cn(
							'text-sidebar-foreground/70 ml-auto size-4 shrink-0 transition-transform',
							!open && '-rotate-90'
						)}
						aria-hidden="true"
					/>
				</button>
			{:else}
				<div class="flex items-center gap-3">
					{@render stageHeading(stage, index)}
				</div>
			{/if}

			{#if stage.sections.length > 0 && open}
				<ul
					id="stage-{stage.id}"
					class="border-sidebar-foreground/20 ml-3.5 flex flex-col gap-0.5 border-l pl-2"
				>
					{#each stage.sections as link (link.path)}
						<li>
							{#if link.lock}
								{#snippet sectionBody()}
									<link.icon class="size-4 shrink-0" />
									<span class="flex-1 truncate">{link.name}</span>
								{/snippet}
								{@render lockedRow(
									link.lock,
									'text-sidebar-foreground/40 flex h-10 cursor-not-allowed items-center gap-2 rounded-lg px-2 text-base',
									sectionBody
								)}
							{:else}
								<a
									href={link.href}
									aria-current={link.active ? 'page' : undefined}
									class={cn(
										LINK_ROW,
										link.active ? ROW_ACTIVE : [LINK_ROW_IDLE, ROW_HOVER]
									)}
								>
									<link.icon class="size-4 shrink-0" />
									<span class="flex-1 truncate">{link.name}</span>
									{#if link.path === 'design'}
										<span
											class={cn(
												'shrink-0 rounded-full px-2 text-sm',
												link.active
													? 'bg-sidebar-accent-foreground/10 text-sidebar-accent-foreground'
													: 'bg-sidebar-foreground/15 text-sidebar-foreground'
											)}
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
{#snippet previewIcon()}
	<SideBar.MenuItem>
		<SideBar.MenuButton tooltipContent="Preview">
			{#snippet child({ props })}
				<a {...props} href={previewHref} target="_blank">
					<Eye />
					<span class="sr-only">Preview</span>
				</a>
			{/snippet}
		</SideBar.MenuButton>
	</SideBar.MenuItem>
{/snippet}

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
		{#if link === lastSetUpSection}
			{@render previewIcon()}
		{/if}
	{/each}
</SideBar.Menu>
