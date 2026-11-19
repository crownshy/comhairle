<script lang="ts">
	import { hasDataProtocolText } from '$lib/dataProtocol';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import ShieldCheck from '@lucide/svelte/icons/shield-check';
	import * as Popover from '$lib/components/ui/popover';
	import PagerButton from '$lib/components/pager/PagerButton.svelte';
	import { PAGER_BUTTON_CLASS, PAGER_ROW_CLASS } from '$lib/components/pager/pager';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { STEP_COLUMN_CLASS } from './styles';
	import DataProtocolDialog from './DataProtocolDialog.svelte';

	type Props = {
		/** `skip` only when an optional step cannot advance yet (ADR-0047). */
		forwardMode: 'next' | 'skip';
		/** A tool's own name for forward while it is moving through its pages. */
		nextLabel?: string;
		canGoBack: boolean;
		canGoForward: boolean;
		loading?: boolean;
		/** Shown when a closed forward is pressed, so a required step doesn't feel like a dead end. */
		blockedReason?: string;
		onBack: () => void;
		onForward: () => void;
		/** This step's own Data protocol text. The link only shows when there is some. */
		dataProtocol?: string | null;
		/** Shown at the top of the pop-up, so it's clear the text is about this step. */
		stepName?: string;
	};

	let {
		forwardMode,
		nextLabel,
		canGoBack,
		canGoForward,
		loading = false,
		blockedReason,
		onBack,
		onForward,
		dataProtocol,
		stepName
	}: Props = $props();

	let forwardLabel = $derived(forwardMode === 'skip' ? m.pager_skip() : (nextLabel ?? m.next()));
	let explainBlocked = $derived(!canGoForward && !loading && blockedReason !== undefined);
	let showDataProtocol = $derived(hasDataProtocolText(dataProtocol));
	let dataProtocolOpen = $state(false);
</script>

<!-- Back and forward only (ADR-0048). The forward button is labelled so it reads as the way
	out of the step, not the tool's own Next. -->
<div class={cn(STEP_COLUMN_CLASS, PAGER_ROW_CLASS)}>
	{#if showDataProtocol}
		<button
			type="button"
			class="text-muted-foreground hover:text-foreground absolute left-1/2 flex -translate-x-1/2 items-center gap-1.5 text-sm underline underline-offset-4 sm:text-base"
			onclick={() => (dataProtocolOpen = true)}
		>
			<ShieldCheck class="size-4 shrink-0" aria-hidden="true" />
			<span class="sm:hidden">{m.data_protocol_link_short()}</span>
			<span class="max-sm:hidden">{m.data_protocol_link()}</span>
		</button>
		<DataProtocolDialog
			stepName={stepName ?? ''}
			text={dataProtocol ?? ''}
			bind:open={dataProtocolOpen}
		/>
	{/if}
	<PagerButton
		direction="back"
		label={m.pager_back()}
		disabled={!canGoBack || loading}
		onclick={onBack}
	/>

	{#if explainBlocked}
		<!-- aria-disabled rather than disabled, so the press still reaches the popover. -->
		<Popover.Root>
			<Popover.Trigger class={cn(PAGER_BUTTON_CLASS, 'ms-auto')} aria-disabled="true">
				<span class="text-base font-medium">{forwardLabel}</span>
				<ChevronRight class="size-6 shrink-0 rtl:-scale-x-100" aria-hidden="true" />
			</Popover.Trigger>
			<Popover.Content side="top" align="end" class="w-auto max-w-72 text-base">
				{blockedReason}
			</Popover.Content>
		</Popover.Root>
	{:else}
		<PagerButton
			direction="forward"
			label={forwardLabel}
			disabled={!canGoForward}
			{loading}
			onclick={onForward}
		/>
	{/if}
</div>
