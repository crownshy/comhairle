import { untrack } from 'svelte';
import type { VideoCallStatus } from '$lib/services/videoCallService.svelte';

export type MeetingPhase = 'loading' | 'lobby' | 'incall' | 'left' | 'ended';

type PhaseInputs = {
	callStatus: VideoCallStatus | null;
	joined: boolean;
	left: boolean;
};

/** Which screen the live page shows. Only people who were in the call get the ended screen. */
export function meetingPhaseFor({ callStatus, joined, left }: PhaseInputs): MeetingPhase {
	if (callStatus === null) return 'loading';
	if (!joined) return 'lobby';
	if (callStatus === 'Ended') return 'ended';
	if (left) return 'left';
	return 'incall';
}

function hasMeetingStarted(callStatus: VideoCallStatus | null): boolean {
	return callStatus === 'InProgress' || callStatus === 'Ended';
}

/**
 * Whether this person is in the current run of the meeting, and whether they left it with
 * Jitsi's own hang-up button. Ending the meeting does not clear `joined`, so everyone who
 * was in the call lands on the ended screen. A reset back to waiting clears both.
 */
export class MeetingAttendance {
	#callStatus: () => VideoCallStatus | null;
	#started = $derived.by(() => hasMeetingStarted(this.#callStatus()));

	// Recomputes only when the meeting starts or goes back to waiting, never when it ends.
	// Someone arriving mid-meeting is put straight in; the host writes true to start it.
	joined = $derived.by(() => this.#started && untrack(() => this.#callStatus()) === 'InProgress');

	left = $derived.by(() => {
		void this.#callStatus();
		return false;
	});

	phase = $derived.by(() =>
		meetingPhaseFor({ callStatus: this.#callStatus(), joined: this.joined, left: this.left })
	);

	constructor(callStatus: () => VideoCallStatus | null) {
		this.#callStatus = callStatus;
	}

	/** The host opens the call before telling everyone it has started. */
	start() {
		this.joined = true;
	}

	/** Jitsi's own hang-up button was pressed. */
	leave() {
		this.left = true;
	}

	rejoin() {
		this.left = false;
	}

	reset() {
		this.joined = false;
		this.left = false;
	}
}
