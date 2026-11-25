import { describe, it, expect } from 'vitest';
import { countWords, wordLimitDescribedBy, wordLimitIds } from './wordLimit';

describe('countWords', () => {
	it('counts words split by any whitespace', () => {
		expect(countWords('  one two\nthree\tfour ')).toBe(4);
	});

	it('returns 0 for blank text', () => {
		expect(countWords('   ')).toBe(0);
	});
});

describe('wordLimitDescribedBy', () => {
	const ids = wordLimitIds('answer');

	it('points at the count only while under the limit', () => {
		expect(wordLimitDescribedBy('answer', { countShown: true, overLimit: false })).toBe(
			ids.wordCount
		);
	});

	it('adds the over-limit message once over the limit', () => {
		expect(wordLimitDescribedBy('answer', { countShown: true, overLimit: true })).toBe(
			`${ids.wordCount} ${ids.overLimit}`
		);
	});

	it('is undefined when nothing is shown', () => {
		expect(wordLimitDescribedBy('answer', { countShown: false, overLimit: false })).toBe(
			undefined
		);
	});
});
