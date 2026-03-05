<script lang="ts">
	import DailyStatsChart from '$lib/components/DailyStatsChart.svelte';
	import StatsBar from '$lib/components/StatsBar.svelte';
	import StatProgressIndicator from '$lib/components/StatProgressIndicator.svelte';
	import PopulationComparison from '$lib/components/PopulationComparison.svelte';
	import GenderComparison from '$lib/components/GenderComparison.svelte';
	import GeoComparison from '$lib/components/GeoComparison/GeoComparison.svelte';
	import * as Card from '$lib/components/ui/card';

	import type { PageProps } from './$types';
	import { BreadcrumbItem } from '$lib/components/ui/breadcrumb';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte';

	let { data }: PageProps = $props();
	let { workflowSteps, workflowStats } = data;

	let stats = [
		{
			name: 'Total Users',
			amount: workflowStats.totalUsers,
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
	];

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

<svelte:head>
	<title>Monitor Conversation - Comhairle Admin</title>
</svelte:head>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Monitor</h1>
{/snippet}

{#snippet breadcrumbSnippet()}
	<BreadcrumbItem>Monitor</BreadcrumbItem>
{/snippet}

<p class="mb-10">
	See how your conversation is working, monitor recruitment and user progress and check out your
	reach
</p>

<h2 class="my-10 text-2xl">Overview</h2>
<StatsBar {stats} />

<DailyStatsChart stats={workflowStats.signupStats} />

<h2 class="my-10 text-2xl">Progress</h2>

<p class="text-mutted text-sm">
	See how users are making progress through the engagment. Indetify any sticking points
</p>

{#each workflowSteps as step (step.id)}
	<h3 class="my-5 text-xl font-bold">{step.name}</h3>
	<div class="grid grid-cols-3 gap-10 overflow-x-auto">
		<StatProgressIndicator
			title="Started"
			currentValue={workflowStats.stepStats.find((s) => s.id == step.id)?.started ?? 0}
			description="participants who have started but not completed this step"
			total={workflowStats.totalUsers}
			varName="started"
			entityType="participants"
			message="have started this workflow step"
		/>
		<StatProgressIndicator
			title="Completed"
			currentValue={workflowStats.stepStats.find((s) => s.id == step.id)?.completed ?? 0}
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
					{#if (workflowStats.stepStats.find((s) => s.id == step.id)?.completed ?? 0) > 0}
						<h1 class="pt-[70px] text-3xl">1 Minute</h1>
						<p>median</p>
					{:else}
						<h1 class="pt-[70px] text-center text-2xl">
							No users have completed this step yet
						</h1>
					{/if}
				</div>
			</Card.Content>
			<Card.Footer class="flex-col gap-2 text-sm">
				{#if (workflowStats.stepStats.find((s) => s.id == step.id)?.completed ?? 0) > 0}
					Most users completed this step in 1 minute. The longest it took someone was 3
					minutes.
				{/if}
			</Card.Footer>
		</Card.Root>
	</div>
{/each}

<h2 class="my-10 text-2xl">Reach</h2>

<div class="grid w-full grid-cols-1 gap-10 md:grid-cols-2">
	<PopulationComparison />

	<GenderComparison />
	<GeoComparison />
</div>
