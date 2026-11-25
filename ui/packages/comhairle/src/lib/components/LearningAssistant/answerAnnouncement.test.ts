import { describe, expect, it } from 'vitest';
import { answerAnnouncement, toSpokenText } from './answerAnnouncement';

const FINDING = 'Finding an answer...';

describe('answerAnnouncement', () => {
	it('stays silent before the person asks anything', () => {
		const loaded = { answer: 'Old answer', streaming: false, error: null };
		expect(answerAnnouncement(loaded, false, FINDING)).toBe('');
	});

	it('keeps the same finding text for every streamed token', () => {
		const first = answerAnnouncement(
			{ answer: 'Th', streaming: true, error: null },
			true,
			FINDING
		);
		const later = answerAnnouncement(
			{ answer: 'The answer so far', streaming: true, error: null },
			true,
			FINDING
		);
		expect(first).toBe(FINDING);
		expect(later).toBe(FINDING);
	});

	it('announces the finished answer as plain text', () => {
		const done = { answer: 'It is **two** weeks [ID:0].', streaming: false, error: null };
		expect(answerAnnouncement(done, true, FINDING)).toBe('It is two weeks.');
	});

	it('leaves errors to the alert', () => {
		const failed = { answer: '', streaming: false, error: 'Timed out' };
		expect(answerAnnouncement(failed, true, FINDING)).toBe('');
	});

	it('handles no questions at all', () => {
		expect(answerAnnouncement(undefined, true, FINDING)).toBe('');
	});
});

describe('toSpokenText', () => {
	it('drops citation markers and collapses whitespace', () => {
		expect(toSpokenText('One [ID:1]\n\n two `code`')).toBe('One two code');
	});
});
