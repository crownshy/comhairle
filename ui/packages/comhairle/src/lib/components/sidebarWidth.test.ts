import { describe, it, expect } from 'vitest';
import {
	clampWidth,
	parseWidthCookie,
	serializeWidthCookie,
	DEFAULT_WIDTH,
	MIN_WIDTH,
	MAX_WIDTH,
	WIDTH_COOKIE_NAME
} from './sidebarWidth';

describe('clampWidth', () => {
	it('passes an in-range width through unchanged', () => {
		expect(clampWidth(300)).toBe(300);
	});

	it('clamps below MIN_WIDTH up to MIN_WIDTH', () => {
		expect(clampWidth(0)).toBe(MIN_WIDTH);
		expect(clampWidth(MIN_WIDTH - 50)).toBe(MIN_WIDTH);
	});

	it('clamps above MAX_WIDTH down to MAX_WIDTH', () => {
		expect(clampWidth(9999)).toBe(MAX_WIDTH);
	});
});

describe('parseWidthCookie', () => {
	it('falls back to DEFAULT_WIDTH when the cookie is absent', () => {
		expect(parseWidthCookie(undefined)).toBe(DEFAULT_WIDTH);
		expect(parseWidthCookie(null)).toBe(DEFAULT_WIDTH);
	});

	it('falls back to DEFAULT_WIDTH for a non-numeric value', () => {
		expect(parseWidthCookie('not-a-number')).toBe(DEFAULT_WIDTH);
	});

	it('parses and returns an in-range numeric value', () => {
		expect(parseWidthCookie('320')).toBe(320);
	});

	it('re-clamps an out-of-range (tampered) value', () => {
		expect(parseWidthCookie('99999')).toBe(MAX_WIDTH);
		expect(parseWidthCookie('10')).toBe(MIN_WIDTH);
	});
});

describe('serializeWidthCookie', () => {
	it('writes the clamped width under the width cookie name with path and max-age', () => {
		const cookie = serializeWidthCookie(320);
		expect(cookie).toContain(`${WIDTH_COOKIE_NAME}=320`);
		expect(cookie).toContain('path=/');
		expect(cookie).toContain('max-age=');
		expect(cookie).toContain('SameSite=Lax');
	});

	it('clamps before serialising so an out-of-range width is never written', () => {
		expect(serializeWidthCookie(99999)).toContain(`${WIDTH_COOKIE_NAME}=${MAX_WIDTH}`);
	});
});
