import { describe, it, expect } from 'vitest';
import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';
import { applyRoomEdits, groupParticipantsByRoom } from './breakoutRoomAssignments';

const participant = (user_id: string, role = 'participant'): VideoCallParticipant => ({
	user_id,
	username: user_id,
	role
});

describe('groupParticipantsByRoom', () => {
	it('buckets each participant into the room they are assigned to', () => {
		const grouped = groupParticipantsByRoom(
			[participant('a'), participant('b'), participant('c')],
			[{ participants: ['a', 'c'] }, { participants: ['b'] }]
		);

		expect(grouped.map((room) => room.map((p) => p.user_id))).toEqual([['a', 'c'], ['b']]);
	});

	it('keeps rooms in room order, not in participant order', () => {
		const grouped = groupParticipantsByRoom(
			[participant('b'), participant('a')],
			[{ participants: ['a'] }, { participants: ['b'] }]
		);

		expect(grouped.map((room) => room.map((p) => p.user_id))).toEqual([['a'], ['b']]);
	});

	it('returns an empty bucket for a room nobody is in, so indices stay aligned', () => {
		const grouped = groupParticipantsByRoom(
			[participant('a')],
			[{ participants: [] }, { participants: ['a'] }]
		);

		expect(grouped).toEqual([[], [participant('a')]]);
	});

	it('drops participants with no room assignment', () => {
		const grouped = groupParticipantsByRoom(
			[participant('a'), participant('unassigned')],
			[{ participants: ['a'] }]
		);

		expect(grouped).toEqual([[participant('a')]]);
	});

	it('puts a participant listed in two rooms in the last one that claims them', () => {
		const grouped = groupParticipantsByRoom(
			[participant('a')],
			[{ participants: ['a'] }, { participants: ['a'] }]
		);

		expect(grouped.map((room) => room.length)).toEqual([0, 1]);
	});

	it('returns no rooms when there is no breakout plan', () => {
		expect(groupParticipantsByRoom([participant('a')], [])).toEqual([]);
	});
});

describe('applyRoomEdits', () => {
	it('sends the edited layout as it is', () => {
		expect(
			applyRoomEdits(
				[['a'], ['b', 'c']],
				[{ participants: ['a', 'b'] }, { participants: ['c'] }]
			)
		).toEqual([['a'], ['b', 'c']]);
	});

	it('keeps a room the facilitator emptied, so later rooms keep their numbers', () => {
		expect(
			applyRoomEdits(
				[[], ['a', 'b'], ['c']],
				[{ participants: ['a'] }, { participants: ['b'] }, { participants: ['c'] }]
			)
		).toEqual([[], ['a', 'b'], ['c']]);
	});

	it('leaves someone placed after the dialog opened in their current room', () => {
		expect(
			applyRoomEdits(
				[['a'], ['b']],
				[{ participants: ['a'] }, { participants: ['b', 'late'] }]
			)
		).toEqual([['a'], ['b', 'late']]);
	});

	it('never has fewer rooms than are open now', () => {
		expect(applyRoomEdits([['a']], [{ participants: ['a'] }, { participants: [] }])).toEqual([
			['a'],
			[]
		]);
	});
});
