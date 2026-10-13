// Geometry and colour for the opinion map, kept out of the component so it can be unit-tested.

import type { MapNode, VoteDistribution } from '../types';

// Colourblind-safe (Okabe-Ito) hues. The theme's --chart-* colours are too similar to
// tell groups apart, and comhairle-themes.css mirrors Figma, so new tokens belong there first.
const GROUP_COLORS = ['#0072b2', '#e69f00', '#009e73', '#cc79a7', '#56b4e9'] as const;

/** Polis rarely makes more than five groups; past that the colours repeat. */
export function groupColor(groupId: number | null): string {
	if (groupId === null) return 'var(--vote-not-voted)';
	return GROUP_COLORS[groupId % GROUP_COLORS.length];
}

/**
 * Colour for a dot while a statement is focused: whichever vote it has most of.
 * A dot holding several people shows only the majority, not the split (see NOTES.md, "Density").
 */
export function voteColor(distribution: VoteDistribution): string {
	const { agrees, disagrees, passes, notVoted } = distribution;
	const top = Math.max(agrees, disagrees, passes, notVoted);
	if (top === 0) return 'var(--vote-not-voted)';
	if (top === agrees) return 'var(--vote-agreed)';
	if (top === disagrees) return 'var(--vote-disagreed)';
	if (top === passes) return 'var(--vote-passed)';
	return 'var(--vote-not-voted)';
}

/** The not-voted colour is near white, so it needs an outline to show on a light background. */
export function needsOutline(fill: string): boolean {
	return fill === 'var(--vote-not-voted)';
}

/**
 * From 0 (just joined, on the outer ring) to 1 (at its cluster position). Polis cannot
 * place someone until they have voted a few times, so dots move in as votes arrive.
 */
export function settleFactor(votesCast: number, settleVotes = 6): number {
	if (settleVotes <= 0) return 1;
	return Math.max(0, Math.min(1, votesCast / settleVotes));
}

/**
 * Where a dot sits between its start on the outer ring and its cluster position.
 * The ring start comes from the node id, not a random number, so re-renders do not make dots jump.
 */
export function nodePosition(node: MapNode, factor: number): { x: number; y: number } {
	// The golden angle spreads consecutive ids evenly around the ring.
	const angle = (node.id * 2.399963) % (Math.PI * 2);
	const ringX = Math.cos(angle) * 1.35;
	const ringY = Math.sin(angle) * 1.35;
	const t = Math.max(0, Math.min(1, factor));
	// Ease out, so a dot moves most on its first few votes instead of at a constant speed.
	const eased = 1 - (1 - t) * (1 - t);
	return {
		x: ringX + (node.x - ringX) * eased,
		y: ringY + (node.y - ringY) * eased
	};
}

/** Area, not radius, grows with members, so a dot of ten people does not look ten times as big. */
export function dotRadius(memberCount: number, base = 9): number {
	return base * Math.sqrt(Math.max(1, memberCount));
}

/** Room size the default radius was tuned for: 28 people, one dot each. */
const TUNED_ROOM = 28;
/** Smallest radius, in viewBox units, that is still visible from the back of the room. */
export const MIN_DOT_RADIUS = 5;
/** Largest radius, about a twentieth of the plot width. Bigger dots merge into one blob. */
export const MAX_DOT_RADIUS = 24;

/**
 * Radius of a one-member dot, sized so each person gets roughly the same area of the plot.
 * `scale` is the board's size setting and stays within the min and max. See NOTES.md, "The board".
 */
export function baseDotRadius(people: number, scale = 1, tunedRadius = 17): number {
	const byDensity = tunedRadius * Math.sqrt(TUNED_ROOM / Math.max(1, people));
	return Math.min(MAX_DOT_RADIUS, Math.max(MIN_DOT_RADIUS, byDensity * scale));
}

export interface DotTarget {
	id: number;
	x: number;
	y: number;
	radius: number;
}

export interface Point {
	x: number;
	y: number;
}

/** Clear space between two neighbouring dots, in viewBox units. */
const DOT_GAP = 2;
// Fraction of the remaining distance a dot moves toward its target each pass. Kept small
// because the resting overlap between dots grows with it; smaller values react more slowly.

const PULL = 0.02;
/** Enough passes for any starting arrangement to settle. */
const SEPARATION_PASSES = 150;

/**
 * Pushes overlapping dots apart while pulling each toward its target, so everyone stays visible.
 * Pass `previous` so a settled layout stays still when nothing near it changes.
 */
export function separateDots(
	targets: DotTarget[],
	previous?: Map<number, Point>
): Map<number, Point> {
	const dots = targets.map((t) => {
		const start = previous?.get(t.id) ?? t;
		return {
			id: t.id,
			x: start.x,
			y: start.y,
			tx: t.x,
			ty: t.y,
			r: t.radius + DOT_GAP / 2,
			dx: 0,
			dy: 0
		};
	});
	// Each pass computes every move from the same snapshot before applying any. Applying moves
	// one pair at a time makes a settled pile keep creeping on every recompute.
	for (let pass = 0; pass < SEPARATION_PASSES; pass++) {
		for (const d of dots) {
			d.dx = (d.tx - d.x) * PULL;
			d.dy = (d.ty - d.y) * PULL;
		}
		for (let i = 0; i < dots.length; i++) {
			for (let j = i + 1; j < dots.length; j++) {
				const a = dots[i];
				const b = dots[j];
				let ux = b.x - a.x;
				let uy = b.y - a.y;
				let distance = Math.hypot(ux, uy);
				const minimum = a.r + b.r;
				if (distance >= minimum) continue;
				// Dots on the same point split along an angle from their ids, so results are repeatable.
				if (distance < 1e-6) {
					const angle = ((a.id + b.id) * 2.399963) % (Math.PI * 2);
					ux = Math.cos(angle);
					uy = Math.sin(angle);
					distance = 1;
				}
				const push = (minimum - distance) / 2;
				ux /= distance;
				uy /= distance;
				a.dx -= ux * push;
				a.dy -= uy * push;
				b.dx += ux * push;
				b.dy += uy * push;
			}
		}
		for (const d of dots) {
			d.x += d.dx;
			d.y += d.dy;
		}
	}
	return new Map(dots.map((d) => [d.id, { x: d.x, y: d.y }]));
}

/** How many votes a node has cast so far, across every statement. */
export function votesCastBy(votesByTid: Map<number, Map<number, unknown>>, nodeId: number): number {
	let count = 0;
	for (const byNode of votesByTid.values()) if (byNode.has(nodeId)) count += 1;
	return count;
}

export interface PlacedDot {
	groupId: number | null;
	x: number;
	y: number;
	radius: number;
	/** Whether the dot has reached its clustered position (see `settleFactor`). */
	settled: boolean;
}

export interface GroupCentroid {
	groupId: number;
	x: number;
	y: number;
	/** Lowest edge of any member dot, so a label can sit under the cluster. */
	bottom: number;
	members: number;
}

/**
 * Label position per opinion group: the average of its settled dots, and the cluster's lower edge.
 * Dots still moving in from the ring are left out so they do not pull the label away.
 */
export function groupCentroids(dots: PlacedDot[]): GroupCentroid[] {
	const byGroup = new Map<number, GroupCentroid>();
	for (const dot of dots) {
		if (dot.groupId === null || !dot.settled) continue;
		const current = byGroup.get(dot.groupId);
		if (current) {
			current.x += dot.x;
			current.y += dot.y;
			current.bottom = Math.max(current.bottom, dot.y + dot.radius);
			current.members += 1;
		} else {
			byGroup.set(dot.groupId, {
				groupId: dot.groupId,
				x: dot.x,
				y: dot.y,
				bottom: dot.y + dot.radius,
				members: 1
			});
		}
	}
	return [...byGroup.values()]
		.map((c) => ({ ...c, x: c.x / c.members, y: c.y / c.members }))
		.sort((a, b) => a.groupId - b.groupId);
}
