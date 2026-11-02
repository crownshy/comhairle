import { describe, expect, it } from 'vitest';
import {
	cardHref,
	hasReportContent,
	readCardNumber,
	splitReportIntoSections,
	toReportPages
} from './reportPages';

const heading = (level: number, text: string) => ({
	type: 'heading',
	attrs: { level },
	content: [{ type: 'text', text }]
});
const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const rule = { type: 'horizontalRule' };
const embed = { type: 'reportComponentEmbed', attrs: { toolStepId: 'step', componentType: 'map' } };
const doc = (...content: unknown[]) => JSON.stringify({ type: 'doc', content });

const shape = (content: string) =>
	splitReportIntoSections(content, 'Introduction').map((section) => ({
		label: section.label,
		titles: section.cards.map((card) => card.title)
	}));

describe('splitReportIntoSections', () => {
	it('starts a section at each H2 and titles its first card with it', () => {
		expect(
			shape(
				doc(
					heading(2, 'Overview'),
					paragraph('a'),
					heading(2, 'Methodology'),
					paragraph('b')
				)
			)
		).toEqual([
			{ label: 'Overview', titles: ['Overview'] },
			{ label: 'Methodology', titles: ['Methodology'] }
		]);
	});

	it('starts a card titled by each H3', () => {
		expect(
			shape(
				doc(
					heading(2, 'What emerged'),
					paragraph('a'),
					heading(3, 'Common ground'),
					paragraph('b'),
					heading(3, 'Where opinions differ'),
					embed
				)
			)
		).toEqual([
			{
				label: 'What emerged',
				titles: ['What emerged', 'Common ground', 'Where opinions differ']
			}
		]);
	});

	it('starts a card that keeps the section title at each horizontal rule', () => {
		expect(shape(doc(heading(2, 'Overview'), paragraph('a'), rule, paragraph('b')))).toEqual([
			{ label: 'Overview', titles: ['Overview', 'Overview'] }
		]);
	});

	it('puts content before the first H2 in an introduction section', () => {
		expect(shape(doc(paragraph('a'), heading(2, 'Overview'), paragraph('b')))).toEqual([
			{ label: 'Introduction', titles: ['Introduction'] },
			{ label: 'Overview', titles: ['Overview'] }
		]);
	});

	it('drops empty cards and sections', () => {
		expect(
			shape(
				doc(
					heading(2, 'Empty'),
					heading(2, 'What emerged'),
					heading(3, 'Common ground'),
					paragraph('a'),
					rule,
					paragraph(' ')
				)
			)
		).toEqual([{ label: 'What emerged', titles: ['Common ground'] }]);
	});

	it('leaves the headings that split the report out of the card content', () => {
		const [section] = splitReportIntoSections(
			doc(heading(2, 'Overview'), paragraph('a')),
			'Introduction'
		);
		expect(JSON.parse(section.cards[0].content)).toEqual({
			type: 'doc',
			content: [paragraph('a')]
		});
	});

	it('shows a legacy plain text body as one card', () => {
		expect(shape('Some **markdown**')).toEqual([
			{ label: 'Introduction', titles: ['Introduction'] }
		]);
	});

	it('returns nothing for an empty body', () => {
		expect(shape('')).toEqual([]);
		expect(shape(doc(paragraph('')))).toEqual([]);
	});
});

describe('toReportPages', () => {
	it('numbers each card within its section', () => {
		const sections = splitReportIntoSections(
			doc(
				heading(2, 'One'),
				paragraph('a'),
				rule,
				paragraph('b'),
				heading(2, 'Two'),
				paragraph('c')
			),
			'Introduction'
		);
		expect(
			toReportPages(sections).map(({ sectionIndex, cardIndex, cardCount }) => [
				sectionIndex,
				cardIndex,
				cardCount
			])
		).toEqual([
			[0, 0, 2],
			[0, 1, 2],
			[1, 0, 1]
		]);
	});
});

describe('hasReportContent', () => {
	it('counts an embedded component as content', () => {
		expect(hasReportContent(doc(embed))).toBe(true);
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
