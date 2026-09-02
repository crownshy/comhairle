import { describe, it, expect } from 'vitest';
import { render } from 'svelte/server';
import BeforeYouStart from './BeforeYouStart.svelte';
import type { BeforeYouStartPage } from './beforeYouStart';

const pages: BeforeYouStartPage[] = [
	{
		id: 'about-0',
		label: 'About',
		heading: 'About this consultation',
		content: 'The first page'
	},
	{ id: 'questions', label: 'Questions', heading: 'Questions', content: 'The second page' }
];

describe('BeforeYouStart', () => {
	it('renders every page, so the deck can be scrolled as well as jumped through', () => {
		const { body } = render(BeforeYouStart, {
			props: { pages, steps: [], conversationId: 'c1' }
		});

		expect(body).toContain('The first page');
		expect(body).toContain('The second page');
		// The chip jumps to the page's own id, so every page has to carry one.
		expect(body).toContain('id="about-0"');
		expect(body).toContain('id="questions"');
	});

	it('draws nothing when the conversation has no pages', () => {
		const { body } = render(BeforeYouStart, {
			props: { pages: [], steps: [], conversationId: 'c1' }
		});

		expect(body).not.toContain('<section');
	});
});
