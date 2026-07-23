import type { HTMLInputAttributes } from 'svelte/elements';

export type ValidationAttributes = { name: string } & Pick<
	HTMLInputAttributes,
	'required' | 'min' | 'max' | 'minlength' | 'maxlength' | 'pattern' | 'accept'
>;

export type Input = {
	[K in keyof ValidationAttributes]: ValidationAttributes[K];
};

export type Schema = {
	[name: string]: Input;
};
