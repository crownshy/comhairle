import { describe, expect, it } from 'vitest';
import { cardHref, hasReportContent, readCardNumber, splitReportIntoPages } from './reportPages';

const heading = (level: number, text: string) => ({
	type: 'heading',
	attrs: { level },
	content: [{ type: 'text', text }]
});
const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const rule = { type: 'horizontalRule' };
const embed = { type: 'reportComponentEmbed', attrs: { toolStepId: 'step', componentType: 'map' } };
const image = { type: 'image', attrs: { src: 'https://example.com/a.png' } };
const hardBreak = { type: 'hardBreak' };
const bulletList = (...items: string[]) => ({
	type: 'bulletList',
	content: items.map((item) => ({ type: 'listItem', content: [paragraph(item)] }))
});
const doc = (...content: unknown[]) => JSON.stringify({ type: 'doc', content });

const pageLabel = (pageNumber: number) => `Page ${pageNumber}`;
const labels = (content: string) =>
	splitReportIntoPages(content, pageLabel).map((page) => page.label);

describe('splitReportIntoPages', () => {
	it('starts a page at each page break', () => {
		expect(
			labels(doc(paragraph('One'), rule, paragraph('Two'), rule, paragraph('Three')))
		).toEqual(['One', 'Two', 'Three']);
	});

	it('names each page by its first line of text, whatever its style', () => {
		expect(
			labels(
				doc(heading(2, 'Overview'), paragraph('a'), rule, paragraph('What emerged'), embed)
			)
		).toEqual(['Overview', 'What emerged']);
	});

	it('keeps headings on the page they sit on', () => {
		expect(
			labels(
				doc(heading(2, 'Overview'), paragraph('a'), heading(2, 'Method'), heading(3, 'Who'))
			)
		).toEqual(['Overview']);
	});

	it('shows the whole page, first line included, as the card content', () => {
		const [page] = splitReportIntoPages(doc(heading(2, 'Overview'), paragraph('a')), pageLabel);
		expect(JSON.parse(page.content)).toEqual({
			type: 'doc',
			content: [heading(2, 'Overview'), paragraph('a')]
		});
	});

	it('names a page by the line before a hard break', () => {
		const twoLines = {
			type: 'paragraph',
			content: [
				{ type: 'text', text: 'Line one' },
				hardBreak,
				{ type: 'text', text: 'Line two' }
			]
		};
		expect(labels(doc(twoLines))).toEqual(['Line one']);
	});

	it('names a page that opens with a list by its first item', () => {
		expect(labels(doc(bulletList('First', 'Second')))).toEqual(['First']);
	});

	it('names a page that opens with an image by the text after it', () => {
		expect(labels(doc(paragraph('Intro'), rule, image, paragraph('Caption')))).toEqual([
			'Intro',
			'Caption'
		]);
	});

	it('ignores page breaks at the start and end', () => {
		expect(labels(doc(rule, paragraph('Only'), rule))).toEqual(['Only']);
	});

	it('numbers a page that has no text', () => {
		expect(labels(doc(paragraph('Intro'), rule, embed))).toEqual(['Intro', 'Page 2']);
	});

	it('keeps a page that holds only an image', () => {
		expect(labels(doc(paragraph('Intro'), rule, image))).toEqual(['Intro', 'Page 2']);
	});

	it('drops empty pages and numbers the pages that are left', () => {
		expect(labels(doc(paragraph(' '), rule, rule, embed, rule, paragraph('Last')))).toEqual([
			'Page 1',
			'Last'
		]);
	});

	it('shows a legacy plain text body as one page', () => {
		expect(labels('Some **markdown**')).toEqual(['Page 1']);
	});

	it('returns nothing for an empty body', () => {
		expect(labels('')).toEqual([]);
		expect(labels(doc(paragraph('')))).toEqual([]);
	});
});

describe('hasReportContent', () => {
	it('counts an embedded component as content', () => {
		expect(hasReportContent(doc(embed))).toBe(true);
	});

	it('counts an image, video, audio or embedded page as content', () => {
		for (const type of ['image', 'video', 'audio', 'iframe']) {
			expect(hasReportContent(doc({ type, attrs: { src: 'https://example.com/a' } }))).toBe(
				true
			);
		}
	});

	it('treats an empty document as blank', () => {
		expect(hasReportContent(doc(paragraph('')))).toBe(false);
		expect(hasReportContent(null)).toBe(false);
	});
});

describe('readCardNumber', () => {
	const at = (search: string) => new URL(`https://example.com/report${search}`);

	it('reads a card or the thank-you screen after the last card', () => {
		expect(readCardNumber(at('?card=2'), 3)).toBe(2);
		expect(readCardNumber(at('?card=4'), 3)).toBe(4);
	});

	it('falls back to the first screen for a missing or stale card', () => {
		expect(readCardNumber(at(''), 3)).toBeUndefined();
		expect(readCardNumber(at('?card=0'), 3)).toBeUndefined();
		expect(readCardNumber(at('?card=5'), 3)).toBeUndefined();
		expect(readCardNumber(at('?card=1.5'), 3)).toBeUndefined();
	});
});

describe('cardHref', () => {
	const url = new URL('https://example.com/report?lang=en&card=2');

	it('points at a card and keeps the other parameters', () => {
		expect(cardHref(url, 3)).toBe('/report?lang=en&card=3');
	});

	it('drops the card for the first screen', () => {
		expect(cardHref(url)).toBe('/report?lang=en');
	});
});
