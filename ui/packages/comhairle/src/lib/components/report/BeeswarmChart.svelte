<script lang="ts">
	import { LayerCake, Svg } from 'layercake';
	import { max } from 'd3-array';
	import type { ReportComment } from '$lib/types/report';

	import BeeswarmDots from './beeswarm/BeeswarmDots.svelte';

	interface Props {
		comments: ReportComment[];
		height?: number;
		dotRadius?: number;
		selectedTid?: number | null;
		onHoverComment?: (comment: ReportComment | null) => void;
	}

	let {
		comments,
		height = 220,
		dotRadius = 5,
		selectedTid = null,
		onHoverComment
	}: Props = $props();

	function handleMouseEnter(_e: MouseEvent, comment: ReportComment) {
		onHoverComment?.(comment);
	}

	function handleMouseLeave() {
		onHoverComment?.(null);
	}
</script>

<div class="chart-container" style="height: {height}px;">
	<LayerCake
		padding={{ top: 10, bottom: 10, left: 16, right: 16 }}
		x="divisiveness"
		data={comments}
	>
		{#snippet children({ width })}
			<Svg>
				<BeeswarmDots
					r={width < 500 ? dotRadius / 1.4 : dotRadius}
					spacing={1}
					{selectedTid}
					onmouseenter={handleMouseEnter}
					onmouseleave={handleMouseLeave}
					onfocus={handleMouseEnter}
					onblur={handleMouseLeave}
				/>
			</Svg>
		{/snippet}
	</LayerCake>
</div>

<div class="border-muted-foreground/40 mx-4 border-t">
	<div class="text-muted-foreground flex justify-between pt-2 text-xs">
		<span>consensus statement</span>
		<span>divisive statement</span>
	</div>
</div>

<style>
	.chart-container {
		width: 100%;
	}
</style>
