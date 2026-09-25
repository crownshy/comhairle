<!--
	@component PROTOTYPE. The consensus continuum stripped to a bare strip: no card, no
	heading, no subtitle, no vote block underneath.

	Exists because the report's `ConsensusContinuum` is a desk component. On a projector
	its card chrome, 5px dots and 12px axis labels are most of what makes the board feel
	busy, while the only thing the room can actually read from eight metres is the
	swarm. Same d3-force layout, larger dots, two words of axis, nothing else.

	Whether the real continuum should grow a room-scale mode or the Room display should
	own a separate strip is the question this variant is asking.
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
		/**
		 * The board's size step for this block. Raises how big a dot may get on a wide
		 * wall; it cannot push dots into each other, because the box and the count
		 * still have the last word.
		 */
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

	// Room scale. The report's continuum uses 5, which disappears on a projector.
	const MAX_RADIUS = 14;
	// Below this a dot stops reading as a dot at any distance.
	const MIN_RADIUS = 6;
	// Plot width that earns a full-size dot. A dot is sized for how far away the screen
	// is, and width is the only proxy for that we have: a wall is wide and eight metres
	// off, a phone is narrow and held at arm's length. Room scale on a phone just means
	// four dots fill the strip.
	const WIDTH_PER_RADIUS = 50;
	// Plot height that earns one, so a strip squeezed by a short wall shrinks its dots
	// rather than packing them into a band they do not fit.
	const HEIGHT_PER_RADIUS = 5;
	// Clear space between neighbouring dots, and between a dot and the plot edge.
	const GAP = 2;
	// Plot area one dot needs to sit in without being stacked against the edges: a
	// dot's own square plus breathing room. Two hundred statements in a strip that fits
	// forty at full size get smaller dots, not a pile.
	const AREA_PER_DOT = 6;

	type SwarmNode = SimulationNodeDatum & { tid: number; text: string; divisiveness: number };

	/**
	 * Keeps every dot inside the plot box. forceX/forceY only pull towards a target, so a
	 * cluster of statements with the same divisiveness gets stacked past the top and left
	 * edges by forceCollide and clipped. Clamping after each tick makes the box a wall that
	 * collision resolves against instead.
	 */
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

	// The plot takes the box it is given rather than declaring a height of its own: the
	// board is a fixed screen divided between blocks, and a block that insists on a pixel
	// height is the thing that pushes its neighbours off the wall.
	let plotWidth = $state(0);
	let plotHeight = $state(0);

	const scored = $derived(scoredComments(comments));

	// The ceiling and the width proxy scale with the board's step, because both stand
	// in for "how far away is this screen". The height and the count do not: they say
	// what fits, and nothing the facilitator dials should make dots overlap.
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

	// x is deliberately weaker than the report's 1: against the bounds the only way a pile of
	// same-score statements can resolve is sideways, and a strip this short has no room to
	// resolve it vertically.
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
							<!--
								`outline-none` rather than `focus-visible:outline-none`: Chrome
								draws a focus ring on an SVG shape as its bounding box, so a
								focused dot gets a square around a circle, and a click counts as
								`:focus` without counting as `:focus-visible`. Nothing is lost by
								removing it because focus already moves the selection: `onfocus`
								sets the focused statement, and the dot grows and turns primary.
							-->
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
