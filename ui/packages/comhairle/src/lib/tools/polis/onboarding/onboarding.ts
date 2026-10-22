import type { PolisReportData, ReportComment, VoteCounts } from '../reportTypes';
import { totalVotes } from '../report';

export type Vote = 'agree' | 'disagree' | 'pass';

/**
 * Where a first-timer is in onboarding. Each stage is one thing on screen: an instruction
 * with a Next button, or one instruction line above a statement to vote on.
 */
export type Stage =
	| 'deciding'
	| 'intro'
	| 'seed'
	| 'seed-result'
	| 'participant'
	| 'free'
	| 'missing'
	| 'groups'
	| 'done';

/** Live votes after the practice statements, before the "is anything missing?" screen. */
export const FREE_VOTES = 3;

/** After onboarding, the groups screen comes back every this many votes. */
export const REVEAL_EVERY = 15;

/** A statement as Polis lists it for participants, with its vote counts. */
export type PracticeStatement = {
	tid: number;
	text: string;
	is_seed: boolean;
	votes: VoteCounts;
};

/** The fields we read from a row of Polis's `/api/v3/comments` with voting patterns. */
export type PolisCommentRow = {
	tid: number;
	txt: string;
	is_seed?: boolean;
	is_meta?: boolean;
	mod?: number;
	agree_count?: number;
	disagree_count?: number;
	pass_count?: number;
};

/**
 * Turns Polis's comment list into statements a participant may see. Polis already filters
 * by moderation for participants; the `mod` check here is a second guard for strict steps.
 */
function isVisible(mod: number | undefined, strictModeration: boolean): boolean {
	if (mod === undefined) return true;
	return mod >= (strictModeration ? 1 : 0);
}

export function toPracticeStatements(
	rows: PolisCommentRow[],
	strictModeration: boolean
): PracticeStatement[] {
	return rows
		.filter((row) => !row.is_meta && isVisible(row.mod, strictModeration))
		.map((row) => ({
			tid: row.tid,
			text: row.txt,
			is_seed: row.is_seed ?? false,
			votes: {
				agrees: row.agree_count ?? 0,
				disagrees: row.disagree_count ?? 0,
				passes: row.pass_count ?? 0
			}
		}));
}

export type PracticePick = {
	seed: PracticeStatement | undefined;
	participant: PracticeStatement | undefined;
};

const votesOn = (s: PracticeStatement) => s.votes.agrees + s.votes.disagrees + s.votes.passes;

/**
 * The organisers' statement and the participant statement the first two votes are on. Most
 * votes first, so the result after the first vote has real numbers to show.
 */
export function pickPractice(statements: PracticeStatement[]): PracticePick {
	const byVotes = [...statements].sort((a, b) => votesOn(b) - votesOn(a) || a.tid - b.tid);
	return {
		seed: byVotes.find((s) => s.is_seed),
		participant: byVotes.find((s) => !s.is_seed)
	};
}

/** The first practice stage, or straight to free voting when there is nothing to practise on. */
export function firstVotingStage(pick: PracticePick): Stage {
	if (pick.seed) return 'seed';
	if (pick.participant) return 'participant';
	return 'free';
}

/** Where the Next button on the seed result goes. */
export function stageAfterSeedResult(pick: PracticePick): Stage {
	return pick.participant ? 'participant' : 'free';
}

export type CrowdBar = { agreed: number; disagreed: number; passed: number; voters: number };

/** Share of voters on a statement, counting the participant's own vote if Polis has not caught up. */
export function crowdBar(statement: PracticeStatement, ownVote: Vote | undefined): CrowdBar {
	const counts = { ...statement.votes };
	if (ownVote === 'agree') counts.agrees++;
	if (ownVote === 'disagree') counts.disagrees++;
	if (ownVote === 'pass') counts.passes++;
	const voters = counts.agrees + counts.disagrees + counts.passes;
	const share = (n: number) => (voters > 0 ? (n / voters) * 100 : 0);
	return {
		agreed: share(counts.agrees),
		disagreed: share(counts.disagrees),
		passed: share(counts.passes),
		voters
	};
}

export type GroupSize = { groupId: number; size: number; isYours: boolean };

export type GroupPortrait = {
	groups: GroupSize[];
	agreed: ReportComment[];
	disagreed: ReportComment[];
};

/**
 * The opinion groups so far, which one the participant is in, and what that group leaned
 * towards or against. Undefined until there are at least two groups and the participant is
 * in one, because a single group says nothing about anyone.
 */
export function groupPortrait(
	report: PolisReportData,
	pid: number | undefined
): GroupPortrait | undefined {
	if (pid === undefined || report.groups.length < 2) return undefined;
	const groupId = report.participants.find((p) => p.pid === pid)?.group_id;
	const group = report.groups.find((g) => g.group_id === groupId);
	if (!group) return undefined;

	const agreed: ReportComment[] = [];
	const disagreed: ReportComment[] = [];
	for (const rep of group.representative_comments) {
		const comment = report.comments.find((c) => c.tid === rep.tid);
		const votes = comment?.group_votes.find((v) => v.group_id === group.group_id);
		if (!comment || !votes) continue;
		if (votes.agrees > votes.disagrees) agreed.push(comment);
		else if (votes.disagrees > votes.agrees) disagreed.push(comment);
	}

	return {
		groups: report.groups.map((g) => ({
			groupId: g.group_id,
			size: g.total_members,
			isYours: g.group_id === group.group_id
		})),
		agreed: agreed.slice(0, 2),
		disagreed: disagreed.slice(0, 2)
	};
}

export type EarlySnapshot = { people: number; votes: number };

/** What we can say before there are groups: how many people and votes are in. */
export function earlySnapshot(report: PolisReportData | undefined): EarlySnapshot {
	if (!report) return { people: 0, votes: 0 };
	return {
		people: report.participants.length,
		votes: report.comments.reduce((sum, c) => sum + totalVotes(c), 0)
	};
}

/** True when this vote count should bring the groups screen back. */
export function isRevealDue(votesSinceOnboarding: number): boolean {
	return votesSinceOnboarding > 0 && votesSinceOnboarding % REVEAL_EVERY === 0;
}
