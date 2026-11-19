import { describe, expect, it } from 'vitest';
import { firstLine, hasReportContent } from './reportDocument';

const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const embed = { type: 'reportComponentEmbed', attrs: { toolStepId: 'step', componentType: 'map' } };
const doc = (...content: unknown[]) => JSON.stringify({ type: 'doc', content });

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
		expect(hasReportContent(JSON.stringify({ type: 'doc' }))).toBe(false);
		expect(hasReportContent(null)).toBe(false);
	});

	it('treats the text the API fills a new report with as blank', () => {
		expect(hasReportContent('Summary to be filled out by facilitator')).toBe(false);
		expect(hasReportContent(doc(paragraph('Body to be filled out by facilitator')))).toBe(
			false
		);
	});

	it('counts legacy plain text as content', () => {
		expect(hasReportContent('What we heard')).toBe(true);
	});
});

describe('firstLine', () => {
	it('skips blank lines', () => {
		expect(firstLine([paragraph(' '), paragraph('Findings')])).toBe('Findings');
	});
});
