<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import * as Tabs from '$lib/components/ui/tabs';
	import '@carbon/charts-svelte/styles.css';
	import { formatDistanceToNow } from 'date-fns';
	import Speech from 'lucide-svelte/icons/speech';
	import Drama from 'lucide-svelte/icons/drama';
	import Scroll from 'lucide-svelte/icons/scroll-text';
	import ReportBody from '$lib/reports/ReportBody.svelte';

	let { data } = $props();
	let { conversation, workflowSteps, report } = data;

	//find polis step
	let polisSteps = $derived(workflowSteps.filter((step) => step.toolConfig.type === 'polis'));

	let pageTitle = $derived(`${conversation.title} Report`);

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
			<!-- this is to be ported in later-->
			Created on 23 September, 2026
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
		</div>
	</header>

	<Tabs.Root value="Overview" class="w-full max-w-[1400px] space-y-4 px-5 md:px-10">
		<Tabs.List>
			<Tabs.Trigger value="Overview">Overview</Tabs.Trigger>
			{#each polisSteps as step (step.id)}
				<Tabs.Trigger value={step.id}>{step.name}</Tabs.Trigger>
			{/each}
		</Tabs.List>
		<Tabs.Content value="Overview" class="space-y-4">
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

		{#each polisSteps as step (step.id)}
			<Tabs.Content value={step.id} class="space-y-4">
				<iframe
					class="h-[100vh] w-full border-none"
					src="https://poliscommunity.crown-shy.com/report/r4hrfdtemrjsxbn3ieyyb"
				>
				</iframe>
			</Tabs.Content>
		{/each}
	</Tabs.Root>
</main>
