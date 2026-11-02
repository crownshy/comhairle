<script lang="ts">
	import { page } from '$app/state';
	import type { ResolvedPathname } from '$app/types';
	import type { Snippet } from 'svelte';
	import { sectionTabClass } from './sectionTabClass';

	interface Props {
		href: ResolvedPathname;
		isActive: (pathname: string) => boolean;
		children: Snippet;
	}

	const { href, isActive, children }: Props = $props();

	const active = $derived(isActive(page.url.pathname));
</script>

<!-- eslint-disable svelte/no-navigation-without-resolve -->
<li>
	<a
		role="tab"
		{href}
		data-sveltekit-noscroll
		class={sectionTabClass(active)}
		aria-selected={active}
	>
		{@render children()}
	</a>
</li>
