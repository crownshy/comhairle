import { describe, expect, it } from 'vitest';
import { backToFromUrl, safeBackTo } from './backTo';

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
});
