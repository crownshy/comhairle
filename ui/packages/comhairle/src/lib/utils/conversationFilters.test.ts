import { describe, expect, it } from 'vitest';
import {
	filterConversations,
	filterFromSegment,
	filterToSegment,
	isConversationFilterSegment
} from './conversationFilters';

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

describe('isConversationFilterSegment', () => {
	it.each(['live', 'draft', 'completed', 'shared'])('accepts %s', (value) => {
		expect(isConversationFilterSegment(value)).toBe(true);
	});

	it.each(['all', '', 'banana', 'LIVE'])('rejects %j so All keeps one URL', (value) => {
		expect(isConversationFilterSegment(value)).toBe(false);
	});
});

describe('filter segments', () => {
	it('gives All the bare list route', () => {
		expect(filterToSegment('all')).toBeUndefined();
		expect(filterFromSegment(undefined)).toBe('all');
	});

	it.each(['live', 'draft', 'completed', 'shared'] as const)('round-trips %s', (filter) => {
		expect(filterFromSegment(filterToSegment(filter))).toBe(filter);
	});

	it('falls back to All for an unknown segment', () => {
		expect(filterFromSegment('banana')).toBe('all');
	});
});
