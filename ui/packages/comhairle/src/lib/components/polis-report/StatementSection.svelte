<script lang="ts">
	import type { Snippet } from 'svelte';
	import { ChevronDown } from '@lucide/svelte';
	import ColumnFilterHeader from './ColumnFilterHeader.svelte';
	import * as Card from '$lib/components/ui/card';

	interface Props {
		/** Optional id on the root section (scroll anchor for deep-links). */
		id?: string;
		title: string;
		/** Trailing description text after the "N STATEMENTS" count, e.g. "with greater than 80%…". */
		description: string;
		/** Count shown as the coloured "N STATEMENTS" lead-in (main + low-quality). */
		count?: number;
		/** Accent colour for the count lead-in. */
		countAccent?: 'consensus' | 'difference' | 'all';
		/** Header label for the per-variant metric column, e.g. "Min Agree". */
		metricLabel: string;
		/** Number of opinion groups, sizing the trailing agree-rings column so the
		    header grid and the (separate) row grids share identical column widths. */
		groupCount?: number;
		/** Full main-list count. When it exceeds what's rendered, show the expander. */
		total?: number;
		/** Rows shown while collapsed (parent does the slicing). */
		collapsedCount?: number;
		/** Bindable expand state; parent slices its list on this. */
		expanded?: boolean;
		/** How many low-quality rows are hidden. 0 → no low-quality reveal. */
		lowQualityCount?: number;
		/** Bindable; reveal the low-quality rows. */
		showLowQuality?: boolean;
		excludePasses?: boolean;
		excludeHosts?: boolean;
		onExcludePassesChange?: (next: boolean) => void;
		onExcludeHostsChange?: (next: boolean) => void;
		children: Snippet;
		/** Optional action rendered on the title row, right-aligned (e.g. a Download button). */
		headerAction?: Snippet;
		/** Optional controls rendered between the description and the table (e.g. theme chips). */
		toolbar?: Snippet;
		/** Low-quality rows, rendered below the reveal toggle when expanded. */
		lowQuality?: Snippet;
	}

	let {
		id,
		title,
		description,
		count,
		countAccent = 'all',
		metricLabel,
		total,
		collapsedCount = 5,
		groupCount = 0,
		expanded = $bindable(false),
		lowQualityCount = 0,
		showLowQuality = $bindable(false),
		excludePasses = $bindable(false),
		excludeHosts = $bindable(false),
		onExcludePassesChange,
		onExcludeHostsChange,
		children,
		headerAction,
		toolbar,
		lowQuality
	}: Props = $props();

	const showExpander = $derived(total !== undefined && total > collapsedCount);

	// One real grid, defined here on the table wrapper; the header and every row are
	// `subgrid`s that adopt these exact tracks (see StatementRow). That's why columns
	// line up without any hand-tuned fr ratios: there's a single source of truth.
	//
	//   #        → auto: hugs the tid/"#" width
	//   Statement→ minmax(12rem,1fr): eats all leftover space (min 12rem so it never
	//              collapses; below that the wrapper scrolls horizontally on mobile)
	//   Author / metric / Agree%|rings → auto: each takes exactly the width its own
	//              content needs, no more. Even column-gap comes from `gap-x-*`.
	//
	// groupCount is no longer needed to size the rings column (`auto` fits the
	// circles for free), but stays in Props for call-site compatibility.
	const insightsCols = 'auto minmax(12rem,1fr) auto auto auto';

	const countAccentClass = $derived(
		countAccent === 'consensus'
			? 'text-primary'
			: countAccent === 'difference'
				? 'text-destructive'
				: 'text-primary'
	);
</script>

<section {id} class="flex scroll-mt-4 flex-col gap-4" style={`--insights-cols: ${insightsCols}`}>
	<div class="flex items-center justify-between gap-4">
		<h2 class="text-foreground text-lg leading-tight font-semibold">{title}</h2>
		{#if headerAction}
			<div class="shrink-0">{@render headerAction()}</div>
		{/if}
	</div>
	<p class="text-foreground text-base font-medium">
		{#if count !== undefined}<span class={`${countAccentClass} whitespace-nowrap`}
				>{count} STATEMENTS</span
			>&nbsp;{/if}{description}
	</p>

	{#if toolbar}
		{@render toolbar()}
	{/if}

	<!-- overflow-visible (overrides Card's default overflow) so the sticky header
	     can pin to the layout scroll container instead of being clipped. p-0 so the
	     header/rows sit flush against the card edge. -->
	<Card.Root
		class="hover:border-muted-foreground/40 overflow-visible rounded-[20px] p-0 shadow-sm transition-colors duration-200"
	>
		<!-- The single owning grid: its columns are defined once here and every direct
		     child (header, each row, the reveal buttons) spans all of them. Rows adopt
		     these exact tracks via `subgrid`, so columns align with zero fr juggling.
		     overflow-x-auto lets the table scroll sideways on a narrow panel instead of
		     crushing columns; md:overflow-visible keeps desktop untouched so the sticky
		     header still pins vertically. -->
		<div
			class="grid [grid-template-columns:var(--insights-cols)] gap-x-4 overflow-x-auto md:overflow-visible [&>*:last-child]:rounded-b-[20px]"
		>
			<div
				class="bg-card text-foreground sticky top-0 z-10 col-span-full grid grid-cols-subgrid items-center rounded-t-[20px] py-3 pr-4 pl-5 text-sm font-semibold uppercase"
			>
				<div>#</div>
				<div>Statement</div>
				<ColumnFilterHeader
					label="Author"
					optionLabel="Include host statements"
					checked={!excludeHosts}
					onchange={(included) => {
						excludeHosts = !included;
						onExcludeHostsChange?.(excludeHosts);
					}}
				/>
				<div class="text-center whitespace-nowrap">{metricLabel}</div>
				<ColumnFilterHeader
					label="Agree %"
					optionLabel="Include passes"
					align="right"
					checked={!excludePasses}
					onchange={(included) => {
						excludePasses = !included;
						onExcludePassesChange?.(excludePasses);
					}}
				/>
			</div>

			{@render children()}

			{#if showExpander}
				<button
					type="button"
					onclick={() => (expanded = !expanded)}
					class="bg-muted/40 hover:bg-muted/70 text-foreground/70 col-span-full flex items-center justify-center gap-2 py-5 text-base font-normal transition-colors"
				>
					{expanded ? 'Show fewer statements' : `See all ${total} statements`}
					<ChevronDown
						class={`text-primary size-4 transition-transform ${expanded ? 'rotate-180' : ''}`}
					/>
				</button>
			{/if}

			{#if lowQualityCount > 0}
				<button
					type="button"
					onclick={() => (showLowQuality = !showLowQuality)}
					class="text-muted-foreground bg-muted/30 hover:bg-muted/50 col-span-full flex items-center justify-center gap-2 py-5 text-base transition-colors"
				>
					{showLowQuality
						? 'Hide low data quality statements'
						: `See ${lowQualityCount} statement${lowQualityCount === 1 ? '' : 's'} with low data quality`}
					<ChevronDown
						class={`size-4 transition-transform ${showLowQuality ? 'rotate-180' : ''}`}
					/>
				</button>
				{#if showLowQuality && lowQuality}
					{@render lowQuality()}
				{/if}
			{/if}
		</div>
	</Card.Root>
</section>
