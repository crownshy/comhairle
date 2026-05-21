import { Node } from '@tiptap/core';
import { Plugin, PluginKey } from '@tiptap/pm/state';

export interface SourceDocumentDoc {
	name: string;
	size: number;
}

export interface SourceDocumentOptions {
	documents?: Record<string, SourceDocumentDoc>;
	conversationId?: string;
	/** When true (editor mode), render full card with a remove (X) button. When false, render a compact badge. */
	editable?: boolean;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		sourceDocument: {
			setSourceDocument: (options: { documentId: string }) => ReturnType;
		};
	}
}

function formatSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export const SourceDocument = Node.create<SourceDocumentOptions>({
	name: 'sourceDocument',
	group: 'block',
	atom: true,

	addOptions() {
		return {
			documents: {},
			conversationId: undefined,
			editable: false
		};
	},

	addAttributes() {
		return {
			documentId: {
				default: null,
				parseHTML: (element) => element.getAttribute('data-document-id'),
				renderHTML: (attributes) => {
					if (!attributes.documentId) return {};
					return { 'data-document-id': attributes.documentId };
				}
			}
		};
	},

	parseHTML() {
		return [
			{
				tag: 'div[data-source-document]',
				getAttrs: (element) => {
					if (typeof element === 'string') return false;
					const id = element.getAttribute('data-document-id');
					if (!id) return false;
					return { documentId: id };
				}
			}
		];
	},

	renderHTML({ HTMLAttributes, node }) {
		const documentId = node.attrs.documentId as string | null;
		const doc = documentId ? this.options.documents?.[documentId] : undefined;
		const name = doc?.name ?? 'Source document';
		const size = doc?.size;
		const conversationId = this.options.conversationId;
		const editable = this.options.editable;

		const downloadHref =
			conversationId && documentId
				? `/api/conversation/${conversationId}/documents/${documentId}/download`
				: '#';

		// Renderer (customer) view: compact badge — icon + name only.
		if (!editable) {
			return [
				'a',
				{
					href: downloadHref,
					download: doc?.name ?? '',
					target: '_blank',
					rel: 'noopener noreferrer',
					class: 'source-document-badge',
					'data-source-document': '',
					...HTMLAttributes
				},
				['span', { class: 'source-document-badge-icon' }, ''],
				['span', { class: 'source-document-badge-name' }, name]
			];
		}

		// Editor (admin) view: full card + remove (X) button.
		const infoChildren: (string | any[])[] = [
			['span', { class: 'source-document-name' }, name]
		];
		if (size !== undefined) {
			infoChildren.push(['span', { class: 'source-document-size' }, formatSize(size)]);
		}
		infoChildren.push(['span', { class: 'source-document-hint' }, 'Click to download']);

		return [
			'div',
			{
				class: 'source-document-card',
				'data-source-document': '',
				...HTMLAttributes
			},
			[
				'a',
				{
					href: downloadHref,
					download: doc?.name ?? '',
					target: '_blank',
					rel: 'noopener noreferrer',
					class: 'source-document-link'
				},
				['span', { class: 'source-document-icon' }, ''],
				['span', { class: 'source-document-info' }, ...infoChildren]
			],
			[
				'button',
				{
					type: 'button',
					class: 'source-document-remove',
					'data-source-document-remove': '',
					'aria-label': 'Remove source document',
					title: 'Remove source document'
				},
				'×'
			]
		];
	},

	addProseMirrorPlugins() {
		const type = this.type;
		return [
			new Plugin({
				key: new PluginKey('sourceDocumentRemove'),
				props: {
					handleDOMEvents: {
						mousedown(view, event) {
							const target = event.target as HTMLElement | null;
							if (!target) return false;
							const btn = target.closest('[data-source-document-remove]');
							if (!btn) return false;
							event.preventDefault();
							event.stopPropagation();
							const card = btn.closest(
								'[data-source-document]'
							) as HTMLElement | null;
							if (!card) return false;
							const pos = view.posAtDOM(card, 0);
							if (pos == null || pos < 0) return false;
							const $pos = view.state.doc.resolve(pos);
							// Walk up to find the node of our type
							for (let depth = $pos.depth; depth >= 0; depth--) {
								const node = $pos.node(depth);
								if (node.type === type) {
									const confirmed =
										typeof window !== 'undefined' &&
										window.confirm(
											'Remove this source document from the content?'
										);
									if (!confirmed) return true;
									const from = $pos.before(depth);
									const to = $pos.after(depth);
									view.dispatch(view.state.tr.delete(from, to));
									return true;
								}
							}
							// Fallback: try nodeAt(pos)
							const nodeAtPos = view.state.doc.nodeAt(pos);
							if (nodeAtPos && nodeAtPos.type === type) {
								const confirmed =
									typeof window !== 'undefined' &&
									window.confirm('Remove this source document from the content?');
								if (!confirmed) return true;
								view.dispatch(view.state.tr.delete(pos, pos + nodeAtPos.nodeSize));
								return true;
							}
							return false;
						}
					}
				}
			})
		];
	},

	addCommands() {
		return {
			setSourceDocument:
				(options: { documentId: string }) =>
				({ tr, dispatch }) => {
					const { selection } = tr;
					const node = this.type.create({
						documentId: options.documentId
					});
					if (dispatch) {
						tr.replaceRangeWith(selection.from, selection.to, node);
					}
					return true;
				}
		};
	}
});
