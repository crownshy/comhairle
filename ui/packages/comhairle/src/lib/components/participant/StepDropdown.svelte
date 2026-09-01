<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronDown, Check, Lock, Sparkles, Moon, Sun } from 'lucide-svelte';
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
</script>

<!-- Step hrefs are built by the step page from `workflow_step_url`, and the legal links are
	plain site paths. Neither is a typed route id, so resolve() has nothing to resolve. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<DropdownMenu.Root>
	<!-- The trigger answers the tap itself: `data-state` flips in the same frame the menu
		opens, so the pill and the chevron confirm the press before the panel has animated. -->
	<DropdownMenu.Trigger
		class="group text-primary active:bg-primary/10 data-[state=open]:bg-primary/10 -mx-2 flex min-w-0 items-center justify-end gap-2 rounded-full px-2 py-1 text-sm font-medium transition-colors duration-75 md:text-base"
		aria-label={m.step_dropdown_open()}
	>
		<span class="truncate">{label}</span>
		<ChevronDown
			class="size-5 shrink-0 transition-transform duration-150 group-data-[state=open]:rotate-180"
			aria-hidden="true"
		/>
	</DropdownMenu.Trigger>

	<!-- Shorter than the shadcn default: 150ms of fade-and-zoom on a menu you open to move
		between steps reads as lag, not as polish. -->
	<DropdownMenu.Content align="end" class="animation-duration-100 w-72">
		<DropdownMenu.Group>
			<DropdownMenu.GroupHeading>{heading}</DropdownMenu.GroupHeading>
			{#each steps as step, index (step.id)}
				{@const position = step.isIntro ? null : realSteps.indexOf(step) + 1}
				{#snippet row()}
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
									`[&_svg:not([class*='text-'])]:text-muted-foreground`, which would
									otherwise paint the tick grey on the filled circle. -->
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

				{#if step.href}
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<a
								{...props}
								href={step.href}
								class={cn(props.class as string, 'cursor-pointer')}
							>
								{@render row()}
							</a>
						{/snippet}
					</DropdownMenu.Item>
				{:else}
					<!-- Inert rather than absent: an unreachable step still tells you where you are. -->
					<div
						class="px-2 py-1.5"
						aria-current={index === currentIndex ? 'step' : undefined}
					>
						{@render row()}
					</div>
				{/if}
			{/each}
		</DropdownMenu.Group>

		{#if onOpenSupport}
			<DropdownMenu.Separator />
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading>
					{m.step_dropdown_support_heading()}
				</DropdownMenu.GroupHeading>
				{#if assistantAvailable}
					<DropdownMenu.Item
						class="cursor-pointer text-base"
						onSelect={() => onOpenSupport?.('learningAssistant')}
					>
						<Sparkles class="text-muted-foreground size-4" aria-hidden="true" />
						{m.learning_assistant()}
					</DropdownMenu.Item>
				{/if}
				<DropdownMenu.Item
					class="cursor-pointer text-base"
					onSelect={() => onOpenSupport?.('faqs')}
				>
					<CircleQuestionMark class="text-muted-foreground size-4 stroke-current" />
					{m.faq()}
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		{/if}

		<DropdownMenu.Separator />
		<DropdownMenu.Item
			class="cursor-pointer text-base"
			onSelect={(event) => {
				// Keep the menu open: the mode is a thing you look at, and closing the menu to show
				// the result means reopening it to change your mind.
				event.preventDefault();
				themeStore.toggleMode();
			}}
		>
			{#if themeStore.isDark}
				<Sun class="text-muted-foreground size-4" aria-hidden="true" />
				{m.theme_light_mode()}
			{:else}
				<Moon class="text-muted-foreground size-4" aria-hidden="true" />
				{m.theme_dark_mode()}
			{/if}
		</DropdownMenu.Item>

		{#if onOpenLegal}
			<DropdownMenu.Separator />
			{#each docs as entry (entry.id)}
				<DropdownMenu.Item
					class="text-muted-foreground cursor-pointer text-sm"
					onSelect={() => onOpenLegal?.(entry.id)}
				>
					{entry.label}
				</DropdownMenu.Item>
			{/each}
		{/if}
	</DropdownMenu.Content>
</DropdownMenu.Root>
