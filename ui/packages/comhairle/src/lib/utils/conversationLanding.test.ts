import { describe, expect, it } from 'vitest';
import { conversationLandingUrl } from './conversationLanding';

const BASE = '/admin/conversations/c1';

describe('conversationLandingUrl', () => {
	it('opens a draft on Configure', () => {
		expect(conversationLandingUrl({ id: 'c1', isLive: false, isComplete: false })).toBe(
			`${BASE}/configure`
		);
	});

	it('opens a live conversation on Monitor', () => {
		expect(conversationLandingUrl({ id: 'c1', isLive: true, isComplete: false })).toBe(
			`${BASE}/monitor`
		);
	});

	it('opens a completed conversation on its Report', () => {
		expect(conversationLandingUrl({ id: 'c1', isLive: true, isComplete: true })).toBe(
			`${BASE}/report`
		);
	});

	it('opens Configure for a completed conversation that never went live', () => {
		expect(conversationLandingUrl({ id: 'c1', isLive: false, isComplete: true })).toBe(
			`${BASE}/configure`
		);
	});
});
