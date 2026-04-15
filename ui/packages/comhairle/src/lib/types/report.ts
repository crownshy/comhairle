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

export interface GroupVotePercent {
	group_id: number;
	label: string;
	agreed: number;
	disagreed: number;
	passed: number;
	notVoted: number;
}
