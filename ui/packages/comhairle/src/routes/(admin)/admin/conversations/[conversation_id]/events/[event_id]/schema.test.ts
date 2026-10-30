import { describe, expect, it } from 'vitest';
import EventSchema from './schema';

const validEvent = {
	name: 'Brainstorming',
	description: 'A session to gather ideas together',
	capacity: 10,
	default_time_zone: 'Europe/London',
	start_date: '2999-01-01',
	start_time: '10:00',
	end_time: '11:00',
	signup_mode: 'invite',
	format: 'online'
};

function linkIsAccepted(custom_event_link: string, format = 'online') {
	return EventSchema.safeParse({ ...validEvent, format, custom_event_link }).success;
}

describe('custom event link', () => {
	it('accepts an empty link or a full https URL', () => {
		expect(linkIsAccepted('')).toBe(true);
		expect(linkIsAccepted('https://meet.google.com/abc-defg-hij')).toBe(true);
	});

	it('rejects anything a browser would not open as an external link', () => {
		expect(linkIsAccepted('brainstorming-workshop')).toBe(false);
		expect(linkIsAccepted('meet.google.com/abc')).toBe(false);
		expect(linkIsAccepted('http://meet.google.com/abc')).toBe(false);
	});

	it('ignores the link on in-person events, where the field is disabled', () => {
		expect(linkIsAccepted('brainstorming-workshop', 'in_person')).toBe(true);
	});
});
