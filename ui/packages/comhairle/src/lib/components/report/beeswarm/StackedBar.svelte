<!--
	@component SVG horizontal stacked bar — renders inside a LayerCake Svg layer.
	Uses LayerCake context for width/height. Data is an array of segments
	with { key, value, color, borderColor? }.
-->
<script lang="ts">
	import { getContext } from 'svelte';
	import { sum } from 'd3-array';
	import { format } from 'd3-format';

	const { width, height } = getContext('LayerCake');

	export interface BarSegment {
		key: string;
		value: number;
		color: string;
		borderColor?: string;
	}

	interface Props {
		segments: BarSegment[];
		showLabels?: boolean;
		rx?: number;
	}

	let { segments, showLabels = true, rx = 6 }: Props = $props();

	const total = $derived(sum(segments, (seg) => seg.value));
	const fmtPct = format('.0f');
	const clipId = $derived(`bar-clip-${segments.map((s) => s.key).join('-')}`);

	const MIN_LABEL_WIDTH = 28;

	const rects = $derived.by(() => {
		if (total === 0) return [];
		const w = $width;
		const h = $height;
		let x = 0;

		return segments
			.filter((seg) => seg.value > 0)
			.map((seg) => {
				const pct = seg.value / total;
				const barW = pct * w;
				const fitsInside = barW >= MIN_LABEL_WIDTH;
				const rect = {
					key: seg.key,
					x,
					y: 0,
					width: barW,
					height: h,
					color: seg.color,
					borderColor: seg.borderColor,
					pct: seg.value,
					fitsInside
				};
				x += barW;
				return rect;
			});
	});
</script>

<g class="stacked-bar">
	<defs>
		<clipPath id={clipId}>
			<rect x={0} y={0} width={$width} height={$height} {rx} ry={rx} />
		</clipPath>
	</defs>
	<g clip-path="url(#{clipId})">
		{#each rects as r (r.key)}
			<rect
				x={r.x}
				y={r.y}
				width={r.width}
				height={r.height}
				fill={r.color}
				stroke={r.borderColor ?? 'none'}
				stroke-width={r.borderColor ? 1 : 0}
			/>
		{/each}
	</g>
	{#each rects as r (r.key)}
		{#if showLabels}
			<text
				x={r.fitsInside ? r.x + r.width - 4 : r.x + r.width + 2}
				y={$height / 2}
				text-anchor={r.fitsInside ? 'end' : 'start'}
				dominant-baseline="central"
				class="text-[8px] font-medium {r.fitsInside ? 'fill-gray-950' : 'fill-gray-500'}"
			>
				{fmtPct(r.pct)}%
			</text>
		{/if}
	{/each}
</g>
