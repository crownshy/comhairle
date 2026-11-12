import { describe, expect, it } from 'vitest';
import type { ReportNode } from '$lib/reports/reportDocument';
import { isUnwritten, reportPages } from './reportDocument';

const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const heading = (text: string) => ({
	type: 'heading',
	attrs: { level: 2 },
	content: [{ type: 'text', text }]
});
const doc = (...content: ReportNode[]) => ({ type: 'doc', content });

describe('isUnwritten', () => {
	it('treats missing, blank and the API default text as unwritten', () => {
		expect(isUnwritten(null)).toBe(true);
		expect(isUnwritten('')).toBe(true);
		expect(isUnwritten('Body to be filled out by facilitator')).toBe(true);
		expect(
			isUnwritten(JSON.stringify(doc(paragraph('Summary to be filled out by facilitator'))))
		).toBe(true);
		expect(isUnwritten(JSON.stringify(doc({ type: 'paragraph' })))).toBe(true);
		expect(isUnwritten('{"type":"doc"}')).toBe(true);
	});

	it('treats any other text, or a lone chart, as written', () => {
		expect(isUnwritten(JSON.stringify(doc(paragraph('People agreed on bike lanes'))))).toBe(
			false
		);
		expect(
			isUnwritten(
				JSON.stringify(
					doc({
						type: 'reportComponentEmbed',
						attrs: { toolStepId: 's', componentType: 'c' }
					})
				)
			)
		).toBe(false);
		expect(isUnwritten('Plain markdown someone wrote')).toBe(false);
	});

	it('treats media or a table with no text as written', () => {
		for (const type of ['image', 'video', 'audio', 'iframe', 'table']) {
			expect(
				isUnwritten(JSON.stringify(doc({ type, attrs: { src: 'https://e.com/a' } })))
			).toBe(false);
		}
	});
});

describe('reportPages', () => {
	const rule = { type: 'horizontalRule' };
	const pageLabel = (pageNumber: number) => `Page ${pageNumber}`;

	it('gives one pill per page, named by its first line of text', () => {
		const body = JSON.stringify(
			doc(heading('What we heard'), paragraph('a'), heading('Next'), rule, paragraph('Then'))
		);
		expect(reportPages(body, pageLabel, 'Untitled')).toEqual([
			{
				label: 'What we heard',
				empty: false,
				firstNodeIndex: 0,
				removal: { firstNodeIndex: 0, nodeCount: 4 }
			},
			{
				label: 'Then',
				empty: false,
				firstNodeIndex: 4,
				removal: { firstNodeIndex: 3, nodeCount: 2 }
			}
		]);
	});

	it('names a page by the line before a hard break, or by the first item of a list', () => {
		const twoLines = {
			type: 'paragraph',
			content: [
				{ type: 'text', text: 'Line one' },
				{ type: 'hardBreak' },
				{ type: 'text', text: 'Line two' }
			]
		};
		const list = {
			type: 'bulletList',
			content: ['First', 'Second'].map((item) => ({
				type: 'listItem',
				content: [paragraph(item)]
			}))
		};
		const body = JSON.stringify(doc(twoLines, rule, list));
		expect(reportPages(body, pageLabel, 'Untitled').map((page) => page.label)).toEqual([
			'Line one',
			'First'
		]);
	});

	it('numbers a page with no text, counting only pages with something on them', () => {
		const chart = { type: 'reportComponentEmbed', attrs: {} };
		const body = JSON.stringify(doc(paragraph(' '), rule, chart, rule, paragraph('Last')));
		expect(reportPages(body, pageLabel, 'Untitled').map((page) => page.label)).toEqual([
			'Untitled',
			'Page 1',
			'Last'
		]);
	});

	it('lists a page as soon as its page break is added, pointing at the break', () => {
		const body = JSON.stringify(doc(paragraph('One'), rule));
		expect(reportPages(body, pageLabel, 'Untitled')).toEqual([
			{
				label: 'One',
				empty: false,
				firstNodeIndex: 0,
				removal: { firstNodeIndex: 0, nodeCount: 2 }
			},
			{
				label: 'Untitled',
				empty: true,
				firstNodeIndex: 1,
				removal: { firstNodeIndex: 1, nodeCount: 1 }
			}
		]);
	});

	it('removes a lone page without reaching past the document', () => {
		const body = JSON.stringify(doc(paragraph('Only')));
		expect(reportPages(body, pageLabel, 'Untitled')[0].removal).toEqual({
			firstNodeIndex: 0,
			nodeCount: 1
		});
	});

	it('lists one untitled page for a blank body', () => {
		expect(reportPages('', pageLabel, 'Untitled').map((page) => page.label)).toEqual([
			'Untitled'
		]);
	});

	it('has no pills for text that is not a TipTap document', () => {
		expect(reportPages('plain text', pageLabel, 'Untitled')).toEqual([]);
	});
});
