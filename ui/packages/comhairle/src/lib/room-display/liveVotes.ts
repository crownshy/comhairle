/**
 * Vote bars computed from the live vote stream rather than the report payload.
 *
 * The report's `computeMemberVoteBars` reads `overall_votes` and `group_votes` off a
 * comment, which in the prototype are the scenario's *end* state. A wall that shows
 * the end-of-session split ten minutes in would contradict the map beside it, so the
 * Room display folds its bars from `votesByTid` and the nodes present so far.
 *
 * Shares are over members present, not over voters, so the not-voted remainder is
 * real: a bar that is mostly empty means most of that group has not answered yet.
 */

import { computeMemberVoteBars, groupLabel } from '$lib/tools/polis/report';
import type { MemberVotePercent, ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';
import type { RoomDisplaySource } from './source';
import type { DisplayState, MapNode } from './types';

function bar(label: string, nodes: MapNode[], state: DisplayState, tid: number): MemberVotePercent {
	const byNode = state.votesByTid.get(tid);
	let agrees = 0;
	let disagrees = 0;
	let passes = 0;
	let members = 0;
	for (const node of nodes) {
		members += node.memberCount;
		const vote = byNode?.get(node.id);
		if (vote === 'agree') agrees += node.memberCount;
		else if (vote === 'disagree') disagrees += node.memberCount;
		else if (vote === 'pass') passes += node.memberCount;
	}
	const share = (n: number) => (members > 0 ? (n / members) * 100 : 0);
	return {
		label,
		agreed: share(agrees),
		disagreed: share(disagrees),
		passed: share(passes),
		notVoted: share(Math.max(0, members - agrees - disagrees - passes))
	};
}

/** Overall plus one bar per opinion group, for one statement, as of `state`. */
export function liveVoteBars(
	state: DisplayState,
	groups: ReportGroup[],
	tid: number
): { overall: MemberVotePercent; groups: MemberVotePercent[] } {
	return {
		overall: bar('Overall', state.nodes, state, tid),
		groups: groups.map((g) =>
			bar(
				`Group ${groupLabel(g.group_id)}`,
				state.nodes.filter((n) => n.groupId === g.group_id),
				state,
				tid
			)
		)
	};
}

/** Participants present per opinion group, as of `state`. */
export function presentByGroup(state: DisplayState, groups: ReportGroup[]): Map<number, number> {
	const counts = new Map<number, number>(groups.map((g) => [g.group_id, 0]));
	for (const node of state.nodes) {
		if (node.groupId === null) continue;
		counts.set(node.groupId, (counts.get(node.groupId) ?? 0) + node.memberCount);
	}
	return counts;
}

/**
 * The same bars read off the report comment instead of the vote stream, for a source
 * whose matrix is apportioned. The bars are where the exact numbers live, so they
 * come from the payload rather than from dots that round. Shares are over each group's total
 * membership, which is what the Insights tab shows too; only the labels differ, so
 * the wall reads the same whichever source is behind it.
 */
export function reportVoteBars(
	comment: ReportComment,
	groups: ReportGroup[]
): { overall: MemberVotePercent; groups: MemberVotePercent[] } {
	const bars = computeMemberVoteBars(comment, groups);
	return {
		overall: { ...bars.overall, label: 'Overall' },
		groups: bars.groups.map((bar, i) => ({
			...bar,
			label: `Group ${groupLabel(groups[i].group_id)}`
		}))
	};
}

/** Picks the bar computation a source supports. */
export function voteBarsFor(
	source: Pick<RoomDisplaySource, 'state' | 'groups' | 'voteMatrix'>,
	comment: ReportComment
): { overall: MemberVotePercent; groups: MemberVotePercent[] } {
	return source.voteMatrix === 'per-participant'
		? liveVoteBars(source.state, source.groups, comment.tid)
		: reportVoteBars(comment, source.groups);
}
