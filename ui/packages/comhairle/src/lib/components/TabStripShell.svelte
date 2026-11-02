<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		/** Accessible label for the strip's `<nav>` (omit for decorative/skeleton strips). */
		ariaLabel?: string;
		/** Hide from the accessibility tree (used by the loading skeleton). */
		ariaHidden?: boolean;
		/** The strip's `<li>` items, rendered inside the shared `<ul>`. */
		children: Snippet;
	};

	let { ariaLabel, ariaHidden = false, children }: Props = $props();
</script>

<!-- Shared shell for a section's tab strip (configure, workflow steps, events). It sits
	 directly under the section header and shares its background, so the two read as one
	 block. Items style themselves with `sectionTabClass`. -->
<nav
	class="border-border scrollbar-none px-gutter w-full shrink-0 overflow-x-auto border-b"
	aria-label={ariaLabel}
	aria-hidden={ariaHidden || undefined}
>
	<ul
		class="sm:max-w-admin flex min-w-max items-center gap-x-6 sm:mx-auto sm:w-full sm:min-w-0 sm:flex-wrap"
	>
		{@render children()}
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
