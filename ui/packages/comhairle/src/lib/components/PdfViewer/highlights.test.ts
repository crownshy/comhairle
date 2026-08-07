import { describe, it, expect } from 'vitest';
import { highlightsFromPositions } from './highlights';

describe('highlightsFromPositions', () => {
	it('returns an empty array for missing or empty input', () => {
		expect(highlightsFromPositions(undefined)).toEqual([]);
		expect(highlightsFromPositions(null)).toEqual([]);
		expect(highlightsFromPositions([])).toEqual([]);
	});

	it('maps [page, x0, x1, top, bottom] to a rectangle', () => {
		// One RAGFlow line box from a real chunk payload.
		expect(highlightsFromPositions([[8, 71, 394, 469, 484]])).toEqual([
			{ page: 8, left: 71, top: 469, width: 323, height: 15 }
		]);
	});

	it('keeps the page number as-is (1-based, matching the viewer)', () => {
		const [h] = highlightsFromPositions([[1, 49, 430, 136, 223]]);
		expect(h.page).toBe(1);
	});

	it('skips malformed rows instead of throwing', () => {
		expect(
			highlightsFromPositions([
				[8, 71, 394],
				[2, 70, 500, 380, 395]
			])
		).toEqual([{ page: 2, left: 70, top: 380, width: 430, height: 15 }]);
	});

	it('clamps negative width/height to zero', () => {
		const [h] = highlightsFromPositions([[3, 400, 100, 500, 480]]);
		expect(h.width).toBe(0);
		expect(h.height).toBe(0);
	});
});
