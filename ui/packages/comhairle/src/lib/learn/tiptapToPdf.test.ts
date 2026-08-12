import { describe, it, expect } from 'vitest';
import {
	sectionsToPdfDefinition,
	collectImageSources,
	type LearnContentSection,
	type LearnContentPage
} from './tiptapToPdf';
import type { Content, ContentImage, ContentTable, ContentText } from 'pdfmake/interfaces';

/**
 * The converter is a pure ProseMirror -> pdfmake mapping (no DOM, no pdfmake runtime), so
 * these assertions pin the document-definition shape. The hard requirement it protects: the
 * output must carry real text runs and real table cells (a text-bearing PDF), never rasterised
 * content. See the "learn content as PDF" spec.
 */

/** Wrap raw ProseMirror JSON into a single-page section for the converter. */
function richSection(heading: string, doc: unknown): LearnContentSection {
	const page: LearnContentPage = { content: JSON.stringify(doc), is_rich: true };
	return { heading, pages: [page] };
}

/** Flatten the top-level content array for easier assertions. */
function contentOf(sections: LearnContentSection[]): Content[] {
	const def = sectionsToPdfDefinition(sections);
	return def.content as Content[];
}

describe('sectionsToPdfDefinition', () => {
	it('emits the step heading as an h1', () => {
		const content = contentOf([richSection('Low Emission Zone', { type: 'doc', content: [] })]);
		expect(content[0]).toEqual({ text: 'Low Emission Zone', style: 'h1' });
	});

	it('maps headings to their level style and paragraphs to normal', () => {
		const content = contentOf([
			richSection('Step', {
				type: 'doc',
				content: [
					{
						type: 'heading',
						attrs: { level: 2 },
						content: [{ type: 'text', text: 'Sub' }]
					},
					{ type: 'paragraph', content: [{ type: 'text', text: 'Body text' }] }
				]
			})
		]);

		const heading = content[1] as ContentText;
		expect(heading.style).toBe('h2');
		expect(heading.text).toEqual([
			{ text: 'Sub', bold: undefined, italics: undefined, decoration: undefined }
		]);

		const paragraph = content[2] as ContentText;
		expect(paragraph.style).toBe('normal');
	});

	it('translates bold / italic / underline marks into run properties', () => {
		const content = contentOf([
			richSection('Step', {
				type: 'doc',
				content: [
					{
						type: 'paragraph',
						content: [
							{ type: 'text', text: 'B', marks: [{ type: 'bold' }] },
							{ type: 'text', text: 'I', marks: [{ type: 'italic' }] },
							{ type: 'text', text: 'U', marks: [{ type: 'underline' }] }
						]
					}
				]
			})
		]);

		const runs = (content[1] as ContentText).text as ContentText[];
		expect(runs[0].bold).toBe(true);
		expect(runs[1].italics).toBe(true);
		expect(runs[2].decoration).toEqual(['underline']);
	});

	it('keeps a hard break as a newline run', () => {
		const content = contentOf([
			richSection('Step', {
				type: 'doc',
				content: [
					{
						type: 'paragraph',
						content: [
							{ type: 'text', text: 'a' },
							{ type: 'hardBreak' },
							{ type: 'text', text: 'b' }
						]
					}
				]
			})
		]);

		const runs = (content[1] as ContentText).text as Content[];
		expect(runs).toContain('\n');
	});

	it('maps nested lists to ul / ol', () => {
		const content = contentOf([
			richSection('Step', {
				type: 'doc',
				content: [
					{
						type: 'bulletList',
						content: [
							{
								type: 'listItem',
								content: [
									{ type: 'paragraph', content: [{ type: 'text', text: 'one' }] }
								]
							},
							{
								type: 'listItem',
								content: [
									{ type: 'paragraph', content: [{ type: 'text', text: 'two' }] }
								]
							}
						]
					}
				]
			})
		]);

		const list = content[1] as { ul: unknown[] };
		expect(Array.isArray(list.ul)).toBe(true);
		expect(list.ul).toHaveLength(2);
	});

	it('maps a table to real cell content with a header row', () => {
		const content = contentOf([
			richSection('Step', {
				type: 'doc',
				content: [
					{
						type: 'table',
						content: [
							{
								type: 'tableRow',
								content: [
									{
										type: 'tableHeader',
										content: [
											{
												type: 'paragraph',
												content: [{ type: 'text', text: 'Vehicle' }]
											}
										]
									},
									{
										type: 'tableHeader',
										content: [
											{
												type: 'paragraph',
												content: [{ type: 'text', text: 'Charge' }]
											}
										]
									}
								]
							},
							{
								type: 'tableRow',
								content: [
									{
										type: 'tableCell',
										content: [
											{
												type: 'paragraph',
												content: [{ type: 'text', text: 'HGV' }]
											}
										]
									},
									{
										type: 'tableCell',
										content: [
											{
												type: 'paragraph',
												content: [{ type: 'text', text: '60 per day' }]
											}
										]
									}
								]
							}
						]
					}
				]
			})
		]);

		const table = content[1] as ContentTable;
		expect(table.table.body).toHaveLength(2);
		expect(table.table.body[0]).toHaveLength(2);
		// Header cells carry bold; the whole thing is real cell content, not a flattened string.
		const headerCell = table.table.body[0][0] as { bold?: boolean };
		expect(headerCell.bold).toBe(true);
		// The serialized definition must contain the actual cell text (text-bearing).
		expect(JSON.stringify(table)).toContain('60 per day');
	});

	it('does not throw on unknown nodes and recovers their text', () => {
		const content = contentOf([
			richSection('Step', {
				type: 'doc',
				content: [
					{
						type: 'mysteryBlock',
						content: [{ type: 'paragraph', content: [{ type: 'text', text: 'kept' }] }]
					},
					{ type: 'somethingElse' }
				]
			})
		]);

		// Heading + the recovered paragraph; the leaf unknown node is dropped, nothing throws.
		expect(JSON.stringify(content)).toContain('kept');
	});

	it('renders a legacy (non-rich) page as plain text', () => {
		const content = contentOf([
			{ heading: 'Legacy step', pages: [{ content: 'Just markdown text', is_rich: false }] }
		]);
		expect(content[1]).toEqual({ text: 'Just markdown text', style: 'normal' });
	});

	it('falls back to plain text when rich content is not valid JSON', () => {
		const content = contentOf([
			{ heading: 'Broken', pages: [{ content: '{not json', is_rich: true }] }
		]);
		expect(content[1]).toEqual({ text: '{not json', style: 'normal' });
	});

	it('parses TipTap JSON even when the page is flagged legacy (is_rich=false)', () => {
		// Regression: learn pages are commonly stored as "legacy markdown" entries whose content
		// is actually ProseMirror JSON. Trusting is_rich dumped the raw JSON into the PDF.
		const doc = {
			type: 'doc',
			content: [
				{ type: 'paragraph', content: [{ type: 'text', text: 'Real prose, not JSON' }] }
			]
		};
		const content = contentOf([
			{ heading: 'Step', pages: [{ content: JSON.stringify(doc), is_rich: false }] }
		]);

		const paragraph = content[1] as ContentText;
		expect(paragraph.style).toBe('normal');
		expect(paragraph.text).toEqual([
			{
				text: 'Real prose, not JSON',
				bold: undefined,
				italics: undefined,
				decoration: undefined
			}
		]);
		// The serialized output must not contain the ProseMirror scaffolding as literal text.
		expect(JSON.stringify(content)).not.toContain('"type":"doc"');
	});

	it('renders genuine markdown headings and bullets', () => {
		const content = contentOf([
			{ heading: 'Step', pages: [{ content: '# Title\n\n- one\n- two', is_rich: false }] }
		]);
		expect(content[1]).toEqual({ text: 'Title', style: 'h1' });
		expect(content[2]).toEqual({ ul: ['one', 'two'], margin: [0, 0, 0, 8] });
	});

	it('collects image sources across pages', () => {
		const doc = {
			type: 'doc',
			content: [
				{ type: 'image', attrs: { src: 'https://cdn.example/a.jpg' } },
				{ type: 'paragraph', content: [{ type: 'text', text: 'x' }] },
				{ type: 'image', attrs: { src: 'https://cdn.example/b.png' } }
			]
		};
		const srcs = collectImageSources([richSection('Step', doc)]);
		expect(srcs.sort()).toEqual(['https://cdn.example/a.jpg', 'https://cdn.example/b.png']);
	});

	it('embeds an image when its data URL is provided, and skips it otherwise', () => {
		const doc = {
			type: 'doc',
			content: [{ type: 'image', attrs: { src: 'https://cdn.example/a.jpg', width: 900 } }]
		};

		// With no image map, the node is skipped (no raw URL leaks into the text layer).
		const skipped = contentOf([richSection('Step', doc)]);
		expect(skipped).toHaveLength(1); // just the heading
		expect(JSON.stringify(skipped)).not.toContain('cdn.example');

		// With the data URL provided, it becomes a real embedded image capped at the content width.
		const dataUrl = 'data:image/png;base64,AAAA';
		const embedded = sectionsToPdfDefinition([richSection('Step', doc)], {
			'https://cdn.example/a.jpg': dataUrl
		}).content as Content[];
		const image = embedded[1] as ContentImage;
		expect(image.image).toBe(dataUrl);
		expect(image.width).toBe(500);
	});

	it('produces an empty content array for no sections', () => {
		expect(sectionsToPdfDefinition([]).content).toEqual([]);
	});
});
