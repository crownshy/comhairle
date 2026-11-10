import { describe, it, expect, vi, beforeEach } from 'vitest';
import type { JitsiBreakoutRoom } from '$lib/utils/jitsiBreakoutRooms';
import type { JitsiMeetExternalApi } from '$lib/components/JitsiMeet/types';
import { JitsiRoom } from './jitsiRoom.svelte';

type SentCommand = [string, ...unknown[]];

function fakeApi(options: { listed?: Record<string, JitsiBreakoutRoom> | Error } = {}) {
	const sent: SentCommand[] = [];
	const api = {
		executeCommand: (command: string, ...args: unknown[]) => {
			sent.push([command, ...args]);
		},
		listBreakoutRooms: async () => {
			if (options.listed instanceof Error) throw options.listed;
			return options.listed ?? {};
		},
		dispose: () => {}
	} as unknown as JitsiMeetExternalApi;
	return { api, sent };
}

/** Jitsi keys rooms by an internal id we never read, so the keys here are arbitrary. */
const roomsPayload = (...rooms: JitsiBreakoutRoom[]): Record<string, JitsiBreakoutRoom> =>
	Object.fromEntries(rooms.map((room, i) => [`key-${i}`, room]));

const breakouts = roomsPayload(
	{ id: 'main', jid: 'main@conf', name: 'Main room', isMainRoom: true },
	{ id: 'one', jid: 'one@breakout', name: 'Breakout room #1' },
	{ id: 'two', jid: 'two@breakout', name: 'Breakout room #2' }
);

beforeEach(() => {
	vi.spyOn(console, 'warn').mockImplementation(() => {});
	vi.spyOn(console, 'error').mockImplementation(() => {});
});

describe('JitsiRoom', () => {
	it('joins the nth breakout room by jid', () => {
		const { api, sent } = fakeApi();
		const room = new JitsiRoom();
		room.attach(api);
		room.setRooms(breakouts);

		expect(room.joinBreakout(1)).toBe(true);
		expect(sent).toEqual([['joinBreakoutRoom', 'two@breakout']]);
	});

	it('sends nothing and reports failure when Jitsi has no room at that index', () => {
		const { api, sent } = fakeApi();
		const room = new JitsiRoom();
		room.attach(api);
		room.setRooms(breakouts);

		expect(room.joinBreakout(5)).toBe(false);
		expect(sent).toEqual([]);
	});

	it('returns to the main room by joining no room at all', () => {
		const { api, sent } = fakeApi();
		const room = new JitsiRoom();
		room.attach(api);

		room.returnToMain();

		expect(sent).toEqual([['joinBreakoutRoom']]);
	});

	it('creates rooms named by their 1-based position', () => {
		const { api, sent } = fakeApi();
		const room = new JitsiRoom();
		room.attach(api);

		room.create(2);

		expect(sent).toEqual([
			['addBreakoutRoom', 'Breakout room #1'],
			['addBreakoutRoom', 'Breakout room #2']
		]);
	});

	it('removes every room on the list Jitsi hands back, not the one it last pushed', async () => {
		const { api, sent } = fakeApi({
			listed: roomsPayload(
				{ id: 'main', jid: 'main@conf', isMainRoom: true },
				{ id: 'three', jid: 'three@breakout', name: 'Breakout room #3' }
			)
		});
		const room = new JitsiRoom();
		room.attach(api);
		room.setRooms(breakouts);

		await room.removeAll();

		expect(sent).toEqual([['removeBreakoutRoom', 'three@breakout']]);
	});

	it('falls back to the rooms it was told about when Jitsi will not list them', async () => {
		const { api, sent } = fakeApi({ listed: new Error('nope') });
		const room = new JitsiRoom();
		room.attach(api);
		room.setRooms(breakouts);

		await room.removeAll();

		expect(sent).toEqual([
			['removeBreakoutRoom', 'one@breakout'],
			['removeBreakoutRoom', 'two@breakout']
		]);
	});

	it('is ready once Jitsi reports as many rooms as the session expects', () => {
		const room = new JitsiRoom();
		room.setRooms(breakouts);

		room.syncReady(3);
		expect(room.ready).toBe(false);

		room.syncReady(2);
		expect(room.ready).toBe(true);
	});

	it('is not ready while no breakout session is running', () => {
		const room = new JitsiRoom();
		room.setRooms(breakouts);
		room.syncReady(2);

		room.syncReady(null);

		expect(room.ready).toBe(false);
	});

	it('forgets its rooms on reset', () => {
		const room = new JitsiRoom();
		room.setRooms(breakouts);
		room.syncReady(2);

		room.reset();

		expect(room.rooms).toEqual([]);
		expect(room.ready).toBe(false);
	});

	describe('when a session ends and another starts', () => {
		const secondSession = roomsPayload(
			{ id: 'main', jid: 'main@conf', name: 'Main room', isMainRoom: true },
			{ id: 'new-one', jid: 'new-one@breakout', name: 'Breakout room #1' },
			{ id: 'new-two', jid: 'new-two@breakout', name: 'Breakout room #2' }
		);

		function endFirstSession() {
			const { api, sent } = fakeApi();
			const room = new JitsiRoom();
			room.attach(api);
			room.setRooms(breakouts);
			room.syncReady(2);
			room.syncReady(null);
			return { room, sent };
		}

		it('forgets the old rooms on every client, not just the one that ended it', () => {
			const { room } = endFirstSession();

			expect(room.rooms).toEqual([]);
		});

		it('is not ready for the next session until its own rooms exist', () => {
			const { room } = endFirstSession();
			room.setRooms(breakouts);

			room.syncReady(2);

			expect(room.ready).toBe(false);
		});

		it('joins the new room when Jitsi still lists an old one with the same name', () => {
			const { room, sent } = endFirstSession();
			room.setRooms(
				roomsPayload(...Object.values(breakouts), ...Object.values(secondSession))
			);

			room.syncReady(2);
			expect(room.ready).toBe(true);
			expect(room.joinBreakout(0)).toBe(true);
			expect(room.joinBreakout(1)).toBe(true);

			expect(sent).toEqual([
				['joinBreakoutRoom', 'new-one@breakout'],
				['joinBreakoutRoom', 'new-two@breakout']
			]);
		});

		it('does not forget rooms when no session was running', () => {
			const room = new JitsiRoom();
			room.setRooms(breakouts);

			room.syncReady(null);
			room.syncReady(2);

			expect(room.ready).toBe(true);
		});
	});
});
