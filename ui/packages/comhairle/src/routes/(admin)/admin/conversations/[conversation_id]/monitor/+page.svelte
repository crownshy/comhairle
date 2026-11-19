<script lang="ts">
	import DailyStatsChart from '$lib/components/DailyStatsChart.svelte';
	import StatsBar from '$lib/components/StatsBar.svelte';
	import StatProgressIndicator from '$lib/components/StatProgressIndicator.svelte';
	import * as Card from '$lib/components/ui/card';
	import Download from '@lucide/svelte/icons/download';
	import { Button } from '$lib/components/ui/button';
	import { permissions } from '$lib/permissions.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { monitorTour } from './tour';
	import type { PageProps } from './$types';

	useTour(() => monitorTour);

	let { data }: PageProps = $props();
	let { workflowSteps, workflowStats } = $derived(data);
	const canExport = $derived(
		permissions.can('conversation', 'conversation_export', data.conversation.id)
	);

	let stats = $derived([
		{
			name: 'Total Users',
			amount: workflowStats.totalUsers,
			unit: null
		},

		{
			name: 'Active Now',
			amount: workflowStats.activeUsers,
			unit: null
		},

		{
			name: 'Time Spent',
			amount: Math.round(workflowStats.totalActiveSeconds / 60),
			unit: ' minutes'
		},
		{
			name: 'Completed',
			amount: workflowStats.finishedUsers,
			unit: null
		}
	]);

	let formatElapsedTime = (seconds: number) => {
		const rounded = Math.round(seconds);
		const mins = Math.floor(rounded / 60);
		const secs = rounded % 60;
		if (mins > 0) {
			return `${mins}m ${secs}s`;
		}
		return `${secs}s`;
	};
</script>

<svelte:head>
	<title>Monitor Conversation - Comhairle Admin</title>
</svelte:head>

<h2 class="my-10 text-2xl">Overview</h2>
<StatsBar {stats} />

<DailyStatsChart stats={workflowStats.signupStats} />

<h2 class="my-10 text-2xl">Progress</h2>

<p class="text-mutted text-sm">
	See how users are making progress through the engagment. Indetify any sticking points
</p>

{#each workflowSteps as step (step.id)}
	{@const stepStats = workflowStats.stepStats.find((stats) => stats.id === step.id)}
	<h3 class="my-5 text-xl font-bold">{step.name}</h3>
	<div class="grid grid-cols-3 gap-10 overflow-x-auto">
		<StatProgressIndicator
			title="Started"
			currentValue={stepStats?.started ?? 0}
			description="participants who have started but not completed this step"
			total={workflowStats.totalUsers}
			varName="started"
			entityType="participants"
			message="have started this workflow step"
		/>
		<StatProgressIndicator
			title="Completed"
			currentValue={stepStats?.completed ?? 0}
			description="participants who have completed this step"
			total={workflowStats.totalUsers}
			varName="completed"
			entityType="participants"
			message="have completed this workflow step"
		/>

		<Card.Root class="flex-inline flex">
			<Card.Header class="items-center">
				<Card.Title>Time to complete</Card.Title>
				<Card.Description>Median user time to complete</Card.Description>
			</Card.Header>
			<Card.Content class="h-full">
				<div class="flex flex-col items-center justify-center">
					{#if stepStats?.medianActiveSeconds != null}
						<p class="pt-[70px] text-3xl">
							{formatElapsedTime(stepStats.medianActiveSeconds)}
						</p>
						<p>median</p>
					{:else}
						<p class="pt-[70px] text-center text-2xl">
							{(stepStats?.completed ?? 0) > 0
								? 'No timing data available for completed participants'
								: 'No users have completed this step yet'}
						</p>
					{/if}
				</div>
			</Card.Content>
			<Card.Footer class="flex-col gap-2 text-sm">
				{#if (workflowStats.stepStats.find((s) => s.id == step.id)?.completed ?? 0) > 0}
					Most users completed this step in {formatElapsedTime(
						stepStats?.medianActiveSeconds ?? 0
					)}. The longest it took someone was {formatElapsedTime(
						stepStats?.maxActiveSeconds ?? 0
					)}.
				{/if}
			</Card.Footer>
		</Card.Root>
	</div>
{/each}

<h2 class="my-10 text-2xl">Follow up</h2>

<div class="grid w-full grid-cols-1 gap-10 md:grid-cols-2">
	<div class="flex flex-col gap-4">
		<p>Download a list of users who have opted in to being contacted on this engagment</p>
		<Button
			href={`/api/conversation/${data.conversation.id}/contacts/export`}
			disabled={!canExport}
			download
			variant="outline"
		>
			<Download class="mr-2 h-4 w-4" />
			Download Contacts
		</Button>
	</div>
	<div class="flex flex-col gap-4">
		<p>Download demographic data from user profiles for participants in this conversation</p>
		<Button
			href={`/api/conversation/${data.conversation.id}/demographics/export`}
			disabled={!canExport}
			download
			variant="outline"
		>
			<Download class="mr-2 h-4 w-4" />
			Download Demographics
		</Button>
	</div>
</div>
