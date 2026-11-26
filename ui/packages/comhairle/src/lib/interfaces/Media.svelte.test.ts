import { afterEach, describe, it, expect, vi } from 'vitest';
import { apiClient } from '@crownshy/api-client/client';
import type { UploadMediaResponseDto } from '@crownshy/api-client/api';
import Media, { UploaderProgress } from './Media.svelte';

vi.mock('@crownshy/api-client/client', () => ({ apiClient: { UploadMedia: vi.fn() } }));

afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});

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
		expect(Media.formatBytes(2_000_012, { size: 'MB' })).toEqual('2MB');
		expect(Media.formatBytes(1_312_424)).toEqual('1.31MB');
		expect(Media.formatBytes(5_712)).toEqual('5.71KB');
		expect(Media.formatBytes(21_357_712, { dp: 0 })).toEqual('21MB');
	});

	it('Should get extensions correctly', () => {
		expect(Media.extensionFromContentType('audio/webm')).toEqual('.weba');
		expect(Media.extensionFromContentType('image/jpeg')).toEqual('.jpg,.jpeg');
		expect(Media.extensionFromContentType('video/mp4')).toEqual('.mp4');
	});

	it('Should get the media type correctly', () => {
		expect(Media.mediaTypeFromContentType('audio/mpeg')).toEqual('audio');
		expect(Media.mediaTypeFromContentType('image/jpeg')).toEqual('image');
		expect(Media.mediaTypeFromContentType('video/mp4')).toEqual('video');
	});

	it.each(['image/png', 'video/mp4', 'audio/mpeg'] as const)(
		'uploads %s with the headers required by the presigned request',
		async (contentType) => {
			// Size is 5 because content is the string "media" which is 5 characters
			const file = new File(['media'], 'upload', { type: contentType });
			const response: UploadMediaResponseDto = {
				presignedUploadUrl: 'https://storage.example.com/upload',
				media: {
					id: 'f4de4c2b-8b5a-40db-9398-cd4accfda268',
					ownerId: '79f205b6-34e5-46ad-b214-f747c9740018',
					createdAt: '2026-01-01T00:00:00Z',
					filename: file.name,
					name: 'Uploaded media',
					alt: 'Description',
					size: 5,
					contentType,
					storageKey: 'upload',
					storeName: 'media',
					url: 'https://storage.example.com/upload'
				}
			};
			vi.mocked(apiClient.UploadMedia).mockResolvedValue(response);

			const request = {
				open: vi.fn(),
				setRequestHeader: vi.fn(),
				send: vi.fn(),
				upload: {},
				status: 200,
				onloadend: () => {}
			};
			request.send.mockImplementation(() => request.onloadend());
			vi.stubGlobal(
				'XMLHttpRequest',
				vi.fn(function () {
					return request;
				})
			);

			const uploader = new Media.uploader();
			const result = await uploader.upload({
				media: file,
				name: response.media.name,
				alt: response.media.alt
			});

			expect(apiClient.UploadMedia).toHaveBeenCalledWith({
				filename: file.name,
				name: response.media.name,
				alt: response.media.alt,
				size: 5,
				contentType
			});
			expect(request.open).toHaveBeenCalledWith('PUT', response.presignedUploadUrl, true);
			expect(request.setRequestHeader).toHaveBeenCalledWith('Content-Type', contentType);
			expect(request.setRequestHeader).toHaveBeenCalledWith('x-amz-acl', 'public-read');
			expect(request.send).toHaveBeenCalledWith(file);
			expect(result).toEqual({ ok: response.media, err: null });
			expect(uploader.step).toBe(UploaderProgress.Completed);
		}
	);
});
