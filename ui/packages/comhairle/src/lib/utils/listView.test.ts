import { describe, expect, it } from 'vitest';
import { listViewFromUrl, listViewHref } from './listView';
const at = (search: string) => new URL(`https://example.com/admin/conversations${search}`);
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
