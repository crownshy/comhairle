<script lang="ts">
	import { page } from '$app/state';
	import { sectionTabClass } from './sectionTabClass';

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

	let isSectionStrip = $derived(tone === 'primary');

	function secondaryTabClass(active: boolean): string {
		const base =
			'text-foreground inline-flex h-10 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity';
		return active ? `${base} text-primary` : `${base} opacity-70 hover:opacity-100`;
	}

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
	class="border-border scrollbar-none px-gutter flex w-full shrink-0 overflow-x-auto border-b {isSectionStrip
		? ''
		: 'bg-accent'}"
	aria-label="Sub sections"
>
	<!-- Secondary tabs carry px-3.5, so `right-3.5` pulls the first label back onto the column edge. -->
	<ul
		class="max-w-admin mx-auto flex w-full items-center {isSectionStrip
			? 'gap-6'
			: 'relative right-3.5 gap-1.5'}"
	>
		{#each items as item (item.value)}
			{@const active = item.value === currentValue}
			<li class="shrink-0">
				<a
					href={hrefFor(item.value)}
					data-sveltekit-replacestate
					class={isSectionStrip ? sectionTabClass(active) : secondaryTabClass(active)}
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
