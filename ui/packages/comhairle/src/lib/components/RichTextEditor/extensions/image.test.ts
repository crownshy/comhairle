import { describe, it, expect } from 'vitest';
import { renderRichTextToHtml } from '$lib/utils/renderRichText';

/**
 * The layout attributes only do anything if the read-only renderer emits the same
 * data attributes the editor stores, since editor-content.css keys off them.
 */
describe('image align / width attributes', () => {
	function docWithImage(attrs: Record<string, unknown>) {
		return JSON.stringify({
			type: 'doc',
			content: [{ type: 'image', attrs: { src: 'https://example.test/a.png', ...attrs } }]
		});
	}

	it('renders align and width as data attributes', () => {
		const html = renderRichTextToHtml(docWithImage({ align: 'center', width: '50' }));
		expect(html).toContain('data-align="center"');
		expect(html).toContain('data-width="50"');
	});

	it('omits the data attributes when unset', () => {
		const html = renderRichTextToHtml(docWithImage({}));
		expect(html).toContain('<img');
		expect(html).not.toContain('data-align');
		expect(html).not.toContain('data-width');
	});

	it('drops values outside the option lists', () => {
		const html = renderRichTextToHtml(docWithImage({ align: 'middle', width: '33' }));
		expect(html).not.toContain('data-align');
		expect(html).not.toContain('data-width');
	});
});
