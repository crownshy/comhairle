<script lang="ts">
	import { scaleBand, scaleUtc } from 'd3-scale';
	import { BarChart, type ChartContextValue } from 'layerchart';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { cubicInOut } from 'svelte/easing';
	import { Spinner } from './ui/spinner';
	import { apiClient } from '$lib/api/client';
	import type { DailyResponseStats } from '$lib/api/api';
	import { parseISO } from 'date-fns';

	let chartData: undefined | Array<DailyResponseStats> = $state();
	let loading = $state(false);

	type Props = {
		invite_id: string;
		conversation_id: string;
	};

	let { invite_id, conversation_id } = $props();

	$effect(() => {
		loading = true;
		apiClient
			.GetInviteStats({ params: { conversation_id, invite_id } })
			.then((stats: Array<DailyResponseStats>) => {
				loading = false;
				let transformed_stats = stats.map((s) => ({
					...s,
					day: parseISO(s.day)
				}));
				chartData = transformed_stats;
			});
	});

	const chartConfig = {
		accepts: { label: 'Accepts', color: 'var(--secondary)' },
		rejects: { label: 'Rejects', color: 'var(--primary)' }
	} satisfies Chart.ChartConfig;

	let context = $state<ChartContextValue>();
</script>

{#if loading}
	<Spinner />
{/if}

{#if chartData}
	<Card.Root>
		<Card.Header>
			<Card.Title>Responses</Card.Title>
		</Card.Header>
		<Card.Content>
			<Chart.Container config={chartConfig}>
				<BarChart
					bind:context
					data={chartData}
					x="day"
					axis="x"
					series={[
						{ key: 'accepts', label: chartConfig.accepts.label, color: chartConfig.accepts.color },
						{ key: 'rejects', label: chartConfig.rejects.label, color: chartConfig.rejects.color }
					]}
					legend
					x1Scale={scaleBand().paddingInner(0.2)}
					seriesLayout="stack"
					rule={false}
					props={{
						bars: {
							stroke: 'none',
							strokeWidth: 0,
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
					{#snippet tooltip()}
						<Chart.Tooltip />
					{/snippet}
				</BarChart>
			</Chart.Container>
		</Card.Content>
	</Card.Root>
{/if}
