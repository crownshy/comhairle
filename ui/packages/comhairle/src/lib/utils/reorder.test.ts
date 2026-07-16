import { describe, it, expect } from 'vitest';
import { moveItem } from './reorder';

describe('moveItem', () => {
	it('moves an item towards the start', () => {
		expect(moveItem(['a', 'b', 'c'], 1, -1)).toEqual(['b', 'a', 'c']);
	});

	it('moves an item towards the end', () => {
		expect(moveItem(['a', 'b', 'c'], 1, 1)).toEqual(['a', 'c', 'b']);
	});

	it('is a no-op moving the first item up', () => {
		expect(moveItem(['a', 'b', 'c'], 0, -1)).toEqual(['a', 'b', 'c']);
	});

	it('is a no-op moving the last item down', () => {
		expect(moveItem(['a', 'b', 'c'], 2, 1)).toEqual(['a', 'b', 'c']);
	});

	it('is a no-op for an out-of-range index', () => {
		expect(moveItem(['a', 'b'], 5, -1)).toEqual(['a', 'b']);
	});

	it('returns a new array, not the input', () => {
		const input = ['a', 'b'];
		const out = moveItem(input, 0, 1);
		expect(out).not.toBe(input);
		expect(input).toEqual(['a', 'b']);
	});
});
