import { describe, it, expect } from 'vitest';
import { countWords, estimateMinutes, learnPageLengths } from './stepDuration';

const doc = (...content: unknown[]) => JSON.stringify({ type: 'doc', content });
const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const page = (words: number, images = 0, videos = 0) => ({ words, images, videos });

describe('countWords', () => {
	it('counts the text nodes of ProseMirror JSON', () => {
		expect(countWords(doc(paragraph('two words'), paragraph('and three more')))).toBe(5);
	});

	it('does not count markdown syntax as words', () => {
		expect(countWords('## A heading\n\n- one\n- two')).toBe(4);
	});

	it('does not count a link target or an embed tag as words', () => {
		expect(countWords('[the guide](https://example.com/guide)')).toBe(2);
		expect(countWords('<iframe src="https://youtube.com/embed/x"></iframe> watch this')).toBe(
			2
		);
	});

	it('keeps a hyphenated word whole', () => {
		expect(countWords('well-being matters')).toBe(2);
	});

	it('counts nothing in empty content', () => {
		expect(countWords('')).toBe(0);
		expect(countWords(null)).toBe(0);
	});
});

describe('learnPageLengths', () => {
	it('prefers the reader language and falls back to the first translation', () => {
		const pages = [
			[
				{ lang: 'en', content: 'one two' },
				{ lang: 'gd', content: 'a h-aon' }
			],
			[{ lang: 'gd', content: 'aon dha tri ceithir' }]
		];
		expect(learnPageLengths(pages, 'en').map((length) => length.words)).toEqual([2, 4]);
	});

	it('counts the images and videos in ProseMirror JSON', () => {
		const content = doc(
			paragraph('watch this first'),
			{ type: 'iframe', attrs: { src: 'https://youtube.com/embed/x' } },
			{ type: 'image', attrs: { src: 'https://example.com/chart.png' } }
		);
		expect(learnPageLengths([[{ lang: 'en', content }]], 'en')).toEqual([page(3, 1, 1)]);
	});

	it('counts the images and videos in legacy markdown', () => {
		const content = '![chart](chart.png)\n\n<iframe src="https://vimeo.com/1"></iframe>';
		expect(learnPageLengths([[{ lang: 'en', content }]], 'en')).toEqual([page(1, 1, 1)]);
	});

	it('returns nothing for a config with no pages', () => {
		expect(learnPageLengths(undefined, 'en')).toEqual([]);
	});
});

describe('estimateMinutes', () => {
	it('reads a learn page at reading pace, plus a moment to take the page in', () => {
		expect(estimateMinutes({ type: 'learn', learn_pages: [page(400)] })).toBe(3);
	});

	it('adds the videos and images on a learn page', () => {
		expect(estimateMinutes({ type: 'learn', learn_pages: [page(0, 1, 1)] })).toBe(4);
	});

	it('does not quote a few short pages and a video as a one-minute read', () => {
		const learn_pages = [page(50, 0, 1), page(50), page(50), page(50), page(50)];
		expect(estimateMinutes({ type: 'learn', learn_pages })).toBe(6);
	});

	it('never quotes zero minutes for a step that exists', () => {
		expect(estimateMinutes({ type: 'learn', learn_pages: [page(4)] })).toBe(1);
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
		expect(estimateMinutes({ type: 'learn', learn_pages: [] })).toBe(10);
		expect(estimateMinutes({ type: 'learn', learn_pages: [page(0), page(0)] })).toBe(10);
		expect(estimateMinutes({ type: 'polis', required_votes: null })).toBe(12);
	});

	it('has nothing to say about an unknown or missing tool', () => {
		expect(estimateMinutes({ type: 'nonsense' })).toBeNull();
		expect(estimateMinutes(null)).toBeNull();
	});
});
