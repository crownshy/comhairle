import { describe, it, expect } from 'vitest';

import { getPage, setPage, calcOffset, calcPage, calcPageCount } from './utils';

describe('pagination', () => {
	it('getPage defaults to page 1', () => {
		const url = new URL('https://example.org/conversations?sort=title%2Bdesc');
		expect(getPage(url)).toEqual(1);
	});

	it('getPage extracts page number from URL', () => {
		const url = new URL('https://example.org/conversations?page=7&sort=title%2Bdesc');
		expect(getPage(url)).toEqual(7);
	});

	it('setPage sets page number in URL', () => {
		const oldUrl = new URL('https://example.org/conversations?page=7&sort=title%2Bdesc');
		const url = setPage(oldUrl, 8);
		expect(url.toString()).toEqual('https://example.org/conversations?page=8&sort=title%2Bdesc');
	});

	it('setPage unsets the page search param on 1', () => {
		const oldUrl = new URL('https://example.org/conversations?page=2&sort=title%2Bdesc');
		const url = setPage(oldUrl, 1);
		expect(url.toString()).toEqual('https://example.org/conversations?sort=title%2Bdesc');
	});

	it('calcOffset calculates offset from pages', () => {
		expect(calcOffset({ page: 1, pageSize: 6 })).toEqual(0);
		expect(calcOffset({ page: 2, pageSize: 6 })).toEqual(6);
		expect(calcOffset({ page: 3, pageSize: 6 })).toEqual(12);
	});

	it('calcPage calculated pages from offset', () => {
		expect(calcPage({ offset: 0, pageSize: 6 }), 'page defaults to 1').toEqual(1);
		expect(calcPage({ offset: 6, pageSize: 6 }), 'first page').toEqual(1);
		expect(calcPage({ offset: 12, pageSize: 6 }), '2nd page').toEqual(2);
		expect(calcPage({ offset: 13, pageSize: 6 }), 'handles remainder').toEqual(2);
	});

	it('calcPageCount calculates pages from total and page size', () => {
		expect(calcPageCount({ pageSize: 6, total: 13 }), 'one item on 3rd page').toEqual(3);
		expect(calcPageCount({ pageSize: 6, total: 12 }), 'exactly 2 pages').toEqual(2);
		expect(calcPageCount({ pageSize: 6, total: 0 }), '0 items').toEqual(1);
		expect(calcPageCount({ pageSize: 6, total: 1 }), '1 item').toEqual(1);
	});
});
