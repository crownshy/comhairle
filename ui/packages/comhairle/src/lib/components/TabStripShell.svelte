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

<!-- Shared shell for the conversation layout's "Row 2" primary strip (section sub-tabs,
	 workflow steps, events). Owns the row's chrome — background, bottom border, gutter
	 alignment, horizontal scroll, wrap behaviour — so every strip and its loading skeleton
	 stay pixel-identical. `bg-secondary` is the Row 2 tone (Row 1 is `bg-background`, the
	 Row 3 sub-strip is `bg-accent`). -->
<nav
	class="border-border bg-secondary scrollbar-none w-full overflow-x-auto border-b"
	aria-label={ariaLabel}
	aria-hidden={ariaHidden || undefined}
>
	<!-- Left padding is one item's px-3.5 short of the full gutter so that every item's
		 text (each item carries px-3.5) starts on the gutter column. Compensating here rather
		 than with -ml-3.5 on the first item means wrapped rows' first items land on the gutter
		 too, not just row 1's. -->
	<ul
		class="flex min-w-max items-center gap-x-1.5 gap-y-0.5 py-1 pr-5 pl-[calc(var(--spacing-gutter)-0.875rem)] sm:w-full sm:min-w-0 sm:flex-wrap"
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
