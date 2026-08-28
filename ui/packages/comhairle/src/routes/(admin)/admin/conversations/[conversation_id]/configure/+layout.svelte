<script lang="ts">
	import { page } from '$app/state';
	import TabStripShell from '$lib/components/TabStripShell.svelte';

	let { data, children } = $props();

	const tabs: { id: string; label: string }[] = $derived(
		[
			{ id: 'details', label: 'Details' },
			{ id: 'content', label: 'Content' },
			{ id: 'glossary', label: 'Glossary' },
			{ id: 'access', label: 'Access' }
		].concat(data.isConversationOwner ? [{ id: 'team', label: 'Team' }] : [])
	);

	let activeTab = $derived(page.url.searchParams.get('tab') ?? tabs[0]?.id);

	// Preserve the rest of the URL, just swap `?tab=`. `noscroll` (below) keeps the
	// scroll position so switching tabs never jumps the page.
	function hrefFor(tabId: string): string {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams(page.url.searchParams);
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

<section class="pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	{@render children?.()}
</section>
