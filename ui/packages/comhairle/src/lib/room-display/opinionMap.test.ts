import { describe, it, expect } from 'vitest';
import {
	groupColor,
	voteColor,
	needsOutline,
	settleFactor,
	nodePosition,
	dotRadius,
	votesCastBy,
	groupCentroids,
	baseDotRadius,
	separateDots,
	MIN_DOT_RADIUS,
	MAX_DOT_RADIUS
} from './opinionMap';
import type { MapNode } from './types';

function dist(agrees = 0, disagrees = 0, passes = 0, notVoted = 0) {
	return { agrees, disagrees, passes, notVoted };
}

const node: MapNode = { id: 3, x: 0.5, y: -0.25, groupId: 1, memberCount: 1 };

describe('groupColor', () => {
	it('gives adjacent groups visibly different colours, not a sequential ramp', () => {
		expect(groupColor(0)).not.toBe(groupColor(1));
		expect(groupColor(1)).not.toBe(groupColor(2));
	});

	it('gives every group up to five a distinct colour', () => {
		const colors = [0, 1, 2, 3, 4].map(groupColor);
		expect(new Set(colors).size).toBe(5);
	});

	it('wraps past five groups rather than running off the ramp', () => {
		expect(groupColor(5)).toBe(groupColor(0));
	});

	it('renders an unclustered dot as empty, not as group A', () => {
		expect(groupColor(null)).toBe('var(--vote-not-voted)');
	});
});

describe('voteColor', () => {
	it('maps each vote to its shared token', () => {
		expect(voteColor(dist(1))).toBe('var(--vote-agreed)');
		expect(voteColor(dist(0, 1))).toBe('var(--vote-disagreed)');
		expect(voteColor(dist(0, 0, 1))).toBe('var(--vote-passed)');
		expect(voteColor(dist(0, 0, 0, 1))).toBe('var(--vote-not-voted)');
	});

	it('takes the dominant category for a multi-member dot', () => {
		expect(voteColor(dist(2, 7, 1))).toBe('var(--vote-disagreed)');
	});

	it('reads an all-zero distribution as not voted rather than as agreement', () => {
		expect(voteColor(dist())).toBe('var(--vote-not-voted)');
	});
});

describe('needsOutline', () => {
	it('outlines only the near-white fill', () => {
		expect(needsOutline('var(--vote-not-voted)')).toBe(true);
		expect(needsOutline('var(--vote-agreed)')).toBe(false);
	});
});

describe('settleFactor', () => {
	it('is zero before anyone votes and one once settled', () => {
		expect(settleFactor(0)).toBe(0);
		expect(settleFactor(6)).toBe(1);
	});

	it('clamps rather than overshooting', () => {
		expect(settleFactor(100)).toBe(1);
		expect(settleFactor(-5)).toBe(0);
	});

	it('is monotonic across its range', () => {
		expect(settleFactor(2)).toBeLessThan(settleFactor(4));
	});
});

describe('nodePosition', () => {
	it('starts on the entry ring and ends at the clustered position', () => {
		const start = nodePosition(node, 0);
		expect(Math.hypot(start.x, start.y)).toBeCloseTo(1.35, 5);
		expect(nodePosition(node, 1)).toEqual({ x: node.x, y: node.y });
	});

	it('is deterministic, so a dot does not jump between renders', () => {
		expect(nodePosition(node, 0.4)).toEqual(nodePosition(node, 0.4));
	});

	it('puts different ids at different places on the ring', () => {
		const a = nodePosition({ ...node, id: 1 }, 0);
		const b = nodePosition({ ...node, id: 2 }, 0);
		expect(a).not.toEqual(b);
	});

	it('clamps a factor outside zero to one', () => {
		expect(nodePosition(node, 5)).toEqual(nodePosition(node, 1));
		expect(nodePosition(node, -1)).toEqual(nodePosition(node, 0));
	});
});

describe('dotRadius', () => {
	it('scales by area, so ten members do not read as ten times one', () => {
		expect(dotRadius(1, 10)).toBe(10);
		expect(dotRadius(4, 10)).toBe(20);
	});

	it('never shrinks below the single-member size', () => {
		expect(dotRadius(0, 10)).toBe(10);
	});
});

describe('votesCastBy', () => {
	it('counts statements a node voted on, not total votes in the room', () => {
		const votes = new Map([
			[0, new Map([[1, 'agree']])],
			[
				1,
				new Map([
					[1, 'agree'],
					[2, 'disagree']
				])
			],
			[2, new Map([[2, 'pass']])]
		]);
		expect(votesCastBy(votes, 1)).toBe(2);
		expect(votesCastBy(votes, 2)).toBe(2);
		expect(votesCastBy(votes, 99)).toBe(0);
	});
});

describe('groupCentroids', () => {
	const settled = (groupId: number | null, x: number, y: number) => ({
		groupId,
		x,
		y,
		radius: 10,
		settled: true
	});

	it('averages settled members and reports the cluster bottom edge', () => {
		const [a] = groupCentroids([settled(0, 0, 0), settled(0, 20, 40)]);
		expect(a).toEqual({ groupId: 0, x: 10, y: 20, bottom: 50, members: 2 });
	});

	it('ignores dots still travelling in from the ring', () => {
		const centroids = groupCentroids([
			settled(0, 0, 0),
			{ groupId: 0, x: 200, y: 200, radius: 10, settled: false }
		]);
		expect(centroids[0]).toMatchObject({ x: 0, y: 0, members: 1 });
	});

	it('gives no label to a group with nothing settled, or to unclustered dots', () => {
		const centroids = groupCentroids([
			{ groupId: 1, x: 0, y: 0, radius: 10, settled: false },
			settled(null, 5, 5)
		]);
		expect(centroids).toEqual([]);
	});

	it('orders labels by group id so A comes before B', () => {
		const ids = groupCentroids([settled(1, 0, 0), settled(0, 0, 0)]).map((c) => c.groupId);
		expect(ids).toEqual([0, 1]);
	});
});

describe('baseDotRadius', () => {
	it('gives the tuned room the tuned radius', () => {
		expect(baseDotRadius(28)).toBe(17);
	});

	it('shrinks dots as the room fills and grows them as it empties', () => {
		expect(baseDotRadius(112)).toBeCloseTo(8.5);
		expect(baseDotRadius(7)).toBe(MAX_DOT_RADIUS);
		expect(baseDotRadius(5000)).toBe(MIN_DOT_RADIUS);
	});

	it('lets the size step nudge inside the same bounds', () => {
		expect(baseDotRadius(28, 1.25)).toBeCloseTo(21.25);
		// A 2x step on a full room hits the ceiling rather than doubling.
		expect(baseDotRadius(28, 2)).toBe(MAX_DOT_RADIUS);
		expect(baseDotRadius(200, 2)).toBeLessThan(MAX_DOT_RADIUS);
	});
});

describe('separateDots', () => {
	const distance = (a: { x: number; y: number }, b: { x: number; y: number }) =>
		Math.hypot(a.x - b.x, a.y - b.y);

	it('leaves dots that do not touch where they are', () => {
		const placed = separateDots([
			{ id: 1, x: 0, y: 0, radius: 10 },
			{ id: 2, x: 100, y: 0, radius: 10 }
		]);
		expect(placed.get(1)!.x).toBeCloseTo(0);
		expect(placed.get(1)!.y).toBeCloseTo(0);
		expect(placed.get(2)!.x).toBeCloseTo(100);
		expect(placed.get(2)!.y).toBeCloseTo(0);
	});

	it('pushes coincident dots apart until they no longer overlap', () => {
		const targets = [1, 2, 3, 4, 5].map((id) => ({ id, x: 50, y: 50, radius: 10 }));
		const placed = separateDots(targets);
		for (const a of targets) {
			for (const b of targets) {
				if (a.id >= b.id) continue;
				expect(distance(placed.get(a.id)!, placed.get(b.id)!)).toBeGreaterThanOrEqual(20);
			}
		}
	});

	it('keeps a separated pile around its target', () => {
		const targets = [1, 2, 3, 4, 5, 6].map((id) => ({ id, x: 50, y: 50, radius: 10 }));
		const placed = separateDots(targets);
		const points = [...placed.values()];
		const mean = {
			x: points.reduce((sum, p) => sum + p.x, 0) / points.length,
			y: points.reduce((sum, p) => sum + p.y, 0) / points.length
		};
		expect(distance(mean, { x: 50, y: 50 })).toBeLessThan(5);
	});

	it('is the same every time for the same input', () => {
		const targets = [1, 2, 3].map((id) => ({ id, x: 10, y: 10, radius: 8 }));
		expect(separateDots(targets)).toEqual(separateDots(targets));
	});

	it('holds still when run again from where it settled', () => {
		const targets = [1, 2, 3, 4].map((id) => ({ id, x: 0, y: 0, radius: 10 }));
		const settled = separateDots(targets, separateDots(targets));
		const again = separateDots(targets, settled);
		for (const id of [1, 2, 3, 4]) {
			expect(distance(settled.get(id)!, again.get(id)!)).toBeLessThan(0.5);
		}
	});

	it('starts from the previous placement so one change does not reshuffle the rest', () => {
		const targets = [1, 2, 3, 4].map((id) => ({ id, x: 0, y: 0, radius: 10 }));
		const settled = separateDots(targets, separateDots(targets));
		const again = separateDots([...targets, { id: 5, x: 200, y: 200, radius: 10 }], settled);
		for (const id of [1, 2, 3, 4]) {
			expect(distance(settled.get(id)!, again.get(id)!)).toBeLessThan(0.5);
		}
	});
});
