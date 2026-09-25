<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import '@carbon/charts-svelte/styles.css';
	import { format } from 'date-fns';
	import ArrowUp from 'lucide-svelte/icons/arrow-up';
	import ReportBody from '$lib/reports/ReportBody.svelte';
	import { onMount } from 'svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';

	let { data } = $props();
	let { conversation, workflowSteps, report } = $derived(data);
	// Previous reports stored the full document in summary and have no body yet.
	let reportBody = $derived(report.body ?? report.summary);
	let reportSummary = $derived(report.body == null ? '' : report.summary);

	let polisSteps = $derived(workflowSteps.filter((step) => step.toolConfig?.type === 'polis'));
	let showPolisiframe = $state(false);

	let reportCreatedAt = $derived(format(new Date(report.createdAt), 'd MMMM yyyy'));
	// The sticky navigation container, used to keep the active pill visible.
	let reportNavElement: HTMLElement;
	// The report content container, used to calculate Back to top visibility.
	let reportElement: HTMLElement;
	let showBackToTop = $state(false);

	let activeReportSection = $state('');
	type ReportNode = {
		type?: string;
		attrs?: {
			level?: number;
		};
		content?: ReportNode[];
		text?: string;
	};
	/**
	 * The public report navigation is derived from top-level H2 headings in the report body.
	 * Each heading receives a unique, predictable ID so the navigation and rendered section
	 * can link to the same anchor.
	 */
	const reportNavItems = $derived.by(() => {
		const items: { id: string; label: string }[] = [];

		try {
			const bodyDocument: unknown = JSON.parse(reportBody);

			if (
				!bodyDocument ||
				typeof bodyDocument !== 'object' ||
				!Array.isArray((bodyDocument as { content?: unknown[] }).content)
			) {
				return items;
			}

			const nodes = (bodyDocument as { content: ReportNode[] }).content;
			const usedIds: Record<string, number> = {};

			for (const node of nodes) {
				if (node.type !== 'heading' || node.attrs?.level !== 2) continue;

				const label = (node.content ?? [])
					.map((child) => child.text ?? '')
					.join('')
					.trim();

				if (!label) continue;

				const slug = label
					.toLowerCase()
					.replace(/[^a-z0-9]+/g, '-')
					.replace(/^-|-$/g, '');

				const baseId = `section-${slug || 'untitled'}`;
				const count = usedIds[baseId] ?? 0;
				usedIds[baseId] = count + 1;

				items.push({
					id: count === 0 ? baseId : `${baseId}-${count + 1}`,
					label
				});
			}
		} catch {
			return items;
		}

		return items;
	});

	$effect(() => {
		if (showPolisiframe || typeof IntersectionObserver === 'undefined') return;
		activeReportSection = reportNavItems[0]?.id ?? '';

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (entry.isIntersecting) {
						activeReportSection = entry.target.id;
						revealActiveNavItem(entry.target.id);
					}
				}
			},
			{ rootMargin: '-20% 0px -70%', threshold: 0 }
		);

		for (const item of reportNavItems) {
			const section = document.getElementById(item.id);
			if (section) observer.observe(section);
		}

		return () => observer.disconnect();
	});

	function scrollBehavior() {
		return window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth';
	}

	function scrollToSection(event: MouseEvent, sectionId: string) {
		event.preventDefault();
		document.getElementById(sectionId)?.scrollIntoView({
			behavior: scrollBehavior(),
			block: 'start'
		});
	}

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: scrollBehavior() });
	}

	onMount(() => {
		function updateBackToTopVisibility() {
			const reportScrollHeight = reportElement.offsetHeight - window.innerHeight;
			const reportScrollPosition = window.scrollY - reportElement.offsetTop;
			showBackToTop =
				reportScrollHeight > 0 && reportScrollPosition / reportScrollHeight >= 0.3;
		}

		window.addEventListener('scroll', updateBackToTopVisibility, { passive: true });
		window.addEventListener('resize', updateBackToTopVisibility);
		updateBackToTopVisibility();

		return () => {
			window.removeEventListener('scroll', updateBackToTopVisibility);
			window.removeEventListener('resize', updateBackToTopVisibility);
		};
	});
	function revealActiveNavItem(sectionId: string) {
		const activeItem = reportNavElement?.querySelector<HTMLElement>(
			`[data-report-nav-id="${sectionId}"]`
		);

		if (!activeItem) return;

		reportNavElement.scrollTo({
			left:
				activeItem.offsetLeft - (reportNavElement.clientWidth - activeItem.offsetWidth) / 2,
			behavior: scrollBehavior()
		});
	}
</script>

<svelte:head>
	<title>Report | {conversation.title}</title>
</svelte:head>

<main bind:this={reportElement} class="bg-background flex flex-col items-center pb-20">
	<header
		class="bg-card mb-8 flex w-full flex-col items-center gap-4 rounded-xl px-5 pt-10 pb-8 text-center md:mb-16 md:px-24 md:pt-24 md:pb-12"
	>
		<Badge
			class="bg-primary/10 text-muted-foreground rounded-3xl px-4 py-2 text-lg font-semibold"
		>
			Final report
		</Badge>
		<h1
			class="text-foreground max-w-4xl text-3xl leading-10 font-bold md:text-5xl md:leading-[52px]"
		>
			{conversation.title}
		</h1>
		<p class="text-muted-foreground max-w-3xl text-lg leading-7">
			Created on {reportCreatedAt}
		</p>
		{#if reportSummary}
			<div class="text-foreground w-full max-w-3xl text-lg leading-7">
				<ReportBody
					content={reportSummary}
					conversationId={conversation.id}
					headingIdPrefix="summary-section"
				/>
			</div>
		{/if}
		<div class="mt-6 grid w-full max-w-4xl grid-cols-1 gap-4 sm:grid-cols-3">
			{#if polisSteps.length > 0}
				<div class="mt-2 flex items-center gap-3 self-start">
					<Switch id="show-polis" bind:checked={showPolisiframe} />
					<label for="show-polis" class="text-foreground text-base font-medium">
						Show Polis iframe
					</label>
				</div>
			{/if}
		</div>
	</header>

	{#if !showPolisiframe && reportNavItems.length > 0}
		<div class="bg-card/95 sticky top-0 z-20 w-full px-5 py-3 shadow-md backdrop-blur md:px-10">
			<nav
				bind:this={reportNavElement}
				class="mx-auto max-w-4xl scroll-px-2 overflow-x-auto px-2"
				aria-label="Report sections"
			>
				<div class="flex w-max min-w-full gap-2 md:justify-center">
					{#each reportNavItems as item (item.id)}
						<a
							href={`#${item.id}`}
							onclick={(event) => scrollToSection(event, item.id)}
							aria-current={activeReportSection === item.id ? 'location' : undefined}
							data-report-nav-id={item.id}
							class="shrink-0 rounded-full px-4 py-2 text-base font-medium shadow-sm transition-colors {activeReportSection ===
							item.id
								? 'bg-primary text-primary-foreground'
								: 'bg-muted text-foreground hover:bg-accent'}"
						>
							{item.label}
						</a>
					{/each}
				</div>
			</nav>
		</div>
	{/if}

	<div class="w-full px-10 font-sans {showPolisiframe ? 'max-w-[1400px]' : 'max-w-4xl'}">
		{#if showPolisiframe}
			{#each polisSteps as step (step.id)}
				<iframe
					class="h-[1000vh] w-full border-none"
					title={`${step.name} results`}
					src="https://poliscommunity.crown-shy.com/report/r4hrfdtemrjsxbn3ieyyb"
				>
				</iframe>
			{/each}
		{:else}
			<div
				class="mb-4 max-w-4xl text-lg leading-8 [&_.tiptap]:text-lg [&_.tiptap]:leading-8 [&_.tiptap_h1]:text-xl [&_.tiptap_h1]:md:text-3xl [&_.tiptap_h2]:text-2xl [&_.tiptap_h2]:leading-8 [&_.tiptap_h2]:font-semibold [&_.tiptap_h3]:text-xl [&_.tiptap_h3]:leading-7 [&_.tiptap_h3]:font-semibold [&_.tiptap_li]:my-2 [&_.tiptap_p]:my-4 [&_.tiptap_p]:text-lg [&_.tiptap_p]:leading-8"
			>
				<ReportBody content={reportBody} conversationId={conversation.id} />
			</div>
		{/if}
	</div>

	{#if showBackToTop}
		<Button
			class="fixed bottom-5 left-1/2 z-30 -translate-x-1/2 rounded-full shadow-lg md:bottom-8"
			onclick={scrollToTop}
		>
			<ArrowUp class="size-4" />
			Back to top
		</Button>
	{/if}
</main>
