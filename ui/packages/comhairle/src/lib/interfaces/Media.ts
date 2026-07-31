import { tryFetch, type FetchErr, type Result } from '$lib/utils/errorHandling';

type FileErr = { id: 'MAX_SIZE_EXCEEDED'; message: string };
type UploadReturn = Result<'ok', Response, FileErr | FetchErr>;

type Opts = {
	maxSize?: number; // in bytes
	fetchRef?: typeof fetch; // If used on the backend and we need to use the alternate fetch
};

class Media {
	async upload(to: string, formData: FormData, opts?: Opts): Promise<UploadReturn> {
		// for (const file of files) {
		// 	if (opts?.maxSize && file.size > opts.maxSize) {
		// 		return {
		// 			ok: null,
		// 			err: {
		// 				id: 'MAX_SIZE_EXCEEDED',
		// 				message: `${file.name} exceeds max size ${Media.formatBytes(opts.maxSize)}`
		// 			}
		// 		};
		// 	}
		//
		// 	formData.append('file', file, file.name);
		// }

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
			opts?.fetchRef ?? fetch
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

	static #isFileList(files: File | FileList): files is FileList {
		//@ts-expect-error exists on FileList but not File, this is expected to differentiate between them
		return !!files.item;
	}

	/**
	 * When you have an input with multiple selections possible you can either get a File or a FileList.
	 * They're very different types that are difficult to handle and FileList has limited TS support.
	 * This function is to sanitise these outputs so that it will return a File[] which is predictable and works well with TS
	 */
	static normalise(files: File | FileList): File[] {
		const filesArray: File[] = [];
		if (this.#isFileList(files)) {
			for (const f of files) {
				filesArray.push(f);
			}
		} else {
			filesArray.push(files as File);
		}
		return filesArray;
	}

	static getExtension(filename: string): string | undefined {
		const index = filename.lastIndexOf('.');
		if (index < 0) return undefined;
		return filename.slice(index);
	}

	static getFilename(filename: string): string {
		const index = filename.lastIndexOf('.');
		if (index < 0) return filename;
		return filename.slice(0, index);
	}

	static formatBytes(bytes: number, size?: 'B' | 'KB' | 'MB' | 'GB'): string {
		const denominations = ['B', 'KB', 'MB', 'GB'];
		const factor = 1_000; // 1_024 for kibibytes

		const calcBytes = (bytes: number, index: number): number =>
			Math.round((bytes / Math.pow(factor, index)) * 100) / 100;

		if (size) {
			const index = denominations.indexOf(size);
			return `${calcBytes(bytes, index)}${denominations[index]}`;
		}

		for (let i = 0; i < denominations.length; i++) {
			const value = calcBytes(bytes, i);

			if (value <= 0.7) {
				const previousSafeIndex = Math.max(i - 1, 0);
				return `${calcBytes(bytes, previousSafeIndex)}${denominations[previousSafeIndex]}`;
			}

			if (i === denominations.length - 1) {
				return `${calcBytes(bytes, i)}${denominations[i]}`;
			}
		}

		return `${bytes}B`;
	}
}

export default Media;
