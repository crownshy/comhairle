import { tryFetch, type FetchErr, type Result } from '$lib/utils/errorHandling';

type FileErr = { id: 'MAX_SIZE_EXCEEDED'; message: string };
type UploadReturn = Result<'ok', Response, FileErr | FetchErr>;

type Opts = {
	maxSizeMB?: number; // in MB
	fetchRef?: typeof fetch; // If used on the backend and we need to use the alternate fetch
};

class Media {
	async #upload(to: string, files: File[], opts?: Opts): Promise<UploadReturn> {
		const formData = new FormData();

		for (const file of files) {
			if (opts?.maxSizeMB && file.size > opts.maxSizeMB * 1024 * 1024) {
				return {
					ok: null,
					err: {
						id: 'MAX_SIZE_EXCEEDED',
						message: `${file.name} exceeds max size ${opts.maxSizeMB}MB`
					}
				};
			}

			formData.append('file', file, file.name);
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
	static sanitiseMulti(files: File | FileList): File[] {
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

	async upload(to: string, files: File[], opts?: Opts): Promise<UploadReturn> {
		return this.#upload(to, files, opts);
	}
}

export default Media;
