<script lang="ts">
	import { Area, Chart, Group, Layer, LinearGradient } from 'layerchart';
	import { scaleLinear } from 'd3-scale';
	import { curveBasis } from 'd3-shape';
	import { max, mean, range } from 'd3-array';

	type Props = {
		minLabel?: string;
		maxLabel?: string;
		category: string;
		rawData: Record<string, number[]>;
		maxX?: number;
		options?: {
			densityLine?: boolean;
			outline?: boolean;
		};
	};

	let {
		minLabel,
		maxLabel,
		category,
		rawData,
		maxX = 10,
		options = { densityLine: true, outline: false }
	}: Props = $props();

	let overlap = $state(4);
	let height = $state(200);

	const categories = $derived([category]);

	// Epanechnikov kernel for KDE
	function epanechnikov(bandwidth: number) {
		return (v: number) =>
			Math.abs((v /= bandwidth)) <= 1 ? (0.75 * (1 - v * v)) / bandwidth : 0;
	}

	// Kernel density estimator
	function kde(kernel: (v: number) => number, thresholds: number[], data: number[]) {
		return thresholds.map((t) => ({
			x: t,
			value: mean(data, (d) => kernel(t - d)) ?? 0
		}));
	}

	const N = categories.length;
	const basePadding = { top: 0, bottom: 0, left: 0, right: 0 };
	const thresholds = range(0, maxX + 1, 1);
	const bandwidth = 7;

	// Compute KDE for each category
	const seriesData = categories.map((name) => ({
		name,
		values: kde(epanechnikov(bandwidth), thresholds, rawData[name])
	}));

	const maxDensity = max(seriesData.flatMap((s) => s.values.map((d) => d.value))) ?? 0.01;

	const overlapExtra = $derived(Math.max(0, overlap - 1));
	const paddingTop = $derived(
		(N * basePadding.top + overlapExtra * (height - basePadding.bottom)) / (N + overlapExtra)
	);
	const padding = $derived({
		...basePadding,
		top: paddingTop
	});

	const innerHeight = $derived(height - paddingTop - basePadding.bottom);
	const step = $derived(innerHeight / N);

	const zScale = $derived(
		scaleLinear()
			.domain([0, maxDensity])
			.range([0, -overlap * step])
	);

	let average = $derived(mean(rawData[category]) ?? 0);
	const averagePercentage = $derived((average / maxX) * 100);
</script>

<div class="relative">
	<Chart
		data={seriesData[0].values}
		x="x"
		y="value"
		yDomain={[0, innerHeight]}
		yRange={({ height }) => [0, height]}
		{padding}
		{height}
	>
		<Layer>
			{#each seriesData as series, i (series.name)}
				{@const rowY = step + i * step}
				<Group y={rowY}>
					<LinearGradient
						vertical
						stops={[
							'oklch(from var(--primary) l c h / 0.3)',
							'oklch(from var(--primary) l c h / 0.05)'
						]}
					>
						{#snippet children({ gradient })}
							<Area
								data={series.values}
								y0={() => 0}
								y1={(d) => zScale(d.value)}
								curve={curveBasis}
								fill={gradient}
								class=""
								line={options.outline
									? { class: 'stroke-primary stroke-2' }
									: undefined}
							/>
						{/snippet}
					</LinearGradient>
				</Group>
			{/each}
		</Layer>
	</Chart>
	{#if options.densityLine}
		<div class="bg-muted-foreground/50 relative mb-2 h-1 rounded-full">
			<div
				class="bg-primary absolute h-full rounded-full"
				style="width: {averagePercentage}%"
			>
				<div
					class="bg-primary text-primary-foreground absolute top-1/2 right-0 -translate-y-1/2 rounded-full px-1.5 py-px text-xs"
				>
					{average.toFixed(1)}
				</div>
			</div>
		</div>
	{/if}
	{#if minLabel && maxLabel}
		<div class="text-muted-foreground relative bottom-0 flex justify-between text-xs">
			<span>{minLabel}</span>
			<span>{maxLabel}</span>
		</div>
	{/if}
</div>
