/**
 * Decides how much of the Room display is unlocked. A stage once reached is never given
 * back. The thresholds approximate when Polis will cluster. See CONTEXT.md, "Reveal stage".
 */

import type { DisplayState, RevealStage } from './types';

/** Weakest to strongest. The index is used to compare stages. */
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
 * Tuned for a room of about 28. `shapedVoters` is under half the room so the map unlocks
 * early on partial data rather than staying hidden for most of the first hour.
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

/** Published statements Polis has given a divisiveness score. */
export function scoredCount(state: DisplayState): number {
	return state.published.filter(
		(c) => typeof c.divisiveness === 'number' && Number.isFinite(c.divisiveness)
	).length;
}

/** The stage the current data supports, ignoring history. Most callers want `ratchet` too. */
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

/** The stronger of two stages, so a stage once reached is never lost. */
export function ratchet(reached: RevealStage, computed: RevealStage): RevealStage {
	return STAGE_ORDER.indexOf(computed) > STAGE_ORDER.indexOf(reached) ? computed : reached;
}

/**
 * What the room is waiting for, shown as a countdown. `remaining` counts people or
 * statements, not a percentage, because "4 more voters" is something a room can act on.
 */
export interface NextUnlock {
	stage: RevealStage;
	metric: 'voters' | 'votes' | 'statements';
	remaining: number;
}

/** `null` once `rich` is reached. */
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
		// `shaped` needs enough voters and enough votes. Report whichever is further away,
		// so the countdown never reaches zero without the stage changing.
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
