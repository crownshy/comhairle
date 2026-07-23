import { StarterKit } from '@tiptap/starter-kit';
import { Link } from '@tiptap/extension-link';
import { Image } from '@tiptap/extension-image';
import { Audio } from '@tiptap/extension-audio';
import { TextAlign } from '@tiptap/extension-text-align';
import { TableKit, TableCell, TableHeader } from '@tiptap/extension-table';
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

/**
 * A `cellColor` attribute added to table cells + headers. It stores a palette KEY
 * (see tableColors.ts), rendered as `data-cell-color`, which editor-content.css
 * turns into a theme-aware background. `this.parent()` preserves the built-in
 * colspan / rowspan / colwidth attributes.
 */
const cellColorAttribute = {
	cellColor: {
		default: null as string | null,
		parseHTML: (element: HTMLElement) => element.getAttribute('data-cell-color'),
		renderHTML: (attributes: Record<string, unknown>) =>
			attributes.cellColor ? { 'data-cell-color': attributes.cellColor as string } : {}
	}
};

const TableCellWithColor = TableCell.extend({
	addAttributes() {
		return { ...this.parent?.(), ...cellColorAttribute };
	}
});

const TableHeaderWithColor = TableHeader.extend({
	addAttributes() {
		return { ...this.parent?.(), ...cellColorAttribute };
	}
});

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
		// there. `renderWrapper` makes the non-editor render paths wrap the table in the
		// same `.tableWrapper` div the editor's NodeView uses, so a stored table looks
		// identical (full-width, same overflow behaviour) in the editor and the renderer.
		// One registration wires editor + both renderers via getBaseExtensions.
		// TableKit's own cell/header are disabled in favour of the colour-aware
		// versions below, so a stored `cellColor` renders in editor and renderer alike.
		TableKit.configure({
			table: { resizable: true, renderWrapper: true },
			tableCell: false,
			tableHeader: false
		}),
		TableCellWithColor,
		TableHeaderWithColor,
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
