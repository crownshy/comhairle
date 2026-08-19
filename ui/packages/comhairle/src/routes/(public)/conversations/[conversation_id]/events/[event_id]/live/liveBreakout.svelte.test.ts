import { describe, it, expect } from 'vitest';
import { LiveBreakout } from './liveBreakout.svelte';

type Session = {
	isActive: boolean;
	endsAt: Date | null;
	isModerator: boolean;
	roomsReady: boolean;
	assignedRoomIndex: number | null;
};

function harness(overrides: Partial<Session> = {}) {
	const session = $state<Session>({
		isActive: false,
		endsAt: null,
		isModerator: false,
		roomsReady: false,
		assignedRoomIndex: null,
		...overrides
	});
	const joined: number[] = [];
	const notices: string[] = [];
	let returnedToPlenary = 0;
	let timeUps = 0;

	const breakout = new LiveBreakout(
		{
			isActive: () => session.isActive,
			endsAt: () => session.endsAt,
			isModerator: () => session.isModerator,
			roomsReady: () => session.roomsReady,
			assignedRoomIndex: () => session.assignedRoomIndex
		},
		{
			joinRoom: (roomIndex) => joined.push(roomIndex),
			returnToPlenary: () => returnedToPlenary++,
			notify: (message) => notices.push(message),
			timeUp: () => timeUps++
		}
	);

	return {
		breakout,
		session,
		joined,
		notices,
		get returnedToPlenary() {
			return returnedToPlenary;
		},
		get timeUps() {
			return timeUps;
		}
	};
}

/** A session with the given milliseconds left on the clock. */
const running = (msLeft: number): Partial<Session> => ({
	isActive: true,
	endsAt: new Date(Date.now() + msLeft)
});

describe('LiveBreakout rooms', () => {
	it('holds a participant in the plenary room until Jitsi has the rooms', () => {
		const live = harness({ isActive: true, assignedRoomIndex: 1 });

		live.breakout.syncRoom();
		expect(live.breakout.roomContext).toBe('plenary');
		expect(live.joined).toEqual([]);

		live.session.roomsReady = true;
		live.breakout.syncRoom();

		expect(live.breakout.inBreakoutRoom).toBe(true);
		expect(live.joined).toEqual([1]);
		expect(live.notices).toEqual([]);
	});

	it('puts a participant with no assignment in the first room', () => {
		const live = harness({ isActive: true, roomsReady: true });

		live.breakout.syncRoom();

		expect(live.joined).toEqual([0]);
	});

	it('tells a participant when the moderator moves them mid-session', () => {
		const live = harness({ isActive: true, roomsReady: true, assignedRoomIndex: 0 });
		live.breakout.syncRoom();

		live.session.assignedRoomIndex = 2;
		live.breakout.syncRoom();

		expect(live.joined).toEqual([0, 2]);
		expect(live.notices).toEqual(["You've been moved to Breakout room #3"]);
	});

	it('leaves a moderator in the plenary room until they enter one by hand', () => {
		const live = harness({
			isActive: true,
			roomsReady: true,
			isModerator: true,
			assignedRoomIndex: 0
		});

		live.breakout.syncRoom();
		expect(live.joined).toEqual([]);

		live.breakout.enterRoom(1);
		live.breakout.syncRoom();

		expect(live.joined).toEqual([1]);
		// A moderator picked the room, so there is nothing to tell them about it.
		expect(live.notices).toEqual([]);
	});

	it('returns everyone to the plenary room when the session ends', () => {
		const live = harness({ isActive: true, roomsReady: true, assignedRoomIndex: 0 });
		live.breakout.syncRoom();

		live.session.isActive = false;
		live.breakout.syncRoom();

		expect(live.breakout.roomContext).toBe('plenary');
		expect(live.returnedToPlenary).toBe(1);
	});

	it('sends nothing while the room it is asked for is the room we are in', () => {
		const live = harness({ isActive: true, roomsReady: true, assignedRoomIndex: 0 });

		live.breakout.syncRoom();
		live.breakout.syncRoom();

		expect(live.joined).toEqual([0]);
	});
});

describe('LiveBreakout closing sequence', () => {
	it('counts down the running session', () => {
		const live = harness(running(30_000));

		live.breakout.tick();

		expect(live.breakout.secondsLeft).toBe(30);
		expect(live.breakout.showEnding).toBe(false);
	});

	it('has no clock to report when no session is running', () => {
		const live = harness();

		live.breakout.tick();

		expect(live.breakout.timeRemaining).toBe(null);
		expect(live.breakout.secondsLeft).toBe(0);
	});

	it('puts the closing notice up in the last five seconds', () => {
		const live = harness(running(4000));

		live.breakout.tick();

		expect(live.breakout.showEnding).toBe(true);
		expect(live.timeUps).toBe(0);
	});

	it('keeps the notice down once it has been acknowledged', () => {
		const live = harness(running(4000));
		live.breakout.tick();

		live.breakout.dismissEnding();
		live.breakout.tick();

		expect(live.breakout.showEnding).toBe(false);
	});

	it('puts the notice back up when it was closed without acknowledging it', () => {
		const live = harness(running(4000));
		live.breakout.tick();

		live.breakout.showEnding = false;
		expect(live.breakout.showEnding).toBe(false);

		live.breakout.tick();
		expect(live.breakout.showEnding).toBe(true);
	});

	it('calls time once, on the way past zero', () => {
		const live = harness(running(4000));
		live.breakout.tick();

		live.session.endsAt = new Date(Date.now() - 1000);
		live.breakout.tick();
		live.breakout.tick();

		expect(live.timeUps).toBe(1);
	});

	it('forgets the session on reset', () => {
		const live = harness({ isActive: true, roomsReady: true, assignedRoomIndex: 1 });
		live.breakout.syncRoom();

		live.breakout.reset();

		expect(live.breakout.roomContext).toBe('plenary');
		expect(live.breakout.showEnding).toBe(false);
	});
});
