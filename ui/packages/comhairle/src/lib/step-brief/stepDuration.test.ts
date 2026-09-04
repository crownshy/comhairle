import { describe, it, expect } from 'vitest';
import { countWords, estimateMinutes, learnPageWords } from './stepDuration';

describe('countWords', () => {
	it('counts the text nodes of ProseMirror JSON', () => {
		const doc = JSON.stringify({
			type: 'doc',
			content: [
				{ type: 'paragraph', content: [{ type: 'text', text: 'two words' }] },
				{ type: 'paragraph', content: [{ type: 'text', text: 'and three more' }] }
			]
		});
		expect(countWords(doc)).toBe(5);
	});

	it('does not count markdown syntax as words', () => {
		expect(countWords('## A heading\n\n- one\n- two')).toBe(4);
	});

	it('keeps a hyphenated word whole', () => {
		expect(countWords('well-being matters')).toBe(2);
	});

	it('counts nothing in empty content', () => {
		expect(countWords('')).toBe(0);
		expect(countWords(null)).toBe(0);
	});
});

describe('learnPageWords', () => {
	it('prefers the reader language and falls back to the first translation', () => {
		const pages = [
			[
				{ lang: 'en', content: 'one two' },
				{ lang: 'gd', content: 'a h-aon' }
			],
			[{ lang: 'gd', content: 'aon dha tri ceithir' }]
		];
		expect(learnPageWords(pages, 'en')).toEqual([2, 4]);
	});

	it('returns nothing for a config with no pages', () => {
		expect(learnPageWords(undefined, 'en')).toEqual([]);
	});
});

describe('estimateMinutes', () => {
	it('reads a learn step at reading pace', () => {
		expect(estimateMinutes({ type: 'learn', page_words: [200, 200] })).toBe(2);
	});

	it('never quotes zero minutes for a step that exists', () => {
		expect(estimateMinutes({ type: 'learn', page_words: [4] })).toBe(1);
	});

	it('scales a poll with the votes it asks for', () => {
		const few = estimateMinutes({ type: 'polis', required_votes: 5 });
		const many = estimateMinutes({ type: 'polis', required_votes: 30 });
		expect(few).toBe(3);
		expect(many).toBeGreaterThan(few!);
	});

	it('counts every follow-up round of a thinking space', () => {
		expect(
			estimateMinutes({
				type: 'thinkingspace',
				root_question_count: 2,
				follow_up_rounds_count: 2
			})
		).toBe(9);
	});

	it('scores each proposal against each question', () => {
		expect(
			estimateMinutes({ type: 'prioritization', question_count: 3, required_reviews: 2 })
		).toBe(4);
	});

	it('watches the recordings a stories step asks for, then adds one of your own', () => {
		expect(estimateMinutes({ type: 'stories', to_see: 3 })).toBe(8);
	});

	it('falls back to the tool default when the config says nothing about length', () => {
		expect(estimateMinutes({ type: 'heyform' })).toBe(9);
		expect(estimateMinutes({ type: 'elicitationbot' })).toBe(10);
		expect(estimateMinutes({ type: 'learn', page_words: [] })).toBe(10);
		expect(estimateMinutes({ type: 'polis', required_votes: null })).toBe(12);
	});

	it('has nothing to say about an unknown or missing tool', () => {
		expect(estimateMinutes({ type: 'nonsense' })).toBeNull();
		expect(estimateMinutes(null)).toBeNull();
	});
});
