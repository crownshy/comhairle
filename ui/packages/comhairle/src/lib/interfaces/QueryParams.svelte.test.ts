import { describe, expect, it } from 'vitest';
import QueryParams from './QueryParams.svelte';

const Url = () => new URL('https://example.com/admin/conversations');

describe('Query params interface', () => {
	it('should get the query parameter correctly', () => {
		const url = Url();
		url.searchParams.set('filter', 'live');
		expect(QueryParams.get('filter', url)).toBe('live');
	});
	it('should set the parameter correctly', () => {
		const url = Url();
		QueryParams.set('filter', 'live', url);
		expect(url.searchParams.get('filter')).toBe('live');
	});
	it('should set multiple parameters correctly', () => {
		const url = Url();
		QueryParams.set('filter', 'live', url).set('page', '1');
		expect(url.searchParams.get('filter')).toBe('live');
		expect(url.searchParams.get('page')).toBe('1');
	});
	it('should remove a parameter if an empty string is supplied', () => {
		const url = Url();
		url.searchParams.set('filter', 'live');
		QueryParams.set('filter', '', url);
		expect(url.searchParams.get('filter')).toBe(null);
	});
	it('should not affect other parameters', () => {
		const url = Url();
		url.searchParams.set('filter', 'live');
		url.searchParams.set('search', 'conversation');
		QueryParams.set('filter', 'draft', url);
		expect(url.searchParams.get('filter')).toBe('draft');
		expect(url.searchParams.get('search')).toBe('conversation');
	});
	it('should remove parameters', () => {
		const url = Url();
		url.searchParams.set('filter', 'live');
		QueryParams.remove('filter', url);
		expect(url.searchParams.get('filter')).toBe(null);
	});
	it('should remove multiple parameters', () => {
		const url = Url();
		url.searchParams.set('filter', 'live');
		url.searchParams.set('search', 'conversation');
		QueryParams.remove('filter', url).remove('search');
		expect(url.searchParams.get('filter')).toBe(null);
		expect(url.searchParams.get('search')).toBe(null);
	});
});
