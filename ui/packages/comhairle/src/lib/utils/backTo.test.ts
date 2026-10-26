import { describe, expect, it } from 'vitest';
import { normaliseBackTo } from './backTo';

describe('normaliseBackTo', () => {
	it('defaults to home when empty or unsafe', () => {
		expect(normaliseBackTo(null)).toBe('/');
		expect(normaliseBackTo('')).toBe('/');
		expect(normaliseBackTo('https://example.com/admin')).toBe('/');
		expect(normaliseBackTo('//example.com/admin')).toBe('/');
		expect(normaliseBackTo('/admin\\conversations')).toBe('/');
	});

	it('keeps safe internal paths', () => {
		expect(normaliseBackTo('/conversations/abc?tab=1')).toBe('/conversations/abc?tab=1');
		expect(normaliseBackTo('/admin')).toBe('/admin');
	});

	it('collapses repeated admin prefixes', () => {
		expect(normaliseBackTo('/admin/admin')).toBe('/admin');
		expect(normaliseBackTo('/admin/admin/conversations?x=1')).toBe('/admin/conversations?x=1');
		expect(normaliseBackTo('/admin/admin/admin/info#help')).toBe('/admin/info#help');
	});
});
