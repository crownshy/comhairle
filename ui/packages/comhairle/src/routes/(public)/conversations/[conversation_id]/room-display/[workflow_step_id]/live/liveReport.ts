/** Converts a Polis `report_data` payload into Room display state. */

import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import { apportionVotes } from './apportionedVotes';
import { totalVotes } from '$lib/tools/polis/report';
import { DEFAULT_THRESHOLDS, scoredCount } from '../revealStage';
import type { DisplayState, MapNode, RevealStage } from '../types';

/** Keeps the outermost dot slightly inside the edge of the plot. */
const PLOT_FILL = 0.9;

/**
 * Participants as map nodes, scaled to fill the map, since Polis position units vary
 * per conversation. Participants without a position yet are left off.
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
		// The report has no publish time, but Polis assigns tids in creation order.
		published: [...report.comments].sort((a, b) => b.tid - a.tid),
		votesByTid: apportionVotes(report),
		totalVotes: report.comments.reduce((sum, c) => sum + totalVotes(c), 0)
	};
}

/**
 * The reveal stage this report supports. Needs at least two groups for `shaped`,
 * because Polis reports one group before it has found a split.
 */
export function stageFromReport(report: PolisReportData): RevealStage {
	const state = displayStateFromReport(report);
	if (state.totalVotes === 0) return 'empty';
	if (report.groups.length < 2) return 'warming';
	return scoredCount(state) >= DEFAULT_THRESHOLDS.richScoredStatements ? 'rich' : 'shaped';
}
