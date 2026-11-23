// @vitest-environment jsdom
import { describe, it, expect, afterEach } from 'vitest';
import { flushSync } from 'svelte';
import type { VideoCallStatus } from '$lib/services/videoCallService.svelte';
import { MeetingAttendance, meetingPhaseFor } from './meetingAttendance.svelte';

let cleanups: (() => void)[] = [];

afterEach(() => {
	cleanups.forEach((cleanup) => cleanup());
	cleanups = [];
});

/** Keeps `phase` read on every change, the way the live page's markup does. */
function attendanceAt(initial: VideoCallStatus | null) {
	let status = $state<VideoCallStatus | null>(initial);
	const attendance = new MeetingAttendance(() => status);
	cleanups.push(
		$effect.root(() => {
			$effect(() => {
				void attendance.phase;
			});
		})
	);
	flushSync();
	return {
		attendance,
		setStatus(next: VideoCallStatus | null) {
			status = next;
			flushSync();
		}
	};
}

describe('meetingPhaseFor', () => {
	it('waits for the call status before showing anything', () => {
		expect(meetingPhaseFor({ callStatus: null, joined: false, left: false })).toBe('loading');
	});

	it('shows the ended screen only to people who were in the call', () => {
		expect(meetingPhaseFor({ callStatus: 'Ended', joined: true, left: false })).toBe('ended');
		expect(meetingPhaseFor({ callStatus: 'Ended', joined: false, left: false })).toBe('lobby');
	});

	it('shows the ended screen over the left screen once the meeting is over', () => {
		expect(meetingPhaseFor({ callStatus: 'Ended', joined: true, left: true })).toBe('ended');
	});

	it('shows the left screen while the meeting is still running', () => {
		expect(meetingPhaseFor({ callStatus: 'InProgress', joined: true, left: true })).toBe(
			'left'
		);
	});

	it('shows the call to people who joined and have not left', () => {
		expect(meetingPhaseFor({ callStatus: 'InProgress', joined: true, left: false })).toBe(
			'incall'
		);
	});
});

describe('MeetingAttendance', () => {
	it('puts everyone on the page into the call when the meeting starts', () => {
		const { attendance, setStatus } = attendanceAt('Waiting');
		expect(attendance.phase).toBe('lobby');

		setStatus('InProgress');

		expect(attendance.phase).toBe('incall');
	});

	it('puts someone arriving mid-meeting straight into the call', () => {
		const { attendance, setStatus } = attendanceAt(null);

		setStatus('InProgress');

		expect(attendance.phase).toBe('incall');
	});

	it('keeps the host in the call they opened while the meeting starts', () => {
		const { attendance, setStatus } = attendanceAt('Waiting');

		attendance.start();
		expect(attendance.phase).toBe('incall');
		setStatus('InProgress');

		expect(attendance.phase).toBe('incall');
	});

	it('shows the ended screen to everyone who was in the call when it ends', () => {
		const { attendance, setStatus } = attendanceAt('InProgress');

		setStatus('Ended');

		expect(attendance.joined).toBe(true);
		expect(attendance.phase).toBe('ended');
	});

	it('shows the lobby to someone arriving after the meeting ended', () => {
		const { attendance, setStatus } = attendanceAt(null);

		setStatus('Ended');

		expect(attendance.phase).toBe('lobby');
	});

	it('shows the left screen after hanging up, and the call again on rejoin', () => {
		const { attendance } = attendanceAt('InProgress');

		attendance.leave();
		expect(attendance.phase).toBe('left');

		attendance.rejoin();
		expect(attendance.phase).toBe('incall');
	});

	it('moves someone who left onto the ended screen when the meeting ends', () => {
		const { attendance, setStatus } = attendanceAt('InProgress');
		attendance.leave();

		setStatus('Ended');

		expect(attendance.phase).toBe('ended');
	});

	it('sends everyone back to the lobby when an ended meeting is reset to waiting', () => {
		const { attendance, setStatus } = attendanceAt('InProgress');
		attendance.leave();
		setStatus('Ended');

		setStatus('Waiting');
		expect(attendance.phase).toBe('lobby');

		setStatus('InProgress');
		expect(attendance.phase).toBe('incall');
	});
});
