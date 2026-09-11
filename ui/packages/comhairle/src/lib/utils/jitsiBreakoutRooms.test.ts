import { describe, it, expect } from 'vitest';
import { getJitsiBreakoutRoomId, type JitsiBreakoutRoom } from './jitsiBreakoutRooms';

const rooms = (...entries: JitsiBreakoutRoom[]): Record<string, JitsiBreakoutRoom> =>
	Object.fromEntries(entries.map((room, i) => [`key-${i}`, room]));

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
		expect(getJitsiBreakoutRoomId({}, 0)).toBeNull();
	});

	it('returns null when a room has no jid', () => {
		expect(getJitsiBreakoutRoomId(rooms({ name: 'A' }), 0)).toBeNull();
	});

	it('sorts unnamed rooms first rather than dropping them', () => {
		const all = rooms({ jid: 'named@breakout', name: 'A' }, { jid: 'unnamed@breakout' });

		expect(getJitsiBreakoutRoomId(all, 0)).toBe('unnamed@breakout');
	});

	// Current behaviour, not desired behaviour: the sort is lexicographic, so once there
	// are ten or more rooms "#10" lands before "#2" and every index past it points at the
	// wrong room. Part of #752; this test is here to fail loudly when that gets fixed.
	it('orders room names lexicographically, so #10 sorts before #2', () => {
		const all = rooms(
			{ jid: 'ten@breakout', name: 'Breakout room #10' },
			{ jid: 'two@breakout', name: 'Breakout room #2' }
		);

		expect(getJitsiBreakoutRoomId(all, 0)).toBe('ten@breakout');
	});
});
