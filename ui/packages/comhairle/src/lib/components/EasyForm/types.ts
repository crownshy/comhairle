export type InputAttr = {
	name: string;
	required?: boolean;
	minlength?: number;
	maxlength?: number;
	min?: number;
	max?: number;
	pattern?: string; // Regular expression
};

export type FileAttr = {
	name: string;
	accept: string;
	required?: boolean;
	maxSize?: number; // In bytes
};

export type Schema = {
	[name: string]: InputAttr | FileAttr;
};
