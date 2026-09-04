<script lang="ts">
	import type { Snippet } from 'svelte';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as m from '$lib/paraglide/messages';

	let {
		forwardMode,
		canGoBack,
		canGoForward,
		loading = false,
		onBack,
		onForward,
		middle
	}: {
		/**
		 * What the right slot means right now. One thing at a time (ADR-0018): `skip` appears
		 * only when an optional step cannot yet advance. The cover has its own bar (ADR-0023),
		 * so the pager never has to say Start.
		 */
		forwardMode: 'next' | 'skip';
		canGoBack: boolean;
		canGoForward: boolean;
		loading?: boolean;
		onBack: () => void;
		onForward: () => void;
		/**
		 * What sits between Back and Next. Empty by default (ADR-0025); the one thing allowed
		 * there is Listen's transport while a page is being read aloud (ADR-0031).
		 */
		middle?: Snippet;
	} = $props();

	let forwardLabel = $derived(forwardMode === 'skip' ? m.pager_skip() : m.pager_next());
</script>

<!-- The bar is navigation (ADR-0025): back on the left, forward on the right, and a middle
	that is empty because a full-width button there means the main move and this phase has
	none. The one exception is Listen's transport, which rides in the middle while a page is
	being read aloud (ADR-0031). Both directions carry their label, because a bare chevron
	next to a tool with its own Next says nothing about which one leaves the step. -->
<div>
	<div class="mx-auto flex h-20 w-full max-w-5xl items-center gap-2 px-4 md:px-6">
		<button
			type="button"
			data-tour="back"
			class="text-foreground inline-flex items-center gap-1 transition-transform active:scale-90 disabled:opacity-30 motion-reduce:transition-none motion-reduce:active:scale-100"
			aria-label={m.pager_back()}
			disabled={!canGoBack || loading}
			onclick={onBack}
		>
			<ChevronLeft class="size-6 shrink-0" />
		</button>

		{#if middle}
			<div class="flex min-w-0 flex-1 items-center justify-center">
				{@render middle()}
			</div>
		{/if}

		<button
			type="button"
			data-tour="forward"
			class="text-foreground ml-auto inline-flex items-center gap-1 transition-transform active:scale-90 disabled:opacity-30 motion-reduce:transition-none motion-reduce:active:scale-100"
			disabled={!canGoForward || loading}
			onclick={onForward}
		>
			{#if loading}
				<Spinner class="size-5" />
			{/if}
			<span class="text-base font-medium">{forwardLabel}</span>
			<ChevronRight class="size-6 shrink-0" />
		</button>
	</div>
</div>
