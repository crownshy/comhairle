import Media from '$lib/interfaces/Media';
import type { Result } from '$lib/utils/errorHandling';
import type { Schema } from '.';
import type { FileAttr, InputAttr } from './types.svelte';

type ValidationError =
	| 'KEY_NOT_FOUND'
	| 'REQUIRED_IS_NULL'
	| 'EXTENSION_UNREADABLE'
	| 'EXTENSION_NOT_ALLOWED'
	| 'TOO_SHORT'
	| 'TOO_LONG'
	| 'MATCH_FAILED'
	| 'TYPE_WRONG';

function isFileAttr(attr: FileAttr | InputAttr): attr is FileAttr {
	return 'accept' in attr;
}

function isFile(value: FormDataEntryValue): value is File {
	return value instanceof File;
}

export function validate(form: FormData, schema: Schema): Result<'ok', true, ValidationError> {
	for (const name of form.keys()) {
		if (!(name in schema)) {
			return { ok: null, err: 'KEY_NOT_FOUND' };
		}
		const value = form.get(name);
		const input = schema[name];

		if (input.required && value === null) {
			return { ok: null, err: 'REQUIRED_IS_NULL' };
		}

		if (value === null) {
			// No value, and not required so no need to continue with checks
			return { ok: true, err: null };
		}

		if (isFileAttr(input)) {
			if (!isFile(value)) {
				return { ok: null, err: 'TYPE_WRONG' };
			}
			const extension = Media.getExtension(value.name);
			if (!extension) {
				return { ok: null, err: 'EXTENSION_UNREADABLE' };
			}

			const extensions = input.accept.split(',');
			if (!extensions.includes(extension)) {
				return { ok: null, err: 'EXTENSION_NOT_ALLOWED' };
			}
		} else {
			if (isFile(value)) {
				return { ok: null, err: 'TYPE_WRONG' };
			}

			// Length for Inputs
			if (input.minlength && value.length < input.minlength) {
				return { ok: null, err: 'TOO_SHORT' };
			}

			// Length for Inputs
			if (input.maxlength && value.length > input.maxlength) {
				return { ok: null, err: 'TOO_LONG' };
			}

			// Length for TextArea
			if (input.min && value.length < Number(input.min)) {
				return { ok: null, err: 'TOO_SHORT' };
			}

			// Length for TextArea
			if (input.max && value.length > Number(input.max)) {
				return { ok: null, err: 'TOO_LONG' };
			}

			if (input.pattern && !new RegExp(input.pattern).test(value)) {
				return { ok: null, err: 'MATCH_FAILED' };
			}
		}
	}

	return { ok: true, err: null };
}
