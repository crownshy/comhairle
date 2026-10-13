import { describe, expect, it } from 'vitest';
import { isBlankRichText } from './isBlankRichText';

describe('isBlankRichText', () => {
	it('treats missing text and empty markup as blank', () => {
		expect(isBlankRichText(null)).toBe(true);
		expect(isBlankRichText(undefined)).toBe(true);
		expect(isBlankRichText('<p></p>')).toBe(true);
		expect(isBlankRichText('<p>  </p>')).toBe(true);
		expect(isBlankRichText('<p>&nbsp;</p>')).toBe(true);
	});

	it('sees text inside markup', () => {
		expect(isBlankRichText('<p>Your votes are anonymous.</p>')).toBe(false);
	});
});
