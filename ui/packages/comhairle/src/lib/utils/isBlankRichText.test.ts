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

	it('treats an empty TipTap doc as blank', () => {
		expect(
			isBlankRichText(
				'{"type":"doc","content":[{"type":"paragraph","attrs":{"textAlign":null}}]}'
			)
		).toBe(true);
		expect(isBlankRichText('{"type":"doc","content":[]}')).toBe(true);
	});

	it('sees text inside a TipTap doc', () => {
		const doc = {
			type: 'doc',
			content: [
				{ type: 'paragraph', content: [{ type: 'text', text: 'Votes are anonymous.' }] }
			]
		};
		expect(isBlankRichText(JSON.stringify(doc))).toBe(false);
	});

	it('sees text inside markup', () => {
		expect(isBlankRichText('<p>Your votes are anonymous.</p>')).toBe(false);
	});
});
