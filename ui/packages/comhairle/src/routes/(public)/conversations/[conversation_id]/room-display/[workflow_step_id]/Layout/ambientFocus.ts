/**
 * Picks the statement to show when nobody is driving the display. It rotates through
 * reasons to look (most divisive, most agreed, ...) rather than through every statement.
 */

import type { DisplayState } from '../types';
import type { ReportComment } from '$lib/tools/polis/reportTypes';

export type FocusIntent = 'mostDivisive' | 'strongestConsensus' | 'newest' | 'mostVoted';

/** Divisive comes first because it is what a facilitator most often wants on screen. */
export const AMBIENT_ROTATION: readonly FocusIntent[] = [
	'mostDivisive',
	'strongestConsensus',
	'newest',
	'mostVoted'
];

export const DEFAULT_DWELL_MS = 12_000;

function scored(comments: ReportComment[]): ReportComment[] {
	return comments.filter(
		(c) => typeof c.divisiveness === 'number' && Number.isFinite(c.divisiveness)
	);
}

/** The sort is stable, so ties keep their published order. */
function ranked(comments: ReportComment[], rank: (c: ReportComment) => number): ReportComment[] {
	return [...comments].sort((a, b) => rank(b) - rank(a));
}

/** Best first. Shared by the deck and the ambient rotation so both rank the same way. */
export function rankIntent(
	state: DisplayState,
	intent: FocusIntent,
	limit: number
): ReportComment[] {
	const published = state.published;
	if (published.length === 0 || limit <= 0) return [];

	switch (intent) {
		case 'mostDivisive':
			return ranked(scored(published), (c) => c.divisiveness ?? 0).slice(0, limit);
		case 'strongestConsensus':
			return ranked(published, (c) => c.group_informed_consensus ?? 0).slice(0, limit);
		case 'newest':
			// `published` is already ordered most recent first.
			return published.slice(0, limit);
		case 'mostVoted':
			return ranked(published, (c) => state.votesByTid.get(c.tid)?.size ?? 0).slice(0, limit);
	}
}

export function resolveIntent(state: DisplayState, intent: FocusIntent): number | null {
	return rankIntent(state, intent, 1)[0]?.tid ?? null;
}

export interface AmbientFocus {
	intent: FocusIntent;
	tid: number | null;
}

/**
 * When an intent has no statement yet, this moves on to the next intent instead of
 * showing nothing, so the display does not blank out early in a session.
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

/** Caption telling the room why this statement is on screen. */
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
