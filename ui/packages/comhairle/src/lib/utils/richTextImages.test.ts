import { describe, expect, it } from 'vitest';
import { imageSourcesIn } from './richTextImages';

const doc = (content: unknown[]) => JSON.stringify({ type: 'doc', content });

describe('imageSourcesIn', () => {
	it('returns nothing for empty content', () => {
		expect(imageSourcesIn('')).toEqual([]);
		expect(imageSourcesIn(null)).toEqual([]);
	});

	it('finds images nested anywhere in a ProseMirror document', () => {
		const content = doc([
			{ type: 'paragraph', content: [{ type: 'text', text: 'intro' }] },
			{ type: 'image', attrs: { src: '/a.png' } },
			{
				type: 'bulletList',
				content: [
					{
						type: 'listItem',
						content: [
							{
								type: 'paragraph',
								content: [{ type: 'image', attrs: { src: '/b.png' } }]
							}
						]
					}
				]
			}
		]);

		expect(imageSourcesIn(content)).toEqual(['/a.png', '/b.png']);
	});

	it('de-duplicates repeated sources', () => {
		const content = doc([
			{ type: 'image', attrs: { src: '/a.png' } },
			{ type: 'image', attrs: { src: '/a.png' } }
		]);

		expect(imageSourcesIn(content)).toEqual(['/a.png']);
	});

	it('finds markdown images', () => {
		expect(imageSourcesIn('# Title\n\n![alt](/a.png)\n\ntext ![](/b.png "t")')).toEqual([
			'/a.png',
			'/b.png'
		]);
	});
});
