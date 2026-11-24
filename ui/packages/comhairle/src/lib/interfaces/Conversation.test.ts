import { describe, expect, it } from 'vitest';
import Conversation from './Conversation';

describe('Get the status correctly', () => {
	it('should return completed', () => {
		expect(Conversation.getStatus({ a: 1, b: 2, isLive: true, isComplete: true })).toBe(
			'completed'
		);
	});
	it('should return live', () => {
		expect(Conversation.getStatus({ a: 1, b: 2, isLive: true, isComplete: false })).toBe(
			'live'
		);
	});
	it('should return draft', () => {
		expect(Conversation.getStatus({ a: 1, b: 2, isLive: false, isComplete: false })).toBe(
			'draft'
		);
	});
});

describe('Get the params correctly', () => {
	it('should return correctly from live', () => {
		expect(Conversation.getParams('live')).toStrictEqual({ is_live: true, is_complete: false });
	});
	it('should return correctly from draft', () => {
		expect(Conversation.getParams('draft')).toStrictEqual({
			is_live: false,
			is_complete: false
		});
	});
	it('should return correctly from completed', () => {
		expect(Conversation.getParams('completed')).toStrictEqual({
			is_live: undefined,
			is_complete: false
		});
	});
});
