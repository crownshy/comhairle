import type { Schema } from '$lib/components/EasyForm';

const MediaSchema: Schema = {
	media: {
		name: 'media',
		required: true
	},
	name: {
		name: 'name',
		required: true
	},
	alt: {
		name: 'alt',
		required: true,
		min: 5
	}
} as const;

export default MediaSchema;
