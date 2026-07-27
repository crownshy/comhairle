/**
 * Static sample data for the Polis report stories (ConsensusContinuum, etc.).
 *
 * A captured-looking 5-statement report with two opinion groups, shaped exactly
 * like `PolisReportData` from the API client. Statements are spread across the
 * consensus continuum via `divisiveness` (tid 0 broadly agreed → tid 4 most
 * divisive), and each `overall_votes` is the sum of its `group_votes` so the
 * stacked vote bars read consistently. Kept dumb: literal data, no computation.
 */
import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';

const GROUP_A_MEMBERS = 60;
const GROUP_B_MEMBERS = 40;

type GroupVote = { group_id: number; agrees: number; disagrees: number; passes: number };

/** Build a comment whose `overall_votes` is the sum of its per-group votes. */
function comment(
	tid: number,
	text: string,
	divisiveness: number,
	groupVotes: GroupVote[]
): ReportComment {
	const sum = (key: 'agrees' | 'disagrees' | 'passes') =>
		groupVotes.reduce((n, gv) => n + gv[key], 0);
	return {
		tid,
		text,
		divisiveness,
		is_seed: false,
		group_votes: groupVotes,
		overall_votes: {
			agrees: sum('agrees'),
			disagrees: sum('disagrees'),
			passes: sum('passes')
		}
	};
}

const comments: ReportComment[] = [
	comment(0, 'Public parks should stay free to enter.', 0.12, [
		{ group_id: 0, agrees: 52, disagrees: 3, passes: 2 },
		{ group_id: 1, agrees: 35, disagrees: 2, passes: 1 }
	]),
	comment(1, 'The city should expand the cycle lane network.', 0.34, [
		{ group_id: 0, agrees: 40, disagrees: 10, passes: 5 },
		{ group_id: 1, agrees: 25, disagrees: 9, passes: 3 }
	]),
	comment(2, 'Introduce a congestion charge for the centre on weekdays.', 0.61, [
		{ group_id: 0, agrees: 33, disagrees: 20, passes: 4 },
		{ group_id: 1, agrees: 14, disagrees: 22, passes: 2 }
	]),
	comment(3, 'Replace on-street parking with wider pavements.', 0.93, [
		{ group_id: 0, agrees: 44, disagrees: 9, passes: 3 },
		{ group_id: 1, agrees: 7, disagrees: 30, passes: 2 }
	]),
	comment(4, 'Ban private cars from the old town entirely.', 1.42, [
		{ group_id: 0, agrees: 50, disagrees: 5, passes: 2 },
		{ group_id: 1, agrees: 4, disagrees: 34, passes: 1 }
	])
];

const groups: ReportGroup[] = [
	{
		group_id: 0,
		total_members: GROUP_A_MEMBERS,
		members: Array.from({ length: GROUP_A_MEMBERS }, (_, i) => i),
		representative_comments: [
			{ tid: 3, text: 'Replace on-street parking with wider pavements.' }
		]
	},
	{
		group_id: 1,
		total_members: GROUP_B_MEMBERS,
		members: Array.from({ length: GROUP_B_MEMBERS }, (_, i) => i + GROUP_A_MEMBERS),
		representative_comments: [{ tid: 4, text: 'Ban private cars from the old town entirely.' }]
	}
];

export const sampleReportData: { comments: ReportComment[]; groups: ReportGroup[] } = {
	comments,
	groups
};
