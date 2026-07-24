import type {
	GroupVotePercent,
	MemberVotePercent,
	PolisReportData,
	ReportComment,
	ReportGroup
} from './reportTypes';

/** A-Z label for a group_id (0 -> "A"). */
export function groupLabel(groupId: number): string {
	return String.fromCharCode(65 + groupId);
}

/**
 * Per-group vote percentages for a single comment.
 *
 * Percentages are taken over the votes cast on THIS statement, not the
 * group's total membership. With `excludePasses`, the denominator drops
 * passes too - agree% becomes agrees / (agrees + disagrees).
 */
export function computeGroupVotePercents(
	comment: ReportComment,
	groups: ReportGroup[],
	{ excludePasses = false }: { excludePasses?: boolean } = {}
): GroupVotePercent[] {
	return comment.group_votes.map((gv) => {
		const group = groups.find((g) => g.group_id === gv.group_id);
		const totalMembers = group ? group.total_members : gv.agrees + gv.disagrees + gv.passes;
		const totalVoted = gv.agrees + gv.disagrees + gv.passes;
		const denominator = excludePasses ? gv.agrees + gv.disagrees : totalVoted;
		// Share of `denominator` a given vote count represents, as a 0-100 percentage.
		const percentOf = (voteCount: number) =>
			denominator > 0 ? (voteCount / denominator) * 100 : 0;
		return {
			group_id: gv.group_id,
			label: groupLabel(gv.group_id),
			totalMembers,
			totalVoted,
			agreed: percentOf(gv.agrees),
			disagreed: percentOf(gv.disagrees),
			passed: excludePasses ? 0 : percentOf(gv.passes)
		};
	});
}

/**
 * Vote bars for the area-of-consensus design: OVERALL plus one per opinion
 * group, with shares taken over MEMBERSHIP (not just voters) so the not-voted
 * remainder shows. Iterates `groups`, so every group gets a bar (all-not-voted
 * if it cast nothing on this statement) and N is variable, not a fixed pair.
 */
export function computeMemberVoteBars(
	comment: ReportComment,
	groups: ReportGroup[]
): { overall: MemberVotePercent; groups: MemberVotePercent[] } {
	const bar = (
		label: string,
		denominator: number,
		agrees: number,
		disagrees: number,
		passes: number
	): MemberVotePercent => {
		const share = (n: number) => (denominator > 0 ? (n / denominator) * 100 : 0);
		const notVoted = Math.max(0, denominator - (agrees + disagrees + passes));
		return {
			label,
			agreed: share(agrees),
			disagreed: share(disagrees),
			passed: share(passes),
			notVoted: share(notVoted)
		};
	};

	const totalParticipants = groups.reduce((sum, g) => sum + g.total_members, 0);
	const overall = bar(
		'OVERALL',
		totalParticipants,
		comment.overall_votes.agrees,
		comment.overall_votes.disagrees,
		comment.overall_votes.passes
	);

	const groupBars = groups.map((g) => {
		const gv = comment.group_votes.find((v) => v.group_id === g.group_id);
		return bar(
			`GROUP ${groupLabel(g.group_id)}`,
			g.total_members,
			gv?.agrees ?? 0,
			gv?.disagrees ?? 0,
			gv?.passes ?? 0
		);
	});

	return { overall, groups: groupBars };
}

/** Top-line stats across the conversation. */
export function getEngagementStats(data: PolisReportData) {
	const totalParticipants = data.groups.reduce((s, g) => s + g.total_members, 0);
	const totalGroups = data.groups.length;
	const totalStatements = data.comments.length;
	const totalVotesCast = data.comments.reduce(
		(s, c) => s + c.overall_votes.agrees + c.overall_votes.disagrees + c.overall_votes.passes,
		0
	);
	return { totalParticipants, totalGroups, totalStatements, totalVotes: totalVotesCast };
}

/** Vote total for a single comment (a + d + p). */
export function totalVotes(c: ReportComment): number {
	return c.overall_votes.agrees + c.overall_votes.disagrees + c.overall_votes.passes;
}

/**
 * Areas of consensus: every statement ranked by Polis's `group_informed_consensus`
 * (highest first). That score is the product of each opinion group's smoothed
 * agree% ((agrees + 1) / (total + 2)), so a statement only scores high when EVERY
 * group agrees - one dissenting group tanks the product. It's agree-oriented:
 * it surfaces "all groups agree", not "all groups disagree". Read straight off
 * the report data (Polis computes it); we do not recompute it.
 */
export function getConsensusStatements(data: PolisReportData): ReportComment[] {
	return [...data.comments].sort(
		(a, b) => (b.group_informed_consensus ?? 0) - (a.group_informed_consensus ?? 0)
	);
}

/**
 * Areas of difference: every statement ranked by Polis's `divisiveness` (highest
 * first) - higher means the opinion groups split harder on it. Read straight off
 * the report data; we do not recompute it.
 */
export function getDifferenceStatements(data: PolisReportData): ReportComment[] {
	return [...data.comments].sort((a, b) => (b.divisiveness ?? 0) - (a.divisiveness ?? 0));
}
