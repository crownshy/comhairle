<script lang="ts">
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	let {
		canGoBack,
		skippable = false,
		onBack,
		onSkip
	}: {
		canGoBack: boolean;
		/** Only an optional step can be left before it is started. */
		skippable?: boolean;
		onBack: () => void;
		onSkip: () => void;
	} = $props();
</script>

<!-- The step-level controls on the cover, desktop only (ADR-0023). On a phone the cover is one
	button and a swipe; here there is room for the two ways out of it that the pager used to
	carry, and putting them at the top keeps the bar below to a single forward action. -->
<div class="mx-auto hidden w-full max-w-5xl shrink-0 items-center px-4 py-2 md:flex md:px-6">
	<button
		type="button"
		class="text-foreground inline-flex size-8 items-center justify-center disabled:opacity-30"
		aria-label={m.pager_back()}
		disabled={!canGoBack}
		onclick={onBack}
	>
		<ChevronLeft class="size-6" />
	</button>
	{#if skippable}
		<button
			type="button"
			class="text-foreground ml-auto inline-flex items-center gap-1 text-base font-medium"
			onclick={onSkip}
		>
			{m.pager_skip()}
			<ChevronRight class="size-6 shrink-0" />
		</button>
	{/if}
</div>
