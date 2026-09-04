/**
 * Room display data shapes.
 *
 * The Room display is a per-Step, large-format surface projected in a room while a
 * Polis conversation runs (see CONTEXT.md, "Room display"). It renders the same
 * `PolisReportData` the Insights tab does, plus two things the report payload does
 * not carry: a per-participant vote matrix, and an ordering over time.
 *
 * In the prototype both come from a scripted scenario rather than a live Polis. The
 * shapes here are what the components consume, so swapping the scenario for real
 * endpoints later does not touch the components.
 */

import type { ReportComment, ReportGroup } from '$lib/tools/polis/reportTypes';

/** A single participant's answer to a statement. `null` = has not voted on it. */
export type Vote = 'agree' | 'disagree' | 'pass';

/**
 * How one map node's members split on one statement. The four counts sum to the
 * node's `memberCount`, so `notVoted` is carried rather than inferred.
 */
export interface VoteDistribution {
	agrees: number;
	disagrees: number;
	passes: number;
	notVoted: number;
}

/**
 * One dot on the opinion map: a Polis **base cluster**, not a participant.
 *
 * Polis caps base clusters at 100 and exposes nothing finer above that, so a dot can
 * never be guaranteed to be one person. Below the cap each cluster holds a single
 * member, which is the case at a 28-person roundtable, and `memberCount` is 1: the
 * dot *is* a person and its `VoteDistribution` has exactly one non-zero count. Above
 * the cap the same component renders sized dots showing a split, with no second mode
 * and no degrade path to write. See CONTEXT.md, "Opinion map".
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

/**
 * The display's operating mode. Ambient is the default and has no pointer; driven
 * makes hover and click live. See CONTEXT.md, "Ambient mode / Driven mode".
 */
export type DisplayMode = 'ambient' | 'driven';

/**
 * A scenario event, stamped with milliseconds from the start of the run.
 *
 * Votes are one event each rather than a batched count, because the cross-highlight
 * needs to know *which* node voted *which* way, and because a fold over individual
 * votes is what a real vote stream will look like.
 */
export type ScenarioEvent =
	| { at: number; kind: 'participantJoined'; nodeId: number }
	| { at: number; kind: 'statementPublished'; tid: number }
	| { at: number; kind: 'voteCast'; tid: number; nodeId: number; vote: Vote };

/**
 * A scripted run of a Polis conversation.
 *
 * `comments` and `groups` are the end state, in the real report shapes: the display
 * reads scores (`divisiveness`, `group_informed_consensus`) straight off them, the
 * same way the live report does. `events` is what makes the end state arrive over
 * time. Statement *text* lives in its own module so placeholder copy can be swapped
 * for the real seeds in one file.
 */
export interface Scenario {
	/** Wall-clock length of the scripted run, in milliseconds. */
	durationMs: number;
	nodes: MapNode[];
	comments: ReportComment[];
	groups: ReportGroup[];
	events: ScenarioEvent[];
}

/**
 * Everything the display renders at one instant, folded from a scenario. Pure and
 * fully determined by `(scenario, atMs)`, so scrubbing backwards is the same
 * operation as playing forwards.
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
 * A folded state with its reveal stage attached. The stage is not part of the fold
 * because it ratchets, and a ratchet needs the history the fold deliberately throws
 * away: `computeStage` is not monotonic (a fresh voter with one vote drags the mean
 * votes-per-voter back down), so the stage at time T is not a function of the state
 * at time T alone. The driver owns that history and hands components this.
 */
export type StagedDisplayState = DisplayState & { stage: RevealStage };
