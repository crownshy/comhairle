import { describe, it, expect } from 'vitest';
import { skeletonBlocks } from './skeletonBlocks';

const text = (value: string) => [{ type: 'text', text: value }];
const paragraph = (value: string) => ({ type: 'paragraph', content: text(value) });
const doc = (...content: object[]) => JSON.stringify({ type: 'doc', content });

describe('skeletonBlocks', () => {
	it('outlines a heading, a video, a quote and its attribution', () => {
		const content = doc(
			{
				type: 'heading',
				attrs: { level: 1 },
				content: text('An introduction to the council and how this conversation works')
			},
			{ type: 'iframe', attrs: { src: 'https://example.com/intro.mp4' } },
			{
				type: 'blockquote',
				content: [
					paragraph(
						'We are glad you are here, and we want to build this council together with the people it serves.'
					)
				]
			},
			paragraph('- A member of the organising team')
		);

		expect(skeletonBlocks(content)).toEqual([
			{ kind: 'heading', level: 1, lines: 2 },
			{ kind: 'media', ratio: 'video' },
			{ kind: 'quote', lines: 2 },
			{ kind: 'text', lines: 1 }
		]);
	});

	it('counts list items and folds h4 to h6 into the h3 shape', () => {
		const item = (value: string) => ({ type: 'listItem', content: [paragraph(value)] });
		const content = doc(
			{ type: 'heading', attrs: { level: 5 }, content: text('Small heading') },
			{ type: 'bulletList', content: [item('one'), item('two'), item('three')] }
		);

		expect(skeletonBlocks(content)).toEqual([
			{ kind: 'heading', level: 3, lines: 1 },
			{ kind: 'list', items: 3 }
		]);
	});

	it('skips empty paragraphs', () => {
		const content = doc(paragraph('Hello'), { type: 'paragraph' }, paragraph('World'));

		expect(skeletonBlocks(content)).toEqual([
			{ kind: 'text', lines: 1 },
			{ kind: 'text', lines: 1 }
		]);
	});

	it('stops after twelve blocks', () => {
		const content = doc(
			...Array.from({ length: 20 }, (_, index) => paragraph(`Line ${index}`))
		);

		expect(skeletonBlocks(content)).toHaveLength(12);
	});

	it('gives null for markdown and for a page with nothing to outline', () => {
		expect(skeletonBlocks('# A markdown page')).toBeNull();
		expect(skeletonBlocks(doc({ type: 'paragraph' }))).toBeNull();
		expect(skeletonBlocks(undefined)).toBeNull();
	});
});
