import { describe, it, expect, vi, beforeEach } from 'vitest';
import type { JitsiBreakoutRoom } from '$lib/utils/jitsiBreakoutRooms';
import type { JitsiMeetExternalApi } from '$lib/components/JitsiMeet/types';
import { JitsiRoom } from './jitsiRoom.svelte';

type SentCommand = [string, ...unknown[]];

type Listing = Record<string, JitsiBreakoutRoom> | Error;

/** `listed` is what every listing returns; `listings` returns each in turn, then repeats the last. */
function fakeApi(options: { listed?: Listing; listings?: Listing[] } = {}) {
	const sent: SentCommand[] = [];
	const listings = options.listings ?? [options.listed ?? {}];
	let listCount = 0;
	const api = {
		executeCommand: (command: string, ...args: unknown[]) => {
			sent.push([command, ...args]);
		},
		listBreakoutRooms: async () => {
			const listing = listings[Math.min(listCount++, listings.length - 1)];
			if (listing instanceof Error) throw listing;
			return listing;
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

		expect(sent).toEqual([
			['closeBreakoutRoom', 'three'],
			['removeBreakoutRoom', 'three@breakout']
		]);
	});

	it('waits for closed rooms to empty before removing them', async () => {
		vi.useFakeTimers();
		const occupied = roomsPayload({
			id: 'one',
			jid: 'one@breakout',
			name: 'Breakout room #1',
			participants: { 'someone@conf': {} }
		});
		const emptied = roomsPayload({ id: 'one', jid: 'one@breakout', name: 'Breakout room #1' });
		const { api, sent } = fakeApi({ listings: [occupied, occupied, emptied] });
		const room = new JitsiRoom();
		room.attach(api);

		const removing = room.removeAll();
		await vi.advanceTimersByTimeAsync(0);
		expect(sent).toEqual([['closeBreakoutRoom', 'one']]);

		await vi.advanceTimersByTimeAsync(250);
		await removing;
		expect(sent).toEqual([
			['closeBreakoutRoom', 'one'],
			['removeBreakoutRoom', 'one@breakout']
		]);
		vi.useRealTimers();
	});

	it('removes the rooms anyway if they never empty', async () => {
		vi.useFakeTimers();
		const occupied = roomsPayload({
			id: 'one',
			jid: 'one@breakout',
			name: 'Breakout room #1',
			participants: { 'someone@conf': {} }
		});
		const { api, sent } = fakeApi({ listed: occupied });
		const room = new JitsiRoom();
		room.attach(api);

		const removing = room.removeAll();
		await vi.advanceTimersByTimeAsync(5000);
		await removing;

		expect(sent.at(-1)).toEqual(['removeBreakoutRoom', 'one@breakout']);
		vi.useRealTimers();
	});

	it('falls back to the rooms it was told about when Jitsi will not list them', async () => {
		const { api, sent } = fakeApi({ listed: new Error('nope') });
		const room = new JitsiRoom();
		room.attach(api);
		room.setRooms(breakouts);

		await room.removeAll();

		expect(sent).toEqual([
			['closeBreakoutRoom', 'one'],
			['closeBreakoutRoom', 'two'],
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
