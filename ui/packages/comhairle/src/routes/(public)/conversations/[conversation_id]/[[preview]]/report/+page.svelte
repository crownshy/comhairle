<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import '@carbon/charts-svelte/styles.css';
	import { format } from 'date-fns';
	import ArrowUp from 'lucide-svelte/icons/arrow-up';
	import ReportBody from '$lib/reports/ReportBody.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';

	let { data } = $props();
	let { conversation, workflowSteps, report } = data;

	let polisSteps = $derived(workflowSteps.filter((step) => step.toolConfig?.type === 'polis'));
	let showPolis = $state(false);

	let reportCreatedAt = $derived(format(new Date(report.createdAt), 'd MMMM yyyy'));
	const reportPills = [
		{ componentType: 'polis-area-consensus', label: 'Areas of agreement' },
		{ componentType: 'polis-area-disagreement', label: 'Areas of disagreement' },
		{ componentType: 'polis-opinion-groups', label: 'Opinion groups' }
	];
	const embeddedComponentTypes = $derived.by<Set<string>>(() => {
		try {
			const document: unknown = JSON.parse(report.summary);
			if (
				!document ||
				typeof document !== 'object' ||
				!Array.isArray((document as { content?: unknown[] }).content)
			) {
				return new Set();
			}

			return new Set(
				(
					document as { content: { type?: string; attrs?: Record<string, unknown> }[] }
				).content
					.filter((node) => node.type === 'reportComponentEmbed')
					.map((node) => String(node.attrs?.componentType ?? ''))
			);
		} catch {
			return new Set();
		}
	});
	const reportNavItems = $derived([
		{ id: 'report-overview', label: 'Overview' },
		...reportPills
			.filter((pill) => embeddedComponentTypes.has(pill.componentType))
			.map((pill) => ({ id: `report-${pill.componentType}`, label: pill.label }))
	]);
	let activeReportSection = $state('report-overview');

	let stats = [
		{
			name: 'Participants took part',
			amount: 300
		},
		{
			name: 'Statements submitted',
			amount: 319
		},
		{
			name: 'Opinion groups identified',
			amount: 3
		}
	];

	$effect(() => {
		if (showPolis || typeof IntersectionObserver === 'undefined') return;
		activeReportSection = 'report-overview';

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (entry.isIntersecting) activeReportSection = entry.target.id;
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

	console.log('data: ', data);
	console.log('polisSteps : ', polisSteps);
</script>

<svelte:head>
	<title>Report | {conversation.title}</title>
</svelte:head>

<main class="bg-background flex flex-col items-center pb-20">
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
		<!-- We will likely need a space to write a short summary here -->
		<p class="text-foreground max-w-3xl text-lg leading-7">
			The consultation was organised by the project team to gather community perspectives and
			create an inclusive space for people to share their views. It ran over 2.5 weeks in an
			accessible format, allowing participants to contribute through basic survey and
			interactive smart discussions where they can vote agree/disagree/neutral on other's
			views. The consultation brought together <b>300 participants</b>, who submitted a total
			of <b>319 statements</b>. Analysis of the responses identified<b
				>three distinct opinion groups</b
			>, highlighting the different perspectives represented across the participants.
		</p>
		<div class="mt-6 grid w-full max-w-4xl grid-cols-1 gap-4 sm:grid-cols-3">
			{#each stats as stat (stat.name)}
				<div
					class="bg-card ring-border content-center items-center rounded-xl p-5 text-center shadow-sm ring-1 md:h-44 md:p-6"
				>
					<p
						class="text-card-foreground text-2xl leading-8 font-semibold tabular-nums md:text-3xl md:leading-9"
					>
						{stat.amount}
					</p>
					<p
						class="text-muted-foreground mt-1 text-base leading-6 font-semibold md:mt-1.5 md:text-lg md:leading-7"
					>
						{stat.name}
					</p>
				</div>
			{/each}

			{#if polisSteps.length > 0}
				<div class="mt-2 flex items-center gap-3 self-start">
					<Switch id="show-polis" bind:checked={showPolis} />
					<label for="show-polis" class="text-foreground text-base font-medium">
						Show Polis iframe
					</label>
				</div>
			{/if}
		</div>
	</header>

	{#if !showPolis}
		<div class="bg-card/95 sticky top-0 z-20 w-full px-5 py-3 shadow-md backdrop-blur md:px-10">
			<nav
				class="mx-auto flex max-w-4xl gap-2 overflow-x-auto md:justify-center"
				aria-label="Report sections"
			>
				{#each reportNavItems as item (item.id)}
					<a
						href={`#${item.id}`}
						onclick={(event) => scrollToSection(event, item.id)}
						aria-current={activeReportSection === item.id ? 'location' : undefined}
						class="shrink-0 rounded-full px-4 py-2 text-base font-medium shadow-sm transition-colors {activeReportSection ===
						item.id
							? 'bg-primary text-primary-foreground'
							: 'bg-muted text-foreground hover:bg-accent'}"
					>
						{item.label}
					</a>
				{/each}
			</nav>
		</div>
	{/if}

	<div class="w-full px-10 font-sans {showPolis ? 'max-w-[1400px]' : 'max-w-4xl'}">
		{#if showPolis}
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
				id="report-overview"
				class="mb-4 max-w-4xl scroll-mt-8 text-lg leading-8 [&_.tiptap]:text-lg [&_.tiptap]:leading-8 [&_.tiptap_h1]:text-xl [&_.tiptap_h1]:md:text-3xl [&_.tiptap_h2]:text-2xl [&_.tiptap_h2]:leading-8 [&_.tiptap_h2]:font-semibold [&_.tiptap_h3]:text-xl [&_.tiptap_h3]:leading-7 [&_.tiptap_h3]:font-semibold [&_.tiptap_li]:my-2 [&_.tiptap_p]:my-4 [&_.tiptap_p]:text-lg [&_.tiptap_p]:leading-8"
			>
				<ReportBody content={report.summary} conversationId={conversation.id} />
			</div>
		{/if}
	</div>

	<Button
		class="fixed bottom-5 left-1/2 z-30 -translate-x-1/2 rounded-full shadow-lg md:bottom-8"
		onclick={scrollToTop}
	>
		<ArrowUp class="size-4" />
		Back to top
	</Button>
</main>
