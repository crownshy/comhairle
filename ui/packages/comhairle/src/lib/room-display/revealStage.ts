/**
 * The Room display's reveal-stage machine.
 *
 * Polis's clustering produces no opinion groups until enough participants have voted
 * on enough statements, so for the first stretch of a session the honest state of the
 * data is "nothing yet". The display has stages whether we design them or not; these
 * make them explicit and turn the wait into a countdown the room can see.
 *
 * Two rules carry most of the value:
 *
 *  - **Stages ratchet.** Polis recomputes continuously and its group count genuinely
 *    oscillates (two groups, then three, then two again) with few votes. A display
 *    that unlocked and then re-locked itself would read as broken, so a stage once
 *    reached is never given back.
 *  - **Warming is not a spinner.** It is the recruitment screen. The caller renders
 *    the question and the QR code there, which is the most useful thing the screen
 *    can do at minute five anyway.
 *
 * The thresholds below are the *display's* approximation, used to drive a scripted
 * scenario. The live display should prefer the real signal where it exists: once
 * `report_data` returns two or more groups, Polis has genuinely clustered and the
 * stage is at least `shaped` regardless of what these counts say.
 */

import type { DisplayState, RevealStage } from './types';

/** Weakest to strongest. Index doubles as the ratchet's ordinal. */
export const STAGE_ORDER: readonly RevealStage[] = ['empty', 'warming', 'shaped', 'rich'];

export interface RevealThresholds {
	/** Voters who have cast at least one vote, to leave `empty`. */
	warmingVoters: number;
	/** Voters needed before clustering is plausible. */
	shapedVoters: number;
	/** Mean votes per voter needed alongside `shapedVoters`. */
	shapedVotesPerVoter: number;
	/** Published statements carrying a `divisiveness` score, to reach `rich`. */
	richScoredStatements: number;
}

/**
 * Defaults tuned for a room of roughly 28. `shapedVoters` sits below half the room
 * because waiting for a majority means the map stays locked through most of the first
 * hour, and a display that unlocks late is worse than one that unlocks on partial
 * data and says so.
 */
export const DEFAULT_THRESHOLDS: RevealThresholds = {
	warmingVoters: 1,
	shapedVoters: 8,
	shapedVotesPerVoter: 4,
	richScoredStatements: 12
};

/** Number of nodes that have cast at least one vote. */
export function voterCount(state: DisplayState): number {
	const voters = new Set<number>();
	for (const byNode of state.votesByTid.values()) {
		for (const nodeId of byNode.keys()) voters.add(nodeId);
	}
	return voters.size;
}

/** Published statements Polis has scored, so the continuum can place them. */
export function scoredCount(state: DisplayState): number {
	return state.published.filter(
		(c) => typeof c.divisiveness === 'number' && Number.isFinite(c.divisiveness)
	).length;
}

/**
 * The stage the current data supports, ignoring history. Callers almost always want
 * `ratchet` over this rather than this alone.
 */
export function computeStage(
	state: DisplayState,
	thresholds: RevealThresholds = DEFAULT_THRESHOLDS
): RevealStage {
	const voters = voterCount(state);
	if (voters < thresholds.warmingVoters || state.totalVotes === 0) return 'empty';

	const votesPerVoter = voters > 0 ? state.totalVotes / voters : 0;
	const clustered =
		voters >= thresholds.shapedVoters && votesPerVoter >= thresholds.shapedVotesPerVoter;
	if (!clustered) return 'warming';

	return scoredCount(state) >= thresholds.richScoredStatements ? 'rich' : 'shaped';
}

/** The stronger of two stages. Never gives a reached stage back. */
export function ratchet(reached: RevealStage, computed: RevealStage): RevealStage {
	return STAGE_ORDER.indexOf(computed) > STAGE_ORDER.indexOf(reached) ? computed : reached;
}

/**
 * What the room is waiting for, for the countdown. `null` once `rich` is reached.
 *
 * `remaining` is deliberately a count of *people* or *statements* rather than a
 * percentage: "4 more voters" is a thing the room can act on, and a progress bar is
 * not.
 */
export interface NextUnlock {
	stage: RevealStage;
	metric: 'voters' | 'votes' | 'statements';
	remaining: number;
}

export function nextUnlock(
	state: DisplayState,
	reached: RevealStage,
	thresholds: RevealThresholds = DEFAULT_THRESHOLDS
): NextUnlock | null {
	const voters = voterCount(state);

	if (reached === 'empty') {
		return {
			stage: 'warming',
			metric: 'voters',
			remaining: Math.max(1, thresholds.warmingVoters - voters)
		};
	}

	if (reached === 'warming') {
		// Two conditions gate `shaped`; report whichever is further away, so the
		// countdown never hits zero without the stage changing.
		const votersShort = Math.max(0, thresholds.shapedVoters - voters);
		const votesNeeded = thresholds.shapedVoters * thresholds.shapedVotesPerVoter;
		const votesShort = Math.max(0, votesNeeded - state.totalVotes);
		if (votersShort > 0 && votersShort * thresholds.shapedVotesPerVoter >= votesShort) {
			return { stage: 'shaped', metric: 'voters', remaining: votersShort };
		}
		return { stage: 'shaped', metric: 'votes', remaining: Math.max(1, votesShort) };
	}

	if (reached === 'shaped') {
		return {
			stage: 'rich',
			metric: 'statements',
			remaining: Math.max(1, thresholds.richScoredStatements - scoredCount(state))
		};
	}

	return null;
}

/** Countdown copy for the room, e.g. "1 more voter until the map appears". */
export function describeUnlock(unlock: NextUnlock): string {
	const nouns: Record<NextUnlock['metric'], [string, string]> = {
		voters: ['voter', 'voters'],
		votes: ['vote', 'votes'],
		statements: ['statement', 'statements']
	};
	const [singular, plural] = nouns[unlock.metric];
	return `${unlock.remaining} more ${unlock.remaining === 1 ? singular : plural}`;
}
