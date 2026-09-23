<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';
	import '@carbon/charts-svelte/styles.css';
	import { format } from 'date-fns';
	import ReportBody from '$lib/reports/ReportBody.svelte';

	let { data } = $props();
	let { conversation, workflowSteps, report } = data;

	let polisSteps = $derived(workflowSteps.filter((step) => step.toolConfig.type === 'polis'));
	let showPolis = $state(false);

	let pageTitle = $derived(`${conversation.title} Report`);
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
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<main class="bg-primary/10 flex flex-col items-center overflow-hidden pb-20">
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
			{data.conversation.title}
		</h1>
		<p class="text-muted-foreground max-w-3xl text-lg leading-7">
			Created on {reportCreatedAt}
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
						Show Polis iframe1
					</label>
				</div>
			{/if}
		</div>
		{#if !showPolis}
			<nav
				class="flex w-full max-w-4xl flex-wrap justify-center gap-2"
				aria-label="Report sections"
			>
				<a
					href="#report-overview"
					class="bg-muted text-foreground hover:bg-accent rounded-full px-4 py-2 text-base font-medium transition-colors"
				>
					Overview
				</a>
				{#each reportPills as pill (pill.componentType)}
					{#if embeddedComponentTypes.has(pill.componentType)}
						<a
							href={`#report-${pill.componentType}`}
							class="bg-muted text-foreground hover:bg-accent rounded-full px-4 py-2 text-base font-medium transition-colors"
						>
							{pill.label}
						</a>
					{/if}
				{/each}
			</nav>
		{/if}
	</header>

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
</main>
