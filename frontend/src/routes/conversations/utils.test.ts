import { describe, it, expect } from 'vitest';

import { getSort, setSort } from './utils';

describe('conversation utils', () => {
	it('getSort extracts sort from URL', () => {
		const url = new URL('https://example.org/conversations?sort=title%2Bdesc');
		expect(getSort(url)).toEqual('title+desc');
	});

	it('setSort sets sort on URL with sort already set', () => {
		const oldUrl = new URL('https://example.org/conversations?sort=title%2Bdesc');
		const url = setSort(oldUrl, 'created_at+asc').toString();
		expect(url).toEqual('https://example.org/conversations?sort=created_at%2Basc');
	});

	it('setSort sets sort on URL without sort already set', () => {
		const oldUrl = new URL('https://example.org/conversations');
		const url = setSort(oldUrl, 'created_at+asc').toString();
		expect(url).toEqual('https://example.org/conversations?sort=created_at%2Basc');
	});
});
