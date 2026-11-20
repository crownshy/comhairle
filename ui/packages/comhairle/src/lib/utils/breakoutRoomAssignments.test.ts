import { describe, it, expect } from 'vitest';
import type {
	BreakoutRoomAssignments,
	VideoCallParticipant
} from '$lib/services/videoCallService.svelte';
import {
	applyRoomEdits,
	groupParticipantsByRoom,
	isFacilitatorRole,
	peopleOutsideRooms,
	placementOf
} from './breakoutRoomAssignments';

const participant = (user_id: string, role = 'participant'): VideoCallParticipant => ({
	user_id,
	username: user_id,
	role
});

const room = (...participants: string[]): BreakoutRoomAssignments => ({
	participants,
	room_jid: null
});

describe('groupParticipantsByRoom', () => {
	it('buckets each participant into the room they are assigned to', () => {
		const grouped = groupParticipantsByRoom(
			[participant('a'), participant('b'), participant('c')],
			[room('a', 'c'), room('b')]
		);

		expect(grouped.map((room) => room.map((p) => p.user_id))).toEqual([['a', 'c'], ['b']]);
	});

	it('keeps rooms in room order, not in participant order', () => {
		const grouped = groupParticipantsByRoom(
			[participant('b'), participant('a')],
			[room('a'), room('b')]
		);

		expect(grouped.map((room) => room.map((p) => p.user_id))).toEqual([['a'], ['b']]);
	});

	it('returns an empty bucket for a room nobody is in, so indices stay aligned', () => {
		const grouped = groupParticipantsByRoom([participant('a')], [room(), room('a')]);

		expect(grouped).toEqual([[], [participant('a')]]);
	});

	it('drops participants with no room assignment', () => {
		const grouped = groupParticipantsByRoom(
			[participant('a'), participant('unassigned')],
			[room('a')]
		);

		expect(grouped).toEqual([[participant('a')]]);
	});

	it('puts a participant listed in two rooms in the last one that claims them', () => {
		const grouped = groupParticipantsByRoom([participant('a')], [room('a'), room('a')]);

		expect(grouped.map((room) => room.length)).toEqual([0, 1]);
	});

	it('returns no rooms when there is no breakout plan', () => {
		expect(groupParticipantsByRoom([participant('a')], [])).toEqual([]);
	});
});

describe('applyRoomEdits', () => {
	it('sends the edited layout as it is', () => {
		expect(applyRoomEdits([['a'], ['b', 'c']], [room('a', 'b'), room('c')])).toEqual([
			['a'],
			['b', 'c']
		]);
	});

	it('keeps a room the facilitator emptied, so later rooms keep their numbers', () => {
		expect(applyRoomEdits([[], ['a', 'b'], ['c']], [room('a'), room('b'), room('c')])).toEqual([
			[],
			['a', 'b'],
			['c']
		]);
	});

	it('leaves someone placed after the dialog opened in their current room', () => {
		expect(applyRoomEdits([['a'], ['b']], [room('a'), room('b', 'late')])).toEqual([
			['a'],
			['b', 'late']
		]);
	});

	it('takes someone moved to plenary out of their room', () => {
		expect(applyRoomEdits([['a'], []], [room('a'), room('b')], ['b'])).toEqual([['a'], []]);
	});

	it('never has fewer rooms than are open now', () => {
		expect(applyRoomEdits([['a']], [room('a'), room()])).toEqual([['a'], []]);
	});
});

describe('isFacilitatorRole', () => {
	it('counts moderators and facilitators as able to run a room', () => {
		expect(isFacilitatorRole('moderator')).toBe(true);
		expect(isFacilitatorRole('facilitator')).toBe(true);
	});

	it('does not count participants', () => {
		expect(isFacilitatorRole('participant')).toBe(false);
	});
});

describe('peopleOutsideRooms', () => {
	it('lists the people who are in no room', () => {
		const [a, b, c] = [participant('a'), participant('b'), participant('c')];
		expect(peopleOutsideRooms([a, b, c], [[a], [], [c]])).toEqual([b]);
	});
});

describe('placementOf', () => {
	const rooms = [room('a'), room('b')];

	it('gives the room someone is in', () => {
		expect(placementOf('b', rooms, new Set())).toBe(1);
	});

	it('gives plenary for someone kept in the main call', () => {
		expect(placementOf('c', rooms, new Set(['c']))).toBe('plenary');
	});

	it('gives null for someone with no placement', () => {
		expect(placementOf('host', rooms, new Set())).toBeNull();
	});
});
