<script lang="ts">
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import StepDropdown from './StepDropdown.svelte';
	import StepProgressBar from './StepProgressBar.svelte';
	import StepLegalSheet from './StepLegalSheet.svelte';
	import StepLeaveDialog from './StepLeaveDialog.svelte';
	import * as m from '$lib/paraglide/messages';
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
		returnUrl,
		anonymousId,
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
		/** Absolute link back to where the participant is. Omitted, the logo just links home. */
		returnUrl?: string;
		/** The anonymous id a participant signs back in with, when they have one. */
		anonymousId?: string;
		/**
		 * Whether this is an admin's preview rather than the live conversation. Marked here,
		 * in the chrome every participant page shares, instead of the full-width banner the
		 * other conversation pages get.
		 */
		preview?: boolean;
	} = $props();

	let leaveOpen = $state(false);

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

<header class="bg-background pb-2">
	<div class="mx-auto flex h-[72px] w-full max-w-5xl items-center gap-5 px-5 md:h-20 md:px-6">
		{#if returnUrl}
			<!-- Leaving mid-step should be a decision rather than a stray tap, so the logo asks
				first and hands over the link back. -->
			<button
				type="button"
				class="-m-2 shrink-0 p-2"
				aria-label={m.step_leave_open()}
				onclick={() => (leaveOpen = true)}
			>
				<ComhairleLogo
					href={null}
					showText={false}
					logoSize="sm"
					color="text-primary"
					class="size-9 shrink-0"
				/>
			</button>
		{:else}
			<ComhairleLogo
				href="/"
				showText={false}
				logoSize="sm"
				color="text-primary"
				class="size-9 shrink-0"
			/>
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

{#if returnUrl}
	<StepLeaveDialog bind:open={leaveOpen} {returnUrl} {anonymousId} />
{/if}
