import { Mark, mergeAttributes } from '@tiptap/core';

/**
 * An inline mark for a glossary term. It is never typed by an author: `applyGlossary`
 * adds it to matched text at render time (see $lib/glossary/applyGlossary). It lives in
 * the shared schema so `renderToHTMLString` knows how to emit it.
 *
 * Renders `<span class="glossary-term" data-glossary-tooltip="...">term</span>`. The
 * tooltip itself is CSS-only (editor-content.css) so it survives the `{@html}` render
 * path, where Svelte components can't mount.
 */
export const GlossaryTerm = Mark.create({
	name: 'glossaryTerm',
	// Not spanning, not part of any input rule: it's applied programmatically only.
	inclusive: false,

	addAttributes() {
		return {
			tooltip: {
				default: null,
				parseHTML: (element) => element.getAttribute('data-glossary-tooltip'),
				renderHTML: (attributes) =>
					attributes.tooltip ? { 'data-glossary-tooltip': attributes.tooltip } : {}
			}
		};
	},

	parseHTML() {
		return [{ tag: 'span[data-glossary-term]' }];
	},

	renderHTML({ HTMLAttributes }) {
		return [
			'span',
			mergeAttributes(HTMLAttributes, {
				'data-glossary-term': '',
				class: 'glossary-term',
				// Focusable so the definition is reachable by keyboard, not only hover.
				tabindex: '0'
			}),
			0
		];
	}
});
