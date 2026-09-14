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
	 * Whether `state.votesByTid` is populated per participant. The scripted scenario
	 * carries every vote; live `report_data` carries only per-group aggregates, so a
	 * live display cannot colour individual dots by a vote (CONTEXT.md,
	 * "Cross-highlight") and falls back to group bars until an endpoint exists.
	 */
	readonly perParticipantVotes: boolean;
	destroy(): void;
}
