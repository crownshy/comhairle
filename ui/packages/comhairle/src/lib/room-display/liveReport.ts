/**
 * Maps a real `report_data` payload onto the shapes the Room display renders, kept
 * pure so it can be unit-tested away from the poller.
 *
 * What the report gives us and what it does not:
 *
 *  - Participants carry a PCA position and a group id, which is exactly a map node.
 *  - Comments carry aggregate and per-group vote counts plus Polis's scores, which is
 *    everything the strip and the vote bars need.
 *  - There is no per-participant vote matrix, so `votesByTid` is empty and the map
 *    cannot be coloured by one statement. The source flags this and the display
 *    degrades to group bars.
 *  - There is no publish timestamp. Polis assigns `tid` in creation order, so newest
 *    first is highest `tid` first.
 */

import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import { totalVotes } from '$lib/tools/polis/report';
import { DEFAULT_THRESHOLDS, scoredCount } from './revealStage';
import type { DisplayState, MapNode, RevealStage } from './types';

/**
 * Fraction of the map's unit extent the outermost participant sits at. Under 1 so a
 * dot on the edge of the PCA cloud is not on the edge of the plot.
 */
const PLOT_FILL = 0.9;

/**
 * Participants as map nodes, scaled so the cloud fills the map.
 *
 * Polis PCA coordinates are in arbitrary units that vary per conversation, so they
 * are normalised by the largest magnitude present rather than by a fixed range. A
 * participant without a position has not been placed by the math yet and is left off
 * the map rather than dropped on the origin.
 */
export function nodesFromReport(report: PolisReportData): MapNode[] {
	const placed = report.participants.filter((p) => p.pca_position != null);
	let maxAbs = 0;
	for (const p of placed) {
		maxAbs = Math.max(maxAbs, Math.abs(p.pca_position!.x), Math.abs(p.pca_position!.y));
	}
	const scale = maxAbs > 0 ? PLOT_FILL / maxAbs : 1;
	return placed.map((p) => ({
		id: p.pid,
		x: p.pca_position!.x * scale,
		y: p.pca_position!.y * scale,
		groupId: p.group_id ?? null,
		memberCount: 1
	}));
}

export function displayStateFromReport(report: PolisReportData, atMs = 0): DisplayState {
	return {
		atMs,
		nodes: nodesFromReport(report),
		published: [...report.comments].sort((a, b) => b.tid - a.tid),
		votesByTid: new Map(),
		totalVotes: report.comments.reduce((sum, c) => sum + totalVotes(c), 0)
	};
}

/**
 * The stage the report supports on its own evidence. Two or more groups means Polis
 * has genuinely clustered, which beats any voter-count heuristic (see
 * `revealStage.ts`). A single group is what Polis reports before it has found a
 * split, so it does not count.
 */
export function stageFromReport(report: PolisReportData): RevealStage {
	const state = displayStateFromReport(report);
	if (state.totalVotes === 0) return 'empty';
	if (report.groups.length < 2) return 'warming';
	return scoredCount(state) >= DEFAULT_THRESHOLDS.richScoredStatements ? 'rich' : 'shaped';
}
