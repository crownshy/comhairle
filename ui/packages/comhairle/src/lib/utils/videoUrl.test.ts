import { describe, it, expect } from 'vitest';
import { isVideoFileUrl, withFirstFrameThumbnail, withoutFirstFrameThumbnail } from './videoUrl';

describe('isVideoFileUrl', () => {
	it('accepts direct video file links', () => {
		expect(isVideoFileUrl('https://media.example.com/clip.mp4')).toBe(true);
		expect(isVideoFileUrl('https://media.example.com/CLIP.MOV?signature=abc')).toBe(true);
	});

	it('rejects player pages and junk', () => {
		expect(isVideoFileUrl('https://www.youtube.com/embed/abc123')).toBe(false);
		expect(isVideoFileUrl('https://player.vimeo.com/video/123')).toBe(false);
		expect(isVideoFileUrl('not a url')).toBe(false);
		expect(isVideoFileUrl(null)).toBe(false);
	});
});

describe('first frame thumbnail fragment', () => {
	it('adds and removes the fragment', () => {
		const withFragment = withFirstFrameThumbnail('https://media.example.com/clip.mp4');
		expect(withFragment).toBe('https://media.example.com/clip.mp4#t=0.001');
		expect(withoutFirstFrameThumbnail(withFragment)).toBe('https://media.example.com/clip.mp4');
	});

	it('leaves a URL that already has a fragment alone', () => {
		expect(withFirstFrameThumbnail('https://media.example.com/clip.mp4#t=5')).toBe(
			'https://media.example.com/clip.mp4#t=5'
		);
	});
});
