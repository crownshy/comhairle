<script lang="ts">
	import { Search } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Props = {
		value: string;
		/** Scenario params to keep when the form submits. */
		hiddenFields: [string, string][];
		/** Where the search goes. The top bar points this at the home page from any page. */
		action?: string;
		/** Smaller field for the top bar. */
		compact?: boolean;
		autofocus?: boolean;
		onsubmit?: () => void;
	};
	let {
		value,
		hiddenFields,
		action = '#all',
		compact = false,
		autofocus = false,
		onsubmit
	}: Props = $props();

	let input: HTMLInputElement | undefined = $state();
	$effect(() => {
		if (autofocus) input?.focus();
	});
</script>

<form method="get" role="search" {action} {onsubmit}>
	{#each hiddenFields as [name, fieldValue] (name)}
		<input type="hidden" {name} value={fieldValue} />
	{/each}
	<label
		class={cn(
			'search-pill bg-card border-border flex w-full items-center gap-3 rounded-full border',
			compact ? 'h-12 px-4' : 'h-14 px-5'
		)}
	>
		<Search class="text-muted-foreground size-5 shrink-0" />
		<span class="sr-only">Search conversations</span>
		<input
			bind:this={input}
			type="search"
			name="q"
			{value}
			placeholder="Search by topic or keyword"
			class="search-input placeholder:text-muted-foreground w-full min-w-0 border-none bg-transparent p-0 text-base focus:ring-0"
		/>
	</label>
</form>

<style>
	/* The site's standard focus indicator (app.css) goes round the whole pill instead of the
		bare text field inside it. Same two bands, so it still meets WCAG 2.4.7 and 1.4.11. */
	.search-input:focus-visible {
		outline: none;
		box-shadow: none;
	}
	.search-pill:has(.search-input:focus-visible) {
		outline: 2px solid var(--focus-ring-inner);
		box-shadow: 0 0 0 4px var(--focus-ring-outer);
	}
</style>
