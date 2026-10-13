/**
 * Room display source that polls a live Polis step. A failed poll keeps the last good
 * data on screen, and polling pauses while the tab is hidden.
 */

import type { createApiClient } from '@crownshy/api-client/client';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { displayStateFromReport, stageFromReport } from './liveReport';
import { ratchet } from '../revealStage';
import type { RoomDisplaySource } from '../source';
import type { RevealStage } from '../types';

export interface LiveSourceOptions {
	api: ReturnType<typeof createApiClient>;
	workflowStepId: string;
	intervalMs?: number;
}

export interface LiveRoomDisplaySource extends RoomDisplaySource {
	/** Epoch ms of the last successful poll, or null before the first. */
	readonly updatedAtMs: number | null;
	/** Whether the most recent poll failed. The wall keeps showing the last good data. */
	readonly stale: boolean;
	refresh(): Promise<void>;
}

const EMPTY: PolisReportData = { comments: [], groups: [], participants: [] };

/** On the server the source stays empty and never polls. */
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
		// The API's WikiPollReport type is PolisReportData without the client-only theme fields.
		report = result.ok;
		reached = ratchet(reached, stageFromReport(result.ok));
		updatedAtMs = Date.now();
		stale = false;
	}

	let timer: ReturnType<typeof setInterval> | null = null;

	function start() {
		if (timer !== null) return;
		void refresh();
		timer = setInterval(() => void refresh(), intervalMs);
	}

	function stop() {
		if (timer !== null) clearInterval(timer);
		timer = null;
	}

	function onVisibilityChange() {
		if (document.hidden) stop();
		else start();
	}

	if (canPoll) {
		if (!document.hidden) start();
		document.addEventListener('visibilitychange', onVisibilityChange);
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
		voteMatrix: 'apportioned',
		get updatedAtMs() {
			return updatedAtMs;
		},
		get stale() {
			return stale;
		},
		refresh,
		destroy() {
			stop();
			if (canPoll) document.removeEventListener('visibilitychange', onVisibilityChange);
		}
	};
}
