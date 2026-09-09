import { Node, mergeAttributes } from '@tiptap/core';
import { mount, unmount } from 'svelte';
import ReportEmbedLive from '$lib/reports/polis/ReportEmbedLive.svelte';

/**
 * A report component embedded into the report's `summary` document.
 *
 * Per ADR-0012 the node stores ONLY a reference (`toolStepId` / `componentType` / `config`).
 * There is no baked HTML — every surface mounts the real, live component against current data:
 * - in the editor, the node view below mounts `ReportEmbedLive`;
 * - on the published page, the report renderer walks the document and interleaves
 *   `ReportEmbedLive` at each embed node (see the public report page).
 *
 * The static-renderer path (`renderRichTextToHtml`, used for no-JS surfaces like email) can't
 * mount a component, so it emits a placeholder via a `nodeMapping` entry there.
 */
export interface ReportComponentEmbedAttrs {
	toolStepId: string | null;
	componentType: string | null;
	config: Record<string, unknown>;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		reportComponentEmbed: {
			setReportComponentEmbed: (options: {
				toolStepId: string;
				componentType: string;
				config?: Record<string, unknown>;
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
			}
		};
	},

	parseHTML() {
		return [{ tag: 'div[data-report-embed]' }];
	},

	renderHTML({ HTMLAttributes }) {
		// Structural marker only. The editor and the published page mount the live component;
		// this is the storage/round-trip form and the fallback container for no-JS renderers.
		return [
			'div',
			mergeAttributes(HTMLAttributes, { 'data-report-embed': '', class: 'report-embed' })
		];
	},

	addNodeView() {
		return ({ node, editor, getPos }) => {
			const dom = document.createElement('div');
			dom.classList.add('report-embed', 'report-embed--editor');
			dom.setAttribute('data-report-embed', '');
			dom.contentEditable = 'false';

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

			const body = document.createElement('div');
			body.classList.add('report-embed__body');

			dom.appendChild(remove);
			dom.appendChild(body);

			const component = mount(ReportEmbedLive, {
				target: body,
				props: {
					toolStepId: (node.attrs.toolStepId as string) ?? '',
					componentType: (node.attrs.componentType as string) ?? ''
				}
			});

			return {
				dom,
				// Atom node: no editable content, and ignore mutations from the mounted
				// component so ProseMirror doesn't try to re-parse it.
				ignoreMutation: () => true,
				// The reference is stable for an embed, so keep the node view on update.
				update: (updatedNode) => updatedNode.type.name === node.type.name,
				destroy: () => {
					unmount(component);
				}
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
							config: options.config ?? {}
						}
					})
		};
	}
});
