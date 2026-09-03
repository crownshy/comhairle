<script lang="ts">
	import { page } from '$app/state';
	import type { ResolvedPathname } from '$app/types';
	import type { Snippet } from 'svelte';

	interface Props {
		tab: string;
		href: ResolvedPathname;
		children: Snippet;
	}

	const { tab, href, children }: Props = $props();

	const active = $derived(page.route.id?.endsWith(tab));
</script>

<!-- eslint-disable svelte/no-navigation-without-resolve -->
<li>
	<a
		role="tab"
		{href}
		data-sveltekit-noscroll
		class="text-foreground inline-flex h-9 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
		class:text-primary={active}
		class:opacity-70={!active}
		class:hover:opacity-100={!active}
		aria-selected={active}
	>
		{@render children()}
	</a>
</li>
