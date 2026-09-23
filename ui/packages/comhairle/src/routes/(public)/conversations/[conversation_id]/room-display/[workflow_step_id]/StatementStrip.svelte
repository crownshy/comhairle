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
		/** Plot height in px. Room-scale wants more air than the report's 144. */
		height?: number;
		onfocusstatement?: (tid: number) => void;
	};

	let {
		comments,
		focusedTid = null,
		interactive = false,
		height = 120,
		onfocusstatement
	}: Props = $props();

	// Room scale. The report's continuum uses 5, which disappears on a projector.
	const RADIUS = 14;
	// The focused dot grows by 3, so keep that much clear of every edge.
	const MARGIN = RADIUS + 3;

	type SwarmNode = SimulationNodeDatum & { tid: number; text: string; divisiveness: number };

	/**
	 * Keeps every dot inside the plot box. forceX/forceY only pull towards a target, so a
	 * cluster of statements with the same divisiveness gets stacked past the top and left
	 * edges by forceCollide and clipped. Clamping after each tick makes the box a wall that
	 * collision resolves against instead.
	 */
	function forceBounds(width: number, height: number): Force<SwarmNode, undefined> {
		let bounded: SwarmNode[] = [];
		const clamp = (v: number, max: number) =>
			Math.min(Math.max(v, MARGIN), Math.max(MARGIN, max));
		const force = () => {
			for (const n of bounded) {
				n.x = clamp(n.x ?? MARGIN, width - MARGIN);
				n.y = clamp(n.y ?? height / 2, height - MARGIN);
			}
		};
		force.initialize = (n: SwarmNode[]) => {
			bounded = n;
		};
		return force;
	}

	let plotWidth = $state(0);

	const scored = $derived(scoredComments(comments));

	const xScale = $derived(
		scaleSqrt()
			.domain([0, Math.max(...scored.map((c) => c.divisiveness), 1)])
			.range([MARGIN, Math.max(MARGIN, plotWidth - MARGIN)])
	);

	const nodes = $derived<SwarmNode[]>(
		scored.map((c) => ({
			tid: c.tid,
			text: c.text,
			divisiveness: c.divisiveness,
			x: xScale(c.divisiveness),
			y: height / 2
		}))
	);

	// x is deliberately weaker than the report's 1: against the bounds the only way a pile of
	// same-score statements can resolve is sideways, and a strip this short has no room to
	// resolve it vertically.
	const forces = $derived<Record<string, Force<SwarmNode, undefined>>>({
		x: forceX<SwarmNode>((d) => xScale(d.divisiveness)).strength(0.5),
		y: forceY<SwarmNode>(height / 2).strength(0.15),
		collide: forceCollide<SwarmNode>(RADIUS + 2).strength(1),
		bounds: forceBounds(plotWidth, height)
	});

	const simData = $derived({ nodes });
</script>

<div class="flex flex-col gap-2">
	<div class="relative overflow-hidden" style="height: {height}px" bind:clientWidth={plotWidth}>
		{#if plotWidth > 0 && scored.length > 0}
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
								cy={n.y ?? height / 2}
								r={isActive ? RADIUS + 3 : RADIUS}
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

	<div class="text-muted-foreground flex justify-between text-base font-medium">
		<span>Agreed on</span>
		<span>Divides the room</span>
	</div>
</div>
