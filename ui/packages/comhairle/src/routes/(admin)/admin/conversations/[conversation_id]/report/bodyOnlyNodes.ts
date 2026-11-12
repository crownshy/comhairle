import { Extension } from '@tiptap/core';
import type { Node as ProseMirrorNode } from '@tiptap/pm/model';
import { Plugin, PluginKey } from '@tiptap/pm/state';

/** Charts and page breaks only go in the report body, never the summary (ADR-0066). */
const BODY_ONLY_NODE_TYPES = new Set(['reportComponentEmbed', 'horizontalRule']);

export function countBodyOnlyNodes(document: ProseMirrorNode): number {
	let count = 0;
	document.descendants((node) => {
		if (BODY_ONLY_NODE_TYPES.has(node.type.name)) count += 1;
	});
	return count;
}

/**
 * Turns away any edit that adds a chart or page break, whether typed (`---`), pasted or
 * dropped. One that is already there can still be deleted.
 */
export const KeepBodyOnlyNodesOut = Extension.create({
	name: 'keepBodyOnlyNodesOut',
	addProseMirrorPlugins() {
		return [
			new Plugin({
				key: new PluginKey('keepBodyOnlyNodesOut'),
				filterTransaction: (transaction) =>
					!transaction.docChanged ||
					countBodyOnlyNodes(transaction.doc) <= countBodyOnlyNodes(transaction.before)
			})
		];
	}
});
