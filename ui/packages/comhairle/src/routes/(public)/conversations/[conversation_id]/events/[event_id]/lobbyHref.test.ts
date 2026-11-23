import { describe, expect, it } from 'vitest';
import { lobbyHref } from './lobbyHref';

const builtInLobby = '/conversations/conversation-1/events/event-1/live';

function hrefFor(customEventLink: string | null | undefined, format: 'online' | 'in_person') {
	return lobbyHref('conversation-1', { id: 'event-1', format, customEventLink });
}

describe('lobbyHref', () => {
	it('opens a valid custom link on an online event', () => {
		const link = 'https://meet.google.com/abc-defg-hij';
		expect(hrefFor(link, 'online')).toBe(link);
	});

	it('falls back to the built-in lobby when there is no custom link', () => {
		expect(hrefFor(null, 'online')).toBe(builtInLobby);
		expect(hrefFor(undefined, 'online')).toBe(builtInLobby);
		expect(hrefFor('', 'online')).toBe(builtInLobby);
	});

	it('falls back to the built-in lobby for links saved before validation', () => {
		expect(hrefFor('brainstorming-workshop', 'online')).toBe(builtInLobby);
		expect(hrefFor('http://meet.google.com/abc', 'online')).toBe(builtInLobby);
		expect(hrefFor('/some/page', 'online')).toBe(builtInLobby);
	});

	it('ignores the custom link on in-person events', () => {
		expect(hrefFor('https://meet.google.com/abc-defg-hij', 'in_person')).toBe(builtInLobby);
	});
});
