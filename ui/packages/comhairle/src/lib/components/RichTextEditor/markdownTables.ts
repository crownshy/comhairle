import { marked } from 'marked';

/**
 * True if the pasted text contains a GFM table, according to marked's lexer (the
 * same markdown family `@tiptap/markdown` parses, and already a dependency).
 *
 * This is only a gate: pastes that are actually tables get re-parsed as markdown,
 * everything else falls through to the normal paste path. The real parsing is the
 * library's job, so we don't hand-roll table detection here.
 */
export function containsMarkdownTable(text: string): boolean {
	try {
		return marked.lexer(text).some((token) => token.type === 'table');
	} catch {
		return false;
	}
}
