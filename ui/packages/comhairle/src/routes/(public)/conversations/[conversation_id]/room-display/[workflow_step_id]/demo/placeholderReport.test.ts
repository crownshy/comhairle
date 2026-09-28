import { describe, it, expect } from 'vitest';
import { buildPlaceholderComments } from './placeholderReport';
import { PLACEHOLDER_STATEMENTS } from './placeholderStatements';

const comments = buildPlaceholderComments(PLACEHOLDER_STATEMENTS);

describe('buildPlaceholderComments', () => {
	it('gives every statement its own divisiveness, so the swarm spreads', () => {
		const scores = comments.map((c) => c.divisiveness);
		expect(new Set(scores).size).toBe(comments.length);
	});

	it('spreads scores across the range rather than clumping on a few columns', () => {
		const scores = comments.map((c) => c.divisiveness as number).sort((a, b) => a - b);
		// Ten buckets across the observed range; a healthy swarm fills most of them.
		const min = scores[0];
		const max = scores[scores.length - 1];
		const buckets = new Set(
			scores.map((s) => Math.min(9, Math.floor(((s - min) / (max - min)) * 10)))
		);
		expect(buckets.size).toBeGreaterThanOrEqual(7);
	});

	it('keeps scores inside the range Polis extremity actually occupies', () => {
		for (const c of comments) {
			expect(c.divisiveness).toBeGreaterThan(0);
			expect(c.divisiveness).toBeLessThan(6);
		}
	});

	it('makes the group split track divisiveness, so map and plot agree', () => {
		const sorted = [...comments].sort((a, b) => (a.divisiveness ?? 0) - (b.divisiveness ?? 0));
		const gap = (c: (typeof comments)[number]) =>
			Math.abs((c.group_votes[0]?.agrees ?? 0) - (c.group_votes[1]?.agrees ?? 0));
		expect(gap(sorted[0])).toBeLessThan(gap(sorted[sorted.length - 1]));
	});

	it('scores consensus inversely to divisiveness', () => {
		const sorted = [...comments].sort((a, b) => (a.divisiveness ?? 0) - (b.divisiveness ?? 0));
		expect(sorted[0].group_informed_consensus).toBeGreaterThan(
			sorted[sorted.length - 1].group_informed_consensus as number
		);
	});

	it('is deterministic', () => {
		expect(buildPlaceholderComments(PLACEHOLDER_STATEMENTS)).toEqual(comments);
	});
});
