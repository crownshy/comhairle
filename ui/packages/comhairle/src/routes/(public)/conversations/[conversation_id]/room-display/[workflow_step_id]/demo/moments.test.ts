import { describe, it, expect } from 'vitest';
import {
	advance,
	initialMomentState,
	DEFAULT_MOMENT_CONFIG,
	type MomentConfig,
	type MomentObservation,
	type MomentState,
	type Moment
} from './moments';

const config: MomentConfig = { minGapMs: 10_000, mergeWindowMs: 5_000, stabilitySamples: 3 };

function observe(
	state: MomentState,
	overrides: Partial<MomentObservation> & { atMs: number }
): { state: MomentState; moment: Moment | null } {
	return advance(state, { joined: 0, stage: 'empty', groupCount: 0, ...overrides }, config);
}

/** Feeds a run of observations, returning every moment that fired. */
function run(observations: (Partial<MomentObservation> & { atMs: number })[]): Moment[] {
	let state = initialMomentState();
	const fired: Moment[] = [];
	for (const o of observations) {
		const step = observe(state, o);
		state = step.state;
		if (step.moment) fired.push(step.moment);
	}
	return fired;
}

describe('burst merge', () => {
	it('collapses a rush of joins into one moment carrying the total', () => {
		const fired = run([
			{ atMs: 0, joined: 6 },
			{ atMs: 1000, joined: 9 },
			{ atMs: 2000, joined: 13 },
			{ atMs: 6000 }
		]);
		expect(fired).toEqual([{ kind: 'peopleJoined', count: 28 }]);
	});

	it('holds joins until the merge window has elapsed', () => {
		expect(run([{ atMs: 0, joined: 3 }, { atMs: 4999 }])).toEqual([]);
	});

	it('starts a fresh window for joins arriving after a flush', () => {
		const fired = run([
			{ atMs: 0, joined: 2 },
			{ atMs: 5000 },
			{ atMs: 16_000, joined: 4 },
			{ atMs: 21_000 }
		]);
		expect(fired).toEqual([
			{ kind: 'peopleJoined', count: 2 },
			{ kind: 'peopleJoined', count: 4 }
		]);
	});
});

describe('rate limit', () => {
	it('suppresses a second moment inside the minimum gap', () => {
		const fired = run([
			{ atMs: 0, joined: 1 },
			{ atMs: 5000 },
			{ atMs: 6000, joined: 1 },
			{ atMs: 11_000 }
		]);
		expect(fired).toEqual([{ kind: 'peopleJoined', count: 1 }]);
	});

	it('releases the held moment once the gap has passed', () => {
		const fired = run([
			{ atMs: 0, joined: 1 },
			{ atMs: 5000 },
			{ atMs: 6000, joined: 1 },
			{ atMs: 11_000 },
			{ atMs: 15_001 }
		]);
		expect(fired).toEqual([
			{ kind: 'peopleJoined', count: 1 },
			{ kind: 'peopleJoined', count: 1 }
		]);
	});
});

describe('stage unlocks', () => {
	it('announces each stage once', () => {
		const fired = run([
			{ atMs: 0, stage: 'warming' },
			{ atMs: 1000, stage: 'warming' },
			{ atMs: 20_000, stage: 'shaped' },
			{ atMs: 40_000, stage: 'shaped' }
		]);
		expect(fired).toEqual([
			{ kind: 'stageUnlocked', stage: 'warming' },
			{ kind: 'stageUnlocked', stage: 'shaped' }
		]);
	});

	it('never announces empty', () => {
		expect(run([{ atMs: 0, stage: 'empty' }])).toEqual([]);
	});

	it('outranks a ready join burst', () => {
		const fired = run([
			{ atMs: 0, joined: 5 },
			{ atMs: 6000, stage: 'warming' }
		]);
		expect(fired[0]).toEqual({ kind: 'stageUnlocked', stage: 'warming' });
	});
});

describe('stability gate', () => {
	it('does not announce a group count that has not held long enough', () => {
		const fired = run([
			{ atMs: 0, stage: 'shaped', groupCount: 2 },
			{ atMs: 20_000, stage: 'shaped', groupCount: 2 }
		]);
		expect(fired.filter((m) => m.kind === 'groupFormed')).toEqual([]);
	});

	it('announces once the count has held for the required samples', () => {
		const fired = run([
			{ atMs: 0, stage: 'shaped', groupCount: 2 },
			{ atMs: 20_000, stage: 'shaped', groupCount: 2 },
			{ atMs: 40_000, stage: 'shaped', groupCount: 2 }
		]);
		expect(fired).toContainEqual({ kind: 'groupFormed', groupCount: 2 });
	});

	it('does not cry wolf when Polis oscillates', () => {
		const fired = run([
			{ atMs: 0, stage: 'shaped', groupCount: 2 },
			{ atMs: 20_000, stage: 'shaped', groupCount: 3 },
			{ atMs: 40_000, stage: 'shaped', groupCount: 2 },
			{ atMs: 60_000, stage: 'shaped', groupCount: 3 }
		]);
		expect(fired.filter((m) => m.kind === 'groupFormed')).toEqual([]);
	});

	it('treats a settled fall as a quiet baseline update, not a moment', () => {
		const observations = [
			{ atMs: 0, stage: 'shaped' as const, groupCount: 3 },
			{ atMs: 20_000, stage: 'shaped' as const, groupCount: 3 },
			{ atMs: 40_000, stage: 'shaped' as const, groupCount: 3 },
			// Settles back to two, which must not fire.
			{ atMs: 60_000, stage: 'shaped' as const, groupCount: 2 },
			{ atMs: 80_000, stage: 'shaped' as const, groupCount: 2 },
			{ atMs: 100_000, stage: 'shaped' as const, groupCount: 2 }
		];
		const fired = run(observations).filter((m) => m.kind === 'groupFormed');
		expect(fired).toEqual([{ kind: 'groupFormed', groupCount: 3 }]);
	});

	it('never announces zero groups, however long it settles', () => {
		const fired = run([
			{ atMs: 0, stage: 'shaped', groupCount: 0 },
			{ atMs: 20_000, stage: 'shaped', groupCount: 0 },
			{ atMs: 40_000, stage: 'shaped', groupCount: 0 },
			{ atMs: 60_000, stage: 'shaped', groupCount: 0 }
		]).filter((m) => m.kind === 'groupFormed');
		expect(fired).toEqual([]);
	});

	it('never announces a single undivided group', () => {
		const fired = run([
			{ atMs: 0, stage: 'shaped', groupCount: 1 },
			{ atMs: 20_000, stage: 'shaped', groupCount: 1 },
			{ atMs: 40_000, stage: 'shaped', groupCount: 1 }
		]).filter((m) => m.kind === 'groupFormed');
		expect(fired).toEqual([]);
	});

	it('stays quiet about groups before the shaped stage', () => {
		const fired = run([
			{ atMs: 0, stage: 'warming', groupCount: 2 },
			{ atMs: 20_000, stage: 'warming', groupCount: 2 },
			{ atMs: 40_000, stage: 'warming', groupCount: 2 }
		]).filter((m) => m.kind === 'groupFormed');
		expect(fired).toEqual([]);
	});

	it('announces a genuine later rise after a settled fall', () => {
		const fired = run([
			{ atMs: 0, stage: 'shaped', groupCount: 2 },
			{ atMs: 20_000, stage: 'shaped', groupCount: 2 },
			{ atMs: 40_000, stage: 'shaped', groupCount: 2 },
			{ atMs: 60_000, stage: 'shaped', groupCount: 3 },
			{ atMs: 80_000, stage: 'shaped', groupCount: 3 },
			{ atMs: 100_000, stage: 'shaped', groupCount: 3 }
		]).filter((m) => m.kind === 'groupFormed');
		expect(fired).toEqual([
			{ kind: 'groupFormed', groupCount: 2 },
			{ kind: 'groupFormed', groupCount: 3 }
		]);
	});
});

describe('defaults', () => {
	it('merges joins faster than it allows repeat moments', () => {
		expect(DEFAULT_MOMENT_CONFIG.mergeWindowMs).toBeLessThan(DEFAULT_MOMENT_CONFIG.minGapMs);
	});
});
