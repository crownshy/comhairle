<script lang="ts">
	import { Area, LineChart } from 'layerchart';
	import { scaleUtc } from 'd3-scale';
	import type { SignupPoint } from './sampleMonitorData';

	type Props = {
		signups: SignupPoint[];
		target: number;
		launchDate: Date;
		closingDate: Date;
	};

	let { signups, target, launchDate, closingDate }: Props = $props();

	let latest = $derived(signups[signups.length - 1]);
	let remaining = $derived(Math.max(0, target - latest.total));
	/** How far along the x axis today sits, to place the "Today" label under the latest point. */
	let todayPercent = $derived(
		((latest.day.getTime() - launchDate.getTime()) /
			(closingDate.getTime() - launchDate.getTime())) *
			100
	);
	let evenPace: SignupPoint[] = $derived([
		{ day: launchDate, total: 0 },
		{ day: closingDate, total: target }
	]);

	let annotations = $derived([
		{
			type: 'line' as const,
			y: target,
			label: `${target}`,
			labelPlacement: 'top-right' as const,
			labelYOffset: 4,
			props: {
				line: { class: 'stroke-orange-500', strokeWidth: 2 },
				label: { class: 'fill-orange-500 text-sm font-semibold' }
			}
		},
		{
			type: 'point' as const,
			x: latest.day,
			y: latest.total,
			r: 4,
			label: `${latest.total}`,
			labelPlacement: 'top' as const,
			labelYOffset: 6,
			props: {
				circle: { class: 'fill-primary stroke-card', strokeWidth: 2 },
				label: { class: 'fill-primary text-sm font-semibold' }
			}
		}
	]);

	function formatDay(day: Date): string {
		return day.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' });
	}
</script>

<!--
	Running total against the target, over the whole conversation window: the orange line
	at the top is the target, the empty space to the right is the time left, and the faint
	diagonal is the even pace you'd need to hit it.
-->
<div class="flex flex-col gap-1">
	<div
		class="h-28"
		role="img"
		aria-label="{latest.total} of a target of {target} joined, {remaining} to go before {formatDay(
			closingDate
		)}"
	>
		<LineChart
			data={[...evenPace, ...signups]}
			x="day"
			xScale={scaleUtc()}
			xDomain={[launchDate, closingDate]}
			y="total"
			yDomain={[0, Math.max(target, ...signups.map((point) => point.total))]}
			padding={{ top: 22, bottom: 2, left: 2, right: 2 }}
			axis={false}
			grid={false}
			highlight={false}
			tooltipContext={false}
			{annotations}
			series={[
				{
					key: 'pace',
					data: evenPace,
					value: 'total',
					color: 'var(--muted-foreground)',
					props: { class: 'opacity-40 [stroke-dasharray:2_4]', strokeWidth: 1.5 }
				},
				{
					key: 'joined',
					data: signups,
					value: 'total',
					color: 'var(--primary)',
					props: { strokeWidth: 2.5 }
				}
			]}
		>
			{#snippet belowMarks()}
				<Area seriesKey="joined" line={false} fillOpacity={1} class="fill-accent" />
			{/snippet}
		</LineChart>
	</div>
	<div class="text-muted-foreground relative flex justify-between text-sm">
		<span>{formatDay(launchDate)}</span>
		<span
			class="text-primary absolute top-0 -translate-x-1/2 text-xs font-semibold"
			style:left="{todayPercent}%"
		>
			Today
		</span>
		<span>Closes {formatDay(closingDate)}</span>
	</div>
</div>
