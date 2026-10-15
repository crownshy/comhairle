<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { ArrowUpRight, Check, ChevronDown, Lock, Rocket } from 'lucide-svelte';
	import type { Snippet } from 'svelte';
	import { resolve } from '$app/paths';
	import { permissions } from '$lib/permissions.svelte';
	import {
		conversationStages,
		sectionLockLabels,
		type ConversationStage,
		type SectionLink,
		type StageConversation
	} from './conversationStages';
	import { Button } from '$lib/components/ui/button';
	import {
		CONVERSATION_STATUS_LABELS,
		conversationStatus,
		type ConversationStatus
	} from '$lib/utils/conversationStatus';

	type Props = {
		conversation: StageConversation;
		stepCount: number;
		/** Whether any workflow step is a Polis step, which adds Moderate to Engage. */
		hasPolisStep: boolean;
		path: string;
	};
	let { conversation, stepCount, hasPolisStep, path }: Props = $props();

	let status = $derived(conversationStatus(conversation));
	const statusDotClass: Record<ConversationStatus, string> = {
		draft: 'bg-orange-400',
		live: 'bg-emerald-400',
		completed: 'bg-sidebar-foreground/40'
	};

	let canAdmin = $derived(permissions.can('conversation', 'conversation_admin', conversation.id));
	let stages = $derived(
		conversationStages({ conversation, pathname: path, canAdmin, hasPolisStep })
	);
	// Collapsed view: every link in order, milestones (Launch, Close) included.
	let sections = $derived(
		stages.flatMap((stage) => (stage.link ? [stage.link] : stage.sections))
	);

	// Engage and Results fold, and so does Set up once launched. Locked stages and a
	// launched Set up start folded, but admins can still open them to look or make changes.
	// A stage holding the page you're on always starts open.
	function isFoldable(stage: ConversationStage): boolean {
		return (
			stage.id === 'engage' ||
			stage.id === 'results' ||
			(stage.id === 'set-up' && conversation.isLive)
		);
	}
	/** Set up after launch: still editable, but marked locked so changes feel deliberate. */
	function isFrozen(stage: ConversationStage): boolean {
		return stage.id === 'set-up' && conversation.isLive;
	}
	let folded = $state<Partial<Record<ConversationStage['id'], boolean>>>({});
	function isOpen(stage: ConversationStage): boolean {
		if (!isFoldable(stage)) return true;
		const startsFolded =
			(stage.state === 'locked' || isFrozen(stage)) &&
			!stage.sections.some((link) => link.active);
		return !(folded[stage.id] ?? startsFolded);
	}
	function toggle(stage: ConversationStage) {
		folded[stage.id] = isOpen(stage);
	}

	// Never draw sidebar text on --sidebar-accent: in some themes (scot-gov) it is white.
	const markerClass: Record<ConversationStage['state'], string> = {
		done: 'bg-primary text-primary-foreground',
		current: 'bg-primary text-primary-foreground',
		locked: 'border-sidebar-foreground/40 text-sidebar-foreground/70 border'
	};
	const rowHover = 'hover:bg-sidebar-foreground/10';
	const rowActive = 'bg-sidebar-accent text-sidebar-accent-foreground font-medium';
</script>

<!-- Section hrefs come from conversationStages, built from the conversation id. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<div class="-mt-3 flex flex-col pr-4 group-data-[collapsible=icon]:hidden">
	<div class="flex h-9 items-center justify-between gap-3">
		<span class="text-sidebar-foreground/80 inline-flex items-center gap-2 text-sm">
			<span class="size-2 shrink-0 rounded-full {statusDotClass[status]}" aria-hidden="true"
			></span>
			{CONVERSATION_STATUS_LABELS[status]}
		</span>
		{#if status === 'draft'}
			<Button
				href={resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
					conversation_id: conversation.id,
					preview: 'preview'
				})}
				target="_blank"
				variant="secondary"
				class="bg-primary/20 text-sidebar-foreground hover:bg-primary/30 inline-flex h-9 rounded-full px-4 text-sm"
			>
				Preview
				<ArrowUpRight class="size-4" />
			</Button>
		{/if}
	</div>
</div>

{#snippet stageHeading(stage: ConversationStage, index: number, active = false)}
	<span
		class="flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold {active &&
		stage.state !== 'done'
			? 'border-sidebar-accent-foreground/40 text-sidebar-accent-foreground border'
			: stage.id === 'launch' && stage.state !== 'done'
				? markerClass.locked
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
		class="text-base font-semibold {active
			? 'text-sidebar-accent-foreground'
			: stage.state === 'locked'
				? 'text-sidebar-foreground/60'
				: 'text-sidebar-foreground'}"
	>
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
					class="-ml-1 flex h-10 items-center gap-3 rounded-lg pr-2 pl-1 {active
						? rowActive
						: rowHover}"
				>
					{@render stageHeading(stage, index, active)}
				</a>
			{:else if stage.link?.lock}
				{#snippet milestoneBody()}
					{@render stageHeading(stage, index)}
				{/snippet}
				{@render lockedRow(
					stage.link.lock,
					'text-sidebar-foreground/40 -ml-1 flex h-10 cursor-not-allowed items-center gap-3 rounded-lg pr-2 pl-1',
					milestoneBody
				)}
			{:else if isFoldable(stage)}
				<button
					type="button"
					aria-expanded={open}
					aria-controls="stage-{stage.id}"
					onclick={() => toggle(stage)}
					class="-ml-1 flex h-10 items-center gap-3 rounded-lg pr-2 pl-1 text-left {rowHover}"
				>
					{@render stageHeading(stage, index)}
					{#if isFrozen(stage)}
						<Lock
							class="text-sidebar-foreground/60 ml-auto size-3.5 shrink-0"
							aria-hidden="true"
						/>
						<span class="sr-only">Locked after launch, still editable</span>
					{/if}
					<ChevronDown
						class="text-sidebar-foreground/70 size-4 shrink-0 transition-transform {isFrozen(
							stage
						)
							? ''
							: 'ml-auto'} {open ? '' : '-rotate-90'}"
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
					class="border-sidebar-border ml-3.5 flex flex-col gap-0.5 border-l pl-2"
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
									class="flex h-10 items-center gap-2 rounded-lg px-2 text-base {link.active
										? rowActive
										: `text-sidebar-foreground/80 hover:text-sidebar-foreground ${rowHover}`}"
								>
									<link.icon class="size-4 shrink-0" />
									<span class="flex-1 truncate">{link.name}</span>
									{#if link.path === 'design'}
										<span
											class="shrink-0 rounded-full px-2 text-sm {link.active
												? 'bg-sidebar-accent-foreground/10 text-sidebar-accent-foreground'
												: 'bg-sidebar-foreground/15 text-sidebar-foreground'}"
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
