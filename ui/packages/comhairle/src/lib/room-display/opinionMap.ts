/**
 * Geometry and colour for the opinion map, kept pure so it can be unit-tested away
 * from the SVG.
 *
 * The map's job on a projector is to be readable from across a room with no pointer,
 * which drives two choices here: colour carries the meaning (not position alone), and
 * dots move rather than appear, because a room notices motion and does not notice a
 * legend.
 */

import type { MapNode, VoteDistribution } from './types';

/**
 * Categorical palette for opinion groups.
 *
 * Deliberately not `--chart-1..5`: that ramp is sequential, and in several themes it
 * is monochromatic (the default green theme runs #7cb342, #6b9b3a, #a3c97a), so two
 * adjacent groups come out indistinguishable. Opinion groups are categories, not
 * magnitudes, and telling them apart is the entire job of the map.
 *
 * These are the Okabe-Ito colourblind-safe categorical hues, which hold up on both the
 * light and dark grounds the display runs on. They overlap in hue with the `--vote-*`
 * tokens, which is safe because group colouring and cross-highlight never appear at
 * the same time: focusing a statement replaces one with the other.
 *
 * Hardcoded here rather than added to `comhairle-themes.css` because that file mirrors
 * the Figma "Themes" collection. Proper categorical group tokens are a design decision,
 * not one to make by editing the mirror.
 */
const GROUP_COLORS = ['#0072b2', '#e69f00', '#009e73', '#cc79a7', '#56b4e9'] as const;

/** Opinion-group fill. Polis rarely exceeds five groups; beyond that the ramp wraps. */
export function groupColor(groupId: number | null): string {
	if (groupId === null) return 'var(--vote-not-voted)';
	return GROUP_COLORS[groupId % GROUP_COLORS.length];
}

/**
 * Cross-highlight fill: how this dot counts on the focused statement.
 *
 * Takes the dominant category rather than drawing a split. At roundtable scale a dot
 * holds one member, so "dominant" is exact and the dot takes a single colour. Above
 * Polis's 100-cluster cap a dot holds several and a 51/49 split would render as a flat
 * majority colour, which is a known gap: that case wants a pie or a size-weighted
 * treatment, and is deferred with the rest of the over-100 work.
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

/** Whether a dot needs the thin outline that keeps near-white off a pale ground. */
export function needsOutline(fill: string): boolean {
	return fill === 'var(--vote-not-voted)';
}

/**
 * How settled a dot is, from 0 (just arrived, out on the ring) to 1 (in its cluster).
 *
 * This is what makes the map appear to move as people vote, which is the client's
 * first ask. It is honest rather than decorative: Polis genuinely does not know where
 * someone belongs until they have voted a few times, so an unsettled dot is a dot
 * whose position is not yet earned.
 */
export function settleFactor(votesCast: number, settleVotes = 6): number {
	if (settleVotes <= 0) return 1;
	return Math.max(0, Math.min(1, votesCast / settleVotes));
}

/**
 * Where a dot sits right now: interpolated from a deterministic point on the entry
 * ring toward its clustered position.
 *
 * The ring start is derived from the node id rather than randomised at render, so a
 * dot does not jump when the component re-renders and a replayed scenario looks the
 * same every time.
 */
export function nodePosition(node: MapNode, factor: number): { x: number; y: number } {
	const angle = (node.id * 2.399963) % (Math.PI * 2); // golden angle, spreads ids evenly
	const ringX = Math.cos(angle) * 1.35;
	const ringY = Math.sin(angle) * 1.35;
	const t = Math.max(0, Math.min(1, factor));
	// Ease out, so a dot moves decisively on its first few votes and then settles,
	// rather than creeping the whole way at a constant speed.
	const eased = 1 - (1 - t) * (1 - t);
	return {
		x: ringX + (node.x - ringX) * eased,
		y: ringY + (node.y - ringY) * eased
	};
}

/** Dot radius. Area scales with members so ten people do not read as ten times one. */
export function dotRadius(memberCount: number, base = 9): number {
	return base * Math.sqrt(Math.max(1, memberCount));
}

/** The room the base radius was tuned in: 28 people at a roundtable, one dot each. */
const TUNED_ROOM = 28;
/** Below this, in viewBox units, a dot stops reading as a person from the back row. */
export const MIN_DOT_RADIUS = 5;
/**
 * Above this a cluster stops being a pile of people and becomes a blob. About a
 * twentieth of the plot's width.
 */
export const MAX_DOT_RADIUS = 24;

/**
 * Radius of a one-member dot for a room of `people`, in viewBox units.
 *
 * The plot is a fixed box, so the only honest way to size a dot is by how many have
 * to share it: area per person stays roughly constant, which means a room of seven
 * gets bigger dots than a room of seventy. `scale` is the board's size step and
 * nudges that inside the same floor and ceiling; it cannot push a busy room's dots
 * into each other, which is what a plain multiplier did.
 */
export function baseDotRadius(people: number, scale = 1, tunedRadius = 17): number {
	const byDensity = tunedRadius * Math.sqrt(TUNED_ROOM / Math.max(1, people));
	return Math.min(MAX_DOT_RADIUS, Math.max(MIN_DOT_RADIUS, byDensity * scale));
}

/** A dot with the position it wants and the room it needs. */
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
/**
 * How far a dot moves toward its target each pass, as a fraction of the distance.
 * Small on purpose: at rest the pull is balanced by the push of a slight overlap,
 * and that overlap is this fraction of how far a dot has been pushed from its
 * target. A dot at the edge of a full pile sits a hundred units out, so this keeps
 * the overlap to a touch of a few percent of a dot, never a stack. Weaker would hold
 * still better but catch up slower when a dot's target moves.
 */
const PULL = 0.02;
/** Passes for the pull and the push to settle from any starting arrangement. */
const SEPARATION_PASSES = 150;

/**
 * Nudges dots apart so none draws on top of another, keeping each as close to its
 * target as its neighbours allow.
 *
 * Two people with near-identical opinions land on the same point, and at room scale
 * a whole cluster can overlap into one shape. Pulling each dot toward its own target
 * while overlapping pairs push apart turns the cluster into a pile with the same
 * outline, which is both more readable and more honest: every person stays visible.
 *
 * A plain relaxation rather than a force simulation, run to rest synchronously,
 * because the dots already animate: the component transitions each one to wherever
 * this says it goes. Every pass works out each dot's pull and every pair's push from
 * the same snapshot and applies them together. That order-free step is what makes a
 * settled layout a fixed point: seeded from `previous`, a layout that has settled
 * stays put when nothing near it changes, so a wall that recomputes on every vote
 * holds still between votes. (Resolving pairs one after another, the way a
 * simulation does, leaves a pile creeping round a little on every run.) Coincident
 * dots are split along a direction taken from their ids, so the same inputs give the
 * same layout on every machine.
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

/** What `groupCentroids` needs from a rendered dot. */
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
 * Where each opinion group's label goes: the mean of its settled dots, plus the
 * cluster's lower edge.
 *
 * Only settled dots count. A dot still travelling in from the entry ring is not in
 * its cluster yet, and averaging it in would drag the label out toward the ring. The
 * upshot is that a label appears once the first member arrives and follows the
 * cluster as it fills, which is the "labels that move with the dots" the client asked
 * for. A group with no settled dot has no label.
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
