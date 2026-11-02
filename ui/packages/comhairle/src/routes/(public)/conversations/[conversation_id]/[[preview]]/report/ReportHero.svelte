<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import Calendar from '@lucide/svelte/icons/calendar';
	import { Button } from '$lib/components/ui/button';
	import { m } from '$lib/paraglide/messages';
	import ReportBody from '$lib/reports/ReportBody.svelte';
	import { cn } from '$lib/utils';
	import ExpandableContent from './ExpandableContent.svelte';
	import { REPORT_COLUMN_CLASS, REPORT_EYEBROW_CLASS, REPORT_TEXT_CLASS } from './styles';

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

<div class={cn(REPORT_COLUMN_CLASS, 'flex flex-col items-center gap-5 py-8')}>
	<p class="bg-accent text-primary rounded-full px-4 py-1 text-base font-medium">
		{m.report_final_report()}
	</p>
	<h1 class="text-foreground text-center text-4xl leading-tight font-bold">{title}</h1>
	<p class="text-muted-foreground flex items-center gap-2 text-sm font-medium">
		<Calendar class="size-4" aria-hidden="true" />
		{publishedOn}
	</p>

	{#if summary}
		<section
			class="bg-card/80 border-border w-full rounded-t-lg rounded-b-3xl border px-4 py-6"
			aria-labelledby="executive-summary"
		>
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

	{#if readMoreHref}
		<Button href={readMoreHref} size="lg" class="mt-3 h-12 w-full text-lg font-semibold">
			{m.report_read_more()}
			<ArrowRight class="size-5 rtl:-scale-x-100" aria-hidden="true" />
		</Button>
	{/if}
</div>
