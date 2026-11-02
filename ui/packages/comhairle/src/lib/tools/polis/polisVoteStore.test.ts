// @vitest-environment jsdom
import { beforeEach, describe, expect, it } from 'vitest';
import {
	getVoteData,
	incrementVotes,
	reconcileServerVotes,
	startNextVotingRound
} from './polisVoteStore';

const USER = 'user-1';

describe('polisVoteStore', () => {
	beforeEach(() => {
		localStorage.clear();
	});

	it('starts empty for an unseen scope', () => {
		expect(getVoteData(USER, 'live-step-a')).toEqual({
			totalVotes: 0,
			hasMetThreshold: false,
			votesAtLastPrompt: 0
		});
	});

	it('flags the threshold once the required votes are reached', () => {
		incrementVotes(USER, 'live-step-a', 2);
		expect(getVoteData(USER, 'live-step-a')).toEqual({
			totalVotes: 1,
			hasMetThreshold: false,
			votesAtLastPrompt: 0
		});

		const data = incrementVotes(USER, 'live-step-a', 2);
		expect(data).toEqual({ totalVotes: 2, hasMetThreshold: true, votesAtLastPrompt: 0 });
	});

	// The bug: two Polis steps sharing one poll used to share their progress.
	// Keying by a per-step scope must keep them fully independent.
	it('keeps progress independent across scopes', () => {
		incrementVotes(USER, 'live-step-a', 2);
		incrementVotes(USER, 'live-step-a', 2);

		// A second step (even one on the same poll) is untouched.
		expect(getVoteData(USER, 'live-step-b')).toEqual({
			totalVotes: 0,
			hasMetThreshold: false,
			votesAtLastPrompt: 0
		});

		incrementVotes(USER, 'live-step-b', 2);
		expect(getVoteData(USER, 'live-step-b')).toEqual({
			totalVotes: 1,
			hasMetThreshold: false,
			votesAtLastPrompt: 0
		});
		// Step A's met-threshold state did not leak into step B.
		expect(getVoteData(USER, 'live-step-a').hasMetThreshold).toBe(true);
	});

	it('separates preview from live for the same step', () => {
		incrementVotes(USER, 'live-step-a', 1);
		expect(getVoteData(USER, 'preview-step-a')).toEqual({
			totalVotes: 0,
			hasMetThreshold: false,
			votesAtLastPrompt: 0
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
			expect(data).toEqual({ totalVotes: 10, hasMetThreshold: true, votesAtLastPrompt: 0 });
		});

		it('persists the reconciled count', () => {
			reconcileServerVotes(USER, 'live-step-a', 5, 10);
			expect(getVoteData(USER, 'live-step-a').totalVotes).toBe(5);
		});
	});

	describe('startNextVotingRound', () => {
		it('keeps the total and marks where the new round starts, for that scope only', () => {
			incrementVotes(USER, 'live-step-a', 5);
			incrementVotes(USER, 'live-step-a', 5);
			incrementVotes(USER, 'live-step-b', 5);

			startNextVotingRound(USER, 'live-step-a');
			expect(getVoteData(USER, 'live-step-a')).toMatchObject({
				totalVotes: 2,
				votesAtLastPrompt: 2
			});
			expect(getVoteData(USER, 'live-step-b').votesAtLastPrompt).toBe(0);
		});

		// Reconciling with the server's lifetime count after a reload must not undo the round.
		it('survives a server reconcile', () => {
			reconcileServerVotes(USER, 'live-step-a', 5, 5);
			startNextVotingRound(USER, 'live-step-a');
			const data = reconcileServerVotes(USER, 'live-step-a', 7, 5);
			expect(data.totalVotes - data.votesAtLastPrompt).toBe(2);
		});

		it('reads data saved before the field existed', () => {
			localStorage.setItem(
				'comhairle-polis-votes-user-1-live-step-a',
				JSON.stringify({ totalVotes: 3, hasMetThreshold: false })
			);
			expect(getVoteData(USER, 'live-step-a').votesAtLastPrompt).toBe(0);
		});
	});
});
