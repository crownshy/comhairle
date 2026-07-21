<script lang="ts">
	import type { ThinkingSpaceInsightsResponse } from '@crownshy/api-client/api';
	import MetricOverviewCard from '../MetricOverviewCard.svelte';
	import ContentCard from '../ContentCard.svelte';
	import InsightsTable from './InsightsTable.svelte';

	type Props = {
		insights: ThinkingSpaceInsightsResponse;
	};

	let { insights }: Props = $props();

	let numParticipants = $derived(insights.users.length);
	let averageNumFollowUps = $derived.by(() => {
		const sumFollowups = insights.users.reduce(
			(acc, user) =>
				acc + user.answers.reduce((acc, answer) => acc + answer.followUps.length, 0),
			0
		);

		return sumFollowups;
	});
</script>

<div class="flex w-full flex-col gap-10">
	<div class="flex gap-4">
		<MetricOverviewCard
			superText="Participants"
			metric={numParticipants}
			subText="curious minds"
		/>
		<MetricOverviewCard
			superText="Average follow-up questions"
			metric={averageNumFollowUps}
			subText="interacted with by participants"
		/>
	</div>

	<ContentCard>
		<div class="mb-10">
			<h2 class="text-lg font-bold">Thinking space results</h2>
			<p class="text-sm">
				Click on a row to see the full summary and responses associated with it
			</p>
		</div>

		<InsightsTable userInsights={insights.users} />
	</ContentCard>
</div>
