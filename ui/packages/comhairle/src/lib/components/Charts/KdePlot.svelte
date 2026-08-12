<script lang="ts">
	import { Area, Chart, Group, Layer, LinearGradient } from 'layerchart';
	import { scaleLinear } from 'd3-scale';
	import { curveBasis } from 'd3-shape';
	import { max, mean, range } from 'd3-array';

	type Props = {
		minLabel?: string;
		centerLabel?: string;
		maxLabel?: string;
		data: Record<string, number[]>;
		maxX?: number;
		minX?: number;
		height?: number;
		options?: {
			densityLine?: boolean;
			outline?: boolean;
		};
	};

	let {
		minLabel,
		centerLabel,
		maxLabel,
		data,
		maxX = 10,
		minX = 0,
		height = 200,
		options = { densityLine: true, outline: false }
	}: Props = $props();

	let overlap = 4;

	const categories = $derived(Object.keys(data));

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

	const N = $derived(categories.length);
	const basePadding = { top: 0, bottom: 0, left: 0, right: 0 };

	// Compute KDE for each category
	const seriesData = $derived.by(() => {
		const thresholds = $derived(range(minX, maxX + 1, 1));
		const bandwidth = 7;

		return categories.map((name) => ({
			name,
			values: kde(epanechnikov(bandwidth), thresholds, data[name])
		}));
	});

	const maxDensity = $derived(
		max(seriesData.flatMap((s) => s.values.map((d) => d.value))) ?? 0.01
	);

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

	let averages = $derived.by(() => {
		const averages: Record<string, { value: number; percentage: number }> = {};
		for (const category of categories) {
			const average = mean(data[category]) ?? 0;
			averages[category] ??= {
				value: average,
				percentage: ((average - minX) / (maxX - minX)) * 100
			};
		}
		return averages;
	});
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
		{#each categories as category (category)}
			{@const average = averages[category]}
			{#if average}
				<div class="bg-muted-foreground/50 relative mb-2 h-1 rounded-full">
					<div
						class="bg-primary absolute h-full rounded-full"
						style="width: {Math.min(average.percentage, 100)}%"
					>
						<div
							class="bg-primary text-primary-foreground absolute top-1/2 right-0 -translate-y-1/2 rounded-full px-1.5 py-px text-xs"
						>
							{average.value.toFixed(1)}
						</div>
					</div>
				</div>
			{/if}
		{/each}
	{/if}
	{#if minLabel && maxLabel}
		<div class="text-muted-foreground relative bottom-0 flex justify-between text-xs">
			<span>{minLabel}</span>
			{#if centerLabel}
				<span>{centerLabel}</span>
			{/if}
			<span>{maxLabel}</span>
		</div>
	{/if}
</div>
