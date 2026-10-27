import { describe, expect, it } from 'vitest';
import {
	conversationCountLabel,
	conversationListQueries,
	conversationNoun,
	isPageTurn,
	listViewFromUrl,
	listViewHref,
	searchFromUrl,
	searchHref,
	statusFilterFromUrl,
	statusFilterHref
} from './conversationFilters';

const at = (search: string) => new URL(`https://example.com/admin/conversations${search}`);

describe('statusFilterFromUrl', () => {
	it.each(['live', 'draft', 'completed', 'shared'] as const)('reads ?status=%s', (filter) => {
		expect(statusFilterFromUrl(at(`?status=${filter}`))).toBe(filter);
	});

	it.each(['', '?status=banana', '?status=LIVE'])('falls back to All for %j', (search) => {
		expect(statusFilterFromUrl(at(search))).toBe('all');
	});
});

describe('statusFilterHref', () => {
	const url = at('?status=live&q=wind&page=3');

	it('swaps the status, keeps the search and goes back to the first page', () => {
		expect(statusFilterHref(url, 'draft')).toBe('/admin/conversations?status=draft&q=wind');
	});

	it('drops status for All so it keeps one URL', () => {
		expect(statusFilterHref(url, 'all')).toBe('/admin/conversations?q=wind');
	});
});

describe('search in the URL', () => {
	it('reads ?q= and defaults to empty', () => {
		expect(searchFromUrl(at('?q=wind'))).toBe('wind');
		expect(searchFromUrl(at(''))).toBe('');
	});

	it('keeps the chip and goes back to the first page', () => {
		expect(searchHref(at('?status=live&page=2'), 'wind')).toBe(
			'/admin/conversations?status=live&q=wind'
		);
	});

	it('drops q for a blank search', () => {
		expect(searchHref(at('?status=live&q=wind'), '  ')).toBe(
			'/admin/conversations?status=live'
		);
	});
});

describe('conversationListQueries', () => {
	const userId = 'user-1';

	it('asks for everything under All', () => {
		expect(conversationListQueries('all', userId)).toEqual({});
	});

	it('keeps completed conversations out of Live and Draft', () => {
		expect(conversationListQueries('live', userId)).toEqual({
			is_live: true,
			is_complete: false
		});
		expect(conversationListQueries('draft', userId)).toEqual({
			is_live: false,
			is_complete: false
		});
		expect(conversationListQueries('completed', userId)).toEqual({ is_complete: true });
	});

	it('leaves out your own conversations under Shared', () => {
		expect(conversationListQueries('shared', userId)).toEqual({ exclude_owner_id: userId });
	});
});

describe('isPageTurn', () => {
	it('is true when only the page changes', () => {
		expect(isPageTurn(at('?status=live&q=wind'), at('?status=live&q=wind&page=2'))).toBe(true);
	});

	it('is false when the chip or search changes too', () => {
		expect(isPageTurn(at('?page=2'), at('?status=live'))).toBe(false);
		expect(isPageTurn(at('?q=wind&page=2'), at('?q=winds'))).toBe(false);
	});

	it('is false when the page stays the same', () => {
		expect(isPageTurn(at('?q=wind'), at('?q=winds'))).toBe(false);
	});
});

describe('listViewFromUrl', () => {
	it('shows cards unless ?view=list', () => {
		expect(listViewFromUrl(at(''))).toBe('card');
		expect(listViewFromUrl(at('?view=banana'))).toBe('card');
		expect(listViewFromUrl(at('?view=list'))).toBe('list');
	});
});

describe('listViewHref', () => {
	const url = at('?status=live&q=wind&page=3');

	it('keeps the filter, search and page', () => {
		expect(listViewHref(url, 'list')).toBe(
			'/admin/conversations?status=live&q=wind&page=3&view=list'
		);
	});

	it('drops view for cards so it keeps one URL', () => {
		expect(listViewHref(at('?view=list&page=2'), 'card')).toBe('/admin/conversations?page=2');
	});
});

describe('conversationNoun', () => {
	it('matches the count and the filter', () => {
		expect(conversationNoun('all', 1)).toBe('conversation');
		expect(conversationNoun('all', 2)).toBe('conversations');
		expect(conversationNoun('live', 1)).toBe('live conversation');
		expect(conversationNoun('shared', 0)).toBe('shared conversations');
	});
});

describe('conversationCountLabel', () => {
	it('says how many match a search', () => {
		expect(conversationCountLabel(1, 'all', 'wind')).toBe('1 conversation matches "wind"');
		expect(conversationCountLabel(4, 'draft', 'wind')).toBe(
			'4 draft conversations match "wind"'
		);
	});

	it('counts everything you have, or one filter', () => {
		expect(conversationCountLabel(3, 'all', '')).toBe('You have 3 conversations');
		expect(conversationCountLabel(1, 'completed', '')).toBe('1 completed conversation');
	});
});
