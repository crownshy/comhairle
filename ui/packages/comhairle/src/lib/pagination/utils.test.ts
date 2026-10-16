import { describe, it, expect } from 'vitest';

import { getPage, setPage, calcOffset, calcPage, calcPageCount, pageItems } from './utils';

describe('pagination', () => {
	it('getPage defaults to page 1', () => {
		const url = new URL('https://example.org/conversations?sort=title%2Bdesc');
		expect(getPage(url)).toEqual(1);
	});

	it('getPage extracts page number from URL', () => {
		const url = new URL('https://example.org/conversations?page=7&sort=title%2Bdesc');
		expect(getPage(url)).toEqual(7);
	});

	it('getPage falls back to page 1 for zero, negative and non-numeric pages', () => {
		for (const page of ['0', '-3', 'abc', 'Infinity']) {
			const url = new URL(`https://example.org/conversations?page=${page}`);
			expect(getPage(url)).toEqual(1);
		}
	});

	it('getPage rounds fractional pages down', () => {
		const url = new URL('https://example.org/conversations?page=2.5');
		expect(getPage(url)).toEqual(2);
	});

	it('setPage sets page number in URL', () => {
		const oldUrl = new URL('https://example.org/conversations?page=7&sort=title%2Bdesc');
		const url = setPage(oldUrl, 8);
		expect(url.toString()).toEqual(
			'https://example.org/conversations?page=8&sort=title%2Bdesc'
		);
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

	describe('pageItems', () => {
		const render = (current: number, pageCount: number) =>
			pageItems({ current, pageCount })
				.map((item) => (item.type === 'page' ? String(item.page) : '...'))
				.join(' ');

		it('shows every page when there are only a few', () => {
			expect(render(1, 1)).toEqual('1');
			expect(render(3, 4)).toEqual('1 2 3 4');
			expect(render(3, 5)).toEqual('1 2 3 4 5');
		});

		it('collapses distant pages into ellipses', () => {
			expect(render(1, 12)).toEqual('1 2 ... 12');
			expect(render(6, 12)).toEqual('1 ... 5 6 7 ... 12');
			expect(render(12, 12)).toEqual('1 ... 11 12');
		});

		it('fills a one-page gap rather than showing an ellipsis', () => {
			expect(render(4, 12)).toEqual('1 2 3 4 5 ... 12');
			expect(render(9, 12)).toEqual('1 ... 8 9 10 11 12');
		});
	});
});
