import { Mark, mergeAttributes } from '@tiptap/core';

/**
 * An inline mark for a glossary term. It is never typed by an author: `applyGlossary`
 * adds it to matched text at render time (see $lib/glossary/applyGlossary). It lives in
 * the shared schema so `renderToHTMLString` knows how to emit it.
 *
 * Renders `<button type="button" class="glossary-term" data-glossary-tooltip="...">` so the
 * term is a real control for keyboard and screen reader users; ContentRenderer drives the
 * tooltip (see glossaryTooltip.ts). The mark is never stored, so changing the tag needs no
 * migration, and older `span` markup still parses.
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
		return [{ tag: 'button[data-glossary-term]' }, { tag: 'span[data-glossary-term]' }];
	},

	renderHTML({ HTMLAttributes }) {
		return [
			'button',
			mergeAttributes(HTMLAttributes, {
				type: 'button',
				'data-glossary-term': '',
				class: 'glossary-term'
			}),
			0
		];
	}
});
