import { describe, expect, it } from 'vitest';
import { filterConversations, statusFilterFromUrl, statusFilterHref } from './conversationFilters';

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
const sharedCompleted = conversation('shared-completed', false, true);

const owned = [ownedLive, ownedDraft, ownedCompleted];
const shared = [sharedDraft, sharedLive, sharedCompleted];

const ids = (conversations: { id: string }[]) => conversations.map(({ id }) => id);

describe('filterConversations', () => {
	it('shows owned and shared together under All', () => {
		expect(ids(filterConversations(owned, shared, 'all'))).toEqual([
			'owned-live',
			'owned-draft',
			'owned-completed',
			'shared-draft',
			'shared-live',
			'shared-completed'
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
		expect(ids(filterConversations(owned, shared, 'completed'))).toEqual([
			'owned-completed',
			'shared-completed'
		]);
	});

	it('shows only conversations shared with the user under Shared', () => {
		expect(ids(filterConversations(owned, shared, 'shared'))).toEqual([
			'shared-draft',
			'shared-live',
			'shared-completed'
		]);
	});
});

describe('statusFilterFromUrl', () => {
	const at = (search: string) => new URL(`https://example.com/admin/conversations${search}`);

	it.each(['live', 'draft', 'completed', 'shared'] as const)('reads ?status=%s', (filter) => {
		expect(statusFilterFromUrl(at(`?status=${filter}`))).toBe(filter);
	});

	it.each(['', '?status=banana', '?status=LIVE'])('falls back to All for %j', (search) => {
		expect(statusFilterFromUrl(at(search))).toBe('all');
	});
});

describe('statusFilterHref', () => {
	const url = new URL('https://example.com/admin/conversations?status=live&q=wind');

	it('swaps the status and keeps the rest of the query', () => {
		expect(statusFilterHref(url, 'draft')).toBe('/admin/conversations?status=draft&q=wind');
	});

	it('drops status for All so it keeps one URL', () => {
		expect(statusFilterHref(url, 'all')).toBe('/admin/conversations?q=wind');
	});
});
