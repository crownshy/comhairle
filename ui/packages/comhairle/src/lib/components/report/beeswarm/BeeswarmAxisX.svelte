<!--
	@component SVG x-axis for beeswarm — adapted from LayerCake's AxisX example.
-->
<script lang="ts">
	import { getContext } from 'svelte';

	const { width, height, xScale, yRange } = getContext('LayerCake');

	interface Props {
		tickMarks?: boolean;
		baseline?: boolean;
		snapLabels?: boolean;
		format?: (d: any) => string;
		ticks?: number | any[] | ((t: any[]) => any[]);
		dx?: number;
		dy?: number;
	}

	let {
		tickMarks = false,
		baseline = false,
		snapLabels = false,
		format = (d: any) => d,
		ticks = undefined,
		dx = 0,
		dy = 12
	}: Props = $props();

	function textAnchor(i: number, snapLabels: boolean): string {
		if (snapLabels === true) {
			if (i === 0) return 'start';
			if (i === tickVals.length - 1) return 'end';
		}
		return 'middle';
	}

	let tickLen = $derived(tickMarks === true ? 6 : 0);
	let isBandwidth = $derived(typeof $xScale.bandwidth === 'function');

	let tickVals = $derived(
		Array.isArray(ticks)
			? ticks
			: isBandwidth
				? $xScale.domain()
				: typeof ticks === 'function'
					? ticks($xScale.ticks())
					: $xScale.ticks(ticks)
	);

	let halfBand = $derived(isBandwidth ? $xScale.bandwidth() / 2 : 0);
</script>

<g class="axis x-axis" class:snapLabels>
	{#each tickVals as tick, i (tick)}
		{#if baseline === true && i === 0}
			<line
				class="baseline"
				y1={$height}
				y2={$height}
				x1="0"
				x2={$width}
				stroke="var(--border)"
			/>
		{/if}
		<g class="tick tick-{i}" transform="translate({$xScale(tick)},{Math.max(...$yRange)})">
			{#if tickMarks === true}
				<line
					class="tick-mark"
					x1={halfBand}
					x2={halfBand}
					y1={0}
					y2={tickLen}
					stroke="var(--muted-foreground)"
				/>
			{/if}
			<text
				x={halfBand}
				y={tickLen}
				{dx}
				{dy}
				text-anchor={textAnchor(i, snapLabels)}
				class="fill-gray-500 text-xs"
			>
				{format(tick)}
			</text>
		</g>
	{/each}
</g>
