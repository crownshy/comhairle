import type {
	GroupVotePercent,
	PolisReportData,
	ReportComment,
	ReportGroup,
	ThemeControversy,
	ThemeSummary
} from './reportTypes';

/** A-Z label for a group_id (0 -> "A"). */
export function groupLabel(groupId: number): string {
	return String.fromCharCode(65 + groupId);
}

/**
 * Thresholds for the Insights consensus/difference labels. Defined in the
 * ticket (286), tunable here. Not Polis concepts - our own classification.
 */
export const CONSENSUS_AGREE = 80; // all groups agree% >= this -> consensus (+)
export const CONSENSUS_DISAGREE = 20; // all groups agree% < this -> consensus (-)
export const DIFFERENCE_SPREAD = 30; // max-min agree% > this -> difference

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

/** Fewest votes any single group cast on a comment (a + d + p per group). */
export function minGroupVotes(c: ReportComment): number {
	if (c.group_votes.length === 0) return 0;
	return Math.min(...c.group_votes.map((gv) => gv.agrees + gv.disagrees + gv.passes));
}

/**
 * "Low data quality" threshold - a per-group minimum. See CONTEXT.md. A
 * statement is low quality when ANY group cast fewer than this many votes on
 * it, because every Insights table compares group agree%s and a barely-voted
 * group yields a meaningless percentage.
 */
export const LOW_QUALITY_MIN_GROUP_VOTES = 10;

/** True when any group has fewer than LOW_QUALITY_MIN_GROUP_VOTES on the comment. */
export function isLowQuality(c: ReportComment): boolean {
	return minGroupVotes(c) < LOW_QUALITY_MIN_GROUP_VOTES;
}

/**
 * Single classification driving the left color sliver, so the same statement
 * shows the same stripe in every table. Consensus and difference are mutually
 * exclusive by their own math (a row can't be all-same-side AND spread > 30).
 */
export function classifyStatement(
	c: ReportComment,
	groups: ReportGroup[],
	{ excludePasses = false }: { excludePasses?: boolean } = {}
): 'consensus' | 'difference' | 'neutral' {
	if (consensusDirection(c, groups, { excludePasses }) !== null) return 'consensus';
	const agreed = computeGroupVotePercents(c, groups, { excludePasses }).map((p) => p.agreed);
	const spread = agreed.length ? Math.max(...agreed) - Math.min(...agreed) : 0;
	return spread > DIFFERENCE_SPREAD ? 'difference' : 'neutral';
}

/** Filter helpers shared by the three areas-of-* lists. */
function withMinVotes(comments: ReportComment[], minVotes: number) {
	return comments.filter((c) => totalVotes(c) >= minVotes);
}

/**
 * Consensus direction for a statement, or null if it isn't a consensus.
 *   '+' -> every group agrees (agree% >= CONSENSUS_AGREE)
 *   '-' -> every group disagrees (agree% < CONSENSUS_DISAGREE)
 */
export function consensusDirection(
	comment: ReportComment,
	groups: ReportGroup[],
	{ excludePasses = false }: { excludePasses?: boolean } = {}
): '+' | '-' | null {
	const agreed = computeGroupVotePercents(comment, groups, { excludePasses }).map(
		(p) => p.agreed
	);
	if (agreed.length === 0) return null;
	if (agreed.every((a) => a >= CONSENSUS_AGREE)) return '+';
	if (agreed.every((a) => a < CONSENSUS_DISAGREE)) return '-';
	return null;
}

/**
 * Areas of consensus: ALL groups land on the same side - either all agree
 * (agree% >= 80) or all disagree (agree% < 20). See CONTEXT.md / ticket 286.
 */
export function getConsensusStatements(
	data: PolisReportData,
	{ minVotes = 5, excludePasses = false }: { minVotes?: number; excludePasses?: boolean } = {}
): ReportComment[] {
	return withMinVotes(data.comments, minVotes)
		.filter((c) => consensusDirection(c, data.groups, { excludePasses }) !== null)
		.sort((a, b) => (b.group_informed_consensus ?? 0) - (a.group_informed_consensus ?? 0));
}

/**
 * Areas of difference: the largest agree% gap between any two groups exceeds
 * the threshold (30 by default). With >2 groups this is max(agree%) -
 * min(agree%), i.e. ONE diverging pair is enough. Sorted by spread, desc.
 */
export function getDifferenceStatements(
	data: PolisReportData,
	{
		threshold = DIFFERENCE_SPREAD,
		minVotes = 5,
		excludePasses = false
	}: {
		threshold?: number;
		minVotes?: number;
		excludePasses?: boolean;
	} = {}
): ReportComment[] {
	return withMinVotes(data.comments, minVotes)
		.map((c) => {
			const agreePercents = computeGroupVotePercents(c, data.groups, { excludePasses }).map(
				(p) => p.agreed
			);
			const spread = agreePercents.length
				? Math.max(...agreePercents) - Math.min(...agreePercents)
				: 0;
			return { c, spread };
		})
		.filter(({ spread }) => spread > threshold)
		.sort((a, b) => b.spread - a.spread)
		.map(({ c }) => c);
}

/**
 * Areas of uncertainty: pass% significantly higher than the average pass
 * rate across all comments. Default = pass% >= avg + stdev (or >= 30%).
 */
export function getUncertaintyStatements(
	data: PolisReportData,
	{ minVotes = 5 }: { minVotes?: number } = {}
): ReportComment[] {
	const filtered = withMinVotes(data.comments, minVotes);
	if (filtered.length === 0) return [];

	const passPercents = filtered.map((c) => {
		const t = totalVotes(c);
		return t === 0 ? 0 : (c.overall_votes.passes / t) * 100;
	});
	const avg = passPercents.reduce((s, x) => s + x, 0) / passPercents.length;
	const variance = passPercents.reduce((s, x) => s + (x - avg) ** 2, 0) / passPercents.length;
	const stdev = Math.sqrt(variance);
	const cutoff = Math.max(avg + stdev, 30);

	return filtered
		.map((c, i) => ({ c, passPercent: passPercents[i] }))
		.filter(({ passPercent }) => passPercent >= cutoff)
		.sort((a, b) => b.passPercent - a.passPercent)
		.map(({ c }) => c);
}

/** Aggregate topic counts across all comments. */
export function getThemeCounts(data: PolisReportData): { theme: string; count: number }[] {
	const counts = new Map<string, number>();
	for (const c of data.comments) {
		for (const t of c.topics ?? []) counts.set(t, (counts.get(t) ?? 0) + 1);
	}
	return [...counts.entries()]
		.map(([theme, count]) => ({ theme, count }))
		.sort((a, b) => b.count - a.count);
}

/**
 * Per-statement group-agree spread = max(agree%) - min(agree%) across groups.
 * Returns 0 when the statement has fewer than 2 groups voting.
 */
function statementSpread(comment: ReportComment, groups: ReportGroup[]): number {
	const agreed = computeGroupVotePercents(comment, groups).map((p) => p.agreed);
	if (agreed.length < 2) return 0;
	return Math.max(...agreed) - Math.min(...agreed);
}

/**
 * Controversy classification for a theme. See CONTEXT.md -> "Controversy".
 * Average per-statement group-agree spread, bucketed:
 *   <15 -> low, 15-30 -> moderate, >30 -> high.
 */
export function themeControversy(theme: string, data: PolisReportData): ThemeControversy {
	const tagged = data.comments.filter((c) => c.topics?.includes(theme));
	if (tagged.length === 0) return 'low';
	const spreads = tagged.map((c) => statementSpread(c, data.groups));
	const avg = spreads.reduce((s, x) => s + x, 0) / spreads.length;
	if (avg > 30) return 'high';
	if (avg >= 15) return 'moderate';
	return 'low';
}

/** Theme roll-ups for the Themes card: statement count + controversy bucket. */
export function getThemeSummaries(data: PolisReportData): ThemeSummary[] {
	return getThemeCounts(data).map(({ theme, count }) => ({
		theme,
		statementCount: count,
		controversy: themeControversy(theme, data)
	}));
}

/** Overall agree% for one comment (denominator = total votes). */
export function overallAgreePct(c: ReportComment): number {
	const t = totalVotes(c);
	return t === 0 ? 0 : (c.overall_votes.agrees / t) * 100;
}
