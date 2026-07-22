import { describe, it, expect } from 'vitest';
import { renderRichTextToHtml } from './renderRichText';
import type { ComhairleDocument } from '@crownshy/api-client/api';

/**
 * These run under vitest's default `node` environment with no DOM, which is the
 * point: it's the same condition as SSR, so a regression back to a DOM-dependent
 * renderer fails here rather than silently blanking the first paint.
 */
describe('renderRichTextToHtml', () => {
	const paragraph = (text: string) =>
		JSON.stringify({
			type: 'doc',
			content: [{ type: 'paragraph', content: [{ type: 'text', text }] }]
		});

	it('renders ProseMirror JSON', () => {
		expect(renderRichTextToHtml(paragraph('A new Survey Step'))).toBe(
			'<p>A new Survey Step</p>'
		);
	});

	it('renders marks within JSON', () => {
		const content = JSON.stringify({
			type: 'doc',
			content: [
				{
					type: 'paragraph',
					content: [
						{ type: 'text', text: 'a ' },
						{ type: 'text', marks: [{ type: 'bold' }], text: 'bold' },
						{ type: 'text', text: ' word' }
					]
				}
			]
		});
		expect(renderRichTextToHtml(content)).toBe('<p>a <strong>bold</strong> word</p>');
	});

	it('renders markdown', () => {
		expect(renderRichTextToHtml('# Title\n\nsome *text*')).toBe(
			'<h1>Title</h1><p>some <em>text</em></p>'
		);
	});

	it('treats a plain string as markdown', () => {
		expect(renderRichTextToHtml('New Survey Step')).toBe('<p>New Survey Step</p>');
	});

	it.each([null, undefined, '', '   '])('returns empty string for %p', (content) => {
		expect(renderRichTextToHtml(content)).toBe('');
	});

	it('escapes raw HTML embedded in markdown rather than passing it through', () => {
		const html = renderRichTextToHtml('hello <img src=x onerror=alert(1)>');
		expect(html).not.toContain('<img');
		expect(html).toContain('&lt;img');
	});

	it('escapes raw HTML embedded in a JSON text node', () => {
		const html = renderRichTextToHtml(paragraph('<script>alert(1)</script>'));
		expect(html).not.toContain('<script>');
		expect(html).toContain('&lt;script&gt;');
	});

	it('falls back to a generic label for a badge whose document is not loaded yet', () => {
		const content = JSON.stringify({
			type: 'doc',
			content: [{ type: 'sourceDocument', attrs: { documentId: 'doc-1' } }]
		});
		const html = renderRichTextToHtml(content, { conversationId: 'conv-1' });
		expect(html).toContain('Source document');
	});

	it('renders a badge with the real filename and download link once documents are known', () => {
		const content = JSON.stringify({
			type: 'doc',
			content: [{ type: 'sourceDocument', attrs: { documentId: 'doc-1' } }]
		});
		const documents = [
			{ id: 'doc-1', name: 'budget.pdf', size: 2048 }
		] as unknown as ComhairleDocument[];

		const html = renderRichTextToHtml(content, { documents, conversationId: 'conv-1' });

		expect(html).toContain('budget.pdf');
		expect(html).toContain('/api/conversation/conv-1/documents/doc-1/download');
		expect(html).toContain('data-document-id="doc-1"');
	});

	it('returns empty string for malformed content instead of throwing', () => {
		expect(renderRichTextToHtml('{"type":"doc","content":[{"type":"nope"}]}')).toBe('');
	});
});
