// @vitest-environment jsdom
import { beforeEach, describe, expect, it } from 'vitest';
import {
	getVoteData,
	incrementVotes,
	reconcileServerVotes,
	resetVoteCount
} from './polisVoteStore';

const USER = 'user-1';

describe('polisVoteStore', () => {
	beforeEach(() => {
		localStorage.clear();
	});

	it('starts empty for an unseen scope', () => {
		expect(getVoteData(USER, 'live-step-a')).toEqual({ totalVotes: 0, hasMetThreshold: false });
	});

	it('flags the threshold once the required votes are reached', () => {
		incrementVotes(USER, 'live-step-a', 2);
		expect(getVoteData(USER, 'live-step-a')).toEqual({ totalVotes: 1, hasMetThreshold: false });

		const data = incrementVotes(USER, 'live-step-a', 2);
		expect(data).toEqual({ totalVotes: 2, hasMetThreshold: true });
	});

	// The bug: two Polis steps sharing one poll used to share their progress.
	// Keying by a per-step scope must keep them fully independent.
	it('keeps progress independent across scopes', () => {
		incrementVotes(USER, 'live-step-a', 2);
		incrementVotes(USER, 'live-step-a', 2);

		// A second step (even one on the same poll) is untouched.
		expect(getVoteData(USER, 'live-step-b')).toEqual({ totalVotes: 0, hasMetThreshold: false });

		incrementVotes(USER, 'live-step-b', 2);
		expect(getVoteData(USER, 'live-step-b')).toEqual({ totalVotes: 1, hasMetThreshold: false });
		// Step A's met-threshold state did not leak into step B.
		expect(getVoteData(USER, 'live-step-a').hasMetThreshold).toBe(true);
	});

	it('separates preview from live for the same step', () => {
		incrementVotes(USER, 'live-step-a', 1);
		expect(getVoteData(USER, 'preview-step-a')).toEqual({
			totalVotes: 0,
			hasMetThreshold: false
		});
	});

	describe('reconcileServerVotes', () => {
		it('adopts the server count when it is ahead (votes from another device)', () => {
			const data = reconcileServerVotes(USER, 'live-step-a', 8, 10);
			expect(data.totalVotes).toBe(8);
			expect(data.hasMetThreshold).toBe(false);
		});

		it('keeps the local count when it is ahead of a lagging server', () => {
			incrementVotes(USER, 'live-step-a', 10);
			incrementVotes(USER, 'live-step-a', 10);
			incrementVotes(USER, 'live-step-a', 10);
			const data = reconcileServerVotes(USER, 'live-step-a', 1, 10);
			expect(data.totalVotes).toBe(3);
		});

		it('flags the threshold once the merged count reaches it', () => {
			const data = reconcileServerVotes(USER, 'live-step-a', 10, 10);
			expect(data).toEqual({ totalVotes: 10, hasMetThreshold: true });
		});

		it('persists the reconciled count', () => {
			reconcileServerVotes(USER, 'live-step-a', 5, 10);
			expect(getVoteData(USER, 'live-step-a').totalVotes).toBe(5);
		});
	});

	it('resetVoteCount zeroes the count for that scope only', () => {
		incrementVotes(USER, 'live-step-a', 5);
		incrementVotes(USER, 'live-step-b', 5);

		resetVoteCount(USER, 'live-step-a');
		expect(getVoteData(USER, 'live-step-a').totalVotes).toBe(0);
		expect(getVoteData(USER, 'live-step-b').totalVotes).toBe(1);
	});
});
