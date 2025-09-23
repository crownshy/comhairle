<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Card from '$lib/components/ui/card';
	import '@carbon/charts-svelte/styles.css';
	import { BarChartSimple } from '@carbon/charts-svelte';
	import { formatDistanceToNow } from 'date-fns';
	import Speech from 'lucide-svelte/icons/speech';
	import Drama from 'lucide-svelte/icons/drama';
	import Scroll from 'lucide-svelte/icons/scroll-text';

	let { data } = $props();
	console.log(data);
	let { conversation, workflow_steps, workflow_stats, report } = data;
</script>

<div class="pt-10">
	<h1 class="mb-4 text-4xl">{conversation.title} report</h1>
	<Tabs.Root value="Overview" class="space-y-4">
		<Tabs.List>
			<Tabs.Trigger value="Overview">Overview</Tabs.Trigger>
			{#each workflow_steps as step}
				<Tabs.Trigger value={step.id}>{step.name}</Tabs.Trigger>
			{/each}
			<Tabs.Trigger value="Feedback">Feedback</Tabs.Trigger>
			<Tabs.Trigger value="ModerationReport">Moderation Report</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="Overview" class="space-y-4">
			<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
				<Card.Root>
					<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
						<Card.Title class="text-sm font-medium">Total Participants</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="text-2xl font-bold">{workflow_stats.total_users}</div>
					</Card.Content>
				</Card.Root>
				<Card.Root>
					<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
						<Card.Title class="text-sm font-medium">Average Time Spent</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="text-2xl font-bold">30</div>
						<p class="text-muted-foreground text-xs">minutes</p>
					</Card.Content>
				</Card.Root>
				<Card.Root>
					<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
						<Card.Title class="text-sm font-medium">Statements Entered</Card.Title>
					</Card.Header>
					<Card.Content>
						<div class="text-2xl font-bold">33</div>
					</Card.Content>
				</Card.Root>
			</div>
			<h2 class="text-xl font-bold">Key Takeaways</h2>

			<p class="mb-4">
				{report.summary}
			</p>

			<h2 class="text-xl font-bold">Impacts</h2>
			<ul class="flex flex-col gap-4 divide-y-3 divide-solid divide-gray-200">
				{#each report.impacts as impact}
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
							<span>{formatDistanceToNow(impact.created_at, { addSuffix: true })}</span>
						</div>
						<p>
							{impact.details}
						</p>
					</li>
				{/each}
			</ul>
		</Tabs.Content>

		{#each workflow_steps as step}
			<Tabs.Content value={step.id} class="spage-y-4">
				<h1>Placeholder for results of {step.title}</h1>
			</Tabs.Content>
		{/each}

		<Tabs.Content value="Moderation">
			<h2 class="text-xl2">Moderation</h2>
		</Tabs.Content>

		<Tabs.Content value="Feedback">
			<h3 class="mb-4 text-xl font-bold">Facilitator Feedback</h3>
			<h3 class="mb-4 text-xl font-bold">Participant feedback</h3>
			{#each report.facilitator_feedback as feedback}
				<article class="relative mb-4 rounded-lg border-l-4 border-blue-500 bg-gray-100 p-6 shadow-md dark:border-blue-400 dark:bg-gray-800">
					<span class="absolute left-3 top-2 font-serif text-5xl text-blue-500 dark:text-blue-400">"</span>
					{feedback.content}
					<span class="absolute bottom-2 right-3 font-serif text-5xl text-blue-500 dark:text-blue-400">"</span>
				</article>
			{/each}
		</Tabs.Content>
	</Tabs.Root>
</div>

