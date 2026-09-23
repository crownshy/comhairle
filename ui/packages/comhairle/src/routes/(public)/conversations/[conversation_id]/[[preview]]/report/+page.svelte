<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import * as Tabs from '$lib/components/ui/tabs';
	import '@carbon/charts-svelte/styles.css';
	import StatsBar from '$lib/components/StatsBar.svelte';
	import { formatDistanceToNow } from 'date-fns';
	import Speech from 'lucide-svelte/icons/speech';
	import Drama from 'lucide-svelte/icons/drama';
	import Scroll from 'lucide-svelte/icons/scroll-text';
	import ReportBody from '$lib/reports/ReportBody.svelte';

	let { data } = $props();
	let { conversation, workflowSteps, report } = data;

	let pageTitle = $derived(`${conversation.title} Report`);

	let stats = [
		{
			name: 'Participants',
			amount: 30
		},
		{
			name: 'Time Spent',
			amount: 30
		},
		{
			name: 'Completed',
			amount: 1
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
			A record of what participants shared, where they found common ground, and where views
			remain different.
		</p>
	</header>

	<Tabs.Root value="Overview" class="space-y-4">
		<Tabs.List>
			<Tabs.Trigger value="Overview">Overview</Tabs.Trigger>
			{#each workflowSteps as step (step.id)}
				<Tabs.Trigger value={step.id}>{step.name}</Tabs.Trigger>
			{/each}
			<Tabs.Trigger value="Feedback">Feedback</Tabs.Trigger>
			<Tabs.Trigger value="ModerationReport">Moderation Report</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="Overview" class="space-y-4">
			<StatsBar {stats} />
			<h2 class="text-xl font-bold">Key Takeaways</h2>

			<div class="mb-4">
				<ReportBody content={report.summary} conversationId={conversation.id} />
			</div>

			<h2 class="text-xl font-bold">Impacts</h2>
			<ul class="flex flex-col gap-4 divide-y-3 divide-solid divide-gray-200">
				{#each report.impacts as impact (impact.id)}
					<li class="flex flex-col gap-2 border-solid p-4">
						<div class="flex flex-row justify-between">
							<div class="flex flex-row gap-2">
								{#if impact.kind === 'policy'}
									<Scroll />
								{:else if impact.kind === 'debate'}
									<Drama />
								{:else}
									<Speech />
								{/if}
								<h3 class="font-bold">{impact.title}</h3>
							</div>
							<span>{formatDistanceToNow(impact.createdAt, { addSuffix: true })}</span
							>
						</div>
						<p>
							{impact.details}
						</p>
					</li>
				{/each}
			</ul>
		</Tabs.Content>

		{#each workflowSteps as step (step.id)}
			<Tabs.Content value={step.id} class="spage-y-4">
				{#if step.toolConfig.type === 'polis'}
					<iframe
						class="h-[100vh] w-full border-none"
						src="https://poliscommunity.crown-shy.com/report/r4hrfdtemrjsxbn3ieyyb"
					>
					</iframe>
				{:else}
					<h1>Placeholder for results of {step.title}</h1>
				{/if}
			</Tabs.Content>
		{/each}

		<Tabs.Content value="Moderation">
			<h2 class="text-xl2">Moderation</h2>
		</Tabs.Content>

		<Tabs.Content value="Feedback">
			<h3 class="mb-4 text-xl font-bold">Facilitator Feedback</h3>
			<h3 class="mb-4 text-xl font-bold">Participant feedback</h3>
			{#each report.facilitatorFeedback as feedback (feedback.id)}
				<article
					class="relative mb-4 rounded-lg border-l-4 border-blue-500 bg-gray-100 p-6 shadow-md dark:border-blue-400 dark:bg-gray-800"
				>
					<span
						class="absolute top-2 left-3 font-serif text-5xl text-blue-500 dark:text-blue-400"
						>"</span
					>
					{feedback.content}
					<span
						class="absolute right-3 bottom-2 font-serif text-5xl text-blue-500 dark:text-blue-400"
						>"</span
					>
				</article>
			{/each}
		</Tabs.Content>
	</Tabs.Root>
</main>
