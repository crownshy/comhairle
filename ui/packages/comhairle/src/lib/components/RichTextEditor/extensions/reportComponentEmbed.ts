import { Node, mergeAttributes } from '@tiptap/core';

/**
 * A report component embedded into the report's `summary` document.
 *
 * Per ADR-0012 the node stores BOTH a reference (`toolStepId` / `componentType` / `config`)
 * and the `frozenHtml` snapshot rendered from that component at insert time. The frozen HTML
 * is what renders everywhere:
 * - in the editor, via the node view below (a plain DOM node whose innerHTML is the snapshot);
 * - on the published page, via a `nodeMapping` entry in `renderRichTextToHtml` that returns
 *   `frozenHtml` straight into the static-renderer string (no DOM, no JS needed).
 *
 * The reference is the recipe: it is what a future "refresh snapshot" re-freezes from, and the
 * hook a future live-component upgrade would attach to. Because the HTML is baked in, deleting
 * the source Step never blanks the report — only refresh is affected.
 */
export interface ReportComponentEmbedAttrs {
	toolStepId: string | null;
	componentType: string | null;
	config: Record<string, unknown>;
	frozenHtml: string;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		reportComponentEmbed: {
			setReportComponentEmbed: (options: {
				toolStepId: string;
				componentType: string;
				config?: Record<string, unknown>;
				frozenHtml: string;
			}) => ReturnType;
		};
	}
}

export const ReportComponentEmbed = Node.create({
	name: 'reportComponentEmbed',
	group: 'block',
	atom: true,
	selectable: true,
	draggable: true,

	addAttributes() {
		return {
			toolStepId: {
				default: null,
				parseHTML: (element) => element.getAttribute('data-tool-step-id'),
				renderHTML: (attributes) =>
					attributes.toolStepId ? { 'data-tool-step-id': attributes.toolStepId } : {}
			},
			componentType: {
				default: null,
				parseHTML: (element) => element.getAttribute('data-component-type'),
				renderHTML: (attributes) =>
					attributes.componentType
						? { 'data-component-type': attributes.componentType }
						: {}
			},
			config: {
				default: {},
				parseHTML: (element) => {
					const raw = element.getAttribute('data-config');
					if (!raw) return {};
					try {
						return JSON.parse(raw);
					} catch {
						return {};
					}
				},
				renderHTML: (attributes) => {
					const config = attributes.config as Record<string, unknown>;
					if (!config || Object.keys(config).length === 0) return {};
					return { 'data-config': JSON.stringify(config) };
				}
			},
			// The frozen snapshot. Kept in a data attribute so it survives a round-trip through
			// storage and parseHTML; the node view and the renderer are what turn it back into
			// visible markup.
			frozenHtml: {
				default: '',
				parseHTML: (element) => {
					const holder = element.querySelector('[data-frozen-html]');
					return holder ? holder.innerHTML : '';
				},
				renderHTML: () => ({})
			}
		};
	},

	parseHTML() {
		return [{ tag: 'div[data-report-embed]' }];
	},

	renderHTML({ HTMLAttributes }) {
		// Structural only. The published render path substitutes the frozen HTML via
		// `nodeMapping` (see renderRichText.ts); this spec is the fallback container.
		return [
			'div',
			mergeAttributes(HTMLAttributes, { 'data-report-embed': '', class: 'report-embed' }),
			['div', { 'data-frozen-html': '' }]
		];
	},

	addNodeView() {
		return ({ node, editor, getPos }) => {
			const dom = document.createElement('div');
			dom.classList.add('report-embed', 'report-embed--editor');
			dom.setAttribute('data-report-embed', '');
			dom.contentEditable = 'false';

			const body = document.createElement('div');
			body.classList.add('report-embed__body');
			body.setAttribute('data-frozen-html', '');
			body.innerHTML = (node.attrs.frozenHtml as string) ?? '';

			const remove = document.createElement('button');
			remove.type = 'button';
			remove.classList.add('report-embed__remove');
			remove.setAttribute('aria-label', 'Remove embedded report component');
			remove.title = 'Remove embedded report component';
			remove.textContent = '×';
			remove.addEventListener('mousedown', (event) => {
				event.preventDefault();
				event.stopPropagation();
				if (typeof getPos !== 'function') return;
				const from = getPos();
				if (from == null) return;
				editor
					.chain()
					.focus()
					.deleteRange({ from, to: from + node.nodeSize })
					.run();
			});

			dom.appendChild(remove);
			dom.appendChild(body);

			return {
				dom,
				// Atom node: no editable content hole, and ignore mutations from our own
				// innerHTML writes so ProseMirror doesn't try to re-parse the snapshot.
				ignoreMutation: () => true
			};
		};
	},

	addCommands() {
		return {
			setReportComponentEmbed:
				(options) =>
				({ commands }) =>
					commands.insertContent({
						type: this.name,
						attrs: {
							toolStepId: options.toolStepId,
							componentType: options.componentType,
							config: options.config ?? {},
							frozenHtml: options.frozenHtml
						}
					})
		};
	}
});
