/**
 * Colours map dots from each group's vote counts, because the report has no
 * per-person votes. Proportions are exact; which dot gets which colour is not.
 * See ADR-0040.
 */

import type { PolisReportData, ReportComment } from '$lib/tools/polis/reportTypes';
import type { Vote } from '../types';

/** A set of dots that share one set of vote counts. */
interface Cohort {
	/** Only participants with a map position. */
	pids: number[];
	/** Members Polis counts in this cohort, which can exceed the dots on screen. */
	members: number;
	votes: { agrees: number; disagrees: number; passes: number };
}

/**
 * Splits `seats` whole seats in proportion to `weights` (largest remainder method),
 * so the result always sums to exactly `seats`.
 */
export function apportion(weights: number[], seats: number): number[] {
	const total = weights.reduce((sum, w) => sum + w, 0);
	if (seats <= 0 || total <= 0) return weights.map(() => 0);

	const exact = weights.map((w) => (w / total) * seats);
	const whole = exact.map((value) => Math.floor(value));
	let left = seats - whole.reduce((sum, n) => sum + n, 0);

	const byRemainder = exact
		.map((value, index) => ({ index, remainder: value - Math.floor(value) }))
		.sort((a, b) => b.remainder - a.remainder || a.index - b.index);

	for (const { index } of byRemainder) {
		if (left <= 0) break;
		whole[index] += 1;
		left -= 1;
	}
	return whole;
}

/**
 * Stable sort key for one dot on one statement, so an unchanged report repaints the
 * same map. Uses the murmur3 finaliser so neighbouring pids are spread out.
 */
function dealKey(tid: number, pid: number): number {
	let h = Math.imul(tid + 1, 0x9e3779b1) ^ Math.imul(pid + 1, 0x85ebca6b);
	h = Math.imul(h ^ (h >>> 16), 0x2c1b3c6d);
	h = Math.imul(h ^ (h >>> 15), 0x297a2d39);
	return (h ^ (h >>> 15)) >>> 0;
}

/**
 * One cohort per group with counts for this statement, plus one for everyone else.
 * The last one gets whatever `overall_votes` has left after the groups.
 */
function cohorts(report: PolisReportData, comment: ReportComment): Cohort[] {
	const placed = report.participants.filter((p) => p.pca_position != null);
	const list: Cohort[] = [];
	const dealt = new Set<number>();

	for (const votes of comment.group_votes) {
		const pids = placed.filter((p) => p.group_id === votes.group_id).map((p) => p.pid);
		if (pids.length === 0) continue;
		for (const pid of pids) dealt.add(pid);
		const group = report.groups.find((g) => g.group_id === votes.group_id);
		list.push({ pids, members: group?.total_members ?? pids.length, votes });
	}

	const grouped = comment.group_votes.reduce(
		(sum, g) => ({
			agrees: sum.agrees + g.agrees,
			disagrees: sum.disagrees + g.disagrees,
			passes: sum.passes + g.passes
		}),
		{ agrees: 0, disagrees: 0, passes: 0 }
	);
	const rest = placed.filter((p) => !dealt.has(p.pid)).map((p) => p.pid);
	if (rest.length > 0) {
		list.push({
			pids: rest,
			members: rest.length,
			votes: {
				agrees: Math.max(0, comment.overall_votes.agrees - grouped.agrees),
				disagrees: Math.max(0, comment.overall_votes.disagrees - grouped.disagrees),
				passes: Math.max(0, comment.overall_votes.passes - grouped.passes)
			}
		});
	}
	return list;
}

/** A cohort's [agrees, disagrees, passes, notVoted]. */
function weights(cohort: Cohort): number[] {
	const { agrees, disagrees, passes } = cohort.votes;
	// Never below the votes cast: Polis can count a vote from someone it has since
	// moved to another group.
	const cast = agrees + disagrees + passes;
	const members = Math.max(cohort.members, cohort.pids.length, cast);
	return [agrees, disagrees, passes, members - cast];
}

const COLOURS: Vote[] = ['agree', 'disagree', 'pass'];

/** Statement tid to participant pid to vote. A missing pid has not voted. */
export function apportionVotes(report: PolisReportData): Map<number, Map<number, Vote>> {
	const matrix = new Map<number, Map<number, Vote>>();

	for (const comment of report.comments) {
		const byPid = new Map<number, Vote>();
		for (const cohort of cohorts(report, comment)) {
			const seats = apportion(weights(cohort), cohort.pids.length);
			const order = [...cohort.pids].sort(
				(a, b) => dealKey(comment.tid, a) - dealKey(comment.tid, b) || a - b
			);
			let next = 0;
			COLOURS.forEach((vote, i) => {
				for (let n = 0; n < seats[i]; n += 1) byPid.set(order[next++], vote);
			});
		}
		matrix.set(comment.tid, byPid);
	}
	return matrix;
}
