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

describe('video captions', () => {
	const CAPTIONS = 'https://media.example.com/clip.vtt';

	it('renders a captions track when the video has a captions file', () => {
		createEditor().commands.setVideo({ src: CLIP, captions: CAPTIONS });

		const html = editor.getHTML();
		expect(html).toContain(`<track kind="captions" src="${CAPTIONS}"`);
		expect(html).toContain('crossorigin="anonymous"');
	});

	it('renders no track or crossorigin without a captions file', () => {
		createEditor().commands.setVideo({ src: CLIP });

		const html = editor.getHTML();
		expect(html).not.toContain('<track');
		expect(html).not.toContain('crossorigin');
	});

	it('keeps the captions file when stored HTML is loaded again', () => {
		createEditor().commands.setVideo({ src: CLIP, captions: CAPTIONS });
		editor.commands.setContent(editor.getHTML());

		expect(editor.getJSON().content?.[0]).toMatchObject({
			type: 'video',
			attrs: { src: CLIP, captions: CAPTIONS }
		});
	});
});

describe('iframe title', () => {
	const EMBED = 'https://www.youtube.com/embed/abc123';

	it('renders the title the author gave', () => {
		createEditor().commands.setIframe({ src: EMBED, title: 'Budget explainer' });

		expect(editor.getHTML()).toContain('title="Budget explainer"');
	});

	it('falls back to a generic title for embeds saved without one', () => {
		createEditor(`<iframe src="${EMBED}"></iframe>`);

		expect(editor.getHTML()).toContain('title="Embedded video"');
	});
});
