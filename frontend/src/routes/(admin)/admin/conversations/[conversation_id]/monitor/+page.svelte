<script lang="ts">
	import DailyStatsChart from '$lib/components/DailyStatsChart.svelte';
	import StatsBar from '$lib/components/StatsBar.svelte';
	import { Binoculars } from 'lucide-svelte';
	let stats = [
		{
			name: 'Total Users',
			amount: 30,
			change: '+10%',
			unit: null,
			from: 'from last month'
		},

		{
			name: 'Active Now',
			amount: 1,
			change: '+1',
			unit: null,
			from: 'since 1 hour ago'
		},

		{
			name: 'Time Spent',
			amount: 30,
			change: '+1',
			unit: 'hours',
			from: 'from last month'
		},
		{
			name: 'Completed',
			amount: 1,
			change: '+1',
			unit: null,
			from: 'from yesterday'
		}
	];
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Binoculars /> Monitor</h1>
<p class="mb-10">Use this space to set up the project and manage the team supporting it</p>

<StatsBar {stats} />

<DailyStatsChart />