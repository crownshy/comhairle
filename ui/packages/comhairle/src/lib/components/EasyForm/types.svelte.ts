import type { HTMLInputAttributes } from 'svelte/elements';

type DerivedInputValidationAttributes = { name: string } & Pick<
	HTMLInputAttributes,
	'required' | 'min' | 'max' | 'minlength' | 'maxlength' | 'pattern'
>;

type DerivedFileInputValidationAttributes = Pick<HTMLInputAttributes, 'name' | 'required'> & {
	accept: string;
};

// Mapped output, gives nicer hover for TS
export type InputAttr = {
	[K in keyof DerivedInputValidationAttributes]: DerivedInputValidationAttributes[K];
};

// Mapped output, gives nicer hover for TS
export type FileAttr = {
	[K in keyof DerivedFileInputValidationAttributes]: DerivedFileInputValidationAttributes[K];
};

export type Schema = {
	[name: string]: InputAttr | FileAttr;
};
