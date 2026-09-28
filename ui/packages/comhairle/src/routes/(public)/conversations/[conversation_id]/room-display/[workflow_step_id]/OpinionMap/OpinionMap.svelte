<!--
	@component One dot per Polis base cluster, placed by its PCA position and coloured by
	opinion group, or by vote on `focusedTid`. Holds no selection state; the parent decides.
-->
<script lang="ts">
	import type { MapNode, Vote } from '../types';
	import {
		groupColor,
		voteColor,
		needsOutline,
		settleFactor,
		nodePosition,
		dotRadius,
		baseDotRadius,
		separateDots,
		votesCastBy,
		groupCentroids,
		type Point
	} from './opinionMap';
	import { groupLabel } from '$lib/tools/polis/report';

	type Props = {
		nodes: MapNode[];
		/** Votes so far: statement id to (node id to vote). */
		votesByTid: Map<number, Map<number, Vote>>;
		/** Statement to cross-highlight against, or null to colour by opinion group. */
		focusedTid?: number | null;
		groupIds?: number[];
		/** Votes a node must cast before it reaches its clustered position. */
		settleVotes?: number;
		/** When false, the map ignores the pointer. */
		interactive?: boolean;
		/** Size multiplier for dots and labels, kept within the min and max dot radius. */
		dotScale?: number;
		onhovernode?: (nodeId: number | null) => void;
	};

	let {
		nodes,
		votesByTid,
		focusedTid = null,
		groupIds = [],
		settleVotes = 6,
		interactive = false,
		dotScale = 1,
		onhovernode
	}: Props = $props();

	// The viewBox is a little wider than the entry ring so a just-arrived dot is not clipped.
	const SCALE = 150;
	const EXTENT = 1.65 * SCALE;

	const baseRadius = $derived(
		baseDotRadius(
			nodes.reduce((sum, node) => sum + Math.max(1, node.memberCount), 0),
			dotScale
		)
	);

	const labelGap = $derived(baseRadius * 1.75);

	let hoveredId = $state<number | null>(null);

	const targets = $derived(
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
				radius: dotRadius(node.memberCount, baseRadius),
				x: position.x * SCALE,
				y: position.y * SCALE,
				settled: cast >= settleVotes
			};
		})
	);

	// Seeds the next separation so one vote moves one dot. Deliberately not $state: the
	// derivation below writes it, and making it reactive would re-run that derivation.
	let lastPlacement = new Map<number, Point>();

	const dots = $derived.by(() => {
		const placement = separateDots(
			targets.map((t) => ({ id: t.node.id, x: t.x, y: t.y, radius: t.radius })),
			lastPlacement
		);
		lastPlacement = placement;
		return targets.map((t) => {
			const at = placement.get(t.node.id) ?? t;
			return { ...t, x: at.x, y: at.y };
		});
	});

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
			: ', coloured by how the room voted on the focused statement'}"
	>
		{#each dots as dot (dot.node.id)}
			<!-- The role only satisfies the a11y lint for the pointer handlers; the svg has the label. -->
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
				style="transform: translate({label.x}px, {label.bottom +
					labelGap}px); font-size: {22 * dotScale}px;"
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
	.dot {
		transition:
			transform 900ms cubic-bezier(0.22, 1, 0.36, 1),
			opacity 200ms ease;
	}

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

	/* Same easing as the dots so a label moves with its cluster instead of lagging behind. */
	.group-label {
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
