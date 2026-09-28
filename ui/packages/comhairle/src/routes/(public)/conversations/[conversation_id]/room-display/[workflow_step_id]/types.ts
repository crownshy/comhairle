/** Data shapes the Room display components consume. See CONTEXT.md, "Room display". */

import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';

/** A participant's answer to a statement. A missing entry means they have not voted. */
export type Vote = 'agree' | 'disagree' | 'pass';

/** How one map node's members split on one statement. The four counts sum to `memberCount`. */
export interface VoteDistribution {
	agrees: number;
	disagrees: number;
	passes: number;
	notVoted: number;
}

/**
 * One dot on the opinion map. It is a Polis base cluster, not a participant: Polis caps
 * base clusters at 100, so a dot is one person only in rooms smaller than that.
 */
export interface MapNode {
	/** Polis base-cluster id. Not a participant id, even when the two coincide. */
	id: number;
	x: number;
	y: number;
	/** Owning opinion group, or null before Polis has clustered. */
	groupId: number | null;
	memberCount: number;
}

/** How much of the display is unlocked. See CONTEXT.md, "Reveal stage". */
export type RevealStage = 'empty' | 'warming' | 'shaped' | 'rich';

/** Ambient (the default) has no pointer; driven makes hover and click live. See CONTEXT.md. */
export type DisplayMode = 'ambient' | 'driven';

/**
 * A scripted event, at milliseconds from the start of the run. Each vote is its own
 * event so the map knows which dot voted which way.
 */
export type ScenarioEvent =
	| { at: number; kind: 'participantJoined'; nodeId: number }
	| { at: number; kind: 'statementPublished'; tid: number }
	| { at: number; kind: 'voteCast'; tid: number; nodeId: number; vote: Vote };

/**
 * A scripted run of a Polis conversation. `comments` and `groups` are the end state, in
 * the real report shapes; `events` make that end state arrive over time.
 */
export interface Scenario {
	durationMs: number;
	nodes: MapNode[];
	comments: ReportComment[];
	groups: ReportGroup[];
	events: ScenarioEvent[];
}

/**
 * Everything the display renders at one moment. Computed purely from the scenario and
 * `atMs`, so seeking backwards works the same as playing forwards.
 */
export interface DisplayState {
	atMs: number;
	/** Nodes that have joined by now, in join order. */
	nodes: MapNode[];
	/** Published statements, most recently published first. */
	published: ReportComment[];
	/** Votes cast so far, per statement tid, keyed by node id. */
	votesByTid: Map<number, Map<number, Vote>>;
	totalVotes: number;
}

/**
 * A display state with its reveal stage attached. The stage is tracked separately because
 * it never goes backwards, and that needs history a single state does not have.
 */
export type StagedDisplayState = DisplayState & { stage: RevealStage };
