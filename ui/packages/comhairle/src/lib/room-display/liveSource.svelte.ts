/**
 * A Room display source fed by a real Polis step.
 *
 * Polls `PolisGetReportData` on an interval; there is no push feed for Polis, and the
 * math behind the report is itself recomputed on a schedule, so polling every few
 * seconds loses nothing. A failed poll keeps the last good payload on screen rather
 * than blanking the wall mid-session.
 *
 * The stage ratchets across polls for the reason `revealStage.ts` gives: Polis's group
 * count oscillates with few votes and a wall that re-locks itself reads as broken.
 */

import type { createApiClient } from '@crownshy/api-client/client';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { displayStateFromReport, stageFromReport } from './liveReport';
import { ratchet } from './revealStage';
import type { RoomDisplaySource } from './source';
import type { RevealStage } from './types';

export interface LiveSourceOptions {
	api: ReturnType<typeof createApiClient>;
	workflowStepId: string;
	intervalMs?: number;
}

export interface LiveRoomDisplaySource extends RoomDisplaySource {
	/** Wall-clock time (epoch ms) of the last successful poll, or null before the first. */
	readonly updatedAtMs: number | null;
	/** Whether the most recent poll failed. The wall keeps showing the last good data. */
	readonly stale: boolean;
	refresh(): Promise<void>;
}

const EMPTY: PolisReportData = { comments: [], groups: [], participants: [] };

/** Timers only exist in the browser; on the server the source renders its empty frame. */
const canPoll = typeof window !== 'undefined';

export function createLiveRoomDisplaySource(options: LiveSourceOptions): LiveRoomDisplaySource {
	const intervalMs = options.intervalMs ?? 8_000;

	let report = $state<PolisReportData>(EMPTY);
	let reached = $state<RevealStage>('empty');
	let updatedAtMs = $state<number | null>(null);
	let stale = $state(false);

	const state = $derived(displayStateFromReport(report, updatedAtMs ?? 0));

	async function refresh() {
		const result = await tryCatchAsync(() =>
			options.api.PolisGetReportData({
				queries: { workflow_step_id: options.workflowStepId }
			})
		);
		if (result.err !== null) {
			stale = true;
			return;
		}
		// WikiPollReport is PolisReportData minus the client-only theme overlay.
		report = result.ok;
		reached = ratchet(reached, stageFromReport(result.ok));
		updatedAtMs = Date.now();
		stale = false;
	}

	let timer: ReturnType<typeof setInterval> | null = null;
	if (canPoll) {
		void refresh();
		timer = setInterval(() => void refresh(), intervalMs);
	}

	return {
		get state() {
			return state;
		},
		get stage() {
			return reached;
		},
		get groups() {
			return report.groups;
		},
		perParticipantVotes: false,
		get updatedAtMs() {
			return updatedAtMs;
		},
		get stale() {
			return stale;
		},
		refresh,
		destroy() {
			if (timer !== null) clearInterval(timer);
			timer = null;
		}
	};
}
