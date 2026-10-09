<script lang="ts">
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { STEP_COLUMN_CLASS } from './styles';

	type Props = {
		/** `skip` only when an optional step cannot advance yet (ADR-0047). */
		forwardMode: 'next' | 'skip';
		canGoBack: boolean;
		canGoForward: boolean;
		loading?: boolean;
		onBack: () => void;
		onForward: () => void;
	};

	let {
		forwardMode,
		canGoBack,
		canGoForward,
		loading = false,
		onBack,
		onForward
	}: Props = $props();

	const PAGER_BUTTON_CLASS =
		'text-foreground inline-flex items-center gap-1 transition-transform active:scale-90 disabled:opacity-30 motion-reduce:transition-none motion-reduce:active:scale-100';

	let forwardLabel = $derived(forwardMode === 'skip' ? m.pager_skip() : m.next());
</script>

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
		<ChevronLeft class="size-6 shrink-0" />
	</button>

	<button
		type="button"
		class={cn(PAGER_BUTTON_CLASS, 'ml-auto')}
		disabled={!canGoForward || loading}
		aria-busy={loading}
		onclick={onForward}
	>
		{#if loading}
			<Spinner class="size-5" />
		{/if}
		<span class="text-base font-medium">{forwardLabel}</span>
		<ChevronRight class="size-6 shrink-0" />
	</button>
</div>
