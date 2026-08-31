<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import DailyStatsChart from '$lib/components/DailyStatsChart.svelte';
	import StatsBar from '$lib/components/StatsBar.svelte';
	import StatProgressIndicator from '$lib/components/StatProgressIndicator.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Download } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import TabContent from '../TabContent.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';

	let { data } = $props();
</script>

<svelte:head>
	<title>Monitor Conversation - Comhairle Admin</title>
</svelte:head>

<TabContent>
	<PageHeader
		title="Monitor"
		description="See how your conversation is working, monitor recruitment and user progress and check out your reach"
	/>

	<h2 class="my-10 text-2xl">Overview</h2>
	{#await data.streamedWorkflowStats}
		<Skeleton class="mb-5 h-26 w-full rounded-xl" />
		<Skeleton class="h-138 w-full rounded-xl" />
	{:then workflowStats}
		{#if workflowStats.err !== null}
			<h3 class="my-10 text-2xl">Could not retreieve stats please try again</h3>
		{:else}
			<StatsBar
				stats={[
					{
						name: 'Total Users',
						amount: workflowStats?.ok.totalUsers,
						unit: null
					},
					{
						name: 'Active Now',
						amount: 1,
						unit: null
					},
					{
						name: 'Time Spent',
						amount: 1,
						unit: 'minute'
					},
					{
						name: 'Completed',
						amount: 1,
						unit: null
					}
				]}
			/>

			<DailyStatsChart stats={workflowStats.ok.signupStats} />

			<h2 class="my-10 text-2xl">Progress</h2>

			<p class="text-muted text-sm">
				See how users are making progress through the engagment. Indetify any sticking
				points
			</p>

			{#await data.streamedWorkflowSteps}
				<p>Other Skeleton</p>
			{:then workflowSteps}
				{#if workflowSteps.err !== null}
					<h3 class="my-10 text-2xl">
						Could not retrieve workflow steps please try again
					</h3>
				{:else}
					{#each workflowSteps.ok as step (step.id)}
						{@const workflowStepStats = workflowStats.ok.stepStats.find(
							(s) => s.id === step.id
						)}
						<h3 class="my-5 text-xl font-bold">{step.name}</h3>
						<div class="grid grid-cols-3 gap-10 overflow-x-auto">
							<StatProgressIndicator
								title="Started"
								currentValue={workflowStepStats?.started ?? 0}
								description="participants who have started but not completed this step"
								total={workflowStats.ok.totalUsers}
								varName="started"
								entityType="participants"
								message="have started this workflow step"
							/>
							<StatProgressIndicator
								title="Completed"
								currentValue={workflowStepStats?.completed ?? 0}
								description="participants who have completed this step"
								total={workflowStats.ok.totalUsers}
								varName="completed"
								entityType="participants"
								message="have completed this workflow step"
							/>

							<Card.Root class="flex-inline flex">
								<Card.Header class="items-center">
									<Card.Title>Time to complete</Card.Title>
									<Card.Description>Median user time to complete</Card.Description
									>
								</Card.Header>
								<Card.Content class="h-full">
									<div class="flex flex-col items-center justify-center">
										{#if (workflowStepStats?.completed ?? 0) > 0}
											<span class="pt-17.5 text-3xl">1 Minute</span>
											<p>median</p>
										{:else}
											<span class="pt-17.5 text-center text-2xl">
												No users have completed this step yet
											</span>
										{/if}
									</div>
								</Card.Content>
								<Card.Footer class="flex-col gap-2 text-sm">
									{#if (workflowStepStats?.completed ?? 0) > 0}
										Most users completed this step in 1 minute. The longest it
										took someone was 3 minutes.
									{/if}
								</Card.Footer>
							</Card.Root>
						</div>
					{/each}
				{/if}
			{/await}

			<h2 class="my-10 text-2xl">Follow up</h2>

			<div class="grid w-full grid-cols-1 gap-10 md:grid-cols-2">
				<div class="flex flex-col gap-4">
					<p>
						Download a list of users who have opted in to being contacted on this
						engagment
					</p>
					<Button
						href={`/api/conversation/${data.conversation.id}/contacts/export`}
						download
						variant="outline"
					>
						<Download class="mr-2 h-4 w-4" />
						Download Contacts
					</Button>
				</div>
				<div class="flex flex-col gap-4">
					<p>
						Download demographic data from user profiles for participants in this
						conversation
					</p>
					<Button
						href={`/api/conversation/${data.conversation.id}/demographics/export`}
						download
						variant="outline"
					>
						<Download class="mr-2 h-4 w-4" />
						Download Demographics
					</Button>
				</div>
			</div>
		{/if}
	{/await}
</TabContent>
