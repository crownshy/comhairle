<script lang="ts">
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { scaleUtc } from 'd3-scale';
	import { BarChart, type ChartContextValue, Highlight } from 'layerchart';
	import { cubicInOut } from 'svelte/easing';
	import { parseISO, subDays } from 'date-fns';
	import type { DailySignupStats } from '$lib/api/api';

	type Props = {
		stats: DailySignupStats[];
	};
	let { stats }: Props = $props();

	let localStats = $derived.by(() => {
		let mapped = stats.map((s) => ({
			...s,
			day: parseISO(s.day),
			signups: s.users,
			active: s.users
		}));
		if (stats.length < 30 && stats.length > 0) {
			let start = mapped[0].day;
			let buffer_before = [];
			for (let i = 0; i < 30; i++) {
				buffer_before.push({ day: subDays(start, 30 - i), signups: 0, active: 0 });
			}
			mapped = [...buffer_before, ...mapped];
		}

		return mapped;
	});

	const chartConfig = {
		signups: { label: 'Signups', color: 'var(--primary)' },
		active: { label: 'Active', color: 'var(--secondary)' }
	} satisfies Chart.ChartConfig;

	let context = $state<ChartContextValue>();

	let activeChart = $state<keyof typeof chartConfig>('signups');

	const total = $derived({
		signups: localStats.reduce((acc, curr) => acc + curr.signups, 0),
		active: localStats.reduce((acc, curr) => acc + curr.active, 0)
	});

	const activeSeries = $derived([
		{
			key: activeChart,
			label: chartConfig[activeChart].label,
			color: chartConfig[activeChart].color
		}
	]);
</script>

<Card.Root>
	<Card.Header class="mt-5 flex flex-col items-stretch space-y-0 border-b p-0 sm:flex-row">
		<div class="flex flex-1 flex-col justify-center gap-1 px-6 py-5 sm:py-6">
			<Card.Title>Daily signups</Card.Title>
			<Card.Description>Showing total number of signups per day</Card.Description>
		</div>
		<div class="flex">
			{#each ['signups', 'active'] as key (key)}
				{@const chart = key as keyof typeof chartConfig}
				<button
					data-active={activeChart === chart}
					class="data-[active=true]:bg-muted/50 relative z-30 flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left even:border-l sm:border-l sm:border-t-0 sm:px-8 sm:py-6"
					onclick={() => (activeChart = chart)}
				>
					<span class="text-xs text-black">
						{chartConfig[chart].label}
					</span>
					<span class="text-lg font-bold leading-none sm:text-3xl">
						{total[key as keyof typeof total].toLocaleString()}
					</span>
				</button>
			{/each}
		</div>
	</Card.Header>
	<Card.Content class="px-2 sm:p-6">
		<Chart.Container config={chartConfig} class="aspect-auto h-[250px] w-full">
			<BarChart
				bind:context
				data={localStats}
				x="day"
				axis="x"
				series={activeSeries}
				props={{
					bars: {
						stroke: 'none',
						rounded: 'none',
						// use the height of the chart to animate the bars
						initialY: context?.height,
						initialHeight: 0,
						motion: {
							y: { type: 'tween', duration: 500, easing: cubicInOut },
							height: { type: 'tween', duration: 500, easing: cubicInOut }
						}
					},
					highlight: { area: { fill: 'none' } },
					xAxis: {
						format: (d: Date) => {
							return d.toLocaleDateString('en-US', {
								month: 'short',
								day: '2-digit'
							});
						},
						ticks: (scale) => scaleUtc(scale.domain(), scale.range()).ticks()
					}
				}}
			>
				{#snippet belowMarks()}
					<Highlight area={{ class: 'fill-muted' }} />
				{/snippet}
				{#snippet tooltip()}
					<Chart.Tooltip
						nameKey="views"
						labelFormatter={(v: Date) => {
							return v.toLocaleDateString('en-US', {
								month: 'short',
								day: 'numeric',
								year: 'numeric'
							});
						}}
					/>
				{/snippet}
			</BarChart>
		</Chart.Container>
	</Card.Content>
	<Card.Footer>
		<div class="flex w-full items-start gap-2 text-sm">
			<div class="grid gap-2">
				<div class="text-muted-foreground flex items-center gap-2 leading-none">
					Showing total signups for the last 6 months
				</div>
			</div>
		</div>
	</Card.Footer>
</Card.Root>
