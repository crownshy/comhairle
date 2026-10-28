import { describe, expect, it } from 'vitest';
import {
	countDraftPrompts,
	draftReport,
	isUnwritten,
	reportSections,
	type ReportDocument
} from './reportDocument';

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

describe('draftReport', () => {
	it('opens each step that has charts with its main charts, and skips the rest', () => {
		const draft = draftReport([
			{ id: 'polis-step', name: 'Bike lanes poll', toolType: 'polis' },
			{ id: 'learn-step', name: 'Background', toolType: 'learn' }
		]);
		const headings = draft.content
			.filter((n) => n.type === 'heading')
			.map((n) => n.content?.[0].text);
		const charts = draft.content
			.filter((n) => n.type === 'reportComponentEmbed')
			.map((n) => [n.attrs?.toolStepId, n.attrs?.componentType]);

		expect(headings).toEqual(['Bike lanes poll', 'What happens next']);
		expect(charts).toEqual([
			['polis-step', 'polis-key-stats'],
			['polis-step', 'polis-area-consensus']
		]);
	});

	it('still gives a starting point when no step has charts', () => {
		expect(draftReport([]).content.length).toBeGreaterThan(0);
	});
});

describe('countDraftPrompts', () => {
	it('counts the prompts nobody has written over', () => {
		const draft = draftReport([{ id: 'p', name: 'Poll', toolType: 'polis' }]);
		expect(countDraftPrompts(JSON.stringify(draft))).toBe(4);

		const edited = {
			...draft,
			content: draft.content.map((node, i) =>
				i === 0 ? paragraph('We asked 40 people') : node
			)
		};
		expect(countDraftPrompts(JSON.stringify(edited))).toBe(3);
	});
});
