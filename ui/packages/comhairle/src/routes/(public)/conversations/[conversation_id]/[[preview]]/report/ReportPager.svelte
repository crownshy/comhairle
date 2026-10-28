<script lang="ts">
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { REPORT_COLUMN_CLASS } from '$lib/reports/styles';

	type Props = {
		backHref: string;
		nextHref: string;
		/** How far through the report the reader is, 0 to 1. */
		progress: number;
		/** The card's place in its section, such as "2 of 3". Left out for one-card sections. */
		position?: string;
	};

	let { backHref, nextHref, progress, position }: Props = $props();

	const PAGER_LINK_CLASS =
		'text-foreground inline-flex items-center gap-1 transition-transform active:scale-90 motion-reduce:transition-none motion-reduce:active:scale-100';
</script>

<!-- The hrefs arrive as props, built by cardHref from the current URL, so they are already
	resolved paths. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<!-- Back and Next only, the same corners as the step pager (ADR-0047, ADR-0048). The middle
	says where the reader is in the section and is not a control. -->
<div class="bg-background/80 sticky bottom-0 backdrop-blur">
	<progress
		class="bg-border [&::-webkit-progress-bar]:bg-border [&::-webkit-progress-value]:bg-primary [&::-moz-progress-bar]:bg-primary block h-1.5 w-full appearance-none border-0"
		value={progress}
		max="1"
		aria-label={m.report_final_report()}
	></progress>
	<nav class={cn(REPORT_COLUMN_CLASS, 'relative flex h-16 items-center')}>
		<a href={backHref} class={PAGER_LINK_CLASS} aria-label={m.previous()}>
			<ChevronLeft class="size-6 rtl:-scale-x-100" aria-hidden="true" />
		</a>
		{#if position}
			<span
				class="bg-accent text-accent-foreground absolute start-1/2 -translate-x-1/2 rounded-full px-3 py-1.5 text-sm font-medium rtl:translate-x-1/2"
			>
				{position}
			</span>
		{/if}
		<a href={nextHref} class={cn(PAGER_LINK_CLASS, 'text-primary ms-auto')}>
			<span class="text-base font-medium">{m.next()}</span>
			<ChevronRight class="text-foreground size-6 rtl:-scale-x-100" aria-hidden="true" />
		</a>
	</nav>
</div>
