import { CONTENT_TYPES, type ContentType } from '$lib/components/RichTextEditor/types';

const PROSEMIRROR_DOC_TYPE = 'doc';

/**
 * @param content - Raw content string (JSON or Markdown)
 * @returns Object containing parsed content and detected type
 * 
 * Detection logic:
 * - Returns empty markdown for null/empty content
 * - Attempts to parse as JSON if starts with '{' or '['
 * - Validates ProseMirror structure (must have type: 'doc')
 * - Falls back to treating as Markdown
 * 
 * @example
 * ```ts
 * detectContentType('{"type":"doc","content":[]}')
 * // Returns: { content: {...}, type: 'json' }
 * 
 * detectContentType('# Hello World')
 * // Returns: { content: '# Hello World', type: 'markdown' }
 * ```
 */
export function detectContentType(content: string | undefined): { 
	content: any; 
	type: ContentType 
} {
	if (!content || !content.trim()) {
		return { content: '', type: CONTENT_TYPES.MARKDOWN };
	}
	
	const trimmed = content.trim();
	
	if (trimmed.startsWith('{') || trimmed.startsWith('[')) {
		try {
			const parsed = JSON.parse(trimmed);
			if (parsed && typeof parsed === 'object' && parsed.type === PROSEMIRROR_DOC_TYPE) {
				return { content: parsed, type: CONTENT_TYPES.JSON };
			}
		} catch (error) {
			if (import.meta.env.DEV) {
				console.warn('[Content Detection] Failed to parse content as JSON:', error);
			}
		}
	}
	
	return { content: trimmed, type: CONTENT_TYPES.MARKDOWN };
}
