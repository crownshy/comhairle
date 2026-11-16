import { describe, expect, it, vi } from 'vitest';
import { backToFromUrl, pathWithQuery, safeBackTo } from './backTo';

const ORIGIN = 'https://comhairle.scot';

describe('safeBackTo', () => {
	it('keeps a relative path with its query and hash', () => {
		expect(safeBackTo('/conversations/abc?step=2#top', ORIGIN)).toBe(
			'/conversations/abc?step=2#top'
		);
	});

	it('turns a full URL on our own origin into its path', () => {
		expect(safeBackTo('https://comhairle.scot/admin/conversations?q=x', ORIGIN)).toBe(
			'/admin/conversations?q=x'
		);
	});

	it('goes home when there is no backTo', () => {
		expect(safeBackTo(null, ORIGIN)).toBe('/');
		expect(safeBackTo(undefined, ORIGIN)).toBe('/');
		expect(safeBackTo('', ORIGIN)).toBe('/');
	});

	it('goes home for another host', () => {
		expect(safeBackTo('https://evil.example/admin', ORIGIN)).toBe('/');
		expect(safeBackTo('//evil.example/admin', ORIGIN)).toBe('/');
		expect(safeBackTo('/\\evil.example/admin', ORIGIN)).toBe('/');
	});

	it('goes home for the same host on another scheme or port', () => {
		expect(safeBackTo('http://comhairle.scot/admin', ORIGIN)).toBe('/');
		expect(safeBackTo('https://comhairle.scot:8443/admin', ORIGIN)).toBe('/');
	});

	it('goes home for script links', () => {
		expect(safeBackTo('javascript:alert(1)', ORIGIN)).toBe('/');
		expect(safeBackTo('data:text/html,<script>alert(1)</script>', ORIGIN)).toBe('/');
	});

	it('goes home for a same-origin path that would read as another host', () => {
		expect(safeBackTo('https://comhairle.scot//evil.example/admin', ORIGIN)).toBe('/');
	});

	it('goes home for input that is not a URL at all', () => {
		expect(safeBackTo('http://', ORIGIN)).toBe('/');
		expect(safeBackTo('https://[bad', ORIGIN)).toBe('/');
	});

	it('reads a path without a leading slash as a path on our site', () => {
		expect(safeBackTo('conversations/abc', ORIGIN)).toBe('/conversations/abc');
	});

	it('ignores whitespace around the value', () => {
		expect(safeBackTo('  /conversations/abc ', ORIGIN)).toBe('/conversations/abc');
		expect(safeBackTo('\t//evil.example/admin', ORIGIN)).toBe('/');
	});

	it('keeps an encoded double slash as a harmless single segment', () => {
		expect(safeBackTo('/%2F%2Fevil.example', ORIGIN)).toBe('/%2F%2Fevil.example');
	});

	it('uses the given fallback instead of home', () => {
		const fallback = '/conversations/abc/preview';
		expect(safeBackTo(null, ORIGIN, fallback)).toBe(fallback);
		expect(safeBackTo('https://evil.example/x', ORIGIN, fallback)).toBe(fallback);
		expect(safeBackTo('/\\evil.example', ORIGIN, fallback)).toBe(fallback);
		expect(safeBackTo('/conversations/abc/step/1', ORIGIN, fallback)).toBe(
			'/conversations/abc/step/1'
		);
	});

	it('works in browsers without URL.parse', () => {
		const parse = vi.spyOn(URL, 'parse').mockImplementation(() => {
			throw new Error('URL.parse is not available');
		});
		expect(safeBackTo(`${ORIGIN}/admin/x`, ORIGIN)).toBe('/admin/x');
		parse.mockRestore();
	});
});

describe('backToFromUrl', () => {
	it('reads and cleans the backTo query param', () => {
		const url = new URL(
			`${ORIGIN}/auth/login?backTo=${encodeURIComponent(`${ORIGIN}/admin/x`)}`
		);
		expect(backToFromUrl(url)).toBe('/admin/x');
	});

	it('goes home when the param is missing', () => {
		expect(backToFromUrl(new URL(`${ORIGIN}/auth/login`))).toBe('/');
	});

	it('uses the given fallback when the param is missing or unsafe', () => {
		const fallback = '/conversations/abc/preview';
		expect(backToFromUrl(new URL(`${ORIGIN}/x/preview-unlock`), fallback)).toBe(fallback);
		expect(
			backToFromUrl(new URL(`${ORIGIN}/x/preview-unlock?backTo=//evil.example`), fallback)
		).toBe(fallback);
	});
});

describe('pathWithQuery', () => {
	it('keeps the path and query and drops the origin and hash', () => {
		expect(pathWithQuery(new URL(`${ORIGIN}/admin/conversations?q=x#top`))).toBe(
			'/admin/conversations?q=x'
		);
	});

	it('round-trips through safeBackTo unchanged', () => {
		const path = pathWithQuery(new URL(`${ORIGIN}/conversations/abc/step/2?lang=gd`));
		expect(safeBackTo(path, ORIGIN)).toBe(path);
	});
});
