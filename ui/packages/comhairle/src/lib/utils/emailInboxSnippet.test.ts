import { describe, it, expect } from 'vitest';
import { inboxSnippet } from './emailInboxSnippet';

describe('inboxSnippet', () => {
	it('keeps only the visible text', () => {
		const html =
			'<html><head><style>p{color:red}</style></head><body><h1>Dear Invited Participant,</h1><p>You have been&nbsp;selected.</p></body></html>';

		expect(inboxSnippet(html)).toBe('Dear Invited Participant, You have been selected.');
	});

	it('shortens long text', () => {
		const snippet = inboxSnippet(`<p>${'word '.repeat(60)}</p>`);

		expect(snippet.endsWith('…')).toBe(true);
		expect(snippet.length).toBeLessThanOrEqual(111);
	});
});
