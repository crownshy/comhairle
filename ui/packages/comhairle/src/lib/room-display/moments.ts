/**
 * The Room display's moments layer.
 *
 * Accents (a vote landing, the ticker scrolling) are the components' own business and
 * never come through here. This module owns only **moments**: the rare takeovers that
 * stop the screen. See CONTEXT.md, "Accent / Moment".
 *
 * Three rules, each of them there because the naive version fails in a specific way in
 * a real room:
 *
 *  - **Burst merge.** All 28 people scan the QR code in the first two minutes. Without
 *    merging that is 28 back-to-back takeovers, so joins collect for a window and fire
 *    once as "6 people joined".
 *  - **Rate limit.** Even merged, moments must not stack up during someone's sentence.
 *    At most one every `minGapMs`; the rest wait or are superseded.
 *  - **Stability gate.** Polis re-clusters continuously and its group count genuinely
 *    oscillates with few votes. Announcing every flip cries wolf three times in ten
 *    minutes and the room stops believing the screen, so a change must hold across
 *    several observations before it is announced.
 *
 * Modelled as a pure reducer over observations so all three rules are testable without
 * a clock.
 */

import type { RevealStage } from './types';

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

/**
 * Fewest opinion groups worth announcing. Polis reports zero before it has clustered
 * and one when the room is undivided; neither is "the room has split", so neither
 * takes the screen.
 */
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
 * Advances the reducer by one observation, returning the next state and the moment to
 * take the screen, if any.
 *
 * Priority when several are eligible is stage unlock, then a new group, then joins.
 * A stage unlock is the highest-value moment the display has and the rarest, so it
 * never waits behind a join burst.
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

	// Track group-count stability every observation, whether or not anything fires, so
	// the gate measures how long a count has held rather than how long since a moment.
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
	// Only a rise to a meaningful count is a moment. A fall is Polis's math wobbling
	// rather than news, and a count below MIN_ANNOUNCEABLE_GROUPS is not a split at
	// all; both update the baseline quietly and never take the screen. Group moments
	// also wait for `shaped`, since before that Polis has not really clustered.
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
