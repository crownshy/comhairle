import type { AnyExtension, JSONContent } from '@tiptap/core';
import { MarkdownManager } from '@tiptap/markdown';
import { renderToHTMLString } from '@tiptap/static-renderer/pm/html-string';
import { getBaseExtensions } from '$lib/components/RichTextEditor/editorConfig';
import { SourceDocument } from '$lib/components/RichTextEditor/extensions/sourceDocument';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
import { detectContentType } from './contentDetection';
import type { ComhairleDocument } from '@crownshy/api-client/api';

/**
 * Documents referenced by `sourceDocument` nodes, so a badge can render its real
 * filename and download link. Content with no badges renders fine without these.
 */
export type RenderRichTextOptions = {
	documents?: ComhairleDocument[];
	conversationId?: string;
};

function buildExtensions(documents: ComhairleDocument[], conversationId: string): AnyExtension[] {
	const documentsById: Record<string, { name: string; size: number }> = {};
	for (const document of documents) {
		documentsById[document.id] = { name: document.name, size: document.size };
	}

	// SourceDocument reads its options inside renderHTML, so it has to be reconfigured
	// (not just re-run) whenever the document set changes.
	return [
		...getBaseExtensions({ mode: 'renderer' }).filter((ext) => ext.name !== 'sourceDocument'),
		SourceDocument.configure({ documents: documentsById, conversationId })
	];
}

/**
 * Renders stored rich-text (ProseMirror JSON, or Markdown/plain text) to an HTML string.
 *
 * This exists so read-only content can be server-rendered. The obvious alternative,
 * `generateHTML` from `@tiptap/core`, calls into ProseMirror's `DOMSerializer` and throws
 * `window is not defined` under SSR; `@tiptap/static-renderer` walks the node tree instead
 * and needs no DOM.
 *
 * Markdown is parsed into a ProseMirror document rather than handed to a markdown-to-HTML
 * library because the schema is what makes this safe to feed to `{@html}`: unknown tags in
 * the source survive as escaped text, so admin-authored content can't inject markup.
 *
 * @param content - stored content: ProseMirror JSON, Markdown, or plain text
 * @param options - documents and conversation id used to render source-document badges
 * @returns an HTML string, or `''` for empty or unrenderable content
 */
export function renderRichTextToHtml(
	content: string | null | undefined,
	options: RenderRichTextOptions = {}
): string {
	const { documents = [], conversationId = '' } = options;

	const detected = detectContentType(content);
	if (!detected.content) return '';

	const extensions = buildExtensions(documents, conversationId);

	try {
		const document =
			detected.type === CONTENT_TYPES.JSON
				? (detected.content as JSONContent)
				: (new MarkdownManager({ extensions }).parse(
						String(detected.content)
					) as JSONContent);

		return renderToHTMLString({ content: document, extensions });
	} catch (error) {
		console.error('[renderRichTextToHtml] Failed to render content:', error);
		return '';
	}
}
