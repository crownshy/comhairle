/**
 * What a Room display renders from, whichever way the data arrives.
 *
 * Two things implement this: the scripted driver (`driver.svelte.ts`) for the demo,
 * and the live source (`liveSource.svelte.ts`) that polls a real Polis step. The
 * display components take a source and nothing else, so the same wall works for a
 * rehearsal and for a room.
 */

import type { ReportGroup } from '$lib/tools/polis/reportTypes';
import type { DisplayState, RevealStage } from './types';

export interface RoomDisplaySource {
	readonly state: DisplayState;
	readonly stage: RevealStage;
	/** Opinion groups Polis has produced, in report shape. Empty before clustering. */
	readonly groups: ReportGroup[];
	/**
	 * Where `state.votesByTid` came from, which is the one thing the two sources do
	 * not agree on.
	 *
	 * `per-participant`: every entry is a real vote by that participant. Only the
	 * scripted scenario has this; no live endpoint returns the matrix.
	 *
	 * `apportioned`: each group's real counts dealt across its dots
	 * (`apportionedVotes.ts`). Proportions are exact, the dot-to-person mapping is
	 * not, so the display may colour the map by a statement but must not claim a dot
	 * shows how that person voted, and anything counting individual voters is off.
	 * See CONTEXT.md, "Cross-highlight".
	 */
	readonly voteMatrix: 'per-participant' | 'apportioned';
	destroy(): void;
}
