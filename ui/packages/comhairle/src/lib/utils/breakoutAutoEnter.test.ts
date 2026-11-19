import { describe, expect, it } from 'vitest';
import { breakoutAutoEnterRoom } from './breakoutAutoEnter';

describe('breakoutAutoEnterRoom', () => {
	it('sends a participant to their assigned room', () => {
		expect(
			breakoutAutoEnterRoom({
				isModerator: false,
				assignedRoom: 2,
				moderatorAlreadyAutoEntered: false
			})
		).toBe(2);
	});

	it('sends an unassigned participant to room 0', () => {
		expect(
			breakoutAutoEnterRoom({
				isModerator: false,
				assignedRoom: null,
				moderatorAlreadyAutoEntered: false
			})
		).toBe(0);
	});

	it('sends a moderator to their assigned room the first time', () => {
		expect(
			breakoutAutoEnterRoom({
				isModerator: true,
				assignedRoom: 1,
				moderatorAlreadyAutoEntered: false
			})
		).toBe(1);
	});

	it('keeps a moderator in plenary after they have been auto-entered once', () => {
		expect(
			breakoutAutoEnterRoom({
				isModerator: true,
				assignedRoom: 1,
				moderatorAlreadyAutoEntered: true
			})
		).toBeNull();
	});

	it('keeps a moderator with no assigned room in plenary', () => {
		expect(
			breakoutAutoEnterRoom({
				isModerator: true,
				assignedRoom: null,
				moderatorAlreadyAutoEntered: false
			})
		).toBeNull();
	});
});
