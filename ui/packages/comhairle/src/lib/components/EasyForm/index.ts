import type { FileAttr, InputAttr, Schema } from './types';
import Input from './Input.svelte';
import Form, { type OnSubmit } from './Form.svelte';
import Submit from './Submit.svelte';
import { validate, type ValidationErr } from './validate';

export {
	type OnSubmit,
	type InputAttr,
	type FileAttr,
	type Schema,
	type ValidationErr,
	validate,
	Input,
	Form,
	Submit
};
