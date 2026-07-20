<script lang="ts">
	import { ArrowRight } from '@lucide/svelte';
	import type { ThemeSummary } from '$lib/tools/polis/reportTypes';

	interface Props {
		summary: ThemeSummary;
		/** Statement count of the largest theme — the bar's full-scale reference. */
		barMax: number;
		onclick?: () => void;
	}

	let { summary, barMax, onclick }: Props = $props();

	// Bar length ranks this theme against the largest theme (100% = the top theme),
	// per the "very simple, based on manually added themes" spec. Theme counts don't
	// sum to totalStatements (a statement can carry many themes), so share-of-total
	// would be a meaningless denominator. No controversy signal here.
	const pct = $derived(barMax > 0 ? (summary.statementCount / barMax) * 100 : 0);
</script>

<div
	class="border-border group hover:bg-muted/40 grid grid-cols-[10rem_3rem_1fr_2.5rem] items-center gap-6 border-b px-2 py-4 transition-colors duration-150"
>
	<div class="text-foreground truncate text-base font-bold">{summary.theme}</div>
	<div class="text-foreground text-center text-base font-bold tabular-nums">
		{summary.statementCount}
	</div>
	<div class="bg-muted h-2 w-full rounded-full">
		<div class="bg-primary h-full rounded-full" style={`width:${pct}%`}></div>
	</div>
	<button
		type="button"
		{onclick}
		aria-label={`Open ${summary.theme}`}
		class="bg-muted text-primary group-hover:bg-primary/15 flex size-8 shrink-0 cursor-pointer items-center justify-center justify-self-end rounded-full transition-all duration-150 hover:scale-110 active:scale-95"
	>
		<ArrowRight class="size-4" />
	</button>
</div>
