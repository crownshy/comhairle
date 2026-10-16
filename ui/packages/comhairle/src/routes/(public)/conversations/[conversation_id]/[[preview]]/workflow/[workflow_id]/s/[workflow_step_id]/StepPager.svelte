<script lang="ts">
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as Popover from '$lib/components/ui/popover';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { STEP_COLUMN_CLASS } from './styles';

	type Props = {
		/** `skip` only when an optional step cannot advance yet (ADR-0047). */
		forwardMode: 'next' | 'skip';
		canGoBack: boolean;
		canGoForward: boolean;
		loading?: boolean;
		/** Shown when a closed forward is pressed, so a required step doesn't feel like a dead end. */
		blockedReason?: string;
		onBack: () => void;
		onForward: () => void;
	};

	let {
		forwardMode,
		canGoBack,
		canGoForward,
		loading = false,
		blockedReason,
		onBack,
		onForward
	}: Props = $props();

	const PAGER_BUTTON_CLASS =
		'text-foreground inline-flex items-center gap-1 transition-transform active:scale-90 disabled:opacity-30 motion-reduce:transition-none motion-reduce:active:scale-100';

	let forwardLabel = $derived(forwardMode === 'skip' ? m.pager_skip() : m.next());
	let explainBlocked = $derived(!canGoForward && !loading && blockedReason !== undefined);
</script>

{#snippet forwardInner()}
	{#if loading}
		<Spinner class="size-5" />
	{/if}
	<span class="text-base font-medium">{forwardLabel}</span>
	<ChevronRight class="size-6 shrink-0 rtl:-scale-x-100" />
{/snippet}

<!-- Back and forward only (ADR-0048). The forward button is labelled so it reads as the way
	out of the step, not the tool's own Next. -->
<div class={cn(STEP_COLUMN_CLASS, 'flex h-20 items-center gap-2')}>
	<button
		type="button"
		class={PAGER_BUTTON_CLASS}
		aria-label={m.pager_back()}
		disabled={!canGoBack || loading}
		onclick={onBack}
	>
		<ChevronLeft class="size-6 shrink-0 rtl:-scale-x-100" />
	</button>

	{#if explainBlocked}
		<!-- aria-disabled rather than disabled, so the press still reaches the popover. -->
		<Popover.Root>
			<Popover.Trigger
				class={cn(PAGER_BUTTON_CLASS, 'ms-auto opacity-30')}
				aria-disabled="true"
			>
				{@render forwardInner()}
			</Popover.Trigger>
			<Popover.Content side="top" align="end" class="w-auto max-w-72 text-base">
				{blockedReason}
			</Popover.Content>
		</Popover.Root>
	{:else}
		<button
			type="button"
			class={cn(PAGER_BUTTON_CLASS, 'ms-auto')}
			disabled={!canGoForward || loading}
			aria-busy={loading}
			onclick={onForward}
		>
			{@render forwardInner()}
		</button>
	{/if}
</div>
