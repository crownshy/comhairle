/**
 * A per-dot vote matrix dealt out from the report's per-group aggregates.
 *
 * `report_data` carries no per-participant votes: a participant is a `pid`, a
 * `group_id` and a PCA position, and the only vote counts are per statement per
 * group. Fetching the real matrix would mean either one `/api/v3/votes` call per
 * participant per poll, or the participant-votes CSV export, and the display polls
 * every few seconds with a room in front of it.
 *
 * So the map does not colour a dot by what that person did. It deals each group's
 * real counts across that group's dots: if Group B cast 9 agrees, 2 disagrees and 1
 * pass, then 9, 2 and 1 of Group B's dots take those colours. Everything the room
 * reads off the map is real:
 *
 *  - the proportion inside each group, to the dot,
 *  - the contrast between groups, which is the story a Polis map tells,
 *  - the overall split, because the parts sum to it.
 *
 * What is not real is which dot got which colour. That is a deliberate trade, and it
 * buys something on top of the request count: at roundtable scale, a dot coloured by
 * a real vote is a person's ballot pinned to a fixed position on a projected wall,
 * and their neighbours can watch it. Dealing the colours out removes that.
 *
 * Two properties keep the wall calm across polls:
 *
 *  - The deal is deterministic in `(tid, pid)`, so re-polling unchanged data
 *    repaints the identical map rather than reshuffling every eight seconds.
 *  - The deal order is a stable ranking and the colours are handed out in blocks
 *    along it, so one new vote nudges each colour boundary along by one dot. At most
 *    one dot per colour changes hands, however big the group, instead of the whole
 *    group reshuffling.
 *
 * The order is keyed on the statement as well as the participant, so a dot is not
 * the same voter from statement to statement. That is on purpose: a dot that agreed
 * with everything would look like a person who agrees with everything, and no such
 * person is in the data.
 */

import type { PolisReportData, ReportComment } from '$lib/tools/polis/reportTypes';
import type { Vote } from './types';

/** One cohort of dots that shares a set of vote counts. */
interface Cohort {
	/** Placed participants only, because an unplaced one has no dot to colour. */
	pids: number[];
	/** Members Polis counts in this cohort, which can exceed the dots on screen. */
	members: number;
	votes: { agrees: number; disagrees: number; passes: number };
}

/**
 * Whole seats for each weight, by largest remainder.
 *
 * Largest remainder rather than rounding each share independently, because the seats
 * have to sum to the dots present: a rounding that hands out one seat too many leaves
 * a dot with two colours, and one too few leaves a real vote invisible. When the dots
 * present equal the members counted, this returns the counts themselves and the map
 * is exact.
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
 * Deal position for one dot on one statement.
 *
 * An integer hash rather than a random draw, so the same report always produces the
 * same map: a poll that changes nothing must not repaint the wall. Mixing is
 * `Math.imul`-based (the murmur3 finaliser) because adjacent `pid`s are exactly what
 * this gets fed, and a weak hash would deal them out in blocks and paint one side of
 * a cluster.
 */
function dealKey(tid: number, pid: number): number {
	let h = Math.imul(tid + 1, 0x9e3779b1) ^ Math.imul(pid + 1, 0x85ebca6b);
	h = Math.imul(h ^ (h >>> 16), 0x2c1b3c6d);
	h = Math.imul(h ^ (h >>> 15), 0x297a2d39);
	return (h ^ (h >>> 15)) >>> 0;
}

/**
 * The cohorts for one statement: one per opinion group the statement has counts for,
 * then everyone else.
 *
 * Built per statement rather than once, because "everyone else" is a property of the
 * statement. It holds the unclustered dots, and also any group Polis has not scored
 * this statement for, which is what the early minutes look like: no groups at all,
 * every dot in the residue, and the whole of `overall_votes` to deal across it. Its
 * counts are what the overall totals have left after the groups, so a vote Polis
 * counts once is drawn once.
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

/** A cohort's [agrees, disagrees, passes, notVoted], for `apportion` to seat. */
function weights(cohort: Cohort): number[] {
	const { agrees, disagrees, passes } = cohort.votes;
	// The denominator is membership, not voters, so the not-voted remainder is real
	// and the map matches the bars. It floors at the votes cast because Polis can
	// count a vote from someone it has since re-clustered elsewhere.
	const cast = agrees + disagrees + passes;
	const members = Math.max(cohort.members, cohort.pids.length, cast);
	return [agrees, disagrees, passes, members - cast];
}

const COLOURS: Vote[] = ['agree', 'disagree', 'pass'];

/**
 * The whole matrix: statement tid to dot pid to vote, in the shape `DisplayState`
 * already carries. A pid absent from a statement's map has not voted on it, which is
 * the convention the scripted driver uses too, so the components need no new branch.
 */
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
			// Whatever is left of `order` did not vote, and stays out of the map.
		}
		matrix.set(comment.tid, byPid);
	}
	return matrix;
}
