<script lang="ts">
	import DailyStatsChart from '$lib/components/DailyStatsChart.svelte';
	import StatsBar from '$lib/components/StatsBar.svelte';
	import StatProgressIndicator from '$lib/components/StatProgressIndicator.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Binoculars } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import { apiClient } from '$lib/api/client';

	let { data }: PageProps = $props();
	let { workflow_steps, workflowStats } = data;
	console.log('workflw stats', workflowStats);

	let stats = [
		{
			name: 'Total Users',
			amount: workflowStats.total_users,
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
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Binoculars /> Monitor</h1>
<p class="mb-10">
	See how your conversation is working, monitor recruitment and user progress and check out your
	reach
</p>

<h2 class="my-10 text-2xl">Overview</h2>
<StatsBar {stats} />

<DailyStatsChart stats={workflowStats.signup_stats} />

<h2 class="my-10 text-2xl">Progress</h2>

<p class="text-mutted text-sm">
	See how users are making progress through the engagment. Indetify any sticking points
</p>

{#each workflow_steps as step}
	<h3 class="my-5 text-xl font-bold">{step.name}</h3>
	<div class="flex flex-row gap-x-10">
		<StatProgressIndicator
			title="Started"
			currentValue={workflowStats.step_stats.find((s) => s.id == step.id).started}
			description="participants who have started but not completed this step"
			total={workflowStats.total_users}
			varName="started"
			entityType={'participants'}
			message="have started this workflow step"
		/>
		<StatProgressIndicator
			title="Completed"
			currentValue={workflowStats.step_stats.find((s) => s.id == step.id).completed}
			description="participants who have completed this step"
			total={workflowStats.total_users}
			varName="completed"
			entityType={'participants'}
			message="have completed this workflow step"
		/>
	</div>
{/each}

<h2 class="my-10 text-2xl">Reach</h2>
