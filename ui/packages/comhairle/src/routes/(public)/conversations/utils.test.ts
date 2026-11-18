import { describe, it, expect } from 'vitest';

import { getSort, setSort, getSearch, setSearch, getStatusFilter, statusFilterHref } from './utils';

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

	it('getSearch extracts search from URL', () => {
		const url = new URL('https://example.org/conversations?sort=title%2Bdesc&search=foo');
		expect(getSearch(url)).toEqual('foo');
	});

	it('getSearch extracts empty search from URL', () => {
		const url = new URL('https://example.org/conversations?sort=title%2Bdesc');
		expect(getSearch(url)).toEqual('');
	});

	it('setSearch updates search in url', () => {
		const oldUrl = new URL('https://example.org/conversations?sort=title%2Bdesc&search=foo');
		const url = setSearch(oldUrl, 'foobar');
		expect(url.toString()).toEqual(
			'https://example.org/conversations?sort=title%2Bdesc&search=foobar'
		);
		expect(getSearch(url)).toEqual('foobar');
	});

	it('setSearch deletes search param if empty', () => {
		const oldUrl = new URL('https://example.org/conversations?sort=title%2Bdesc&search=foo');
		const url = setSearch(oldUrl, '');
		expect(url.toString()).toEqual('https://example.org/conversations?sort=title%2Bdesc');
		expect(getSearch(url)).toEqual('');
	});
});

describe('status filter', () => {
	const at = (search: string) => new URL(`https://example.com/${search}`);

	it('defaults to open, including for unknown values', () => {
		expect(getStatusFilter(at(''))).toBe('open');
		expect(getStatusFilter(at('?status=banana'))).toBe('open');
		expect(getStatusFilter(at('?status=ended'))).toBe('ended');
	});

	it('keeps search and view, drops page, and drops status for open', () => {
		expect(statusFilterHref(at('?search=bus&view=list&page=2'), 'ended')).toBe(
			'/?search=bus&view=list&status=ended'
		);
		expect(statusFilterHref(at('?status=ended&page=2'), 'open')).toBe('/');
	});
});
