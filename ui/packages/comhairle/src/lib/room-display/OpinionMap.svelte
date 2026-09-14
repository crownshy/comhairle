<!--
	@component The opinion map: one dot per Polis base cluster, positioned by its PCA
	coordinates and coloured by opinion group.

	Two behaviours in one component, because the display is ambient by default and
	driven only when the facilitator touches it (CONTEXT.md, "Ambient mode / Driven
	mode"):

	- Colour carries the meaning, never a tooltip. A projector has no pointer for most
	  of a session, so the legend is always on screen and every state is readable from
	  across a room.
	- Dots migrate rather than appear. Each one enters on a ring and travels to its
	  clustered position as its owner votes, which is the client's "clusters moving
	  with voting". It is honest rather than decorative: Polis does not know where
	  someone belongs until they have voted a few times, so an unsettled dot is one
	  whose position is not yet earned.

	Cross-highlight: pass `focusedTid` and every dot recolours to how that node voted
	on that statement. At roundtable scale a dot is one person and the colour is simply
	their vote.

	Group labels sit under each cluster and follow it: they are placed at the centroid
	of the group's settled dots, so a label appears when the first member arrives and
	drifts as the cluster fills. They stay up during a cross-highlight, because that is
	exactly when the room wants to know which cluster is which.

	Dumb: takes nodes and votes, renders them. It owns no selection state; the parent
	decides what is focused and what a hover means.
-->
<script lang="ts">
	import type { MapNode, Vote } from './types';
	import {
		groupColor,
		voteColor,
		needsOutline,
		settleFactor,
		nodePosition,
		dotRadius,
		votesCastBy,
		groupCentroids
	} from './opinionMap';
	import { groupLabel } from '$lib/tools/polis/report';

	type Props = {
		nodes: MapNode[];
		/** Votes so far, per statement tid, keyed by node id. */
		votesByTid: Map<number, Map<number, Vote>>;
		/** Statement to cross-highlight against, or null to colour by opinion group. */
		focusedTid?: number | null;
		/** Opinion groups present, for the legend. */
		groupIds?: number[];
		/** Votes a node must cast before it reaches its clustered position. */
		settleVotes?: number;
		/** Driven mode makes hover live; ambient ignores the pointer entirely. */
		interactive?: boolean;
		onhovernode?: (nodeId: number | null) => void;
	};

	let {
		nodes,
		votesByTid,
		focusedTid = null,
		groupIds = [],
		settleVotes = 6,
		interactive = false,
		onhovernode
	}: Props = $props();

	// User units, mapped 1:1 to px by the CSS transform on each dot. The viewBox is a
	// little wider than the entry ring so a just-arrived dot is never clipped.
	const SCALE = 150;
	const EXTENT = 1.65 * SCALE;

	/**
	 * Radius of a one-member dot, in the same user units. Sized for a room rather than
	 * for the density of the plot: at roundtable scale a dot is a person, and a person
	 * has to be visible from the back row. Crowding is the trade, and clusters at 28
	 * participants have the room for it.
	 */
	const BASE_RADIUS = 17;

	/** Gap between a cluster's lowest dot and its label, in user units. */
	const LABEL_GAP = 30;

	let hoveredId = $state<number | null>(null);

	const dots = $derived(
		nodes.map((node) => {
			const cast = votesCastBy(votesByTid, node.id);
			const position = nodePosition(node, settleFactor(cast, settleVotes));

			let fill: string;
			if (focusedTid === null) {
				fill = groupColor(node.groupId);
			} else {
				const vote = votesByTid.get(focusedTid)?.get(node.id);
				fill = voteColor({
					agrees: vote === 'agree' ? node.memberCount : 0,
					disagrees: vote === 'disagree' ? node.memberCount : 0,
					passes: vote === 'pass' ? node.memberCount : 0,
					notVoted: vote ? 0 : node.memberCount
				});
			}

			return {
				node,
				fill,
				outlined: needsOutline(fill),
				radius: dotRadius(node.memberCount, BASE_RADIUS),
				x: position.x * SCALE,
				y: position.y * SCALE,
				settled: cast >= settleVotes
			};
		})
	);

	const labels = $derived(
		groupCentroids(
			dots.map((d) => ({
				groupId: d.node.groupId,
				x: d.x,
				y: d.y,
				radius: d.radius,
				settled: d.settled
			}))
		)
	);

	const legend = $derived(
		focusedTid === null
			? groupIds.map((id) => ({
					key: `g${id}`,
					label: `Group ${groupLabel(id)}`,
					color: groupColor(id),
					outlined: false
				}))
			: [
					{
						key: 'agreed',
						label: 'Agreed',
						color: 'var(--vote-agreed)',
						outlined: false
					},
					{
						key: 'disagreed',
						label: 'Disagreed',
						color: 'var(--vote-disagreed)',
						outlined: false
					},
					{
						key: 'passed',
						label: 'Passed',
						color: 'var(--vote-passed)',
						outlined: false
					},
					{
						key: 'notVoted',
						label: 'Not voted',
						color: 'var(--vote-not-voted)',
						outlined: true
					}
				]
	);

	function hover(nodeId: number | null) {
		if (!interactive) return;
		hoveredId = nodeId;
		onhovernode?.(nodeId);
	}
</script>

<div class="flex h-full min-h-0 flex-col gap-4">
	<svg
		viewBox="{-EXTENT} {-EXTENT} {EXTENT * 2} {EXTENT * 2}"
		class="min-h-0 w-full flex-1"
		role="img"
		aria-label="Opinion map: {nodes.length} participants{focusedTid === null
			? ''
			: ', coloured by their vote on the focused statement'}"
	>
		{#each dots as dot (dot.node.id)}
			<!--
				The parent svg is role="img" with a summary label, so this subtree is
				already presentational to assistive tech. The role here exists to satisfy
				the interaction rule for the pointer handlers, which only do anything in
				driven mode.
			-->
			<g
				class="dot"
				class:interactive
				class:dimmed={hoveredId !== null && hoveredId !== dot.node.id}
				style="transform: translate({dot.x}px, {dot.y}px);"
				role="img"
				aria-label={dot.node.memberCount === 1
					? 'One participant'
					: `${dot.node.memberCount} participants`}
				onpointerenter={() => hover(dot.node.id)}
				onpointerleave={() => hover(null)}
			>
				<circle
					r={dot.radius}
					fill={dot.fill}
					stroke={dot.outlined ? 'var(--vote-not-voted-border)' : 'transparent'}
					stroke-width={dot.outlined ? 2 : 0}
				/>
			</g>
		{/each}

		{#each labels as label (label.groupId)}
			<text
				class="group-label fill-foreground"
				style="transform: translate({label.x}px, {label.bottom + LABEL_GAP}px);"
				text-anchor="middle"
				dominant-baseline="hanging"
			>
				Group {groupLabel(label.groupId)}
			</text>
		{/each}
	</svg>

	<ul class="flex flex-wrap items-center justify-center gap-x-6 gap-y-2">
		{#each legend as item (item.key)}
			<li class="text-muted-foreground flex items-center gap-2 text-xl">
				<span
					class="inline-block size-6 rounded-full"
					style="background: {item.color};{item.outlined
						? ' box-shadow: inset 0 0 0 1px var(--vote-not-voted-border);'
						: ''}"
				></span>
				{item.label}
			</li>
		{/each}
	</ul>
</div>

<style>
	/*
	 * Movement is the point: a room notices a dot travelling and does not notice a
	 * legend changing. The fill transition is quicker than the position one so a
	 * cross-highlight reads as instant while dots keep drifting to their clusters.
	 */
	.dot {
		transition:
			transform 900ms cubic-bezier(0.22, 1, 0.36, 1),
			opacity 200ms ease;
	}

	/*
	 * Someone joining is announced by their dot arriving and nothing else. No banner,
	 * no caption: a room notices a new thing appear on a map, and text that flashes up
	 * and vanishes is worse than silence on a projector. The keyframes run on element
	 * insertion, so every newly joined participant gets it for free.
	 */
	.dot :global(circle) {
		animation: dot-arrive 700ms cubic-bezier(0.22, 1, 0.36, 1) both;
	}

	@keyframes dot-arrive {
		from {
			r: 0;
			opacity: 0;
		}
		60% {
			opacity: 1;
		}
		to {
			opacity: 1;
		}
	}

	.dot :global(circle) {
		transition: fill 300ms ease;
	}

	/*
	 * Same easing as the dots so a label rides with its cluster rather than lagging
	 * it. Sized in user units to match the dots; 22 is about the width of one.
	 */
	.group-label {
		font-size: 22px;
		font-weight: 700;
		transition: transform 900ms cubic-bezier(0.22, 1, 0.36, 1);
		animation: label-arrive 500ms ease both;
	}

	@keyframes label-arrive {
		from {
			opacity: 0;
		}
	}

	.interactive {
		cursor: pointer;
	}

	.dimmed {
		opacity: 0.25;
	}

	@media (prefers-reduced-motion: reduce) {
		.dot,
		.dot :global(circle),
		.group-label {
			transition: none;
			animation: none;
		}
	}
</style>
