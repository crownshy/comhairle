<!--
	@component The "Consensus continuum" section from the new Polis report design:
	a beeswarm plotting every scored statement along a consensus→divisive axis,
	above the vote breakdown of the currently-focused statement.

	The x axis is Polis `divisiveness` (comment extremity from the PCA math): left =
	consensus, right = divisive. Dots are packed by d3-force via layerchart's
	`ForceSimulation` (forceX pins each dot to its divisiveness position, forceY
	pulls to the vertical centre, forceCollide stops overlap). Dumb: takes the
	comments + groups; the focused statement is local view state (hover/focus a dot).
-->
<script lang="ts">
	import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
	import { scoredComments, mostDivisiveTid } from '$lib/tools/polis/beeswarm';
	import { scaleSqrt } from 'd3-scale';
	import {
		forceSimulation,
		forceX,
		forceY,
		forceCollide,
		type Force,
		type SimulationNodeDatum
	} from 'd3-force';
	import { ForceSimulation } from 'layerchart/force';
	import * as Card from '$lib/components/ui/card';
	import StatementVoteBlock from './StatementVoteBlock.svelte';

	type Props = {
		comments: ReportComment[];
		groups: ReportGroup[];
		/**
		 * Frozen-snapshot render (ADR-0012). The live plot needs a measured width and an
		 * animated d3-force layout — neither survives being baked to static HTML. In frozen
		 * mode we run the simulation to completion synchronously at a fixed viewBox width and
		 * emit plain, settled `<circle>`s, so the snapshot is a finished swarm, not a blank box.
		 */
		frozen?: boolean;
	};

	let { comments, groups, frozen = false }: Props = $props();

	const HEIGHT = 144; // h-36, matches the design's plot box
	const RADIUS = 5; // w-2.5 dots -> 5px radius
	const PADDING = 16;
	// Fixed layout width for the frozen SVG viewBox; it scales to fit the report column.
	const FROZEN_WIDTH = 800;

	type SwarmNode = SimulationNodeDatum & { tid: number; text: string; divisiveness: number };

	// Measured plot width; the x scale is degenerate until the box has laid out. In frozen
	// mode there is nothing to measure, so we lay out against FROZEN_WIDTH.
	let plotWidth = $state(0);
	const layoutWidth = $derived(frozen ? FROZEN_WIDTH : plotWidth);

	const scored = $derived(scoredComments(comments));
	const hasScored = $derived(scored.length > 0);

	// divisiveness -> x pixel. forceX reads this, so a new scale (on resize) means
	// new forces, which restarts the layout at the correct positions.
	const xScale = $derived(
		scaleSqrt()
			.domain([0, Math.max(...scored.map((c) => c.divisiveness))])
			.range([PADDING, Math.max(PADDING, layoutWidth - PADDING)])
	);

	// One d3-force node per scored comment. Fresh objects each time so the
	// simulation (which mutates x/y onto them) never touches the comment data.
	// Seed each at its x target and the vertical centre so collision only has to
	// nudge overlapping dots apart, rather than fighting d3-force's default spiral
	// seed (which would otherwise pin everything to the top edge).
	const nodes = $derived<SwarmNode[]>(
		scored.map((c) => ({
			tid: c.tid,
			text: c.text,
			divisiveness: c.divisiveness,
			x: xScale(c.divisiveness),
			y: HEIGHT / 2
		}))
	);

	const forces = $derived<Record<string, Force<SwarmNode, undefined>>>({
		x: forceX<SwarmNode>((d) => xScale(d.divisiveness)).strength(1),
		y: forceY<SwarmNode>(HEIGHT / 2).strength(0.2),
		collide: forceCollide<SwarmNode>(RADIUS + 1).strength(1)
	});

	// Stable wrapper so the simulation only restarts when the nodes actually change
	// (resize / new data), not on every hover-driven re-render of the dots.
	const simData = $derived({ nodes });

	// Frozen layout: run the same forces to a settled state synchronously (no animation,
	// no measurement) so a snapshot captures the finished swarm. Fresh node objects, since
	// the simulation mutates x/y onto them.
	const frozenPlaced = $derived.by<SwarmNode[]>(() => {
		if (!frozen || !hasScored) return [];
		const simNodes: SwarmNode[] = scored.map((c) => ({
			tid: c.tid,
			text: c.text,
			divisiveness: c.divisiveness,
			x: xScale(c.divisiveness),
			y: HEIGHT / 2
		}));
		forceSimulation(simNodes)
			.force('x', forceX<SwarmNode>((d) => xScale(d.divisiveness)).strength(1))
			.force('y', forceY<SwarmNode>(HEIGHT / 2).strength(0.2))
			.force('collide', forceCollide<SwarmNode>(RADIUS + 1).strength(1))
			.stop()
			.tick(300);
		return simNodes;
	});

	// Default focus = the most divisive statement, so the block below is always
	// populated. Hover/focus/click a dot to pin a different one; leaving keeps the
	// last selection (sticky).
	let selectedTid = $state<number | null>(null);
	const activeTid = $derived(selectedTid ?? mostDivisiveTid(comments));
	const activeComment = $derived(comments.find((c) => c.tid === activeTid) ?? null);
</script>

<Card.Root class="gap-4 rounded-md p-0 px-4 py-3.5 shadow-none">
	<header class="flex flex-col gap-0.5">
		<h2 class="text-foreground text-lg font-bold">Consensus continuum</h2>
		<p class="text-muted-foreground text-sm font-medium">
			Statements distribution on a consensus continuum
		</p>
	</header>

	{#if !hasScored}
		<p class="text-muted-foreground bg-muted rounded-[10px] px-4 py-10 text-center text-base">
			Not enough votes yet to place statements on the continuum.
		</p>
	{:else}
		<!-- Plot box -->
		<div
			class="bg-muted relative h-36 overflow-hidden rounded-[10px]"
			bind:clientWidth={plotWidth}
		>
			{#if frozen}
				<!-- Settled, non-interactive swarm for the snapshot; viewBox scales to fit. -->
				<svg
					class="h-full w-full"
					viewBox="0 0 {FROZEN_WIDTH} {HEIGHT}"
					preserveAspectRatio="none"
					role="presentation"
				>
					{#each frozenPlaced as n (n.tid)}
						{@const isActive = n.tid === activeTid}
						<circle
							cx={n.x ?? 0}
							cy={n.y ?? HEIGHT / 2}
							r={RADIUS}
							fill={isActive ? 'var(--primary)' : 'var(--card-foreground)'}
							opacity={isActive ? 1 : 0.9}
						/>
					{/each}
				</svg>
			{:else if plotWidth > 0}
				<svg class="h-full w-full" role="presentation">
					<ForceSimulation {forces} data={simData} cloneNodes>
						{#snippet children({ nodes: placed })}
							{#each placed as n (n.tid)}
								{@const isActive = n.tid === activeTid}
								<circle
									role="button"
									tabindex="0"
									aria-label={n.text.trim()}
									cx={n.x ?? 0}
									cy={n.y ?? HEIGHT / 2}
									r={RADIUS}
									fill={isActive ? 'var(--primary)' : 'var(--card-foreground)'}
									opacity={isActive ? 1 : 0.9}
									class="cursor-pointer transition-opacity duration-150 focus-visible:outline-none"
									onmouseenter={() => (selectedTid = n.tid)}
									onfocus={() => (selectedTid = n.tid)}
									onclick={() => (selectedTid = n.tid)}
									onkeydown={(e) => {
										if (e.key === 'Enter' || e.key === ' ') {
											e.preventDefault();
											selectedTid = n.tid;
										}
									}}
								/>
							{/each}
						{/snippet}
					</ForceSimulation>
				</svg>
			{/if}
		</div>

		<!-- Axis labels -->
		<div class="text-foreground flex items-start justify-between text-xs font-medium">
			<span>CONSENSUS STATEMENT</span>
			<span>DIVISIVE STATEMENT</span>
		</div>

		<!-- Focused statement -->
		{#if activeComment}
			<div class="border-border rounded-[10px] border">
				<StatementVoteBlock comment={activeComment} {groups} />
			</div>
		{/if}
	{/if}
</Card.Root>
