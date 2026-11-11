<script lang="ts">
	import PagerButton from '$lib/components/pager/PagerButton.svelte';
	import { PAGER_BAR_SURFACE_CLASS, PAGER_ROW_CLASS } from '$lib/components/pager/pager';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { REPORT_COLUMN_CLASS } from '$lib/reports/styles';

	type Props = {
		backHref: string;
		nextHref: string;
	};

	let { backHref, nextHref }: Props = $props();
</script>

<!-- The participant step pager's buttons and bar (ADR-0047, ADR-0048), as links so browser
	Back steps back a page (ADR-0058). -->
<div class={cn(PAGER_BAR_SURFACE_CLASS, 'sticky bottom-0 z-10')}>
	<nav class={cn(REPORT_COLUMN_CLASS, PAGER_ROW_CLASS)}>
		<PagerButton direction="back" label={m.pager_back()} href={backHref} />
		<PagerButton direction="forward" label={m.next()} href={nextHref} />
	</nav>
</div>
