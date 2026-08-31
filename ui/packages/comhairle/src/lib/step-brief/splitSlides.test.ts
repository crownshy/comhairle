import { describe, it, expect } from 'vitest';
import { splitSlides, firstImageSrc, withoutFirstImage } from './splitSlides';

function doc(...nodes: unknown[]) {
	return JSON.stringify({ type: 'doc', content: nodes });
}

const rule = { type: 'horizontalRule' };
const para = (text: string) => ({
	type: 'paragraph',
	content: [{ type: 'text', text }]
});
const image = (src: string) => ({ type: 'image', attrs: { src } });

describe('splitSlides, ProseMirror content', () => {
	it('returns one slide when there is no rule', () => {
		expect(splitSlides(doc(para('one'), para('two')))).toHaveLength(1);
	});

	it('splits at a rule and drops the rule itself', () => {
		const slides = splitSlides(doc(para('one'), rule, para('two')));
		expect(slides).toHaveLength(2);
		expect(slides.every((slide) => !slide.includes('horizontalRule'))).toBe(true);
	});

	it('keeps each slide a valid doc with only its own nodes', () => {
		const slides = splitSlides(doc(para('one'), rule, para('two')));
		const first = JSON.parse(slides[0]);
		expect(first.type).toBe('doc');
		expect(first.content).toHaveLength(1);
		expect(first.content[0].content[0].text).toBe('one');
	});

	it('ignores consecutive rules rather than emitting an empty slide', () => {
		expect(splitSlides(doc(para('one'), rule, rule, para('two')))).toHaveLength(2);
	});

	it('ignores leading and trailing rules', () => {
		expect(splitSlides(doc(rule, para('only'), rule))).toHaveLength(1);
	});

	it('returns no slides for an empty document', () => {
		expect(splitSlides(doc())).toEqual([]);
	});

	it('returns no slides for a document of nothing but rules', () => {
		expect(splitSlides(doc(rule, rule))).toEqual([]);
	});

	it('only splits at top level, not inside a nested node', () => {
		const nested = { type: 'blockquote', content: [para('quoted'), rule, para('more')] };
		expect(splitSlides(doc(nested))).toHaveLength(1);
	});
});

describe('splitSlides, Markdown and plain text', () => {
	it('splits on a thematic break', () => {
		expect(splitSlides('one\n\n---\n\ntwo')).toEqual(['one', 'two']);
	});

	it('accepts asterisk and underscore breaks', () => {
		expect(splitSlides('one\n\n***\n\ntwo')).toHaveLength(2);
		expect(splitSlides('one\n\n___\n\ntwo')).toHaveLength(2);
	});

	it('does not split a setext heading, where --- underlines text', () => {
		expect(splitSlides('Heading\n---\nbody')).toHaveLength(1);
	});

	it('does not split on a line that merely starts with dashes', () => {
		expect(splitSlides('one\n\n--- not a break\n\ntwo')).toHaveLength(1);
	});

	it('drops empty runs between consecutive breaks', () => {
		expect(splitSlides('one\n\n---\n\n---\n\ntwo')).toEqual(['one', 'two']);
	});

	it('returns no slides for empty or whitespace-only input', () => {
		expect(splitSlides('')).toEqual([]);
		expect(splitSlides('   \n  ')).toEqual([]);
		expect(splitSlides(null)).toEqual([]);
		expect(splitSlides(undefined)).toEqual([]);
	});

	it('treats malformed JSON as markdown rather than throwing', () => {
		expect(splitSlides('{ not json')).toEqual(['{ not json']);
	});
});

describe('firstImageSrc', () => {
	it('finds a top-level image', () => {
		expect(firstImageSrc(doc(image('/a.png'), para('after')))).toBe('/a.png');
	});

	it('finds an image nested inside another node', () => {
		const wrapper = { type: 'blockquote', content: [image('/nested.png')] };
		expect(firstImageSrc(doc(para('before'), wrapper))).toBe('/nested.png');
	});

	it('returns the first image when there are several', () => {
		expect(firstImageSrc(doc(image('/first.png'), image('/second.png')))).toBe('/first.png');
	});

	it('returns null when there is no image', () => {
		expect(firstImageSrc(doc(para('text only')))).toBeNull();
	});

	it('finds a markdown image', () => {
		expect(firstImageSrc('text\n\n![alt](/md.png)')).toBe('/md.png');
	});

	it('returns null for empty content', () => {
		expect(firstImageSrc('')).toBeNull();
	});
});

describe('withoutFirstImage', () => {
	it('removes a top-level image and keeps everything else', () => {
		const result = JSON.parse(withoutFirstImage(doc(image('/a.png'), para('after'))));
		expect(result.content).toHaveLength(1);
		expect(result.content[0].content[0].text).toBe('after');
	});

	it('removes only the first of several images', () => {
		const result = JSON.parse(withoutFirstImage(doc(image('/a.png'), image('/b.png'))));
		expect(result.content).toHaveLength(1);
		expect(result.content[0].attrs.src).toBe('/b.png');
	});

	it('drops the empty paragraph an image was alone in', () => {
		const wrapped = { type: 'paragraph', content: [image('/a.png')] };
		const result = JSON.parse(withoutFirstImage(doc(wrapped, para('after'))));
		expect(result.content).toHaveLength(1);
		expect(result.content[0].content[0].text).toBe('after');
	});

	it('keeps a paragraph that had text beside the image', () => {
		const mixed = {
			type: 'paragraph',
			content: [image('/a.png'), { type: 'text', text: 'caption' }]
		};
		const result = JSON.parse(withoutFirstImage(doc(mixed)));
		expect(result.content).toHaveLength(1);
		expect(result.content[0].content).toHaveLength(1);
	});

	it('leaves content with no image unchanged', () => {
		const source = doc(para('text only'));
		expect(JSON.parse(withoutFirstImage(source))).toEqual(JSON.parse(source));
	});

	it('does not mutate its input', () => {
		const source = doc(image('/a.png'), para('after'));
		withoutFirstImage(source);
		expect(JSON.parse(source).content).toHaveLength(2);
	});

	it('removes a markdown image', () => {
		expect(withoutFirstImage('![alt](/md.png)\n\ntext')).toBe('text');
	});

	it('returns an empty string for empty content', () => {
		expect(withoutFirstImage('')).toBe('');
	});
});
