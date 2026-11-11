<script lang="ts">
	import { page } from '$app/state';
	import { m } from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime';
	import { REPORT_COLUMN_CLASS } from '$lib/reports/styles';
	import { cn } from '$lib/utils';
	import ReportCard from './ReportCard.svelte';
	import ReportHeader from './ReportHeader.svelte';
	import ReportHero from './ReportHero.svelte';
	import ReportPager from './ReportPager.svelte';
	import ReportThankYou from './ReportThankYou.svelte';
	import {
		cardHref,
		hasReportContent,
		readCardNumber,
		splitReportIntoPages
	} from './reportPages';

	let { data } = $props();
	let { conversation, report } = $derived(data);

	// Reports written before the summary and body were split keep everything in `summary`.
	let bodyContent = $derived(report.body ?? report.summary);
	let summaryContent = $derived(
		report.body != null && hasReportContent(report.summary) ? report.summary : undefined
	);

	let pages = $derived(
		splitReportIntoPages(bodyContent, (pageNumber) => m.report_page_number({ pageNumber }))
	);
	let cardNumber = $derived(readCardNumber(page.url, pages.length));
	let currentPage = $derived(cardNumber === undefined ? undefined : pages[cardNumber - 1]);
	let showingThankYou = $derived(cardNumber === pages.length + 1);

	let pageLinks = $derived(
		pages.map((reportPage, index) => ({
			label: reportPage.label,
			href: cardHref(page.url, index + 1)
		}))
	);

	let publishedOn = $derived(
		new Date(report.createdAt).toLocaleDateString(getLocale(), {
			day: 'numeric',
			month: 'long',
			year: 'numeric'
		})
	);
</script>

<svelte:head>
	<title>{m.report_final_report()} | {conversation.title}</title>
</svelte:head>

<main class="bg-background flex min-h-svh flex-col">
	{#if currentPage && cardNumber}
		<ReportHeader
			title={conversation.title}
			startHref={cardHref(page.url)}
			pages={pageLinks}
			activePageIndex={cardNumber - 1}
		/>
		<div class={cn(REPORT_COLUMN_CLASS, 'grow py-5')}>
			{#key cardNumber}
				<ReportCard content={currentPage.content} conversationId={conversation.id} />
			{/key}
		</div>
		<ReportPager
			backHref={cardHref(page.url, cardNumber > 1 ? cardNumber - 1 : undefined)}
			nextHref={cardHref(page.url, cardNumber + 1)}
		/>
	{:else if showingThankYou}
		<ReportHeader title={conversation.title} startHref={cardHref(page.url)} />
		<ReportThankYou
			title={conversation.title}
			shareUrl={page.url.origin + cardHref(page.url)}
		/>
	{:else}
		<ReportHero
			title={conversation.title}
			{publishedOn}
			conversationId={conversation.id}
			summary={summaryContent}
			readMoreHref={pages.length > 0 ? cardHref(page.url, 1) : undefined}
		/>
	{/if}
</main>
