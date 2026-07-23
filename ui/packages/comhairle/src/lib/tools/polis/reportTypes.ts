/**
 * Polis report data shapes.
 *
 * The wire types are the single source of truth in the generated
 * `@crownshy/api-client` (`WikiPollReport` and friends) — this module does NOT
 * re-declare them, it only re-exports them under the names this feature uses
 * and adds what the client can't know about:
 *   - `topics`/`subtopics`: a client-side overlay tagged from
 *     `polis_statement_aux.themes`; the backend payload carries neither.
 *   - view models computed in `report.ts` (`GroupVotePercent`) and the Themes
 *     card roll-ups (`ThemeControversy`, `ThemeSummary`).
 * See PolisInsights.svelte and CONTEXT.md.
 */

import type {
	CommentReportData,
	GroupReportData,
	ParticipantReportData
} from '@crownshy/api-client/api';

// Generated wire types, aliased to the names this feature already uses so
// consumers get the client's types without importing the client directly.
export type {
	VoteCounts,
	GroupVoteCounts as GroupVote,
	RepresentativeComment,
	PcaPosition,
	ParticipantReportData as ReportParticipant
} from '@crownshy/api-client/api';

export type ReportGroup = GroupReportData;

/**
 * A report comment plus the client-only theme overlay. `topics`/`subtopics`
 * are tagged from `polis_statement_aux.themes` after load; the backend payload
 * (`CommentReportData`) carries neither.
 */
export type ReportComment = CommentReportData & {
	topics?: string[];
	subtopics?: string[];
};

/**
 * `WikiPollReport` with the client-side theme overlay applied to its comments.
 *
 * Spelled out from the generated element types rather than
 * `Omit<WikiPollReport, 'comments'>`: the client's `.passthrough()` adds a
 * `[k: string]: unknown` index signature, which makes `Omit` collapse the
 * remaining props to `unknown`.
 */
export interface PolisReportData {
	comments: ReportComment[];
	groups: GroupReportData[];
	participants: ParticipantReportData[];
}

/** Per-group vote percentage breakdown for one comment. Computed in report.ts. */
export interface GroupVotePercent {
	group_id: number;
	label: string;
	totalMembers: number;
	totalVoted: number;
	agreed: number;
	disagreed: number;
	passed: number;
}

/**
 * Vote breakdown for the report's stacked VoteBar, taken over a group's total
 * MEMBERSHIP (not just voters) so the not-voted remainder is shown. Distinct
 * from GroupVotePercent, which is of-voters and carries no notVoted. The four
 * shares sum to ~100. Computed in report.ts (`computeMemberVoteBars`).
 */
export interface MemberVotePercent {
	label: string;
	agreed: number;
	disagreed: number;
	passed: number;
	notVoted: number;
}

/**
 * Roll-up for one theme as shown in the Themes card on the Insights page.
 *
 * `controversy` is defined in CONTEXT.md and computed in `themeControversy()`
 * - it is our own classification, not from Polis/T3C. `subtopics` is left
 * empty until a T3C-shaped source provides them.
 */
export type ThemeControversy = 'low' | 'moderate' | 'high';

export interface ThemeSummary {
	theme: string;
	statementCount: number;
	controversy: ThemeControversy;
	subtopics?: string[];
}
