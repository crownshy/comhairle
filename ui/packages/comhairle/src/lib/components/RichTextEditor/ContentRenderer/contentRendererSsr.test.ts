import { describe, it, expect } from 'vitest';
import { render } from 'svelte/server';
import ContentRenderer from './ContentRenderer.svelte';

/**
 * The regression this guards: ContentRenderer used to build its output in `onMount`, so
 * server-rendered markup contained an empty div and the text only appeared after hydration.
 * Rendering it through `svelte/server` here is the same path SvelteKit takes for SSR.
 */
describe('ContentRenderer server-side rendering', () => {
	it('includes the content in server-rendered markup', () => {
		const content = JSON.stringify({
			type: 'doc',
			content: [{ type: 'paragraph', content: [{ type: 'text', text: 'A new Survey Step' }] }]
		});

		const { body } = render(ContentRenderer, { props: { content } });

		expect(body).toContain('A new Survey Step');
	});

	it('keeps the .tiptap wrapper that carries the content styling', () => {
		const { body } = render(ContentRenderer, { props: { content: 'hello' } });
		expect(body).toContain('tiptap');
	});
});
