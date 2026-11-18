import type { FileAttr, InputAttr, Schema } from './types';
import Input from './Input.svelte';
import { Select, SelectList, type Option } from './Select';
import Form, { type OnSubmit } from './Form.svelte';
import Submit from './Submit.svelte';
import { validate, type ValidationErr } from './validate';

export {
	type OnSubmit,
	type FileAttr,
	type InputAttr,
	type Option,
	type Schema,
	type ValidationErr,
	validate,
	Input,
	Select,
	SelectList,
	Form,
	Submit
};
