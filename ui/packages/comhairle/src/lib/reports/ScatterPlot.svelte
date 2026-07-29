<script lang="ts">
	import {
		ScatterChart,
		Tooltip,
		defaultChartPadding,
		Points,
		Layer,
		Rule,
		Highlight
	} from 'layerchart';

	export type ScatterPoint = {
		id: string;
		x: number;
		y: number;
	};

	type Props = {
		xAxisLabel: string;
		yAxisLabel: string;
		xDomain: [number, number];
		yDomain: [number, number];
		height?: number;
		points: ScatterPoint[];
	};

	let { xAxisLabel, yAxisLabel, xDomain, yDomain, height = 500, points }: Props = $props();

	let midPoints = $derived.by(() => {
		return {
			x: (xDomain[0] + xDomain[1]) / 2,
			y: (yDomain[0] + yDomain[1]) / 2
		};
	});

	let flipAxes = $state(false);
	function handleFlipAxes(e: Event) {
		const target = e.target as HTMLSelectElement;

		if (target.name === 'yAxis') {
			flipAxes = target.value === xAxisLabel;
		} else if (target.name === 'xAxis') {
			flipAxes = target.value === yAxisLabel;
		}
	}

	function truncateLabel(value: string) {
		const limit = 20;
		return value.length < limit ? value : value.slice(0, limit).concat('...');
	}

	const SELECT_CLASSES = 'rounded-full border px-2 py-1 focus:outline-none';
	const OPTION_CLASSES = 'dark:bg-background dark:text-foreground';
</script>

<article class="relative w-full pr-16">
	<div class="grid h-full grid-cols-[85%_1fr]">
		<div class="h-full">
			<div class="mb-2 flex justify-center">
				<select
					class={SELECT_CLASSES}
					name="yAxis"
					onchange={handleFlipAxes}
					value={flipAxes ? xAxisLabel : yAxisLabel}
				>
					<option class={OPTION_CLASSES} value={yAxisLabel}
						>{truncateLabel(yAxisLabel)}</option
					>
					<option class={OPTION_CLASSES} value={xAxisLabel}
						>{truncateLabel(xAxisLabel)}</option
					>
				</select>
			</div>
		</div>
	</div>
	<div class="grid h-full grid-cols-[85%_1fr]">
		<ScatterChart
			data={points}
			xBaseline={0}
			yBaseline={5}
			xDomain={flipAxes ? yDomain : xDomain}
			yDomain={flipAxes ? xDomain : yDomain}
			{height}
			x={flipAxes ? 'y' : 'x'}
			y={flipAxes ? 'x' : 'y'}
			padding={defaultChartPadding({ top: 20, bottom: 20, left: 20, right: 20 })}
		>
			{#snippet children({ context })}
				<Layer>
					<Points class="fill-primary/10 stroke-primary" />
					<Rule
						x={flipAxes ? midPoints.y : midPoints.x}
						y={flipAxes ? midPoints.x : midPoints.y}
						class="stroke-muted-foreground stroke-2"
					/>
					<Highlight points lines axis="both" />
				</Layer>
				<Tooltip.Root>
					{#snippet children({ data })}
						<Tooltip.Header>Response</Tooltip.Header>
						<Tooltip.List>
							<Tooltip.Item label={xAxisLabel} value={context.x(data)} />
							<Tooltip.Item label={yAxisLabel} value={context.y(data)} />
						</Tooltip.List>
					{/snippet}
				</Tooltip.Root>
			{/snippet}
		</ScatterChart>
		<div class="flex items-center text-center">
			<select
				class={SELECT_CLASSES}
				name="xAxis"
				onchange={handleFlipAxes}
				value={flipAxes ? yAxisLabel : xAxisLabel}
			>
				<option class={OPTION_CLASSES} value={xAxisLabel}
					>{truncateLabel(xAxisLabel)}</option
				>
				<option class={OPTION_CLASSES} value={yAxisLabel}
					>{truncateLabel(yAxisLabel)}</option
				>
			</select>
		</div>
	</div>
</article>
