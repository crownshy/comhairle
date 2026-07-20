<script lang="ts">
	import { page } from '$app/state';

	type Item = { label: string; value: string };

	type Props = {
		items: Item[];
		paramName?: string;
		defaultValue?: string;
		/**
		 * Which layout row this strip sits in, which sets its background tone:
		 * - `'secondary'` (default) = the deepest sub-tab strip → `bg-accent`.
		 * - `'primary'` = a section's sole strip directly under the section tabs
		 *   (e.g. Recruit/invites) → `bg-secondary`.
		 */
		tone?: 'primary' | 'secondary';
		/**
		 * When set, each tab is a route link (`${basePath}/${value}`) and the active tab is
		 * matched by the last path segment instead of the `?subtab=` query param. Use this when
		 * the sub-tabs are real routes (e.g. a workflow step's Configure/Setup/…), so the strip
		 * server-renders and deep-links cleanly. Omit for query-param sub-tabs.
		 */
		basePath?: string;
	};

	let {
		items,
		paramName = 'subtab',
		defaultValue,
		tone = 'secondary',
		basePath
	}: Props = $props();

	let currentValue = $derived(
		basePath
			? (page.url.pathname.replace(/\/+$/, '').split('/').pop() ?? '')
			: (page.url.searchParams.get(paramName) ?? defaultValue ?? items[0]?.value)
	);

	function hrefFor(value: string): string {
		if (basePath) return `${basePath}/${value}`;
		const params = new URLSearchParams(page.url.searchParams);
		params.set(paramName, value);
		return `${page.url.pathname}?${params.toString()}`;
	}
</script>

<!-- `shrink-0`: when this strip is a direct flex child of the scrolling `main` column (route
	 sub-tabs, e.g. a workflow step), a tall page below it would otherwise let flexbox crush the
	 strip to its 1px border. Harmless where it sits inside a `shrink-0` header block (Events). -->
<nav
	class="border-border scrollbar-none flex w-full shrink-0 overflow-x-auto border-b {tone ===
	'primary'
		? 'bg-secondary'
		: 'bg-accent'}"
	aria-label="Sub sections"
>
	<!-- Left padding is one item's px-3.5 short of the full gutter so the first tab's text
		 lands on the gutter column, matching the Row 2 strip above. See TabStripShell. -->
	<ul class="flex min-w-full items-center gap-1.5 pr-5 pl-[calc(var(--spacing-gutter)-0.875rem)]">
		{#each items as item (item.value)}
			{@const active = item.value === currentValue}
			<li class="shrink-0">
				<a
					href={hrefFor(item.value)}
					data-sveltekit-replacestate
					class="text-foreground inline-flex h-10 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
					class:text-primary={active}
					class:opacity-70={!active}
					class:hover:opacity-100={!active}
					aria-current={active ? 'page' : undefined}
				>
					{item.label}
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
