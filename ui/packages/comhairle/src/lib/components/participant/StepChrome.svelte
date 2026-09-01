<script lang="ts">
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import StepDropdown from './StepDropdown.svelte';
	import StepProgressBar from './StepProgressBar.svelte';
	import StepLegalSheet from './StepLegalSheet.svelte';
	import * as m from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { stepBriefLabel } from './stepBriefLabel';
	import type { StepItem } from './stepItems';
	import type { LegalDocId } from '$lib/components/rights/legalDocs';
	import { supportPanel, type SupportPanelTab } from './supportPanel.svelte';

	let {
		steps,
		currentIndex,
		label,
		fill,
		count,
		assistantAvailable = false,
		showSupport = true,
		introUrl,
		briefOpen = false,
		onBrief,
		preview = false
	}: {
		steps: StepItem[];
		currentIndex: number;
		label: string;
		fill: number;
		/** Optional within-step count, e.g. Polis's "Opinion 3 of 12". */
		count?: string;
		/** Whether this conversation has a Learning Assistant to offer. */
		assistantAvailable?: boolean;
		/**
		 * Whether the support panel exists on this page. It is mounted by the workflow layout,
		 * so the landing page renders this chrome with it off.
		 */
		showSupport?: boolean;
		/**
		 * Where the logo goes: this conversation's Before you start. Omitted, it links home,
		 * which is what the landing page itself does.
		 */
		introUrl?: string;
		/** Whether the step brief is open over the step. */
		briefOpen?: boolean;
		/**
		 * Opens the step brief. Omitted on a page that has no brief to reopen, which is the
		 * landing page and a step with no description.
		 */
		onBrief?: () => void;
		/**
		 * Whether this is an admin's preview rather than the live conversation. Marked here,
		 * in the chrome every participant page shares, instead of the full-width banner the
		 * other conversation pages get.
		 */
		preview?: boolean;
	} = $props();

	let legalDoc = $state<LegalDocId | null>(null);

	function openSupport(tab: SupportPanelTab) {
		// Same focus-restore race as openLegal below: let the menu finish closing first.
		requestAnimationFrame(() => supportPanel.openAt(tab));
	}

	function openLegal(doc: LegalDocId) {
		// The dropdown returns focus to its trigger as it closes. Mounting the sheet in the same
		// frame lands its focus trap inside that restore, so wait until the menu is gone.
		requestAnimationFrame(() => (legalDoc = doc));
	}
</script>

<!-- `introUrl` is built by $lib/urls, not a typed route id, so resolve() has nothing to
	resolve. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<header class="bg-background pb-2">
	<div class="mx-auto flex h-[72px] w-full max-w-5xl items-center gap-5 px-5 md:h-20 md:px-6">
		{#if introUrl}
			<!-- Inside a step the mark goes back to Before you start rather than off the site:
				the way out of a step is a move within the conversation (ADR-0024). -->
			<a
				href={introUrl}
				class="-m-2 shrink-0 p-2"
				aria-label={m.landing_before_you_start()}
				data-tour="intro"
			>
				<ComhairleLogo href={null} logoSize="sm" color="text-primary" class="shrink-0" />
			</a>
		{:else}
			<ComhairleLogo href="/" logoSize="sm" color="text-primary" class="shrink-0" />
		{/if}
		{#if preview}
			<span
				class="bg-sidebar text-sidebar-foreground shrink-0 rounded-full px-2.5 py-1 text-xs font-medium tracking-wide uppercase"
			>
				{m.conversation_preview_badge()}
			</span>
		{/if}
		<div class="flex min-w-0 flex-1 items-center justify-end gap-3">
			{#if count}
				<span class="text-muted-foreground shrink-0 text-sm md:text-base">{count}</span>
			{/if}
			{#if onBrief}
				<!-- Up here with the step's name rather than in the pager: what the brief answers
					is "what is this step", which is what this corner is already about, and the
					bottom bar is left as navigation only (ADR-0025). -->
				<button
					type="button"
					data-tour="brief"
					class={cn(
						'inline-flex h-8 shrink-0 items-center rounded-full px-4 text-sm font-medium italic',
						briefOpen
							? 'bg-foreground text-primary-foreground'
							: 'bg-accent text-accent-foreground'
					)}
					aria-expanded={briefOpen}
					onclick={onBrief}
				>
					{stepBriefLabel()}
				</button>
			{/if}
			<StepDropdown
				{steps}
				{currentIndex}
				{label}
				{assistantAvailable}
				onOpenLegal={openLegal}
				onOpenSupport={showSupport ? openSupport : undefined}
			/>
		</div>
	</div>
	<div class="mx-auto w-full max-w-5xl">
		<StepProgressBar {steps} {currentIndex} {fill} />
	</div>
</header>

<StepLegalSheet bind:doc={legalDoc} />
