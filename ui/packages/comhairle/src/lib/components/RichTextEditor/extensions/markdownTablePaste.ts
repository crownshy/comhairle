import { Extension } from '@tiptap/core';
import { Plugin } from '@tiptap/pm/state';
import { containsMarkdownTable } from '../markdownTables';

/**
 * Editor-only support for pasting markdown tables. `@tiptap/markdown` can parse a GFM
 * table (that's what powers `contentType: 'markdown'`), but it does no paste handling
 * of its own. When the pasted plain text contains a GFM table, we re-parse the whole
 * paste as markdown so the table (and any surrounding markdown) comes in as real nodes.
 * Ordinary pastes fall through untouched, since the separator-row check keeps stray
 * pipes from triggering it.
 *
 * (Typing markdown table syntax is intentionally NOT supported - building a table is
 * better served by the toolbar insert button and the hover "+" controls.)
 */
export const MarkdownTablePaste = Extension.create({
	name: 'markdownTablePaste',

	addProseMirrorPlugins() {
		const editor = this.editor;
		return [
			new Plugin({
				props: {
					handlePaste(_view, event) {
						const text = event.clipboardData?.getData('text/plain');
						if (!text || !containsMarkdownTable(text)) return false;
						editor.commands.insertContent(text, { contentType: 'markdown' });
						return true;
					}
				}
			})
		];
	}
});
