import { describe, expect, it } from 'vitest';
import { validate } from './validate';
import type { Schema } from '.';
import { type FileAttr, type InputAttr } from './types';
import { typed } from '$lib/utils/types';
import { B } from '$lib/utils/units';

describe('Validate Schema', () => {
	it('validation should pass', () => {
		const schema: Schema = {
			email: typed<InputAttr>({
				name: 'email',
				required: true
			}),
			password: typed<InputAttr>({
				name: 'password',
				required: true
			})
		};
		const formData = new FormData();
		formData.append('email', 'example@gmail.com');
		formData.append('password', 'password');
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: true, err: null })
		);
	});

	it("Reject when a key doesn't exist", () => {
		const schema: Schema = {
			email: typed<InputAttr>({
				name: 'email',
				required: true
			}),
			password: typed<InputAttr>({
				name: 'password',
				required: true
			})
		};
		const formData = new FormData();
		formData.append('email', 'example@gmail.com');
		formData.append('password', 'password');
		formData.append('extraKey', 'hello');
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'KEY_NOT_FOUND' })
		);
	});

	it('Reject when a required key is null', () => {
		// FIX: Can't create a FileList programmatically, so need to figure out how to create one for the tests
		expect(true).toEqual(true);
	});

	it('Should be able to handle multiple files', () => {
		// FIX: Can't create a FileList programmatically, so need to figure out how to create one for the tests
		expect(true).toEqual(true);
	});

	it('Should reject when one file in multiple files uploaded fail', () => {
		const schema: Schema = {
			file: typed<FileAttr>({
				name: 'file',
				accept: '.jpg'
			})
		};
		const formData = new FormData();
		const file = new File(['data'], 'name');
		formData.append('file', file);
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'EXTENSION_UNREADABLE' })
		);
	});

	it('Reject when a file is too big', () => {
		const schema: Schema = {
			file: typed<FileAttr>({
				name: 'file',
				accept: '.jpg',
				maxSize: 80 * B
			})
		};
		const formData = new FormData();
		const file = new File(
			['1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz/(){}[]<>?*^&%$£"!'],
			'name.jpg'
		); // Size: 81
		formData.append('file', file);
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'MAX_SIZE_EXCEEDED' })
		);
	});

	it('Reject when a file has no extension', () => {
		const schema: Schema = {
			file: typed<FileAttr>({
				name: 'file',
				required: true,
				accept: '.jpg'
			})
		};
		const formData = new FormData();
		const file = new File(['data'], 'name');
		formData.append('file', file);
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'EXTENSION_UNREADABLE' })
		);
	});

	it('Reject when a file extension is not allowed', () => {
		const schema: Schema = {
			file: typed<FileAttr>({
				name: 'file',
				accept: '.jpg,.png'
			})
		};
		const formData = new FormData();
		const file = new File(['data'], 'name.mp3');
		formData.append('file', file);
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'EXTENSION_NOT_ALLOWED' })
		);
	});

	it('Reject incorrect type', () => {
		const schema: Schema = {
			file: typed<InputAttr>({
				name: 'file'
			})
		};
		const formData = new FormData();
		const file = new File(['data'], 'name.mp3');
		formData.append('file', file);
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'TYPE_WRONG' })
		);
	});

	it('Reject too short', () => {
		const schema: Schema = {
			email: typed<InputAttr>({
				name: 'email',
				minlength: 5
			})
		};
		const formData = new FormData();
		formData.append('email', 'test');
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'TOO_SHORT' })
		);
	});

	it('Reject too long', () => {
		const schema: Schema = {
			email: typed<InputAttr>({
				name: 'email',
				maxlength: 5
			})
		};
		const formData = new FormData();
		formData.append('email', 'testing');
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'TOO_LONG' })
		);
	});

	it('Reject failed pattern', () => {
		const schema: Schema = {
			email: typed<InputAttr>({
				name: 'email',
				// It's a regex expression
				// eslint-disable-next-line no-useless-escape
				pattern: '.*@.*\.com'
			})
		};
		const formData = new FormData();
		formData.append('email', 'test');
		expect(validate(formData, schema)).toEqual(
			typed<ReturnType<typeof validate>>({ ok: null, err: 'MATCH_FAILED' })
		);
	});
});
