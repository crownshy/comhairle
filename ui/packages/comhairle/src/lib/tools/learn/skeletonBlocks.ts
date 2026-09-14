import { detectContentType } from '$lib/utils/contentDetection';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';

export type SkeletonBlock =
	| { kind: 'heading'; level: 1 | 2 | 3; lines: number }
	| { kind: 'text'; lines: number }
	| { kind: 'quote'; lines: number }
	| { kind: 'list'; items: number }
	| { kind: 'media'; ratio: 'video' | 'image' };

type DocumentNode = {
	type?: string;
	text?: string;
	attrs?: Record<string, unknown>;
	content?: DocumentNode[];
};

/** Past this the skeleton is well below the fold, and the rest of a long page adds nothing. */
const MAX_BLOCKS = 12;

/**
 * Rough characters per line in the 46rem column: body copy at 18px, headings at their desktop
 * sizes. A phone wraps sooner, so there the outline runs a little short.
 */
const CHARS_PER_LINE = { 1: 40, 2: 48, 3: 56, body: 80 } as const;

function textOf(node: DocumentNode): string {
	if (node.text) return node.text;
	return (node.content ?? []).map(textOf).join(' ');
}

function lineCount(text: string, perLine: number, max: number) {
	return Math.min(max, Math.max(1, Math.ceil(text.trim().length / perLine)));
}

function blockFor(node: DocumentNode): SkeletonBlock | null {
	switch (node.type) {
		case 'heading': {
			const stored = Number(node.attrs?.level ?? 1);
			const level = stored <= 1 ? 1 : stored === 2 ? 2 : 3;
			return {
				kind: 'heading',
				level,
				lines: lineCount(textOf(node), CHARS_PER_LINE[level], 3)
			};
		}
		case 'blockquote':
			return { kind: 'quote', lines: lineCount(textOf(node), CHARS_PER_LINE.body, 6) };
		case 'bulletList':
		case 'orderedList':
			return { kind: 'list', items: Math.min(8, Math.max(1, node.content?.length ?? 0)) };
		case 'iframe':
			return { kind: 'media', ratio: 'video' };
		case 'image':
			return { kind: 'media', ratio: 'image' };
		default: {
			// Paragraphs, and anything else that carries text (a table, a code block) as lines.
			// An empty paragraph is a blank line in the editor and draws as almost nothing.
			const text = textOf(node);
			if (!text.trim()) return null;
			const max = node.type === 'paragraph' ? 8 : 4;
			return { kind: 'text', lines: lineCount(text, CHARS_PER_LINE.body, max) };
		}
	}
}

/**
 * The outline of a Learn page, read from its stored document without rendering it, so the
 * skeleton shown while LearnUI's code loads puts a video box where the video will be and a
 * quote where the quote will be. Markdown has no cheap outline and gives null, as does a
 * page with nothing in it; the skeleton then falls back to a generic article.
 */
export function skeletonBlocks(content: string | null | undefined): SkeletonBlock[] | null {
	const detected = detectContentType(content);
	if (detected.type !== CONTENT_TYPES.JSON) return null;

	const document = detected.content as DocumentNode;
	const blocks = (document.content ?? [])
		.map(blockFor)
		.filter((block): block is SkeletonBlock => block !== null)
		.slice(0, MAX_BLOCKS);
	return blocks.length > 0 ? blocks : null;
}
