<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabStripShell from '$lib/components/TabStripShell.svelte';

	type Props = {
		/**
		 * Render a leading icon + label placeholder, matching strips whose first item is an
		 * icon tab (design's "Design", events' "All events"). Configure's plain sub-tabs omit it.
		 */
		leadingIcon?: boolean;
		/**
		 * Approximate widths (in `rem`) of the placeholder tab labels, in display order.
		 * The count and widths are cosmetic; they only shape the loading row.
		 */
		widths?: number[];
	};

	// Defaults roughly match the Configure sub-tabs (Details / Content / Access / Team).
	let { leadingIcon = false, widths = [3.5, 4, 3, 2.75] }: Props = $props();
</script>

<!-- Placeholder for the injected "Row 2" primary strip: reserves the row's height so a
	 refresh doesn't shift the layout. Reuses TabStripShell to stay pixel-identical to the
	 real strips. -->
<TabStripShell ariaHidden>
	{#if leadingIcon}
		<!-- Leading icon tab (bleeds into the gutter like the real strips' first item). -->
		<li class="-ml-3.5 flex h-9 items-center gap-1.5 px-3.5">
			<Skeleton class="bg-primary/15 size-4 rounded" />
			<Skeleton class="bg-primary/15 h-4 w-14 rounded-md" />
		</li>
	{/if}
	{#each widths as width, i (i)}
		<!-- Without a leading icon, the first pill bleeds into the gutter (-ml-3.5 cancels
			 its own px-3.5) to align with the real strip's first tab. Tinted with the brand
			 color so the shimmer reads against the secondary strip instead of gray-on-gray. -->
		<li class="flex h-9 items-center px-3.5" class:-ml-3.5={!leadingIcon && i === 0}>
			<Skeleton class="bg-primary/15 h-4 rounded-md" style="width: {width}rem" />
		</li>
	{/each}
</TabStripShell>
