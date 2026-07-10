/**
 * Polis report data shapes.
 *
 * Mirrors the comhairle backend `WikiPollReport` returned by
 * `GET /tools/polis/report_data?workflow_step_id=...` (the generated
 * `WikiPollReport` type from `@crownshy/api-client`). `topics`/`subtopics`
 * are a client-side overlay from `polis_statement_aux.themes` and are not on
 * the backend payload — see PolisInsights.svelte.
 */

export interface VoteCounts {
	agrees: number;
	disagrees: number;
	passes: number;
}

export interface GroupVote {
	group_id: number;
	agrees: number;
	disagrees: number;
	passes: number;
}

export interface ReportComment {
	tid: number;
	text: string;
	overall_votes: VoteCounts;
	group_votes: GroupVote[];
	group_informed_consensus: number;
	divisiveness: number;
	is_seed?: boolean;
	/** Theme tags overlaid from polis_statement_aux.themes (not from the backend). */
	topics?: string[];
	subtopics?: string[];
}

export interface RepresentativeComment {
	tid: number;
	text: string;
}

export interface ReportGroup {
	group_id: number;
	representative_comments: RepresentativeComment[];
	members: number[];
	total_members: number;
}

export interface PcaPosition {
	x: number;
	y: number;
}

export interface ReportParticipant {
	pid: number;
	group_id: number;
	pca_position: PcaPosition | null;
}

export interface PolisReportData {
	comments: ReportComment[];
	groups: ReportGroup[];
	participants: ReportParticipant[];
}

/** Per-group vote percentage breakdown for one comment. */
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
