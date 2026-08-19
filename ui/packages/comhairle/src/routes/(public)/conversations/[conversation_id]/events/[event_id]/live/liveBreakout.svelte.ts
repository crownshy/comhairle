import type { RoomContext } from '$lib/components/LiveEvent/types';

/** How long before the end we put the closing notice up. */
const ENDING_WARNING_MS = 5000;

/** Everything the machine reads. Getters, so the page keeps owning the sources. */
export type LiveBreakoutInputs = {
	/** Whether the backend says a breakout session is running. */
	isActive: () => boolean;
	/** When the running session ends, null when none is running. */
	endsAt: () => Date | null;
	/** Whether this client is hosting. Hosts are never placed in a room for them. */
	isModerator: () => boolean;
	/** Whether Jitsi has caught up with the rooms the session expects. */
	roomsReady: () => boolean;
	/** The room the backend has put this client in, null when it has not said. */
	assignedRoomIndex: () => number | null;
};

/** Everything the machine does outside itself. */
export type LiveBreakoutHandlers = {
	/** Move this client into the nth Jitsi breakout room. */
	joinRoom: (roomIndex: number) => void;
	/** Move this client back into the main Jitsi room. */
	returnToPlenary: () => void;
	/** Tell this client about something that happened to them. */
	notify: (message: string) => void;
	/** The session has run out of time. */
	timeUp: () => void;
};

/** How far a running session has got through its closing sequence. */
type EndingStage = 'inactive' | 'running' | 'warned' | 'dismissed';

const breakoutRoom = (roomIndex: number): RoomContext => ({
	type: 'breakout',
	roomIndex,
	roomName: `Breakout room #${roomIndex + 1}`
});

/**
 * The live event page's breakout state machine: which room this client is in, how long
 * the session has left, and how a session closes.
 *
 * This used to be five effects that read each other's writes, one of them wrapped in
 * `untrack` to stop it looping. Everything that follows from the session is derived here
 * and everything else arrives as a named transition, which leaves the page two effects:
 * the one that keeps Jitsi in step, and the clock the closing sequence runs on.
 */
export class LiveBreakout {
	#inputs: LiveBreakoutInputs;
	#handlers: LiveBreakoutHandlers;

	#now = $state(Date.now());

	/**
	 * The room Jitsi was last told to put us in. Deliberately not reactive: it is
	 * bookkeeping for `syncRoom`, and as state it would make that effect watch its own
	 * writes, which is what the `untrack` here used to be for.
	 */
	#joinedRoomIndex: number | null = null;

	/** Time left at the previous tick, so the crossing to zero is spotted once. */
	#previousRemaining: number | null = null;

	/**
	 * The room this client is in.
	 *
	 * Participants do not choose: the moderator's assignment puts them in a room as soon
	 * as Jitsi has one, and being moved mid-session is that same derivation running
	 * again. Moderators stay in the plenary room until they enter one by hand. Either
	 * way `enterRoom` and `leaveRoom` write over this, and the write holds until the
	 * session or the assignment moves under it.
	 */
	roomContext: RoomContext = $derived.by(() => {
		if (!this.#inputs.isActive() || !this.#inputs.roomsReady()) return 'plenary';
		if (this.#inputs.isModerator()) return 'plenary';
		// No assignment means the mock rooms used in dev, which are all room 0.
		return breakoutRoom(this.#inputs.assignedRoomIndex() ?? 0);
	});

	/** Milliseconds left in the running session, null when none is running. */
	timeRemaining: number | null = $derived.by(() => {
		const endsAt = this.#inputs.endsAt();
		if (!endsAt) return null;
		return Math.max(0, endsAt.getTime() - this.#now);
	});

	/** Whole seconds left, for the closing notice. */
	secondsLeft: number = $derived(
		this.timeRemaining === null ? 0 : Math.ceil(this.timeRemaining / 1000)
	);

	/** Derived, so a session starting or ending resets the sequence on its own. */
	#endingStage: EndingStage = $derived.by(() =>
		this.#inputs.isActive() ? 'running' : 'inactive'
	);

	constructor(inputs: LiveBreakoutInputs, handlers: LiveBreakoutHandlers) {
		this.#inputs = inputs;
		this.#handlers = handlers;
	}

	get inBreakoutRoom(): boolean {
		return this.roomContext !== 'plenary';
	}

	/** Whether the closing notice is up. */
	get showEnding(): boolean {
		return this.#endingStage === 'warned';
	}

	/**
	 * Dismissing the notice any other way than "Go back" hides it until the next tick
	 * puts it up again, which is what the effect this replaced did.
	 */
	set showEnding(open: boolean) {
		if (!open && this.#endingStage === 'warned') this.#endingStage = 'running';
	}

	/** This client asked to be in the nth breakout room. */
	enterRoom(roomIndex: number) {
		this.roomContext = breakoutRoom(roomIndex);
	}

	/** This client is back in the plenary room. */
	leaveRoom() {
		this.roomContext = 'plenary';
	}

	/** The moderator moved this client while the session was running. */
	moderatorMovedUs(roomIndex: number) {
		this.#handlers.notify(`You've been moved to Breakout room #${roomIndex + 1}`);
	}

	/** The session is nearly over. */
	sessionEnding() {
		if (this.#endingStage === 'running') this.#endingStage = 'warned';
	}

	/** The session is over. */
	sessionEnded() {
		this.#handlers.timeUp();
	}

	/** The user acknowledged the closing notice, so it stays down. */
	dismissEnding() {
		this.#endingStage = 'dismissed';
	}

	/**
	 * Put the call in step with `roomContext`. Call it from an `$effect`: reading
	 * `roomContext` is the dependency, and what it compares that against is plain
	 * bookkeeping, so nothing here writes to what the effect is watching.
	 */
	syncRoom() {
		const context = this.roomContext;
		const target = context === 'plenary' ? null : context.roomIndex;
		const previous = this.#joinedRoomIndex;
		if (target === previous) return;

		this.#joinedRoomIndex = target;
		if (target === null) {
			this.#handlers.returnToPlenary();
			return;
		}
		this.#handlers.joinRoom(target);
		// A participant never picks their own room, so changing rooms mid-session is the
		// moderator having moved them.
		if (previous !== null && !this.#inputs.isModerator()) this.moderatorMovedUs(target);
	}

	/**
	 * Start the clock the closing sequence runs on. Returns the teardown, so the page can
	 * hand it straight to an `$effect`. Nothing reactive is read here and the first tick
	 * waits for the interval: a read would make that effect watch the clock it starts.
	 */
	startClock(): () => void {
		const clock = setInterval(() => this.tick(), 1000);
		return () => clearInterval(clock);
	}

	/** Advance the clock. Public so tests can drive it without waiting a second. */
	tick() {
		const previous = this.#previousRemaining;
		this.#now = Date.now();
		const remaining = this.timeRemaining;
		this.#previousRemaining = remaining;

		if (!this.#inputs.isActive() || remaining === null) return;
		if (remaining > 0) {
			if (remaining <= ENDING_WARNING_MS) this.sessionEnding();
			return;
		}
		// Only on the crossing: a session sits at zero until the backend closes it.
		if (previous === null || previous > 0) this.sessionEnded();
	}

	/** Forget the session, for the dev reset. */
	reset() {
		this.roomContext = 'plenary';
		this.#endingStage = 'inactive';
		this.#joinedRoomIndex = null;
		this.#previousRemaining = null;
	}
}
