import { describe, it, expect } from 'vitest';
import { beforeYouStartPages } from './beforeYouStart';
import type { LocalizedConversationDto } from '@crownshy/api-client/api';
import type { StepPreview } from './stepPreview';

function conversation(fields: Partial<LocalizedConversationDto>) {
	return fields as LocalizedConversationDto;
}

const steps: StepPreview[] = [{ id: 'step-1', name: 'Quick poll', optional: false }];

describe('beforeYouStartPages', () => {
	it('splits the description at horizontal rules', () => {
		const pages = beforeYouStartPages(conversation({ description: 'One\n\n---\n\nTwo' }), []);
		expect(pages.map((p) => p.id)).toEqual(['about-0', 'about-1']);
	});

	it('names a page after the heading it opens with', () => {
		const pages = beforeYouStartPages(
			conversation({ description: '# What is this about?\n\nBody' }),
			[]
		);
		expect(pages[0].label).toBe('What is this about?');
		// The content carries the heading already, so the page must not add a second one.
		expect(pages[0].heading).toBeUndefined();
	});

	it('falls back to the default label and heading without one', () => {
		const pages = beforeYouStartPages(conversation({ description: 'Body' }), []);
		expect(pages[0].label).toBe('About');
		expect(pages[0].heading).toBe('About this consultation');
	});

	it('ignores a heading that is not the first block', () => {
		const pages = beforeYouStartPages(conversation({ description: 'Body\n\n## Later' }), []);
		expect(pages[0].label).toBe('About');
	});

	it('numbers repeated labels', () => {
		const pages = beforeYouStartPages(
			conversation({ description: 'One\n\n---\n\nTwo\n\n---\n\nThree' }),
			[]
		);
		expect(pages.map((p) => p.label)).toEqual(['About', 'More', 'More 2']);
	});

	it('puts the computed step list last', () => {
		const pages = beforeYouStartPages(
			conversation({
				description: 'Body',
				faqs: 'Questions',
				privacyPolicy: 'Policy'
			}),
			steps
		);
		expect(pages.map((p) => p.id)).toEqual(['about-0', 'questions', 'your-data', 'steps']);
	});

	it('leaves out sections with nothing in them', () => {
		expect(beforeYouStartPages(conversation({}), [])).toEqual([]);
	});
});
