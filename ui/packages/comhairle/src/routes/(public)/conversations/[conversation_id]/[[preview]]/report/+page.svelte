<script lang="ts">
	import { page } from '$app/state';
	import { m } from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime';
	import ReportCard from './ReportCard.svelte';
	import ReportHeader from './ReportHeader.svelte';
	import ReportHero from './ReportHero.svelte';
	import ReportPager from './ReportPager.svelte';
	import ReportThankYou from './ReportThankYou.svelte';
	import {
		cardHref,
		hasReportContent,
		readCardNumber,
		splitReportIntoSections,
		toReportPages
	} from './reportPages';

	let { data } = $props();
	let { conversation, report } = $derived(data);

	// Reports written before the summary and body were split keep everything in `summary`.
	let bodyContent = $derived(report.body ?? report.summary);
	let summaryContent = $derived(
		report.body != null && hasReportContent(report.summary) ? report.summary : undefined
	);

	let sections = $derived(splitReportIntoSections(bodyContent, m.report_introduction()));
	let pages = $derived(toReportPages(sections));
	let cardNumber = $derived(readCardNumber(page.url, pages.length));
	let currentPage = $derived(cardNumber === undefined ? undefined : pages[cardNumber - 1]);
	let showingThankYou = $derived(cardNumber === pages.length + 1);

	let sectionLinks = $derived(
		sections.map((section, sectionIndex) => ({
			label: section.label,
			href: cardHref(page.url, pages.findIndex((p) => p.sectionIndex === sectionIndex) + 1)
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
			sections={sectionLinks}
			activeSectionIndex={currentPage.sectionIndex}
		/>
		<div class="mx-auto w-full max-w-2xl grow px-5 py-5">
			{#key cardNumber}
				<ReportCard
					title={currentPage.title}
					content={currentPage.content}
					conversationId={conversation.id}
				/>
			{/key}
		</div>
		<ReportPager
			backHref={cardHref(page.url, cardNumber > 1 ? cardNumber - 1 : undefined)}
			nextHref={cardHref(page.url, cardNumber + 1)}
			progress={cardNumber / pages.length}
			position={currentPage.cardCount > 1
				? m.report_card_x_of_y({ x: currentPage.cardIndex + 1, y: currentPage.cardCount })
				: undefined}
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
