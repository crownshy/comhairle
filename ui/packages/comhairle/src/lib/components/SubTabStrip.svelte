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
	};

	let { items, paramName = 'subtab', defaultValue, tone = 'secondary' }: Props = $props();

	let currentValue = $derived(
		page.url.searchParams.get(paramName) ?? defaultValue ?? items[0]?.value
	);

	function hrefFor(value: string): string {
		const params = new URLSearchParams(page.url.searchParams);
		params.set(paramName, value);
		return `${page.url.pathname}?${params.toString()}`;
	}
</script>

<nav
	class="border-border scrollbar-none flex w-full overflow-x-auto border-b {tone === 'primary'
		? 'bg-secondary'
		: 'bg-accent'}"
	aria-label="Sub sections"
>
	<ul class="flex min-w-full items-center gap-1.5 px-5">
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
