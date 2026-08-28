<script lang="ts">
	import { page } from '$app/state';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { capitalise } from '$lib/utils/casingUtils';

	let { data, children } = $props();

	const tabs = ['details', 'content', 'glossary', 'access'] as const;

	let activeTab = $derived(page.url.searchParams.get('tab') ?? tabs[0]);

	// Preserve the rest of the URL, just swap `?tab=`. `noscroll` (below) keeps the
	// scroll position so switching tabs never jumps the page.
	function hrefFor(tabId: string): string {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams(page.url.searchParams);
		params.set('tab', tabId);
		return `${page.url.pathname}?${params.toString()}`;
	}
</script>

{#snippet Tab(id: string)}
	{@const active = activeTab === id}
	<li>
		<a
			href={hrefFor(id)}
			data-sveltekit-noscroll
			class="text-foreground inline-flex h-9 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
			class:text-primary={active}
			class:opacity-70={!active}
			class:hover:opacity-100={!active}
			aria-current={active ? 'page' : undefined}
		>
			{capitalise(id)}
		</a>
	</li>
{/snippet}

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab)}
		{@render Tab(tab)}
	{/each}
	{#if data.isConversationOwner}
		{@render Tab('team')}
	{/if}
</TabStripShell>

<section class="pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	{@render children?.()}
</section>
