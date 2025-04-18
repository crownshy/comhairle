<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Card from '$lib/components/ui/card';
	import '@carbon/charts-svelte/styles.css';
	import { BarChartSimple } from '@carbon/charts-svelte';
	let { data } = $props();
	let { conversation, workflow_steps, workflow_stats } = data;
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
						<p class="text-xs text-muted-foreground">minutes</p>
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
			<h2 class="font-bold">Key Takeaways</h2>
			<p class="mb-4">
				One prominent consensus among participants was the need for increased funding and resources
				for wildfire prevention and response efforts. Many agreed that investing in modern
				firefighting equipment, enhancing early warning systems, and supporting community
				preparedness programs are crucial steps to mitigate future wildfire risks. Additionally,
				there was significant support for implementing stricter land-use regulations to prevent
				construction in high-risk fire zones, thereby reducing potential damages.
			</p>

			<p class="mb-4">
				Another key finding from the poll was the emphasis on environmental stewardship and
				sustainable land management practices. Participants highlighted the importance of restoring
				natural fire regimes through controlled burns and forest thinning to reduce fuel loads.
				There was also a strong advocacy for addressing climate change as a fundamental factor
				exacerbating wildfire intensity and frequency, suggesting that comprehensive climate
				policies are integral to long-term wildfire mitigation strategies.​
			</p>
			<h2 class="font-bold">Impacts</h2>
			<p class="mb-4">
				These findings where discussed at a meeting of the disaster recovery group.
			</p>
			<p class="mb-4">
				The results of this conversation where used to inform a number of in person deliberations
				around California
			</p>
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
			<article class="article-quote mb-4">
				<p>
					One of the biggest successes of this project was how it brought together a wide range of
					voices from the community. People from different backgrounds and experiences engaged
					thoughtfully with the issue, offering solutions that went beyond the usual policy debates.
					The use of the Polis platform helped highlight areas of consensus while also making space
					for disagreement in a respectful way. Moving forward, we could work on increasing
					participation from groups that are often left out of these discussions, such as rural
					communities directly impacted by wildfires and frontline workers in firefighting and
					disaster response.
				</p>
			</article>
			<article class="article-quote mb-4">
				The discussion generated many thoughtful and creative ideas about how the community should
				respond to wildfires, from better land management to stronger rebuilding efforts. However,
				one challenge was translating this collective input into concrete steps for action. While
				the conversation was productive, the next phase should focus on connecting these ideas with
				decision-makers and resources to ensure real-world impact. Future sessions could include
				more direct engagement with policymakers and community leaders to help turn the public’s
				insights into meaningful change.
			</article>
			<h3 class="mb-4 text-xl font-bold">Participant feedback</h3>
			<article class="article-quote mb-4">
				I really appreciated how this process allowed for a wide range of opinions to be heard. Even
				when people disagreed, the discussion remained respectful, and it was great to see where we
				had common ground. The Polis tool made it easy to see which ideas had the most support,
				which helped highlight solutions that could actually work for our community.
			</article>

			<article class="article-quote mb-4">
				I noticed that many of the ideas came from people who weren’t directly affected by
				wildfires. It would have been helpful to hear more from those who have lost homes or had to
				evacuate. Their firsthand experiences could have given us deeper insight into what kind of
				support is actually needed after a fire.
			</article>
			<article class="article-quote mb-4">
				This conversation introduced me to perspectives I hadn’t considered before. I came in
				thinking mostly about firefighting strategies, but hearing about controlled burns, land-use
				policies, and community rebuilding efforts helped me understand that wildfire response needs
				to be about long-term prevention too. It felt like a well-rounded and thoughtful discussion.
			</article>

			<article class="article-quote mb-4">
				While the discussion was engaging, I left feeling unsure about what happens next. We talked
				about a lot of great ideas, but I didn’t get a clear sense of how this input would be used
				or if decision-makers were actually paying attention. It would be helpful to have a clearer
				process for turning our ideas into action.
			</article>
		</Tabs.Content>
	</Tabs.Root>
</div>

<style>
	.article-quote {
		@apply relative rounded-lg border-l-4 border-blue-500 bg-gray-100 p-6 shadow-md dark:border-blue-400 dark:bg-gray-800;
	}

	.article-quote::before {
		content: '“';
		@apply absolute left-3 top-2 font-serif text-5xl text-blue-500 dark:text-blue-400;
	}

	.article-quote::after {
		content: '”';
		@apply absolute bottom-2 right-3 font-serif text-5xl text-blue-500 dark:text-blue-400;
	}

	.article-quote p {
		@apply p-4 text-lg font-medium leading-relaxed text-gray-900 dark:text-gray-100;
	}

	.article-quote cite {
		@apply mt-3 block text-sm italic text-gray-600 dark:text-gray-400;
	}
</style>
