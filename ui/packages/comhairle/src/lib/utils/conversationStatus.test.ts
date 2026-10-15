import { describe, expect, it } from 'vitest';
import { conversationStatus } from './conversationStatus';

describe('conversationStatus', () => {
	it('treats a complete conversation as completed even while still flagged live', () => {
		expect(conversationStatus({ isLive: true, isComplete: true })).toBe('completed');
	});

	it('splits the rest into live and draft', () => {
		expect(conversationStatus({ isLive: true, isComplete: false })).toBe('live');
		expect(conversationStatus({ isLive: false, isComplete: false })).toBe('draft');
	});
});
