<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import Calendar from '@lucide/svelte/icons/calendar';
	import { Button } from '$lib/components/ui/button';
	import { m } from '$lib/paraglide/messages';
	import ReportBody from '$lib/reports/ReportBody.svelte';
	import { PAGER_BAR_SURFACE_CLASS, PAGER_ROW_CLASS } from '$lib/components/pager/pager';
	import { cn } from '$lib/utils';
	import ExpandableContent from './ExpandableContent.svelte';
	import {
		REPORT_COLUMN_CLASS,
		REPORT_EYEBROW_CLASS,
		REPORT_TEXT_CLASS
	} from '$lib/reports/styles';

	type Props = {
		title: string;
		publishedOn: string;
		conversationId: string;
		/** The executive summary, left out when the admin hasn't written one. */
		summary?: string;
		/** Where Read more goes, left out when the report has no body yet. */
		readMoreHref?: string;
	};

	let { title, publishedOn, conversationId, summary, readMoreHref }: Props = $props();
</script>

<div class={cn(REPORT_COLUMN_CLASS, 'flex grow flex-col items-center gap-5 py-8')}>
	<p class="bg-accent text-primary rounded-full px-4 py-1 text-base font-medium">
		{m.report_final_report()}
	</p>
	<h1 class="text-foreground text-center text-4xl leading-tight font-bold">{title}</h1>
	<p class="text-muted-foreground flex items-center gap-2 text-sm font-medium">
		<Calendar class="size-4" aria-hidden="true" />
		{publishedOn}
	</p>

	{#if summary}
		<section class="w-full" aria-labelledby="executive-summary">
			<h2 id="executive-summary" class={REPORT_EYEBROW_CLASS}>
				{m.report_executive_summary()}
			</h2>
			<div class="pt-2">
				<ExpandableContent collapsedHeight={16}>
					<div class={REPORT_TEXT_CLASS}>
						<ReportBody
							content={summary}
							{conversationId}
							headingIdPrefix="summary-section"
						/>
					</div>
				</ExpandableContent>
			</div>
		</section>
	{/if}
</div>

<!-- Pinned to the bottom of the screen, like the participant step bar, so the way on is
	always in reach however long the summary is. -->
{#if readMoreHref}
	<div class={cn(PAGER_BAR_SURFACE_CLASS, 'sticky bottom-0 z-10')}>
		<div class={cn(REPORT_COLUMN_CLASS, PAGER_ROW_CLASS)}>
			<Button
				href={readMoreHref}
				size="lg"
				class="h-12 w-full text-lg font-semibold active:scale-[0.97] motion-reduce:active:scale-100"
			>
				{m.report_read_more()}
				<ArrowRight class="size-5 rtl:-scale-x-100" aria-hidden="true" />
			</Button>
		</div>
	</div>
{/if}
