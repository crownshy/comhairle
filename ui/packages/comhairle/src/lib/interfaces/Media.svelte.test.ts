import { describe, it, expect } from 'vitest';
import Media from './Media';

describe('Media interface', () => {
	it('Should get the extension', () => {
		expect(Media.getExtension('photo.jpg')).toEqual('.jpg');
		expect(Media.getExtension('video.webm')).toEqual('.webm');
		expect(Media.getExtension('audio.mp3')).toEqual('.mp3');
	});

	it('Should get the filename', () => {
		expect(Media.getFilename('photo.jpg')).toEqual('photo');
		expect(Media.getFilename('video.webm')).toEqual('video');
		expect(Media.getFilename('audio.mp3')).toEqual('audio');
	});

	it('Should format bytes correctly', () => {
		expect(Media.formatBytes(2_000_012, 'MB')).toEqual('2MB');
		expect(Media.formatBytes(1_312_424)).toEqual('1.31MB');
		expect(Media.formatBytes(5_712)).toEqual('5.71KB');
	});
});
