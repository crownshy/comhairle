/**
 * Decides when a moment (a brief full-screen announcement) should fire: joins are
 * merged, moments are rate limited, and a new group count must hold steady first.
 * See CONTEXT.md, "Accent / Moment".
 */

import type { RevealStage } from '../types';

export type Moment =
	| { kind: 'peopleJoined'; count: number }
	| { kind: 'stageUnlocked'; stage: RevealStage }
	| { kind: 'groupFormed'; groupCount: number };

export interface MomentConfig {
	/** Minimum gap between two moments taking the screen. */
	minGapMs: number;
	/** How long joins collect before firing as one merged moment. */
	mergeWindowMs: number;
	/** Consecutive observations a new group count must hold before it is announced. */
	stabilitySamples: number;
}

/** Zero or one group means the room has not split, so it is not announced. */
export const MIN_ANNOUNCEABLE_GROUPS = 2;

export const DEFAULT_MOMENT_CONFIG: MomentConfig = {
	minGapMs: 12_000,
	mergeWindowMs: 6_000,
	stabilitySamples: 3
};

export interface MomentState {
	lastFiredAtMs: number | null;
	pendingJoins: { count: number; firstAtMs: number } | null;
	announcedStages: RevealStage[];
	/** A group count seen but not yet held long enough to announce. */
	groupCandidate: { count: number; samples: number } | null;
	announcedGroupCount: number | null;
}

export function initialMomentState(): MomentState {
	return {
		lastFiredAtMs: null,
		pendingJoins: null,
		announcedStages: [],
		groupCandidate: null,
		announcedGroupCount: null
	};
}

export interface MomentObservation {
	atMs: number;
	/** Participants who joined since the previous observation. */
	joined: number;
	stage: RevealStage;
	/** Opinion groups Polis currently reports. */
	groupCount: number;
}

/**
 * Applies one observation and returns the next state plus the moment to show, if any.
 * When several are due, a stage unlock wins, then a new group, then joins.
 */
export function advance(
	state: MomentState,
	observation: MomentObservation,
	config: MomentConfig = DEFAULT_MOMENT_CONFIG
): { state: MomentState; moment: Moment | null } {
	let next: MomentState = { ...state };

	if (observation.joined > 0) {
		next.pendingJoins = next.pendingJoins
			? {
					count: next.pendingJoins.count + observation.joined,
					firstAtMs: next.pendingJoins.firstAtMs
				}
			: { count: observation.joined, firstAtMs: observation.atMs };
	}

	// Count stable samples on every observation, even when nothing can fire yet.
	if (observation.groupCount !== next.announcedGroupCount) {
		next.groupCandidate =
			next.groupCandidate && next.groupCandidate.count === observation.groupCount
				? { count: observation.groupCount, samples: next.groupCandidate.samples + 1 }
				: { count: observation.groupCount, samples: 1 };
	} else {
		next.groupCandidate = null;
	}

	const canFire =
		next.lastFiredAtMs === null || observation.atMs - next.lastFiredAtMs >= config.minGapMs;

	const stageUnlocked =
		observation.stage !== 'empty' && !next.announcedStages.includes(observation.stage);

	const groupSettled =
		next.groupCandidate !== null && next.groupCandidate.samples >= config.stabilitySamples;
	// Only a rise is announced. A fall is usually Polis's clustering wobbling, so it
	// updates the baseline quietly.
	const clusteringVisible = observation.stage === 'shaped' || observation.stage === 'rich';
	const groupRose =
		groupSettled &&
		clusteringVisible &&
		next.groupCandidate !== null &&
		next.groupCandidate.count >= MIN_ANNOUNCEABLE_GROUPS &&
		(next.announcedGroupCount === null || next.groupCandidate.count > next.announcedGroupCount);

	if (groupSettled && next.groupCandidate !== null && !groupRose) {
		next.announcedGroupCount = next.groupCandidate.count;
		next.groupCandidate = null;
	}

	const joinsReady =
		next.pendingJoins !== null &&
		observation.atMs - next.pendingJoins.firstAtMs >= config.mergeWindowMs;

	if (!canFire) return { state: next, moment: null };

	if (stageUnlocked) {
		next = {
			...next,
			announcedStages: [...next.announcedStages, observation.stage],
			lastFiredAtMs: observation.atMs
		};
		return { state: next, moment: { kind: 'stageUnlocked', stage: observation.stage } };
	}

	if (groupRose && next.groupCandidate !== null) {
		const groupCount = next.groupCandidate.count;
		next = {
			...next,
			announcedGroupCount: groupCount,
			groupCandidate: null,
			lastFiredAtMs: observation.atMs
		};
		return { state: next, moment: { kind: 'groupFormed', groupCount } };
	}

	if (joinsReady && next.pendingJoins !== null) {
		const count = next.pendingJoins.count;
		next = { ...next, pendingJoins: null, lastFiredAtMs: observation.atMs };
		return { state: next, moment: { kind: 'peopleJoined', count } };
	}

	return { state: next, moment: null };
}
