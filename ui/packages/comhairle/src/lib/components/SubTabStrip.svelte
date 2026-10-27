<script lang="ts">
	import { page } from '$app/state';
	import { cn } from '$lib/utils';
	import { SUB_TAB_LIST_OFFSET, sectionTabClass, subTabClass } from './sectionTabClass';

	type Item = { label: string; value: string };

	type Props = {
		items: Item[];
		paramName?: string;
		defaultValue?: string;
		/**
		 * Which layout row this strip sits in:
		 * - `'secondary'` (default) = a sub-tab strip under the section's tabs, on `bg-accent`.
		 * - `'primary'` = the section's own tabs directly under the section header
		 *   (e.g. Recruit), styled like the other section strips.
		 */
		tone?: 'primary' | 'secondary';
		/**
		 * When set, each tab is a route link (`${basePath}/${value}`) and the active tab is
		 * matched by the last path segment instead of the `?subtab=` query param. Use this when
		 * the sub-tabs are real routes (e.g. a workflow step's Configure/Setup/…), so the strip
		 * server-renders and deep-links cleanly. Omit for query-param sub-tabs.
		 */
		basePath?: string;
		/** The page below is full-bleed, so the tabs sit at the left gutter instead of centring. */
		fullBleed?: boolean;
	};

	let {
		items,
		paramName = 'subtab',
		defaultValue,
		tone = 'secondary',
		basePath,
		fullBleed = false
	}: Props = $props();

	let currentValue = $derived(
		basePath
			? (page.url.pathname.replace(/\/+$/, '').split('/').pop() ?? '')
			: (page.url.searchParams.get(paramName) ?? defaultValue ?? items[0]?.value)
	);

	let isSectionStrip = $derived(tone === 'primary');

	function hrefFor(value: string): string {
		if (basePath) return `${basePath}/${value}`;
		// A throwaway copy to build one href, not reactive state.
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const params = new URLSearchParams(page.url.searchParams);
		params.set(paramName, value);
		return `${page.url.pathname}?${params.toString()}`;
	}
</script>

<!-- `shrink-0`: when this strip is a direct flex child of the scrolling `main` column (route
	 sub-tabs, e.g. a workflow step), a tall page below it would otherwise let flexbox crush the
	 strip to its 1px border. Harmless where it sits inside a `shrink-0` header block (Events). -->
<nav
	class={cn(
		'border-border scrollbar-none px-gutter flex w-full shrink-0 overflow-x-auto border-b',
		!isSectionStrip && 'bg-accent'
	)}
	aria-label="Sub sections"
>
	<ul
		class={cn(
			'flex w-full items-center',
			!fullBleed && 'max-w-admin mx-auto',
			isSectionStrip ? 'gap-6' : cn(SUB_TAB_LIST_OFFSET, 'gap-1.5')
		)}
	>
		{#each items as item (item.value)}
			{@const active = item.value === currentValue}
			<li class="shrink-0">
				<!-- eslint-disable svelte/no-navigation-without-resolve -- hrefFor builds on basePath or this page's URL -->
				<a
					href={hrefFor(item.value)}
					data-sveltekit-replacestate
					class={isSectionStrip ? sectionTabClass(active) : subTabClass(active)}
					aria-current={active ? 'page' : undefined}
				>
					{item.label}
				</a>
				<!-- eslint-enable svelte/no-navigation-without-resolve -->
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
