import { describe, it, expect } from 'vitest';
import { getJitsiBreakoutRoomId, type JitsiBreakoutRoom } from './jitsiBreakoutRooms';

const rooms = (...entries: JitsiBreakoutRoom[]): JitsiBreakoutRoom[] => entries;

describe('getJitsiBreakoutRoomId', () => {
	it('maps a room index to the jid of the nth non-main room, in name order', () => {
		const all = rooms(
			{ jid: 'main@conf', name: 'Main room', isMainRoom: true },
			{ jid: 'two@breakout', name: 'Breakout room #2' },
			{ jid: 'one@breakout', name: 'Breakout room #1' }
		);

		expect(getJitsiBreakoutRoomId(all, 0)).toBe('one@breakout');
		expect(getJitsiBreakoutRoomId(all, 1)).toBe('two@breakout');
	});

	it('returns null for an index past the last room', () => {
		expect(getJitsiBreakoutRoomId(rooms({ jid: 'one@breakout', name: 'A' }), 3)).toBeNull();
	});

	it('returns null when Jitsi has not reported any rooms yet', () => {
		expect(getJitsiBreakoutRoomId([], 0)).toBeNull();
	});

	it('returns null when a room has no jid', () => {
		expect(getJitsiBreakoutRoomId(rooms({ name: 'A' }), 0)).toBeNull();
	});

	it('sorts unnamed rooms first rather than dropping them', () => {
		const all = rooms({ jid: 'named@breakout', name: 'A' }, { jid: 'unnamed@breakout' });

		expect(getJitsiBreakoutRoomId(all, 0)).toBe('unnamed@breakout');
	});

	it('orders rooms by their number, so #10 sorts after #2', () => {
		const all = rooms(
			...Array.from({ length: 11 }, (_, i) => ({
				jid: `room-${i + 1}@breakout`,
				name: `Breakout room #${i + 1}`
			})).reverse()
		);

		expect(getJitsiBreakoutRoomId(all, 1)).toBe('room-2@breakout');
		expect(getJitsiBreakoutRoomId(all, 9)).toBe('room-10@breakout');
		expect(getJitsiBreakoutRoomId(all, 10)).toBe('room-11@breakout');
	});

	it('picks the same room for duplicate names whatever order Jitsi lists them in', () => {
		const first = { jid: 'a@breakout', name: 'Breakout room #1' };
		const second = { jid: 'b@breakout', name: 'Breakout room #1' };

		expect(getJitsiBreakoutRoomId(rooms(first, second), 0)).toBe('a@breakout');
		expect(getJitsiBreakoutRoomId(rooms(second, first), 0)).toBe('a@breakout');
	});
});
