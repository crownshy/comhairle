<script lang="ts">
	import { page } from '$app/state';
	import { SvelteURLSearchParams } from 'svelte/reactivity';
	import TabStripShell from '$lib/components/TabStripShell.svelte';

	type Props = {
		/** Tabs to render, in display order. First tab is the default when `?tab=` is absent. */
		tabs: { id: string; label: string }[];
	};

	let { tabs }: Props = $props();

	let activeTab = $derived(page.url.searchParams.get('tab') ?? tabs[0]?.id);

	// Preserve the rest of the URL, just swap `?tab=`. `noscroll` (below) keeps the
	// scroll position so switching tabs never jumps the page.
	function hrefFor(tabId: string): string {
		const params = new SvelteURLSearchParams(page.url.searchParams);
		params.set('tab', tabId);
		return `${page.url.pathname}?${params.toString()}`;
	}
</script>

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab.id)}
		{@const active = activeTab === tab.id}
		<li>
			<a
				href={hrefFor(tab.id)}
				data-sveltekit-noscroll
				class="text-foreground inline-flex h-9 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
				class:text-primary={active}
				class:opacity-70={!active}
				class:hover:opacity-100={!active}
				aria-current={active ? 'page' : undefined}
			>
				{tab.label}
			</a>
		</li>
	{/each}
</TabStripShell>
