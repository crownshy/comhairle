import { Extension } from '@tiptap/core';
import type { Node as ProseMirrorNode } from '@tiptap/pm/model';
import { Plugin, PluginKey } from '@tiptap/pm/state';

/** Page breaks split the report body into pages, so the summary never takes one (ADR-0066). */
export function countPageBreaks(document: ProseMirrorNode): number {
	let count = 0;
	document.descendants((node) => {
		if (node.type.name === 'horizontalRule') count += 1;
	});
	return count;
}

/**
 * Turns away any edit that adds a page break, whether typed (`---`), pasted or dropped. One
 * that is already there can still be deleted.
 */
export const KeepPageBreaksOut = Extension.create({
	name: 'keepPageBreaksOut',
	addProseMirrorPlugins() {
		return [
			new Plugin({
				key: new PluginKey('keepPageBreaksOut'),
				filterTransaction: (transaction) =>
					!transaction.docChanged ||
					countPageBreaks(transaction.doc) <= countPageBreaks(transaction.before)
			})
		];
	}
});
