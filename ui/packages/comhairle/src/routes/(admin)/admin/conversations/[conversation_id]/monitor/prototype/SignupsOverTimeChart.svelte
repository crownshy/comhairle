<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { LineChart } from 'layerchart';
	import { scaleUtc } from 'd3-scale';
	import type { RecruitmentEvent, SignupPoint } from './sampleMonitorData';

	type Props = {
		signups: SignupPoint[];
		targetPace: SignupPoint[];
		projection: SignupPoint[];
		events: RecruitmentEvent[];
		target: number;
	};

	let { signups, targetPace, projection, events, target }: Props = $props();

	const chartConfig = {
		joined: { label: 'Joined', color: 'var(--primary)' },
		projection: { label: 'If nothing changes', color: 'var(--primary)' },
		target: { label: 'Even pace to target', color: 'var(--muted-foreground)' }
	} satisfies Chart.ChartConfig;

	let allPoints = $derived([...targetPace, ...signups, ...projection]);

	let annotations = $derived(
		events.map((event) => ({
			type: 'line' as const,
			x: event.day,
			label: event.label,
			labelPlacement: 'top-right' as const,
			props: {
				line: { class: 'stroke-muted-foreground/50 [stroke-dasharray:3_3]' },
				label: { class: 'fill-foreground text-sm' }
			}
		}))
	);

	function formatDay(day: Date): string {
		return day.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' });
	}
</script>

<!--
	New data viz component: running total of signups against an even pace to the target,
	with the admin's recruitment activity marked so you can see which push worked.
-->
<Card.Root>
	<Card.Header>
		<Card.Title>Signups over time</Card.Title>
		<Card.Description>Running total, with your recruitment activity marked</Card.Description>
	</Card.Header>
	<Card.Content>
		<Chart.Container config={chartConfig} class="aspect-auto h-[300px] w-full">
			<LineChart
				data={allPoints}
				x="day"
				xScale={scaleUtc()}
				y="total"
				yDomain={[0, target]}
				legend
				{annotations}
				series={[
					{
						key: 'target',
						label: chartConfig.target.label,
						data: targetPace,
						value: 'total',
						color: chartConfig.target.color,
						props: { class: '[stroke-dasharray:2_4]' }
					},
					{
						key: 'projection',
						label: chartConfig.projection.label,
						data: projection,
						value: 'total',
						color: chartConfig.projection.color,
						props: { class: '[stroke-dasharray:6_5]' }
					},
					{
						key: 'joined',
						label: chartConfig.joined.label,
						data: signups,
						value: 'total',
						color: chartConfig.joined.color,
						props: { strokeWidth: 3 }
					}
				]}
				props={{
					xAxis: { format: (day: Date) => formatDay(day) }
				}}
			>
				{#snippet tooltip()}
					<Chart.Tooltip labelFormatter={(day: Date) => formatDay(day)} />
				{/snippet}
			</LineChart>
		</Chart.Container>
	</Card.Content>
</Card.Root>
