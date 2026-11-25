import type { ValidationErr } from '$lib/components/EasyForm';
import { tryCatchAsync, tryFetch, type FetchErr, type Result } from '$lib/utils/errorHandling';
import { MediaContentType, MediaDto } from '@crownshy/api-client/api';
import { apiClient } from '@crownshy/api-client/client';

export const UploaderProgress = {
	Idle: 1,
	Creating: 2,
	Uploading: 3,
	Completed: 4
} as const;

type UploaderProgressValues = (typeof UploaderProgress)[keyof typeof UploaderProgress];

type UploaderData = {
	media: File;
	// User defined name of the file
	name: string;
	// Alt text for the media
	alt: string;
};

type Opts = {
	maxSize?: number; // in bytes
};

class Uploader {
	#stepProgress = $state<UploaderProgressValues>(UploaderProgress.Idle);
	#error = $state<string>();
	#currentBytes = $state<number>(0);
	#totalBytes = $state<number>(0);

	#uploadFile(url: string, file: File) {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			xhr.open('PUT', url, true);
			xhr.setRequestHeader('Content-Type', file.type);
			xhr.setRequestHeader('x-amz-acl', 'public-read');
			xhr.upload.onprogress = (e) => {
				if (e.lengthComputable) {
					this.#currentBytes = e.loaded;
					this.#totalBytes = e.total;
				}
			};
			xhr.onloadend = () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					this.#currentBytes = this.#totalBytes;
					resolve(true);
				} else {
					reject(`bad HTTP status ${xhr.status}`);
				}
			};
			xhr.onerror = () => reject('network error');
			xhr.send(file);
		});
	}

	async upload(data: UploaderData, opts?: Opts): Promise<Result<MediaDto, 'FAILED'>> {
		// Errors are handled via the #error state, so we don't need to return them here
		this.reset();

		this.#stepProgress = UploaderProgress.Creating;

		if (opts?.maxSize && opts.maxSize > data.media.size) {
			this.#error = `Max file size (${formatBytes(opts.maxSize)}) exceeded`;
			return { ok: null, err: 'FAILED' };
		}

		this.#totalBytes = data.media.size;

		const createMedia = await tryCatchAsync(() =>
			apiClient.UploadMedia({
				filename: data.media.name,
				name: data.name,
				alt: data.alt,
				contentType: data.media.type,
				size: data.media.size
			})
		);

		if (createMedia.err !== null) {
			console.error(createMedia.err);
			this.#error = 'Could not create the media, please try again';
			return { ok: null, err: 'FAILED' };
		}

		this.#stepProgress = UploaderProgress.Uploading;

		const uploadMedia = await tryCatchAsync(() =>
			this.#uploadFile(createMedia.ok.presignedUploadUrl, data.media)
		);
		if (uploadMedia.err !== null) {
			this.#error = `Could not upload media: ${uploadMedia.err}`;
			return { ok: null, err: 'FAILED' };
		}

		this.#stepProgress = UploaderProgress.Completed;
		return { ok: createMedia.ok.media, err: null };
	}

	get percentage() {
		return Math.trunc((this.#currentBytes * 100) / this.#totalBytes).toString();
	}

	get step() {
		return this.#stepProgress;
	}

	get error() {
		return this.#error;
	}

	reset() {
		this.#stepProgress = UploaderProgress.Idle;
		this.#error = undefined;
		this.#currentBytes = 0;
		this.#totalBytes = 0;
	}
}

async function uploadAsForm(
	to: string,
	formData: FormData,
	opts?: Opts
): Promise<Result<Response, ValidationErr | FetchErr>> {
	if (opts?.maxSize) {
		for (const key of formData.keys()) {
			const field = formData.get(key);
			if (field instanceof File && field.size > opts.maxSize) {
				return { ok: null, err: 'MAX_SIZE_EXCEEDED' };
			}
		}
	}

	const response = await tryFetch(
		to,
		{
			method: 'POST',
			credentials: 'include',
			body: formData,
			headers: {
				enctype: 'multipart/form-data'
			}
		},
		fetch
	);

	if (response.err !== null) {
		return {
			ok: null,
			err: {
				...response.err,
				message: 'Failed to upload - ' + response.err.message
			} as FetchErr
		};
	}

	return { ok: response.ok, err: null };
}

function isFileList(files: File | FileList): files is FileList {
	//@ts-expect-error exists on FileList but not File, this is expected to differentiate between them
	return !!files.item;
}

/**
 * When you have an input with multiple selections possible you can either get a File or a FileList.
 * They're very different types that are difficult to handle and FileList has limited TS support.
 * This function is to sanitise these outputs so that it will return a File[] which is predictable and works well with TS
 */
function normalise(files: File | FileList | null): File[] {
	if (!files) {
		return [];
	}
	const filesArray: File[] = [];
	if (isFileList(files)) {
		for (const f of files) {
			filesArray.push(f);
		}
	} else {
		filesArray.push(files as File);
	}
	return filesArray;
}

function getExtension(filename: string): string | undefined {
	const index = filename.lastIndexOf('.');
	if (index < 0) return undefined;
	return filename.slice(index);
}

function getFilename(filename: string): string {
	const index = filename.lastIndexOf('.');
	if (index < 0) return filename;
	return filename.slice(0, index);
}

const denominations = ['B', 'KB', 'MB', 'GB'] as const;

type FormatBytesOptions = {
	size?: (typeof denominations)[number];
	dp?: number;
};
/**
 * Format bytes into a condensed string, automatically picks the size for best readability
 * @param bytes - value in bytes
 * @param options -
 * - size?: forces a fixed size, e.g. "GB"
 * - dp?: decimal points to round to, defaults to 2
 * @example
 * formatBytes(3_000) = "3KB"
 * formatBytes(21_123_223) = "21.12MB"
 */
function formatBytes(bytes: number, options: FormatBytesOptions = { dp: 2 }): string {
	const factor = 1_000; // 1_024 for kibibytes

	const calcBytes = (bytes: number, index: number): number =>
		Math.round((bytes / Math.pow(factor, index)) * 100) / 100;

	if (options?.size) {
		const index = denominations.indexOf(options.size);
		return `${calcBytes(bytes, index)}${denominations[index]}`;
	}

	for (let i = 0; i < denominations.length; i++) {
		const value = calcBytes(bytes, i);

		if (value <= 0.7) {
			const previousSafeIndex = Math.max(i - 1, 0);
			return `${calcBytes(bytes, previousSafeIndex).toFixed(options.dp)}${denominations[previousSafeIndex]}`;
		}

		if (i === denominations.length - 1) {
			return `${calcBytes(bytes, i).toFixed(options.dp)}${denominations[i]}`;
		}
	}

	return `${bytes}B`;
}

function acceptedExtensions(type: 'all' | 'audio' | 'video' | 'image' = 'all'): string {
	const acceptedExtensions: AcceptedExtensions[] = [];

	for (const mediaContentType of MediaContentType.options) {
		// mediaContentType.match =  'audio' | 'video' | 'image'
		if (type === 'all' || mediaContentType.match(/.*(?=\/)/)?.[0] === type) {
			acceptedExtensions.push(contentTypeToAcceptedExtension(mediaContentType));
		}
	}

	return acceptedExtensions.join(',');
}

/**
 * Function to convert content-type into a file extension e.g.
 * contentTypeToExtension("image/png") = ".png"
 * contentTypeToExtension("video/webm") = ".webm"
 * contentTypeToExtension("audio/mpeg") = ".mp3"
 */
function contentTypeToAcceptedExtension(contentType: MediaContentType) {
	switch (contentType) {
		case 'image/jpeg':
			return '.jpg,.jpeg';
		case 'image/png':
			return '.png';
		case 'image/gif':
			return '.gif';
		case 'image/webp':
			return '.webp';
		case 'video/mp4':
			return '.mp4';
		case 'video/mpeg':
			return '.mpeg';
		case 'video/webm':
			return '.webm';
		case 'audio/mpeg':
			return '.mp3';
		case 'audio/mp4':
			return '.m4a';
		case 'audio/webm':
			return '.weba';
		case 'audio/wav':
			return '.wav';
		case 'audio/ogg':
			return '.oga';
	}
}

export type AcceptedExtensions = ReturnType<typeof contentTypeToAcceptedExtension>;

export default {
	uploader: Uploader,
	uploadAsForm,
	normalise,
	getExtension,
	getFilename,
	formatBytes,
	acceptedExtensions,
	contentTypeToAcceptedExtension
};
