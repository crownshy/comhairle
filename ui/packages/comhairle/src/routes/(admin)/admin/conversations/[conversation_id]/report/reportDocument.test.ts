import { describe, expect, it } from 'vitest';
import { isUnwritten, reportSections, type ReportDocument } from './reportDocument';

const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const heading = (text: string) => ({
	type: 'heading',
	attrs: { level: 2 },
	content: [{ type: 'text', text }]
});
const doc = (...content: ReportDocument['content']): ReportDocument => ({ type: 'doc', content });

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

describe('reportSections', () => {
	it('gives one pill per H2, plus an introduction when text comes first', () => {
		const body = JSON.stringify(
			doc(paragraph('Intro'), heading('What we heard'), paragraph('a'), heading('Next'))
		);
		expect(reportSections(body, 'Introduction')).toEqual([
			{ label: 'Introduction', index: 0, firstNodeIndex: 0, nodeCount: 1 },
			{ label: 'What we heard', index: 1, firstNodeIndex: 1, nodeCount: 2 },
			{ label: 'Next', index: 2, firstNodeIndex: 3, nodeCount: 1 }
		]);
	});

	it('leaves out the introduction when the body opens with a heading', () => {
		const body = JSON.stringify(doc(heading('Only'), paragraph('a')));
		expect(reportSections(body, 'Introduction').map((s) => s.label)).toEqual(['Only']);
	});

	it('has no pills for text that is not a TipTap document', () => {
		expect(reportSections('plain text', 'Introduction')).toEqual([]);
	});
});
