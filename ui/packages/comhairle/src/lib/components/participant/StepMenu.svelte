<script lang="ts">
	/**
	 * Where you are in the conversation, where you can go, and the standing material that sits
	 * beside it. One list, two shells: an anchored menu on a pointer, a bottom sheet on a phone
	 * (ADR-0033).
	 *
	 * The rows are shared. Steps render through the `stepRow` snippet, and everything under it
	 * is uniform enough to be data, so neither shell restates the other's contents.
	 */
	import type { Component, ComponentType, SvelteComponent } from 'svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Drawer from '$lib/components/ui/drawer';
	import { ChevronDown, Check, Lock, Sparkles, Moon, Sun, X } from 'lucide-svelte';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import CircleQuestionMark from '$lib/components/icons/CircleQuestionMark.svelte';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';
	import type { StepItem } from './stepItems';
	import { legalDocs, type LegalDocId } from '$lib/components/rights/legalDocs';
	import type { SupportPanelTab } from './supportPanel.svelte';

	let {
		steps,
		currentIndex,
		label,
		onOpenLegal,
		onOpenSupport,
		assistantAvailable = false
	}: {
		steps: StepItem[];
		currentIndex: number;
		/** The trigger's text: the current step's name, prefixed with its position. */
		label: string;
		/** Opens a Your Rights document in place. Omitted, the legal section is hidden. */
		onOpenLegal?: (doc: LegalDocId) => void;
		/** Opens the support panel on a tab. Omitted, the support section is hidden. */
		onOpenSupport?: (tab: SupportPanelTab) => void;
		/** Whether this conversation has a Learning Assistant to offer. */
		assistantAvailable?: boolean;
	} = $props();

	const docs = legalDocs();
	const isMobile = new IsMobile();

	let open = $state(false);

	/** Positions count workflow steps only; the intro segment is not one of them. */
	let realSteps = $derived(steps.filter((s) => !s.isIntro));

	/**
	 * The count belongs in the heading, not on every row: repeating "Step N of M" down the
	 * list made the menu read as six variations on the same sentence. On the intro segment
	 * there is no position to state, so the heading falls back to its plain name.
	 */
	let heading = $derived.by(() => {
		const current = steps[currentIndex];
		if (!current || current.isIntro) return m.step_dropdown_heading();
		return m.step_position_label({
			current: realSteps.indexOf(current) + 1,
			total: realSteps.length
		});
	});

	/** Lucide's icons are still legacy class components; ours are Svelte 5 functions. */
	type IconComponent =
		| Component<{ class?: string }>
		| ComponentType<SvelteComponent<{ class?: string }>>;

	type MenuAction = {
		id: string;
		label: string;
		icon?: IconComponent;
		/** Leaves the menu open, for a row whose result you look at in place. */
		keepOpen?: boolean;
		/** Quieter than a step or a support row: the legal documents. */
		muted?: boolean;
		run: () => void;
	};

	let supportActions = $derived<MenuAction[]>(
		!onOpenSupport
			? []
			: [
					...(assistantAvailable
						? [
								{
									id: 'assistant',
									label: m.learning_assistant(),
									icon: Sparkles,
									run: () => onOpenSupport?.('learningAssistant')
								}
							]
						: []),
					{
						id: 'faqs',
						label: m.faq(),
						icon: CircleQuestionMark,
						run: () => onOpenSupport?.('faqs')
					}
				]
	);

	// Keep the menu open: the mode is a thing you look at, and closing the menu to show the
	// result means reopening it to change your mind.
	let themeAction = $derived<MenuAction>({
		id: 'theme',
		label: themeStore.isDark ? m.theme_light_mode() : m.theme_dark_mode(),
		icon: themeStore.isDark ? Sun : Moon,
		keepOpen: true,
		run: () => themeStore.toggleMode()
	});

	let legalActions = $derived<MenuAction[]>(
		!onOpenLegal
			? []
			: docs.map((entry) => ({
					id: entry.id,
					label: entry.label,
					muted: true,
					run: () => onOpenLegal?.(entry.id)
				}))
	);
</script>

{#snippet triggerInner()}
	<span class="truncate">{label}</span>
	<ChevronDown
		class="size-5 shrink-0 transition-transform duration-150 group-data-[state=open]:rotate-180"
		aria-hidden="true"
	/>
{/snippet}

{#snippet stepRow(step: StepItem, position: number | null)}
	<span class="flex w-full items-center gap-3">
		<span
			class={cn(
				'flex size-6 shrink-0 items-center justify-center rounded-full text-xs font-medium tabular-nums',
				step.status === 'upcoming'
					? 'bg-muted text-muted-foreground'
					: 'bg-primary text-primary-foreground'
			)}
		>
			{#if step.status === 'completed' || step.status === 'completed-locked'}
				<!-- `text-current` opts out of the menu item's blanket
					`[&_svg:not([class*='text-'])]:text-muted-foreground`, which would otherwise
					paint the tick grey on the filled circle. -->
				<Check class="size-3.5 text-current" strokeWidth={3} />
			{:else if position}
				{position}
			{:else}
				<span class="size-2 rounded-full bg-current"></span>
			{/if}
		</span>
		<span
			class={cn(
				'min-w-0 truncate text-base font-medium',
				step.status === 'completed-locked' || step.status === 'upcoming'
					? 'text-muted-foreground'
					: 'text-foreground'
			)}
		>
			{step.name}
		</span>
		{#if step.status === 'completed-locked'}
			<Lock class="text-muted-foreground ml-auto size-4 shrink-0" />
		{/if}
	</span>
{/snippet}

<!-- Step hrefs are built by the step page from `workflow_step_url`, and the legal links are
	plain site paths. Neither is a typed route id, so resolve() has nothing to resolve. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
{#if isMobile.current}
	<Drawer.Root bind:open>
		<Drawer.Trigger
			data-tour="menu"
			class="group text-primary active:bg-primary/10 data-[state=open]:bg-primary/10 -mx-2 flex min-w-0 items-center justify-end gap-2 rounded-full px-2 py-1 text-sm font-medium transition-colors duration-75"
			aria-label={m.step_dropdown_open()}
		>
			{@render triggerInner()}
		</Drawer.Trigger>

		<Drawer.Content class="max-h-[85vh]">
			<Drawer.Header class="flex flex-row items-center gap-4 px-5 pt-2 pb-3 text-left">
				<Drawer.Title class="text-foreground min-w-0 flex-1 text-lg font-bold">
					{heading}
				</Drawer.Title>
				<Drawer.Close
					class="text-muted-foreground hover:bg-muted hover:text-foreground grid size-10 shrink-0 place-items-center rounded-full transition-colors"
					aria-label={m.step_menu_close()}
				>
					<X class="size-5" />
				</Drawer.Close>
			</Drawer.Header>

			<!-- The sheet is capped at 85vh, so a long conversation scrolls its steps rather than
				pushing the standing material off the bottom of the screen. -->
			<div
				class="min-h-0 flex-1 overflow-y-auto overscroll-contain px-2 pb-[env(safe-area-inset-bottom)]"
			>
				{#each steps as step, index (step.id)}
					{@const position = step.isIntro ? null : realSteps.indexOf(step) + 1}
					{#if step.href}
						<a
							href={step.href}
							class="hover:bg-muted active:bg-muted flex min-h-14 items-center rounded-xl px-3 transition-colors"
							onclick={() => (open = false)}
						>
							{@render stepRow(step, position)}
						</a>
					{:else}
						<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
						<div
							class="flex min-h-14 items-center px-3"
							aria-current={index === currentIndex ? 'step' : undefined}
						>
							{@render stepRow(step, position)}
						</div>
					{/if}
				{/each}

				{#if supportActions.length > 0}
					<div class="border-border mt-2 border-t pt-2">
						<p class="text-muted-foreground px-3 py-2 text-sm font-semibold">
							{m.step_dropdown_support_heading()}
						</p>
						{#each supportActions as action (action.id)}
							{@render sheetAction(action)}
						{/each}
					</div>
				{/if}

				<div class="border-border mt-2 border-t pt-2">
					{@render sheetAction(themeAction)}
				</div>

				{#if legalActions.length > 0}
					<div class="border-border mt-2 border-t pt-2 pb-2">
						{#each legalActions as action (action.id)}
							{@render sheetAction(action)}
						{/each}
					</div>
				{/if}
			</div>
		</Drawer.Content>
	</Drawer.Root>
{:else}
	<!-- The page behind goes soft while the menu is up, so a menu that overlaps the step's own
		text still reads as one layer above it. The trigger climbs over the veil: it is the
		control you just pressed, and blurring it makes the press look like it missed. -->
	{#if open}
		<div
			class="motion-safe:animate-in motion-safe:fade-in-0 bg-background/20 fixed inset-0 z-40 backdrop-blur-[3px] duration-150"
			aria-hidden="true"
		></div>
	{/if}

	<DropdownMenu.Root bind:open>
		<!-- The trigger answers the tap itself: `data-state` flips in the same frame the menu
			opens, so the pill and the chevron confirm the press before the panel has animated. -->
		<DropdownMenu.Trigger
			data-tour="menu"
			class="group text-primary active:bg-primary/10 data-[state=open]:bg-primary/10 relative -mx-2 flex min-w-0 items-center justify-end gap-2 rounded-full px-2 py-1 text-base font-medium transition-colors duration-75 data-[state=open]:z-50"
			aria-label={m.step_dropdown_open()}
		>
			{@render triggerInner()}
		</DropdownMenu.Trigger>

		<!-- Shorter than the shadcn default: 150ms of fade-and-zoom on a menu you open to move
			between steps reads as lag, not as polish. -->
		<DropdownMenu.Content align="end" class="animation-duration-100 w-80 p-2 shadow-xl">
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading class="text-sm">
					{heading}
				</DropdownMenu.GroupHeading>
				{#each steps as step, index (step.id)}
					{@const position = step.isIntro ? null : realSteps.indexOf(step) + 1}
					{#if step.href}
						<DropdownMenu.Item class="py-2.5">
							{#snippet child({ props })}
								<a
									{...props}
									href={step.href}
									class={cn(props.class as string, 'cursor-pointer')}
								>
									{@render stepRow(step, position)}
								</a>
							{/snippet}
						</DropdownMenu.Item>
					{:else}
						<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
						<div
							class="px-2 py-2.5"
							aria-current={index === currentIndex ? 'step' : undefined}
						>
							{@render stepRow(step, position)}
						</div>
					{/if}
				{/each}
			</DropdownMenu.Group>

			{#if supportActions.length > 0}
				<DropdownMenu.Separator />
				<DropdownMenu.Group>
					<DropdownMenu.GroupHeading class="text-sm">
						{m.step_dropdown_support_heading()}
					</DropdownMenu.GroupHeading>
					{#each supportActions as action (action.id)}
						{@render menuAction(action)}
					{/each}
				</DropdownMenu.Group>
			{/if}

			<DropdownMenu.Separator />
			{@render menuAction(themeAction)}

			{#if legalActions.length > 0}
				<DropdownMenu.Separator />
				{#each legalActions as action (action.id)}
					{@render menuAction(action)}
				{/each}
			{/if}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}

{#snippet menuAction(action: MenuAction)}
	{@const Icon = action.icon}
	<DropdownMenu.Item
		class={cn(
			'cursor-pointer py-2.5',
			action.muted ? 'text-muted-foreground text-sm' : 'text-base'
		)}
		onSelect={(event) => {
			if (action.keepOpen) event.preventDefault();
			action.run();
		}}
	>
		{#if Icon}
			<Icon class="text-muted-foreground size-4 stroke-current" />
		{/if}
		{action.label}
	</DropdownMenu.Item>
{/snippet}

{#snippet sheetAction(action: MenuAction)}
	{@const Icon = action.icon}
	<button
		type="button"
		class={cn(
			'hover:bg-muted active:bg-muted flex min-h-14 w-full items-center gap-3 rounded-xl px-3 text-left transition-colors',
			action.muted ? 'text-muted-foreground min-h-12 text-base' : 'text-foreground text-base'
		)}
		onclick={() => {
			// Closed first, then run: the support panel is a drawer of its own, and the two fight
			// over the body's scroll lock if the second opens while this one is still up.
			if (!action.keepOpen) open = false;
			action.run();
		}}
	>
		{#if Icon}
			<Icon class="text-muted-foreground size-5 shrink-0 stroke-current" />
		{/if}
		{action.label}
	</button>
{/snippet}
