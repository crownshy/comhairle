<!--
	@component The consensus continuum at room scale: statements as dots from agreed on to
	divisive, with bigger dots and none of the report card's chrome.
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import { scoredComments } from '$lib/tools/polis/beeswarm';
	import { scaleSqrt } from 'd3-scale';
	import { forceX, forceY, forceCollide, type Force, type SimulationNodeDatum } from 'd3-force';
	import { ForceSimulation } from 'layerchart/force';

	type Props = {
		comments: ReportComment[];
		focusedTid?: number | null;
		interactive?: boolean;
		/** The board's size setting. Raises the maximum dot size but never makes dots overlap. */
		dotScale?: number;
		onfocusstatement?: (tid: number) => void;
	};

	let {
		comments,
		focusedTid = null,
		interactive = false,
		dotScale = 1,
		onfocusstatement
	}: Props = $props();

	// The report's continuum uses radius 5, which is too small to see on a projector.
	const MAX_RADIUS = 14;
	const MIN_RADIUS = 6;
	// Width stands in for viewing distance: a wide wall is far away, a narrow phone is close.
	const WIDTH_PER_RADIUS = 50;
	const HEIGHT_PER_RADIUS = 5;
	const GAP = 2;
	// Plot area per dot, in radius-squared units. Many statements get smaller dots instead of a pile.
	const AREA_PER_DOT = 6;

	type SwarmNode = SimulationNodeDatum & { tid: number; text: string; divisiveness: number };

	// Clamps dots inside the plot. Without it, forceCollide pushes a pile of equal scores
	// past the edges, because forceX and forceY only pull toward a target.
	function forceBounds(
		width: number,
		height: number,
		margin: number
	): Force<SwarmNode, undefined> {
		let bounded: SwarmNode[] = [];
		const clamp = (v: number, max: number) =>
			Math.min(Math.max(v, margin), Math.max(margin, max));
		const force = () => {
			for (const n of bounded) {
				n.x = clamp(n.x ?? margin, width - margin);
				n.y = clamp(n.y ?? height / 2, height - margin);
			}
		};
		force.initialize = (n: SwarmNode[]) => {
			bounded = n;
		};
		return force;
	}

	// Measured from the parent box, not a fixed height, so the strip never pushes blocks off screen.
	let plotWidth = $state(0);
	let plotHeight = $state(0);

	const scored = $derived(scoredComments(comments));

	// dotScale only applies to the distance-based limits. The height and count limits are
	// about what fits, so the size setting cannot make dots overlap.
	const radius = $derived(
		plotWidth <= 0 || plotHeight <= 0
			? MAX_RADIUS * dotScale
			: Math.max(
					MIN_RADIUS,
					Math.min(
						MAX_RADIUS * dotScale,
						(plotWidth / WIDTH_PER_RADIUS) * dotScale,
						plotHeight / HEIGHT_PER_RADIUS,
						Math.sqrt(
							(plotWidth * plotHeight) / (Math.max(1, scored.length) * AREA_PER_DOT)
						)
					)
				)
	);

	// The focused dot grows, so keep that much clear of every edge too.
	const activeGrowth = $derived(Math.round(radius / 4));
	const margin = $derived(radius + activeGrowth);

	const xScale = $derived(
		scaleSqrt()
			.domain([0, Math.max(...scored.map((c) => c.divisiveness), 1)])
			.range([margin, Math.max(margin, plotWidth - margin)])
	);

	const nodes = $derived<SwarmNode[]>(
		scored.map((c) => ({
			tid: c.tid,
			text: c.text,
			divisiveness: c.divisiveness,
			x: xScale(c.divisiveness),
			y: plotHeight / 2
		}))
	);

	// Weaker x pull than the report's 1, so equal scores can spread sideways in a short strip.
	const forces = $derived<Record<string, Force<SwarmNode, undefined>>>({
		x: forceX<SwarmNode>((d) => xScale(d.divisiveness)).strength(0.5),
		y: forceY<SwarmNode>(plotHeight / 2).strength(0.15),
		collide: forceCollide<SwarmNode>(radius + GAP).strength(1),
		bounds: forceBounds(plotWidth, plotHeight, margin)
	});

	const simData = $derived({ nodes });
</script>

<div class="flex h-full min-h-0 flex-col gap-2">
	<div
		class="relative min-h-0 w-full flex-1 overflow-hidden"
		bind:clientWidth={plotWidth}
		bind:clientHeight={plotHeight}
	>
		{#if plotWidth > 0 && plotHeight > 0 && scored.length > 0}
			<svg class="h-full w-full" role="presentation">
				<ForceSimulation {forces} data={simData} cloneNodes>
					{#snippet children({ nodes: placed })}
						{#each placed as n (n.tid)}
							<!-- Plain outline-none: Chrome draws a square focus ring on SVG shapes on click.
								Focus still shows, because a focused dot grows and turns primary. See NOTES.md. -->
							{@const isActive = n.tid === focusedTid}
							<circle
								role="button"
								tabindex={interactive ? 0 : -1}
								aria-disabled={!interactive}
								aria-label={n.text.trim()}
								cx={n.x ?? 0}
								cy={n.y ?? plotHeight / 2}
								r={isActive ? radius + activeGrowth : radius}
								fill={isActive ? 'var(--primary)' : 'var(--muted-foreground)'}
								opacity={isActive ? 1 : 0.45}
								class="transition-all duration-200 outline-none"
								class:cursor-pointer={interactive}
								onmouseenter={() => interactive && onfocusstatement?.(n.tid)}
								onfocus={() => interactive && onfocusstatement?.(n.tid)}
								onclick={() => interactive && onfocusstatement?.(n.tid)}
								onkeydown={(e) => {
									if (interactive && (e.key === 'Enter' || e.key === ' ')) {
										e.preventDefault();
										onfocusstatement?.(n.tid);
									}
								}}
							/>
						{/each}
					{/snippet}
				</ForceSimulation>
			</svg>
		{/if}
	</div>

	<div class="text-muted-foreground flex shrink-0 justify-between text-base font-medium">
		<span>Agreed on</span>
		<span>Divides the room</span>
	</div>
</div>
