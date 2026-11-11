// @vitest-environment jsdom
import { describe, it, expect } from 'vitest';
import { Editor, generateJSON, type JSONContent } from '@tiptap/core';
import { renderRichTextToHtml } from '$lib/utils/renderRichText';
import { getBaseExtensions } from '../editorConfig';

const SOURCE = 'https://example.test/a.png';

function docWithImage(attrs: Record<string, unknown>): JSONContent {
	return { type: 'doc', content: [{ type: 'image', attrs: { src: SOURCE, ...attrs } }] };
}

function imageAttributes(document: JSONContent): Record<string, unknown> | undefined {
	return document.content?.find((node) => node.type === 'image')?.attrs;
}

// The stylesheet keys off these data attributes, so the read-only renderer has to emit them.
describe('rendering image align and width', () => {
	function render(attrs: Record<string, unknown>) {
		return renderRichTextToHtml(JSON.stringify(docWithImage(attrs)));
	}

	it('renders align and width as data attributes', () => {
		const html = render({ align: 'center', width: '50' });
		expect(html).toContain('data-align="center"');
		expect(html).toContain('data-width="50"');
	});

	it('leaves the data attributes out when unset', () => {
		const html = render({});
		expect(html).toContain('<img');
		expect(html).not.toContain('data-align');
		expect(html).not.toContain('data-width');
	});

	it('drops values outside the option lists', () => {
		const html = render({ align: 'middle', width: '33' });
		expect(html).not.toContain('data-align');
		expect(html).not.toContain('data-width');
	});
});

describe('reading image align and width back', () => {
	const extensions = getBaseExtensions({ mode: 'editor' });

	it('parses the data attributes from pasted HTML', () => {
		const json = generateJSON(
			`<img src="${SOURCE}" data-align="right" data-width="75">`,
			extensions
		);
		expect(imageAttributes(json)).toMatchObject({ align: 'right', width: '75' });
	});

	it('ignores unknown data attribute values in pasted HTML', () => {
		const json = generateJSON(
			`<img src="${SOURCE}" data-align="middle" data-width="33">`,
			extensions
		);
		expect(imageAttributes(json)).toMatchObject({ align: null, width: null });
	});

	it('keeps the layout through a save and reload of the editor JSON', () => {
		const first = new Editor({
			extensions,
			content: docWithImage({ align: 'left', width: '25' })
		});
		const saved = JSON.stringify(first.getJSON());
		first.destroy();

		const reloaded = new Editor({ extensions, content: JSON.parse(saved) });
		expect(imageAttributes(reloaded.getJSON())).toMatchObject({ align: 'left', width: '25' });
		reloaded.destroy();
	});

	it('loads an image saved before the layout keys existed with the default layout', () => {
		const editor = new Editor({ extensions, content: docWithImage({}) });
		expect(imageAttributes(editor.getJSON())).toMatchObject({ align: null, width: null });
		editor.destroy();
	});
});
