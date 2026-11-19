import { describe, it, expect } from 'vitest';
import { resolvePageMeta, DESCRIPTION_MAX_LENGTH } from './pageMeta';

const DEFAULTS = {
	title: 'Comhairle',
	description: 'Have your say.',
	image: '/comhairle-social.png'
};

const PAGE_URL = new URL('https://stage.comhairle.scot/conversations/climate?ref=abc#top');

describe('resolvePageMeta', () => {
	it('falls back to the site defaults when the page sets nothing', () => {
		expect(resolvePageMeta(undefined, DEFAULTS, PAGE_URL)).toEqual({
			title: 'Comhairle',
			description: 'Have your say.',
			url: 'https://stage.comhairle.scot/conversations/climate',
			image: 'https://stage.comhairle.scot/comhairle-social.png'
		});
	});

	it('uses what the page sets', () => {
		const meta = {
			title: 'Climate migration and you',
			description: 'Tell us how climate change affects where you live.',
			image: 'https://media.example.com/climate.jpg'
		};
		expect(resolvePageMeta(meta, DEFAULTS, PAGE_URL)).toMatchObject(meta);
	});

	it('treats blank values as missing', () => {
		const resolved = resolvePageMeta(
			{ title: ' ', description: '', image: '' },
			DEFAULTS,
			PAGE_URL
		);
		expect(resolved.title).toBe('Comhairle');
		expect(resolved.description).toBe('Have your say.');
		expect(resolved.image).toBe('https://stage.comhairle.scot/comhairle-social.png');
	});

	it('makes a relative image absolute', () => {
		const resolved = resolvePageMeta({ image: '/media/cover.png' }, DEFAULTS, PAGE_URL);
		expect(resolved.image).toBe('https://stage.comhairle.scot/media/cover.png');
	});

	it('encodes spaces in an uploaded image name', () => {
		const resolved = resolvePageMeta(
			{ image: 'https://media.example.com/images/QE Image 1.png' },
			DEFAULTS,
			PAGE_URL
		);
		expect(resolved.image).toBe('https://media.example.com/images/QE%20Image%201.png');
	});

	it('trims a long description to one line', () => {
		const description = 'word\n'.repeat(100);
		const resolved = resolvePageMeta({ description }, DEFAULTS, PAGE_URL);
		expect(resolved.description.length).toBe(DESCRIPTION_MAX_LENGTH);
		expect(resolved.description).not.toContain('\n');
		expect(resolved.description.endsWith('…')).toBe(true);
	});
});
