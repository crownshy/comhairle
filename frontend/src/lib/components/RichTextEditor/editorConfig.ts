import { StarterKit } from '@tiptap/starter-kit';
import { Link } from '@tiptap/extension-link';
import { Image } from '@tiptap/extension-image';
import { Markdown } from '@tiptap/markdown';
import { Iframe } from '$lib/components/RichTextEditor/extensions/iframe';
import type { Extensions } from '@tiptap/core';

export const EDITOR_HTML_ATTRIBUTES = {
	link: {
		class: 'text-blue-600 underline hover:text-blue-800'
	},
	image: {
		class: 'max-w-full h-auto rounded-lg'
	},
	editor: {
		class: 'prose prose-sm max-w-none focus:outline-none'
	}
} as const;

export const RENDERER_LINK_ATTRIBUTES = {
	...EDITOR_HTML_ATTRIBUTES.link,
	target: '_blank',
	rel: 'noopener noreferrer'
} as const;

export type EditorMode = 'editor' | 'renderer';

export interface EditorConfigOptions {
	mode: EditorMode;
}

export function getBaseExtensions(options: EditorConfigOptions): Extensions {
	const { mode } = options;
	const isRenderer = mode === 'renderer';

	return [
		Link.configure({
			openOnClick: isRenderer,
			HTMLAttributes: isRenderer ? RENDERER_LINK_ATTRIBUTES : EDITOR_HTML_ATTRIBUTES.link
		}),
		Image.configure({
			HTMLAttributes: EDITOR_HTML_ATTRIBUTES.image
		}),
		Iframe,
		StarterKit.configure({
			heading: {
				levels: [1, 2, 3, 4, 5, 6]
			}
		}),
		Markdown
	];
}

export function getEditorProps() {
	return {
		attributes: EDITOR_HTML_ATTRIBUTES.editor
	};
}
