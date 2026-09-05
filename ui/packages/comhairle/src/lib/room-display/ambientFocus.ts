/**
 * What the Room display looks at when nobody is driving it.
 *
 * Ambient is the default mode and has no pointer, so something has to choose which
 * statement the [[consensus continuum]] is pointing at and therefore what the opinion
 * map is coloured by. Round-robin through every statement would be the obvious answer
 * and a bad one: most statements are unremarkable, and a display that spends four
 * hours cycling through them evenly is a screensaver.
 *
 * Instead the rotation cycles **intents**, not statements. Each intent is a reason to
 * look at something ("the sharpest split right now"), resolved against current data
 * every time it comes round. So the display keeps returning to the interesting parts
 * of the conversation, and what counts as interesting changes as the room votes.
 *
 * Pure and derived from elapsed time, so it costs the driver no extra state and
 * scrubbing lands on the same focus every time.
 */

import type { DisplayState } from './types';
import type { ReportComment } from '$lib/tools/polis/reportTypes';

export type FocusIntent = 'mostDivisive' | 'strongestConsensus' | 'newest' | 'mostVoted';

/**
 * The rotation. Divisive first because it is the most useful thing to have on screen
 * when a facilitator turns to the display mid-argument, and because it is what the
 * room is there to surface.
 */
export const AMBIENT_ROTATION: readonly FocusIntent[] = [
	'mostDivisive',
	'strongestConsensus',
	'newest',
	'mostVoted'
];

/** How long one focus holds before the rotation advances. */
export const DEFAULT_DWELL_MS = 12_000;

function scored(comments: ReportComment[]): ReportComment[] {
	return comments.filter(
		(c) => typeof c.divisiveness === 'number' && Number.isFinite(c.divisiveness)
	);
}

function best(comments: ReportComment[], rank: (c: ReportComment) => number): ReportComment | null {
	if (comments.length === 0) return null;
	return comments.reduce((a, b) => (rank(b) > rank(a) ? b : a));
}

/** Resolves one intent against the statements published so far. */
export function resolveIntent(state: DisplayState, intent: FocusIntent): number | null {
	const published = state.published;
	if (published.length === 0) return null;

	switch (intent) {
		case 'mostDivisive':
			return best(scored(published), (c) => c.divisiveness ?? 0)?.tid ?? null;
		case 'strongestConsensus':
			return best(published, (c) => c.group_informed_consensus ?? 0)?.tid ?? null;
		case 'newest':
			// `published` is most-recent-first, which is the order the ticker wants too.
			return published[0]?.tid ?? null;
		case 'mostVoted':
			return best(published, (c) => state.votesByTid.get(c.tid)?.size ?? 0)?.tid ?? null;
	}
}

export interface AmbientFocus {
	intent: FocusIntent;
	tid: number | null;
}

/**
 * The ambient focus at a point in the run.
 *
 * Falls forward through the rotation rather than showing nothing when an intent
 * resolves to nothing: early on, several intents have no statement to point at, and a
 * display that blanks every other cycle looks broken rather than patient.
 */
export function ambientFocusAt(
	state: DisplayState,
	elapsedMs: number,
	dwellMs = DEFAULT_DWELL_MS
): AmbientFocus {
	const step = dwellMs > 0 ? Math.floor(elapsedMs / dwellMs) : 0;

	for (let offset = 0; offset < AMBIENT_ROTATION.length; offset++) {
		const intent = AMBIENT_ROTATION[(step + offset) % AMBIENT_ROTATION.length];
		const tid = resolveIntent(state, intent);
		if (tid !== null) return { intent, tid };
	}

	return { intent: AMBIENT_ROTATION[step % AMBIENT_ROTATION.length], tid: null };
}

/** Caption for the current focus, so the room knows why it is looking at this one. */
export function describeIntent(intent: FocusIntent): string {
	switch (intent) {
		case 'mostDivisive':
			return 'Where the room splits hardest';
		case 'strongestConsensus':
			return 'What the room agrees on most';
		case 'newest':
			return 'Just added';
		case 'mostVoted':
			return 'Most voted on';
	}
}
