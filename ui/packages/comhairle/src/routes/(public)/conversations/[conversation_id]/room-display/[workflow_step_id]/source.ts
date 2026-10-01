import type { ReportGroup } from '$lib/tools/polis/reportTypes';
import type { DisplayState, RevealStage } from './types';

/** What a Room display renders from: the scripted demo driver or the live Polis poller. */
export interface RoomDisplaySource {
	readonly state: DisplayState;
	readonly stage: RevealStage;
	/** Opinion groups Polis has produced, in report shape. Empty before clustering. */
	readonly groups: ReportGroup[];
	/**
	 * `per-participant`: every entry is a real vote (demo only). `apportioned`: each group's
	 * real counts dealt across its dots, so no dot shows one person's vote. See ADR-0040.
	 */
	readonly voteMatrix: 'per-participant' | 'apportioned';
	destroy(): void;
}
