import { describe, it, expect } from 'vitest';
import {
	computeStage,
	ratchet,
	nextUnlock,
	describeUnlock,
	voterCount,
	scoredCount,
	DEFAULT_THRESHOLDS,
	type RevealThresholds
} from './revealStage';
import type { DisplayState, Vote } from './types';
import type { ReportComment } from '$lib/tools/polis/reportTypes';

function comment(tid: number, divisiveness: number | null): ReportComment {
	return {
		tid,
		text: `stmt ${tid}`,
		is_seed: false,
		overall_votes: { agrees: 0, disagrees: 0, passes: 0 },
		group_votes: [],
		divisiveness
	};
}

/** A state with `voters` people who have each voted `each` times. */
function state(voters: number, each: number, scored = 0, unscored = 0): DisplayState {
	const votesByTid = new Map<number, Map<number, Vote>>();
	for (let tid = 0; tid < each; tid++) {
		const byNode = new Map<number, Vote>();
		for (let n = 0; n < voters; n++) byNode.set(n, 'agree');
		votesByTid.set(tid, byNode);
	}
	const published = [
		...Array.from({ length: scored }, (_, i) => comment(i, 1.5)),
		...Array.from({ length: unscored }, (_, i) => comment(1000 + i, null))
	];
	return { atMs: 0, nodes: [], published, votesByTid, totalVotes: voters * each };
}

describe('voterCount', () => {
	it('counts distinct nodes across all statements, not votes', () => {
		expect(voterCount(state(6, 4))).toBe(6);
	});

	it('is zero with no votes', () => {
		expect(voterCount(state(0, 0))).toBe(0);
	});
});

describe('scoredCount', () => {
	it('counts only statements Polis has given a finite divisiveness', () => {
		expect(scoredCount(state(1, 1, 3, 5))).toBe(3);
	});
});

describe('computeStage', () => {
	it('is empty with no votes at all', () => {
		expect(computeStage(state(0, 0))).toBe('empty');
	});

	it('is warming once someone votes but too few have clustered', () => {
		expect(computeStage(state(3, 6))).toBe('warming');
	});

	it('is warming when enough people voted but each too little', () => {
		expect(computeStage(state(12, 2))).toBe('warming');
	});

	it('is shaped once both voter and depth thresholds are met', () => {
		expect(computeStage(state(8, 4))).toBe('shaped');
	});

	it('reaches rich only once enough statements are scored', () => {
		expect(computeStage(state(10, 6, 11))).toBe('shaped');
		expect(computeStage(state(10, 6, 12))).toBe('rich');
	});

	it('is not monotonic, which is why the caller must ratchet', () => {
		// Eight voters at four votes each clusters. A ninth arriving with a single
		// vote drags the mean below the depth threshold and the raw stage falls back.
		const clustered = state(8, 4);
		expect(computeStage(clustered)).toBe('shaped');

		const withNewcomer = state(8, 4);
		withNewcomer.votesByTid.get(0)?.set(99, 'agree');
		withNewcomer.totalVotes += 1;
		expect(computeStage(withNewcomer)).toBe('warming');
	});
});

describe('ratchet', () => {
	it('takes a stronger computed stage', () => {
		expect(ratchet('warming', 'shaped')).toBe('shaped');
	});

	it('never gives a reached stage back', () => {
		expect(ratchet('shaped', 'warming')).toBe('shaped');
		expect(ratchet('rich', 'empty')).toBe('rich');
	});

	it('is stable when nothing changed', () => {
		expect(ratchet('shaped', 'shaped')).toBe('shaped');
	});
});

describe('nextUnlock', () => {
	it('counts voters toward warming', () => {
		expect(nextUnlock(state(0, 0), 'empty')).toEqual({
			stage: 'warming',
			metric: 'voters',
			remaining: 1
		});
	});

	it('counts the missing voters toward shaped while people are the constraint', () => {
		expect(nextUnlock(state(3, 8), 'warming')).toEqual({
			stage: 'shaped',
			metric: 'voters',
			remaining: 5
		});
	});

	it('switches to counting votes once the room is big enough but shallow', () => {
		const unlock = nextUnlock(state(10, 1), 'warming');
		expect(unlock?.metric).toBe('votes');
		expect(unlock?.remaining).toBe(
			DEFAULT_THRESHOLDS.shapedVoters * DEFAULT_THRESHOLDS.shapedVotesPerVoter - 10
		);
	});

	it('never reports zero remaining, so the countdown cannot stall at nothing', () => {
		const thresholds: RevealThresholds = { ...DEFAULT_THRESHOLDS, richScoredStatements: 2 };
		const unlock = nextUnlock(state(10, 6, 5), 'shaped', thresholds);
		expect(unlock?.remaining).toBeGreaterThan(0);
	});

	it('counts scored statements toward rich', () => {
		expect(nextUnlock(state(10, 6, 4), 'shaped')).toEqual({
			stage: 'rich',
			metric: 'statements',
			remaining: 8
		});
	});

	it('is null once rich is reached', () => {
		expect(nextUnlock(state(10, 6, 20), 'rich')).toBeNull();
	});
});

describe('describeUnlock', () => {
	it('uses the singular for one', () => {
		expect(describeUnlock({ stage: 'warming', metric: 'voters', remaining: 1 })).toBe(
			'1 more voter'
		);
		expect(describeUnlock({ stage: 'shaped', metric: 'votes', remaining: 1 })).toBe(
			'1 more vote'
		);
		expect(describeUnlock({ stage: 'rich', metric: 'statements', remaining: 1 })).toBe(
			'1 more statement'
		);
	});

	it('uses the plural for anything else', () => {
		expect(describeUnlock({ stage: 'warming', metric: 'voters', remaining: 4 })).toBe(
			'4 more voters'
		);
	});
});
