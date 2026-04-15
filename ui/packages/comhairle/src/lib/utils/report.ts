import type {
	GroupVotePercent,
	PolisReportData,
	ReportComment,
	ReportGroup
} from '$lib/types/report';

export function computeOverallVotePercents(
	comment: ReportComment,
	totalParticipants: number
): GroupVotePercent {
	const { agrees, disagrees, passes } = comment.overall_votes;
	const totalVoted = agrees + disagrees + passes;
	const denom = Math.max(totalParticipants, totalVoted);

	if (denom === 0) {
		return {
			group_id: -1,
			label: 'OVERALL',
			agreed: 0,
			disagreed: 0,
			passed: 0,
			notVoted: 100
		};
	}

	return {
		group_id: -1,
		label: 'OVERALL',
		agreed: (agrees / denom) * 100,
		disagreed: (disagrees / denom) * 100,
		passed: (passes / denom) * 100,
		notVoted: Math.max(0, ((denom - totalVoted) / denom) * 100)
	};
}

export function computeGroupVotePercents(
	comment: ReportComment,
	groups: ReportGroup[]
): GroupVotePercent[] {
	return comment.group_votes.map((gv) => {
		const group = groups.find((g) => g.group_id === gv.group_id);
		const totalMembers = group ? group.total_members : gv.agrees + gv.disagrees + gv.passes;
		const totalVoted = gv.agrees + gv.disagrees + gv.passes;
		const denom = Math.max(totalMembers, totalVoted);

		if (denom === 0) {
			return {
				group_id: gv.group_id,
				label: `Group ${String.fromCharCode(65 + gv.group_id)}`,
				agreed: 0,
				disagreed: 0,
				passed: 0,
				notVoted: 100
			};
		}

		return {
			group_id: gv.group_id,
			label: `Group ${String.fromCharCode(65 + gv.group_id)}`,
			agreed: (gv.agrees / denom) * 100,
			disagreed: (gv.disagrees / denom) * 100,
			passed: (gv.passes / denom) * 100,
			notVoted: Math.max(0, ((denom - totalVoted) / denom) * 100)
		};
	});
}

export function getEngagementStats(data: PolisReportData) {
	const totalParticipants = data.groups.reduce((sum, g) => sum + g.total_members, 0);
	const totalGroups = data.groups.length;
	const totalStatements = data.comments.length;
	const totalVotes = data.comments.reduce((sum, c) => {
		return sum + c.overall_votes.agrees + c.overall_votes.disagrees + c.overall_votes.passes;
	}, 0);

	return { totalParticipants, totalGroups, totalStatements, totalVotes };
}

export function getConsensusStatements(data: PolisReportData, count = 10): ReportComment[] {
	const minVotes = 10;
	return [...data.comments]
		.filter(
			(c) =>
				c.overall_votes.agrees + c.overall_votes.disagrees + c.overall_votes.passes >=
				minVotes
		)
		.sort((a, b) => b.group_informed_consensus - a.group_informed_consensus)
		.slice(0, count);
}

export function getDivisiveStatements(data: PolisReportData, count = 10): ReportComment[] {
	const minVotes = 10;
	return [...data.comments]
		.filter(
			(c) =>
				c.overall_votes.agrees + c.overall_votes.disagrees + c.overall_votes.passes >=
				minVotes
		)
		.sort((a, b) => b.divisiveness - a.divisiveness)
		.slice(0, count);
}

export function getSignificantComments(data: PolisReportData, minVotes = 10): ReportComment[] {
	return data.comments.filter(
		(c) =>
			c.overall_votes.agrees + c.overall_votes.disagrees + c.overall_votes.passes >= minVotes
	);
}
