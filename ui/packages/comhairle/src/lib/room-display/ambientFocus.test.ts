import { describe, it, expect } from 'vitest';
import {
	ambientFocusAt,
	resolveIntent,
	describeIntent,
	AMBIENT_ROTATION,
	DEFAULT_DWELL_MS
} from './ambientFocus';
import type { DisplayState, Vote } from './types';
import type { ReportComment } from '$lib/tools/polis/reportTypes';

function comment(
	tid: number,
	{ divisiveness = 1, consensus = 0.5 }: { divisiveness?: number | null; consensus?: number } = {}
): ReportComment {
	return {
		tid,
		text: `stmt ${tid}`,
		is_seed: false,
		overall_votes: { agrees: 1, disagrees: 1, passes: 0 },
		group_votes: [],
		divisiveness,
		group_informed_consensus: consensus
	};
}

/** `published` is most-recent-first, matching what the fold produces. */
function state(published: ReportComment[], votes: Record<number, number[]> = {}): DisplayState {
	const votesByTid = new Map<number, Map<number, Vote>>();
	for (const [tid, voters] of Object.entries(votes)) {
		votesByTid.set(Number(tid), new Map(voters.map((v) => [v, 'agree' as Vote])));
	}
	return { atMs: 0, nodes: [], published, votesByTid, totalVotes: 0 };
}

const three = [
	comment(3, { divisiveness: 0.4, consensus: 0.9 }),
	comment(2, { divisiveness: 4.5, consensus: 0.1 }),
	comment(1, { divisiveness: 2.0, consensus: 0.5 })
];

describe('resolveIntent', () => {
	it('finds the most divisive statement', () => {
		expect(resolveIntent(state(three), 'mostDivisive')).toBe(2);
	});

	it('finds the strongest consensus', () => {
		expect(resolveIntent(state(three), 'strongestConsensus')).toBe(3);
	});

	it('treats the head of the list as newest', () => {
		expect(resolveIntent(state(three), 'newest')).toBe(3);
	});

	it('finds the most voted on by voter count, not by score', () => {
		expect(resolveIntent(state(three, { 1: [1, 2, 3, 4], 2: [1] }), 'mostVoted')).toBe(1);
	});

	it('ignores unscored statements when ranking divisiveness', () => {
		const mixed = [comment(9, { divisiveness: null }), comment(8, { divisiveness: 0.3 })];
		expect(resolveIntent(state(mixed), 'mostDivisive')).toBe(8);
	});

	it('returns null when nothing is published', () => {
		for (const intent of AMBIENT_ROTATION) {
			expect(resolveIntent(state([]), intent)).toBeNull();
		}
	});
});

describe('ambientFocusAt', () => {
	it('advances through the rotation as time passes', () => {
		const s = state(three, { 1: [1, 2, 3], 2: [1] });
		const intents = [0, 1, 2, 3].map((i) => ambientFocusAt(s, i * DEFAULT_DWELL_MS).intent);
		expect(intents).toEqual([...AMBIENT_ROTATION]);
	});

	it('wraps back to the start of the rotation', () => {
		const s = state(three, { 1: [1] });
		expect(ambientFocusAt(s, 4 * DEFAULT_DWELL_MS).intent).toBe(AMBIENT_ROTATION[0]);
	});

	it('holds one focus for the whole dwell', () => {
		const s = state(three, { 1: [1] });
		const a = ambientFocusAt(s, 0);
		const b = ambientFocusAt(s, DEFAULT_DWELL_MS - 1);
		expect(b).toEqual(a);
	});

	it('is pure, so scrubbing lands on the same focus', () => {
		const s = state(three, { 1: [1] });
		expect(ambientFocusAt(s, 30_000)).toEqual(ambientFocusAt(s, 30_000));
	});

	it('always points at something once anything is published', () => {
		const s = state([comment(7, { divisiveness: null })]);
		for (let i = 0; i < AMBIENT_ROTATION.length; i++) {
			expect(ambientFocusAt(s, i * DEFAULT_DWELL_MS).tid).toBe(7);
		}
	});

	it('reports no focus when nothing is published, rather than throwing', () => {
		expect(ambientFocusAt(state([]), 0).tid).toBeNull();
	});

	it('treats a zero dwell as a fixed focus rather than dividing by zero', () => {
		const s = state(three, { 1: [1] });
		expect(ambientFocusAt(s, 99_999, 0).intent).toBe(AMBIENT_ROTATION[0]);
	});
});

describe('describeIntent', () => {
	it('gives every intent a caption, so the room knows why it is looking at this', () => {
		for (const intent of AMBIENT_ROTATION) {
			expect(describeIntent(intent).length).toBeGreaterThan(0);
		}
	});
});
