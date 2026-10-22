import { describe, expect, it } from 'vitest';
import { hasInsights } from './insightsTools';

describe('hasInsights', () => {
	it('is true for the tools with an Insights page', () => {
		for (const type of ['polis', 'thinkingspace', 'prioritization', 'heyform']) {
			expect(hasInsights(type), type).toBe(true);
		}
	});

	it('is false for the tools without one and for a missing tool', () => {
		for (const type of ['learn', 'stories', 'elicitationbot', undefined, null]) {
			expect(hasInsights(type), String(type)).toBe(false);
		}
	});
});
