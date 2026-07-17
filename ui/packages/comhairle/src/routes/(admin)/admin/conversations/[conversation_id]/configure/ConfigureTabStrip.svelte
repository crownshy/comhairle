<script lang="ts">
	import { page } from '$app/state';
	import { SvelteURLSearchParams } from 'svelte/reactivity';

	/** One selectable sub-tab of the Configure page. `id` is the `?tab=` value. */
	export type ConfigureTab = { id: string; label: string };

	type Props = {
		/** Tabs to render, in display order. First tab is the default when `?tab=` is absent. */
		tabs: ConfigureTab[];
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

<nav
	class="border-border bg-muted/50 scrollbar-none w-full overflow-x-auto border-b"
	aria-label="Configure sections"
>
	<ul
		class="pl-gutter flex min-w-max items-center gap-x-1.5 gap-y-0.5 py-1 pr-5 sm:w-full sm:min-w-0 sm:flex-wrap"
	>
		{#each tabs as tab, i (tab.id)}
			{@const active = activeTab === tab.id}
			<!-- First item bleeds left into the gutter (-ml-3.5 cancels its own px-3.5)
				 so it aligns to the shared gutter column, matching WorkflowStepStrip. -->
			<li class={i === 0 ? '-ml-3.5' : undefined}>
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
	</ul>
</nav>

<style>
	.scrollbar-none {
		scrollbar-width: none;
	}
	.scrollbar-none::-webkit-scrollbar {
		display: none;
	}
</style>
