import { describe, expect, it } from 'vitest';
import { filterConversations, parseStatusFilter, statusFilterHref } from './conversationFilters';

const conversation = (id: string, isLive: boolean, isComplete: boolean) => ({
	id,
	isLive,
	isComplete
});

const ownedLive = conversation('owned-live', true, false);
const ownedDraft = conversation('owned-draft', false, false);
const ownedCompleted = conversation('owned-completed', true, true);
const sharedDraft = conversation('shared-draft', false, false);
const sharedLive = conversation('shared-live', true, false);

const owned = [ownedLive, ownedDraft, ownedCompleted];
const shared = [sharedDraft, sharedLive];

const ids = (conversations: { id: string }[]) => conversations.map(({ id }) => id);

describe('filterConversations', () => {
	it('shows owned and shared together under All', () => {
		expect(ids(filterConversations(owned, shared, 'all'))).toEqual([
			'owned-live',
			'owned-draft',
			'owned-completed',
			'shared-draft',
			'shared-live'
		]);
	});

	it('includes shared conversations in the status chips', () => {
		expect(ids(filterConversations(owned, shared, 'live'))).toEqual([
			'owned-live',
			'shared-live'
		]);
		expect(ids(filterConversations(owned, shared, 'draft'))).toEqual([
			'owned-draft',
			'shared-draft'
		]);
		expect(ids(filterConversations(owned, shared, 'completed'))).toEqual(['owned-completed']);
	});

	it('shows only conversations shared with the user under Shared', () => {
		expect(ids(filterConversations(owned, shared, 'shared'))).toEqual([
			'shared-draft',
			'shared-live'
		]);
	});
});

describe('parseStatusFilter', () => {
	it.each(['live', 'draft', 'completed', 'shared'] as const)('accepts %s', (value) => {
		expect(parseStatusFilter(value)).toBe(value);
	});

	it.each([null, '', 'banana', 'LIVE'])('falls back to all for %j', (value) => {
		expect(parseStatusFilter(value)).toBe('all');
	});
});

describe('statusFilterHref', () => {
	it('sets the status and keeps other params', () => {
		const url = new URL('https://example.test/admin?q=wind');
		expect(statusFilterHref(url, 'live')).toBe('/admin?q=wind&status=live');
	});

	it('drops the status param for All', () => {
		expect(statusFilterHref(new URL('https://example.test/admin?status=live'), 'all')).toBe(
			'/admin'
		);
	});
});
