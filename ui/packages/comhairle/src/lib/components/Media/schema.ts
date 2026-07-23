import type { FileAttr, InputAttr } from '../EasyForm/types.svelte';

const media: FileAttr = {
	name: 'media',
	required: true,
	accept: '.png,.jpg'
};

const name: InputAttr = {
	name: 'name',
	required: true
};

const alt: InputAttr = {
	name: 'alt',
	required: true
};

export const MediaSchema = {
	media,
	name,
	alt
} as const;

export default MediaSchema;
