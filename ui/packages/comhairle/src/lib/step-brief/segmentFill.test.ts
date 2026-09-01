import { describe, it, expect } from 'vitest';
import { segmentFill } from './segmentFill';

const cover = (slideIndex: number, slideCount: number) =>
	segmentFill({ phase: 'cover', slideIndex, slideCount });

const body = (toolProgress?: number) =>
	segmentFill({ phase: 'body', slideIndex: 0, slideCount: 1, toolProgress });

describe('segmentFill', () => {
	it('creeps across the cover without reaching the body', () => {
		expect(cover(0, 3)).toBeLessThan(cover(1, 3));
		expect(cover(1, 3)).toBeLessThan(cover(2, 3));
		expect(cover(2, 3)).toBeLessThanOrEqual(body(0));
	});

	it('hands over at the same point the cover ends', () => {
		expect(cover(0, 1)).toBeCloseTo(body(0));
	});

	it('holds at the handover point when a tool reports nothing', () => {
		expect(body(undefined)).toBeCloseTo(body(0));
	});

	it('fills completely when a tool reports done', () => {
		expect(body(1)).toBe(1);
	});

	it('clamps a tool reporting outside 0 to 1', () => {
		expect(body(5)).toBe(1);
		expect(body(-3)).toBeCloseTo(body(0));
		expect(body(NaN)).toBeCloseTo(body(0));
	});

	it('never returns a value outside 0 to 1', () => {
		const samples = [cover(0, 1), cover(9, 3), body(0), body(0.5), body(1)];
		for (const value of samples) {
			expect(value).toBeGreaterThanOrEqual(0);
			expect(value).toBeLessThanOrEqual(1);
		}
	});

	it('fills the segment on the completion screen', () => {
		expect(segmentFill({ phase: 'done', slideIndex: 0, slideCount: 3 })).toBe(1);
		expect(segmentFill({ phase: 'done', slideIndex: 0, slideCount: 3, toolProgress: 0 })).toBe(
			1
		);
	});

	it('survives a slide count of zero', () => {
		expect(cover(0, 0)).toBeGreaterThan(0);
		expect(cover(0, 0)).toBeLessThanOrEqual(1);
	});
});
