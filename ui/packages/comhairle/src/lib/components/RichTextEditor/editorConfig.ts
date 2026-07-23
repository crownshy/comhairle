import { StarterKit } from '@tiptap/starter-kit';
import { Link } from '@tiptap/extension-link';
import { Image } from '@tiptap/extension-image';
import { Audio } from '@tiptap/extension-audio';
import { TextAlign } from '@tiptap/extension-text-align';
import { TableKit } from '@tiptap/extension-table';
import { Markdown } from '@tiptap/markdown';
import { Color } from '@tiptap/extension-color';
import { Iframe } from '$lib/components/RichTextEditor/extensions/iframe';
import { ListItem } from '@tiptap/extension-list-item';
import { TextStyle } from '@tiptap/extension-text-style';
import { Underline } from '@tiptap/extension-underline';
import { SourceDocument } from './extensions/sourceDocument';
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
		TextStyle,
		ListItem,
		Underline,
		Color.configure({ types: ['textStyle', 'listItem'] }),
		Link.configure({
			openOnClick: isRenderer,
			HTMLAttributes: isRenderer ? RENDERER_LINK_ATTRIBUTES : EDITOR_HTML_ATTRIBUTES.link
		}),
		Image.configure({
			HTMLAttributes: EDITOR_HTML_ATTRIBUTES.image
		}),
		Iframe,
		Audio,
		SourceDocument,
		TextAlign.configure({
			types: ['heading', 'paragraph']
		}),
		// Tables. `resizable` only takes effect in the live editor (it needs a DOM
		// NodeView); the SSR/email renderers just walk the schema, so this is a no-op
		// there. One registration wires editor + both renderers via getBaseExtensions.
		TableKit.configure({
			table: { resizable: true }
		}),
		StarterKit.configure({
			heading: {
				levels: [1, 2, 3, 4, 5, 6]
			},
			link: false,
			listItem: false,
			underline: false
		}),
		Markdown
	];
}

export function getEditorProps() {
	return {
		attributes: EDITOR_HTML_ATTRIBUTES.editor
	};
}
