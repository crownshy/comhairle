// @vitest-environment jsdom
import { describe, it, expect, afterEach } from 'vitest';
import { Editor } from '@tiptap/core';
import { getBaseExtensions } from '../editorConfig';

const CLIP = 'https://media.example.com/clip.mp4';

let editor: Editor;

function createEditor(content = '') {
	editor = new Editor({ extensions: getBaseExtensions({ mode: 'renderer' }), content });
	return editor;
}

afterEach(() => editor?.destroy());

describe('video node', () => {
	it('renders an inline, non-autoplaying video with a thumbnail fragment', () => {
		createEditor().commands.setVideo({ src: CLIP });

		const html = editor.getHTML();
		expect(html).toContain('playsinline');
		expect(html).toContain('preload="metadata"');
		expect(html).toContain(`<source src="${CLIP}#t=0.001">`);
		expect(html).not.toContain('autoplay');
	});

	it('keeps the src when stored HTML is loaded again', () => {
		createEditor().commands.setVideo({ src: CLIP });
		const html = editor.getHTML();

		editor.commands.setContent(html);
		expect(editor.getJSON().content?.[0]).toMatchObject({
			type: 'video',
			attrs: { src: CLIP }
		});
	});

	it('turns an old iframe pointing at a video file into a video node', () => {
		createEditor(`<div class="iframe-wrapper"><iframe src="${CLIP}"></iframe></div>`);

		expect(editor.getJSON().content?.[0]).toMatchObject({
			type: 'video',
			attrs: { src: CLIP }
		});
		expect(editor.getHTML()).not.toContain('<iframe');
	});

	it('leaves YouTube iframes as iframes', () => {
		createEditor('<iframe src="https://www.youtube.com/embed/abc123"></iframe>');

		expect(editor.getHTML()).toContain('<iframe');
	});
});
