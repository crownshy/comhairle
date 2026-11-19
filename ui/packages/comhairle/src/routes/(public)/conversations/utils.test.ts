import { describe, it, expect } from 'vitest';

import { getSearch, setSearch } from './utils';

describe('conversation utils', () => {
	it('getSearch extracts search from URL', () => {
		const url = new URL('https://example.org/conversations?sort=title%2Bdesc&search=foo');
		expect(getSearch(url)).toEqual('foo');
	});

	it('getSearch extracts empty search from URL', () => {
		const url = new URL('https://example.org/conversations');
		expect(getSearch(url)).toEqual('');
	});

	it('setSearch updates search in url', () => {
		const oldUrl = new URL('https://example.org/conversations?search=foo');
		const url = setSearch(oldUrl, 'foobar');
		expect(url.toString()).toEqual('https://example.org/conversations?search=foobar');
		expect(getSearch(url)).toEqual('foobar');
	});

	it('setSearch deletes search param if empty', () => {
		const oldUrl = new URL('https://example.org/conversations?search=foo');
		const url = setSearch(oldUrl, '');
		expect(url.toString()).toEqual('https://example.org/conversations');
		expect(getSearch(url)).toEqual('');
	});
});
