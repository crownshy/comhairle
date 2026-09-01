<script lang="ts">
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';
	import { stepBriefLabel } from './stepBriefLabel';

	let {
		forwardMode,
		briefOpen = false,
		canGoBack,
		canGoForward,
		showBrief = true,
		loading = false,
		onBack,
		onForward,
		onBrief
	}: {
		/**
		 * What the right slot means right now. One thing at a time (ADR-0018): `skip` appears
		 * only when an optional step cannot yet advance. The cover has its own bar (ADR-0023),
		 * so the pager never has to say Start.
		 */
		forwardMode: 'next' | 'skip';
		briefOpen?: boolean;
		canGoBack: boolean;
		canGoForward: boolean;
		showBrief?: boolean;
		loading?: boolean;
		onBack: () => void;
		onForward: () => void;
		onBrief: () => void;
	} = $props();

	let forwardLabel = $derived(forwardMode === 'skip' ? m.pager_skip() : m.pager_next());
	let showForwardLabel = $derived(forwardMode !== 'next');
</script>

<div class="bg-background border-t md:border-t-0">
	<div class="relative mx-auto flex h-20 w-full max-w-5xl items-center px-4 md:px-6">
		<button
			type="button"
			class="text-foreground relative z-10 inline-flex items-center gap-1 disabled:opacity-30"
			aria-label={m.pager_back()}
			disabled={!canGoBack || loading}
			onclick={onBack}
		>
			<ChevronLeft class="size-6 shrink-0" />
		</button>

		{#if showBrief}
			<button
				type="button"
				class={cn(
					'absolute top-1/2 left-1/2 z-10 inline-flex h-8 -translate-x-1/2 -translate-y-1/2 items-center rounded-full px-4 text-sm font-medium italic',
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

		<button
			type="button"
			class="text-foreground relative z-10 ml-auto inline-flex items-center gap-1 disabled:opacity-30"
			aria-label={forwardLabel}
			disabled={!canGoForward || loading}
			onclick={onForward}
		>
			{#if loading}
				<Spinner class="size-5" />
			{/if}
			{#if showForwardLabel}
				<span class="text-base font-medium">{forwardLabel}</span>
			{/if}
			<ChevronRight class="size-6 shrink-0" />
		</button>
	</div>
</div>
